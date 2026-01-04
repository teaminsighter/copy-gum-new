// Database service - Frontend direct access to SQLite via tauri-plugin-sql
// Uses @tauri-apps/plugin-sql for direct database queries from frontend

import Database from '@tauri-apps/plugin-sql';

export interface ClipboardItem {
  id?: number;
  content: string;
  content_type: string;
  category: string;
  app_name?: string;
  app_icon?: string;
  is_image: boolean;
  image_path?: string;
  image_thumbnail?: Uint8Array;
  image_dominant_color?: string;
  image_width?: number;
  image_height?: number;
  image_size?: number;
  timestamp: number;
  created_at?: number;
  updated_at?: number;
  is_pinned: boolean;
  is_favorite?: boolean;
  tags?: string; // JSON array as string (legacy)
  // Tag fields from clipboard_items_with_tags VIEW
  tag_names?: string; // Comma-separated tag names
  tag_icons?: string; // Comma-separated tag icons
  tag_colors?: string; // Comma-separated tag colors
  is_deleted: boolean;
  deleted_at?: number;
  search_text?: string;
}

export interface Category {
  id?: number;
  name: string;
  icon: string;
  color?: string;
  is_custom: boolean;
  sort_order: number;
}

export interface Tag {
  id?: number;
  name: string;
  icon?: string;
  color?: string;
  is_custom: boolean;
}

// Database connection singleton
let db: Database | null = null;

/**
 * Get or create database connection
 */
async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:copygum.db');
  }
  return db;
}

// ============================================
// CLIPBOARD ITEMS
// ============================================

/**
 * Get clipboard items from database
 */
export async function getClipboardItems(
  limit: number = 100,
  category?: string
): Promise<ClipboardItem[]> {
  try {
    const database = await getDb();

    // Use VIEW that includes tags joined from item_tags and tags tables
    let query = 'SELECT * FROM clipboard_items_with_tags WHERE is_deleted = 0';
    const params: any[] = [];

    if (category && category !== 'all') {
      query += ' AND category = $1';
      params.push(category);
    }

    query += ' ORDER BY is_pinned DESC, timestamp DESC';

    if (limit) {
      query += ` LIMIT ${limit}`;
    }

    const results = await database.select<ClipboardItem[]>(query, params);
    return results;
  } catch (error) {
    console.error('Failed to get clipboard items:', error);
    return [];
  }
}

/**
 * Save new clipboard item to database
 */
export async function saveClipboardItem(
  content: string,
  contentType: string,
  category: string,
  appName?: string,
  appIcon?: string
): Promise<{ id: number; isNew: boolean }> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // STRONG duplicate prevention: Check if this exact content already exists
    // We NEVER want the same content appearing multiple times in the history
    const existingItems = await database.select<ClipboardItem[]>(
      `SELECT id, timestamp FROM clipboard_items
       WHERE content = $1
       AND is_deleted = 0
       ORDER BY timestamp DESC
       LIMIT 1`,
      [content]
    );

    if (existingItems.length > 0) {
      // Content already exists - just update its timestamp to "bump" it to the top
      const existingId = existingItems[0].id!;
      await database.execute(
        `UPDATE clipboard_items
         SET timestamp = $1,
             updated_at = $2,
             content_type = $3,
             category = $4
         WHERE id = $5`,
        [timestamp, timestamp, contentType, category, existingId]
      );
      return { id: existingId, isNew: false };
    }

    const result = await database.execute(
      `INSERT INTO clipboard_items
       (content, content_type, category, app_name, app_icon, is_image, timestamp, is_pinned, is_deleted)
       VALUES ($1, $2, $3, $4, $5, 0, $6, 0, 0)`,
      [content, contentType, category, appName || null, appIcon || null, timestamp]
    );

    const newId = result.lastInsertId || 0;
    return { id: newId, isNew: true };
  } catch (error) {
    console.error('Failed to save clipboard item:', error);
    throw error;
  }
}

/**
 * Save image clipboard item to database
 */
export async function saveImageClipboardItem(
  imagePath: string,
  thumbnailPath: string,
  imageWidth: number,
  imageHeight: number,
  imageSize: number,
  dominantColor: string | null,
  appName?: string,
  appIcon?: string
): Promise<{ id: number; isNew: boolean }> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // For images, use the image path as the unique identifier for deduplication
    const existingItems = await database.select<ClipboardItem[]>(
      `SELECT id, timestamp FROM clipboard_items
       WHERE image_path = $1
       AND is_deleted = 0
       ORDER BY timestamp DESC
       LIMIT 1`,
      [imagePath]
    );

    if (existingItems.length > 0) {
      // Image already exists - update timestamp
      const existingId = existingItems[0].id!;
      await database.execute(
        `UPDATE clipboard_items
         SET timestamp = $1, updated_at = $2
         WHERE id = $3`,
        [timestamp, timestamp, existingId]
      );
      return { id: existingId, isNew: false };
    }

    // Create filename from path for display
    const filename = imagePath.split('/').pop() || 'Image';

    const result = await database.execute(
      `INSERT INTO clipboard_items
       (content, content_type, category, app_name, app_icon, is_image,
        image_path, image_width, image_height, image_size, image_dominant_color,
        timestamp, is_pinned, is_deleted)
       VALUES ($1, $2, $3, $4, $5, 1, $6, $7, $8, $9, $10, $11, 0, 0)`,
      [
        filename,           // content - use filename as display text
        'image',            // content_type
        'image',            // category
        appName || null,
        appIcon || null,
        imagePath,
        imageWidth,
        imageHeight,
        imageSize,
        dominantColor,
        timestamp
      ]
    );

    const newId = result.lastInsertId || 0;
    return { id: newId, isNew: true };
  } catch (error) {
    console.error('Failed to save image clipboard item:', error);
    throw error;
  }
}

/**
 * Delete clipboard item (soft delete)
 */
export async function deleteClipboardItem(id: number): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    await database.execute(
      'UPDATE clipboard_items SET is_deleted = 1, deleted_at = $1 WHERE id = $2',
      [timestamp, id]
    );
  } catch (error) {
    console.error('Failed to delete clipboard item:', error);
    throw error;
  }
}

/**
 * Toggle pin status of clipboard item
 */
export async function togglePin(id: number, pinned: boolean): Promise<void> {
  try {
    const database = await getDb();

    await database.execute(
      'UPDATE clipboard_items SET is_pinned = $1 WHERE id = $2',
      [pinned ? 1 : 0, id]
    );
  } catch (error) {
    console.error('Failed to toggle pin:', error);
    throw error;
  }
}

/**
 * Update category of clipboard item
 */
export async function updateItemCategory(id: number, category: string): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // First verify the category exists in the categories table
    const categoryExists = await database.select<Array<{ name: string }>>(
      'SELECT name FROM categories WHERE name = $1',
      [category]
    );

    if (categoryExists.length === 0) {
      throw new Error(`Category "${category}" does not exist in the database. Please create it first.`);
    }

    await database.execute(
      'UPDATE clipboard_items SET category = $1, updated_at = $2 WHERE id = $3',
      [category, timestamp, id]
    );
  } catch (error) {
    console.error('Failed to update item category:', error);
    throw error;
  }
}

/**
 * Delete multiple items at once
 */
export async function deleteMultipleItems(ids: number[]): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // Use multiple statements for each ID
    for (const id of ids) {
      await database.execute(
        'UPDATE clipboard_items SET is_deleted = 1, deleted_at = $1 WHERE id = $2',
        [timestamp, id]
      );
    }
  } catch (error) {
    console.error('Failed to delete multiple items:', error);
    throw error;
  }
}

/**
 * Pin multiple items at once
 */
export async function pinMultipleItems(ids: number[]): Promise<void> {
  try {
    const database = await getDb();

    // Use multiple statements for each ID
    for (const id of ids) {
      await database.execute(
        'UPDATE clipboard_items SET is_pinned = 1 WHERE id = $1',
        [id]
      );
    }
  } catch (error) {
    console.error('Failed to pin multiple items:', error);
    throw error;
  }
}

/**
 * Search clipboard items
 */
export async function searchClipboard(query: string): Promise<ClipboardItem[]> {
  try {
    const database = await getDb();

    const results = await database.select<ClipboardItem[]>(
      `SELECT * FROM clipboard_items
       WHERE is_deleted = 0
       AND content LIKE $1
       ORDER BY is_pinned DESC, timestamp DESC
       LIMIT 100`,
      [`%${query}%`]
    );

    return results;
  } catch (error) {
    console.error('Failed to search clipboard:', error);
    return [];
  }
}

// ============================================
// CATEGORIES
// ============================================

/**
 * Get all categories
 */
export async function getCategories(): Promise<Category[]> {
  try {
    const database = await getDb();

    const results = await database.select<Category[]>(
      'SELECT id, name, icon, color, is_custom, sort_order FROM categories ORDER BY sort_order ASC'
    );

    return results;
  } catch (error) {
    console.error('Failed to get categories:', error);
    return [];
  }
}

/**
 * Ensure all default categories exist in database (migration helper)
 */
export async function ensureDefaultCategories(): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // Default categories that should exist
    const defaultCategories = [
      { name: 'all', icon: '📌', sort_order: 0 },
      { name: 'password', icon: '🔐', sort_order: 1 },
      { name: 'apikey', icon: '🔑', sort_order: 2 },
      { name: 'private', icon: '🔒', sort_order: 3 },
      { name: 'email', icon: '📧', sort_order: 4 },
      { name: 'phone', icon: '📱', sort_order: 5 },
      { name: 'links', icon: '🔗', sort_order: 6 },
      { name: 'code', icon: '💻', sort_order: 7 },
      { name: 'color', icon: '🎨', sort_order: 8 },
      { name: 'image', icon: '🖼️', sort_order: 9 },
      { name: 'number', icon: '🔢', sort_order: 10 },
      { name: 'text', icon: '📝', sort_order: 11 },
    ];

    for (const cat of defaultCategories) {
      // Check if category exists
      const existing = await database.select<Array<{ name: string }>>(
        'SELECT name FROM categories WHERE name = $1',
        [cat.name]
      );

      if (existing.length === 0) {
        // Insert missing category
        await database.execute(
          `INSERT INTO categories (name, icon, color, is_custom, sort_order, created_at, updated_at)
           VALUES ($1, $2, 'rgba(50, 50, 50, 0.6)', 0, $3, $4, $5)`,
          [cat.name, cat.icon, cat.sort_order, timestamp, timestamp]
        );
      }
    }
  } catch (error) {
    console.error('Failed to ensure default categories:', error);
  }
}

/**
 * Create custom category
 */
export async function createCustomCategory(
  name: string,
  icon: string,
  color?: string
): Promise<Category> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // Get max sort_order
    const maxResult = await database.select<Array<{ max_order: number | null }>>(
      'SELECT MAX(sort_order) as max_order FROM categories'
    );
    const sortOrder = (maxResult[0]?.max_order || 0) + 1;

    const result = await database.execute(
      `INSERT INTO categories (name, icon, color, is_custom, sort_order, created_at, updated_at)
       VALUES ($1, $2, $3, 1, $4, $5, $6)`,
      [name, icon, color || null, sortOrder, timestamp, timestamp]
    );

    return {
      id: result.lastInsertId,
      name,
      icon,
      color,
      is_custom: true,
      sort_order: sortOrder
    };
  } catch (error) {
    console.error('Failed to create custom category:', error);
    throw error;
  }
}

/**
 * Update category
 */
export async function updateCategory(
  id: number,
  name: string,
  icon: string,
  color?: string
): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    // CRITICAL: Get the old category name before updating
    const oldCategory = await database.select<Array<{ name: string }>>(
      'SELECT name FROM categories WHERE id = $1',
      [id]
    );

    if (oldCategory.length === 0) {
      throw new Error(`Category with id ${id} not found`);
    }

    const oldName = oldCategory[0].name;

    // If name is NOT changing, just update icon and color
    if (oldName === name) {
      await database.execute(
        'UPDATE categories SET icon = $1, color = $2, updated_at = $3 WHERE id = $4',
        [icon, color || null, timestamp, id]
      );
      return;
    }

    // Name IS changing - we need to handle the foreign key constraint
    // The problem: clipboard_items.category REFERENCES categories.name
    // Solution: Temporarily disable foreign keys, update both tables, re-enable

    // Step 1: Disable foreign key constraints temporarily
    await database.execute('PRAGMA foreign_keys = OFF');

    try {
      // Step 2: Update the category name
      await database.execute(
        'UPDATE categories SET name = $1, icon = $2, color = $3, updated_at = $4 WHERE id = $5',
        [name, icon, color || null, timestamp, id]
      );

      // Step 3: Update all clipboard items that reference the old name
      await database.execute(
        'UPDATE clipboard_items SET category = $1, updated_at = $2 WHERE category = $3',
        [name, timestamp, oldName]
      );
    } finally {
      // Step 4: Always re-enable foreign key constraints
      await database.execute('PRAGMA foreign_keys = ON');
    }
  } catch (error) {
    console.error('Failed to update category:', error);
    throw error;
  }
}

/**
 * Delete category
 */
export async function deleteCategory(id: number): Promise<void> {
  try {
    const database = await getDb();

    await database.execute('DELETE FROM categories WHERE id = $1 AND is_custom = 1', [id]);
  } catch (error) {
    console.error('Failed to delete category:', error);
    throw error;
  }
}

// ============================================
// TAGS
// ============================================

/**
 * Get all tags
 */
export async function getTags(): Promise<Tag[]> {
  try {
    const database = await getDb();

    const results = await database.select<Tag[]>(
      'SELECT id, name, icon, color, is_custom FROM tags ORDER BY name ASC'
    );

    return results;
  } catch (error) {
    console.error('Failed to get tags:', error);
    return [];
  }
}

/**
 * Create tag
 */
export async function createTag(
  name: string,
  icon?: string,
  color?: string
): Promise<Tag> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    const result = await database.execute(
      `INSERT INTO tags (name, icon, color, is_custom, created_at, updated_at)
       VALUES ($1, $2, $3, 1, $4, $5)`,
      [name, icon || null, color || null, timestamp, timestamp]
    );

    return {
      id: result.lastInsertId,
      name,
      icon,
      color,
      is_custom: true
    };
  } catch (error) {
    console.error('Failed to create tag:', error);
    throw error;
  }
}

/**
 * Update tag
 */
export async function updateTag(
  id: number,
  name: string,
  icon?: string,
  color?: string
): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    await database.execute(
      'UPDATE tags SET name = $1, icon = $2, color = $3, updated_at = $4 WHERE id = $5',
      [name, icon || null, color || null, timestamp, id]
    );
  } catch (error) {
    console.error('Failed to update tag:', error);
    throw error;
  }
}

/**
 * Delete tag
 */
export async function deleteTag(id: number): Promise<void> {
  try {
    const database = await getDb();

    // Delete tag and its associations
    await database.execute('DELETE FROM item_tags WHERE tag_id = $1', [id]);
    await database.execute('DELETE FROM tags WHERE id = $1 AND is_custom = 1', [id]);
  } catch (error) {
    console.error('Failed to delete tag:', error);
    throw error;
  }
}

/**
 * Add tag to item
 */
export async function addTagToItem(itemId: number, tagId: number): Promise<void> {
  try {
    const database = await getDb();
    const timestamp = Date.now();

    await database.execute(
      'INSERT INTO item_tags (item_id, tag_id, created_at) VALUES ($1, $2, $3)',
      [itemId, tagId, timestamp]
    );
  } catch (error) {
    console.error('Failed to add tag to item:', error);
    throw error;
  }
}

/**
 * Remove tag from item
 */
export async function removeTagFromItem(itemId: number, tagId: number): Promise<void> {
  try {
    const database = await getDb();

    await database.execute(
      'DELETE FROM item_tags WHERE item_id = $1 AND tag_id = $2',
      [itemId, tagId]
    );
  } catch (error) {
    console.error('Failed to remove tag from item:', error);
    throw error;
  }
}

/**
 * Get tags for specific item
 */
export async function getTagsForItem(itemId: number): Promise<Tag[]> {
  try {
    const database = await getDb();

    const results = await database.select<Tag[]>(
      `SELECT t.id, t.name, t.icon, t.color, t.is_custom
       FROM tags t
       INNER JOIN item_tags it ON t.id = it.tag_id
       WHERE it.item_id = $1
       ORDER BY t.name ASC`,
      [itemId]
    );

    return results;
  } catch (error) {
    console.error('Failed to get tags for item:', error);
    return [];
  }
}

/**
 * Add tag to item by tag name (wrapper for convenience)
 */
export async function addTagToItemByName(itemId: number, tagName: string): Promise<void> {
  try {
    const database = await getDb();

    // Get tag ID from tag name
    const tagResult = await database.select<{ id: number }[]>(
      'SELECT id FROM tags WHERE name = $1',
      [tagName]
    );

    if (tagResult.length === 0) {
      throw new Error(`Tag "${tagName}" not found in database`);
    }

    const tagId = tagResult[0].id;

    // Use existing function to add tag
    await addTagToItem(itemId, tagId);
  } catch (error) {
    console.error('Failed to add tag to item by name:', error);
    throw error;
  }
}

/**
 * Remove tag from item by tag name (wrapper for convenience)
 */
export async function removeTagFromItemByName(itemId: number, tagName: string): Promise<void> {
  try {
    const database = await getDb();

    // Get tag ID from tag name
    const tagResult = await database.select<{ id: number }[]>(
      'SELECT id FROM tags WHERE name = $1',
      [tagName]
    );

    if (tagResult.length === 0) {
      throw new Error(`Tag "${tagName}" not found`);
    }

    const tagId = tagResult[0].id;

    // Use existing function to remove tag
    await removeTagFromItem(itemId, tagId);
  } catch (error) {
    console.error('Failed to remove tag from item by name:', error);
    throw error;
  }
}
