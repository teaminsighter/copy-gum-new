// Global store for category customizations
import { writable, get } from 'svelte/store';
import {
  updateCategory as dbUpdateCategory,
  createCustomCategory as dbCreateCustomCategory,
  deleteCategory as dbDeleteCategory,
  getCategories as dbGetCategories
} from '../services/database';
import { broadcast, createCategoryRenamedEvent } from '../events/eventBus';

// Default category names (cannot be used for custom categories)
const DEFAULT_CATEGORY_NAMES = [
  'all', 'password', 'apikey', 'private', 'email',
  'phone', 'links', 'code', 'color', 'image', 'number', 'text'
];

/**
 * Validate category/tag name
 * @throws Error if name is invalid
 */
export function validateName(name: string, type: 'category' | 'tag' = 'category'): void {
  if (!name || typeof name !== 'string') {
    throw new Error(`${type} name is required`);
  }

  const trimmed = name.trim();

  if (trimmed.length === 0) {
    throw new Error(`${type} name cannot be empty`);
  }

  if (trimmed.length > 50) {
    throw new Error(`${type} name must be less than 50 characters`);
  }

  // Check for reserved names (only for categories)
  if (type === 'category' && DEFAULT_CATEGORY_NAMES.includes(trimmed.toLowerCase())) {
    throw new Error(`"${trimmed}" is a reserved category name`);
  }
}

interface CategoryCustomization {
  icon?: string;
  color?: string;
}

type CategoryCustomizations = Record<string, CategoryCustomization>;

export interface CustomCategory {
  id: string;
  name: string;
  icon: string;
  color?: string;
  isCustom: boolean;
}

// Store for category icon/color overrides
export const categoryCustomizations = writable<CategoryCustomizations>({});

// Store for custom user-created categories
export const customCategories = writable<CustomCategory[]>([]);

// Store for category order (array of category IDs)
// Default order matches the 12 default categories
export const categoryOrder = writable<string[]>([
  'all', 'password', 'apikey', 'private', 'email',
  'phone', 'links', 'code', 'color', 'image', 'number', 'text'
]);

// Update a category's icon (saves to database)
export async function updateCategoryIcon(categoryId: string, icon: string) {
  // Update store immediately for responsive UI
  categoryCustomizations.update(customs => ({
    ...customs,
    [categoryId]: {
      ...customs[categoryId],
      icon
    }
  }));

  // Save to database
  try {
    const dbCategories = await dbGetCategories();
    const dbCategory = dbCategories.find(c => c.name === categoryId);
    if (dbCategory && dbCategory.id) {
      await dbUpdateCategory(dbCategory.id, dbCategory.name, icon, dbCategory.color);
    }
  } catch (error) {
    console.error('Failed to save category icon to database:', error);
  }
}

// Update a category's color (saves to database)
export async function updateCategoryColor(categoryId: string, color: string) {
  // Update store immediately for responsive UI
  categoryCustomizations.update(customs => ({
    ...customs,
    [categoryId]: {
      ...customs[categoryId],
      color
    }
  }));

  // Save to database
  try {
    const dbCategories = await dbGetCategories();
    const dbCategory = dbCategories.find(c => c.name === categoryId);
    if (dbCategory && dbCategory.id) {
      await dbUpdateCategory(dbCategory.id, dbCategory.name, dbCategory.icon, color);
    }
  } catch (error) {
    console.error('Failed to save category color to database:', error);
  }
}

// Get category customization
export function getCategoryCustomization(categoryId: string, customs: CategoryCustomizations) {
  return customs[categoryId] || {};
}

// Reorder categories by moving from one index to another
export function reorderCategories(fromIndex: number, toIndex: number) {
  categoryOrder.update(order => {
    const newOrder = [...order];
    const [movedItem] = newOrder.splice(fromIndex, 1);
    newOrder.splice(toIndex, 0, movedItem);
    return newOrder;
  });
}

// Track in-flight category creations to prevent race conditions
const creatingCategories = new Set<string>();

// Create a new custom category (async - saves to database)
export async function createCustomCategory(name: string, icon: string = '📁', color?: string) {
  // Validate the name
  validateName(name, 'category');

  const trimmedName = name.trim();

  // Check if already being created (race condition prevention)
  if (creatingCategories.has(trimmedName.toLowerCase())) {
    throw new Error(`Category "${trimmedName}" is already being created`);
  }

  // Check if category already exists in store
  const existingCategories = get(customCategories);
  if (existingCategories.some(c => c.name.toLowerCase() === trimmedName.toLowerCase())) {
    throw new Error(`Category "${trimmedName}" already exists`);
  }

  // Mark as creating
  creatingCategories.add(trimmedName.toLowerCase());

  try {
    // Save to database first
    await dbCreateCustomCategory(trimmedName, icon, color);

    // Use the category name as the ID (this matches the database schema)
    const newCategory: CustomCategory = {
      id: trimmedName, // Use name as ID to match database foreign key
      name: trimmedName,
      icon,
      isCustom: true
    };

    // Update store - check for duplicates before adding
    customCategories.update(categories => {
      const exists = categories.some(c => c.id === trimmedName);
      if (exists) {
        return categories;
      }
      return [...categories, newCategory];
    });

    // Add to category order at the end - check for duplicates
    categoryOrder.update(order => {
      const exists = order.includes(trimmedName);
      if (exists) {
        return order;
      }
      return [...order, trimmedName];
    });

    return newCategory;
  } catch (error) {
    console.error('Failed to create custom category:', error);
    throw error;
  } finally {
    // Always remove from creating set
    creatingCategories.delete(trimmedName.toLowerCase());
  }
}

// Update a custom category (saves to database and broadcasts change)
export async function updateCustomCategory(categoryId: string, updates: Partial<CustomCategory>) {
  try {
    // Get the old category data before updating
    const currentCategories = get(customCategories);
    const oldCategory = currentCategories.find(c => c.id === categoryId);
    if (!oldCategory) {
      throw new Error(`Category with id ${categoryId} not found`);
    }

    // Get the database category to find its numeric ID
    // CRITICAL: We search by the OLD name (before any rename), not by categoryId
    const { getCategories } = await import('../services/database');
    const dbCategories = await getCategories();

    // Search by the current name in the store (oldCategory.name), not categoryId
    const dbCategory = dbCategories.find(c => c.name === oldCategory.name);

    if (dbCategory && dbCategory.id) {
      const newName = updates.name || oldCategory.name;
      const newIcon = updates.icon || oldCategory.icon;

      // Update in database
      await dbUpdateCategory(
        dbCategory.id,
        newName,
        newIcon,
        updates.color
      );
    } else {
      console.error('DB category not found! Cannot update database.');
    }

    // Update in store
    customCategories.update(categories =>
      categories.map(cat => {
        if (cat.id === categoryId) {
          // If name is changing, also update the ID to match
          const updatedCat = { ...cat, ...updates };
          if (updates.name) {
            updatedCat.id = updates.name;
          }
          return updatedCat;
        }
        return cat;
      })
    );

    // Broadcast rename event if name changed
    if (updates.name && updates.name !== oldCategory.name) {
      const newName = updates.name;
      const event = createCategoryRenamedEvent(categoryId, oldCategory.name, newName);
      broadcast(event);

      // Also update category order to use new name
      categoryOrder.update(order =>
        order.map(id => id === categoryId ? newName : id)
      );
    }
  } catch (error) {
    console.error('Failed to update category:', error);
    throw error;
  }
}

// Delete a custom category (async - deletes from database)
export async function deleteCustomCategory(categoryId: string) {
  try {
    // Get the database category to find its numeric ID
    const { getCategories } = await import('../services/database');
    const dbCategories = await getCategories();

    const dbCategory = dbCategories.find(c => c.name === categoryId);

    if (dbCategory && dbCategory.id) {
      // Delete from database
      await dbDeleteCategory(dbCategory.id);
    }

    // Remove from store
    customCategories.update(categories =>
      categories.filter(cat => cat.id !== categoryId)
    );

    // Remove from category order
    categoryOrder.update(order => order.filter(id => id !== categoryId));
  } catch (error) {
    console.error('Failed to delete custom category:', error);
    throw error;
  }
}

// Load custom categories from database
export async function loadCategoriesFromDatabase() {
  try {
    const dbCategories = await dbGetCategories();

    // Filter only custom categories
    const customCats = dbCategories
      .filter(cat => cat.is_custom)
      .map(cat => ({
        id: cat.name, // Use name as ID to match clipboard items
        name: cat.name,
        icon: cat.icon,
        isCustom: true
      }));

    // Load customizations for default categories (icon/color overrides)
    const customizations: CategoryCustomizations = {};
    dbCategories
      .filter(cat => !cat.is_custom)
      .forEach(cat => {
        customizations[cat.name] = {
          icon: cat.icon,
          color: cat.color
        };
      });

    // Update stores - this will REPLACE any stale in-memory categories
    customCategories.set(customCats);
    categoryCustomizations.set(customizations);

    // Update category order to remove any stale categories
    const allCategoryNames = dbCategories.map(c => c.name);
    categoryOrder.update(order => {
      // Keep only categories that exist in the database
      const validOrder = order.filter(name => allCategoryNames.includes(name));

      // Add any new categories that aren't in the order yet
      const missingCategories = allCategoryNames.filter(name => !validOrder.includes(name));

      return [...validOrder, ...missingCategories];
    });
  } catch (error) {
    console.error('Failed to load categories from database:', error);
  }
}
