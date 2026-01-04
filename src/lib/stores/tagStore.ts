// Global store for tag management
import { writable, get } from 'svelte/store';
import { getTags, createTag as dbCreateTag, updateTag as dbUpdateTag, deleteTag as dbDeleteTag, type Tag as DBTag } from '../services/database';
import { broadcast, createTagRenamedEvent } from '../events/eventBus';

export interface Tag {
  id: string;
  name: string;
  icon: string;
  color: string;
  isDefault?: boolean; // True for built-in tags that cannot be deleted
}

// Default tag names (reserved)
const DEFAULT_TAG_NAMES = ['work', 'personal', 'important', 'urgent', 'project', 'meeting', 'todo', 'archive'];

/**
 * Validate tag name
 * @throws Error if name is invalid
 */
function validateTagName(name: string): void {
  if (!name || typeof name !== 'string') {
    throw new Error('Tag name is required');
  }

  const trimmed = name.trim();

  if (trimmed.length === 0) {
    throw new Error('Tag name cannot be empty');
  }

  if (trimmed.length > 50) {
    throw new Error('Tag name must be less than 50 characters');
  }

  // Check for reserved names
  if (DEFAULT_TAG_NAMES.includes(trimmed.toLowerCase())) {
    throw new Error(`"${trimmed}" is a reserved tag name`);
  }
}

// Track in-flight tag creations to prevent race conditions
const creatingTags = new Set<string>();

// Default built-in tags
export const defaultTags: Tag[] = [
  { id: 'work', name: 'Work', icon: '💼', color: 'rgba(59, 130, 246, 0.6)', isDefault: true },
  { id: 'personal', name: 'Personal', icon: '👤', color: 'rgba(139, 79, 155, 0.6)', isDefault: true },
  { id: 'important', name: 'Important', icon: '⭐', color: 'rgba(247, 228, 121, 0.6)', isDefault: true },
  { id: 'urgent', name: 'Urgent', icon: '🔥', color: 'rgba(239, 68, 68, 0.6)', isDefault: true },
  { id: 'project', name: 'Project', icon: '📁', color: 'rgba(34, 197, 94, 0.6)', isDefault: true },
  { id: 'meeting', name: 'Meeting', icon: '📅', color: 'rgba(255, 165, 0, 0.6)', isDefault: true },
  { id: 'todo', name: 'Todo', icon: '✅', color: 'rgba(16, 185, 129, 0.6)', isDefault: true },
  { id: 'archive', name: 'Archive', icon: '📦', color: 'rgba(50, 50, 50, 0.6)', isDefault: true },
];

// Store for all tags (default + custom)
export const allTags = writable<Tag[]>([...defaultTags]);

// Store for custom user-created tags (for backwards compatibility)
export const customTags = writable<Tag[]>([]);

// Load tags from database
export async function loadTagsFromDatabase() {
  try {
    const dbTags = await getTags();

    // Convert database tags to store format
    const convertedTags: Tag[] = dbTags.map(dbTag => ({
      id: String(dbTag.id),
      name: dbTag.name,
      icon: dbTag.icon || '🏷️',
      color: dbTag.color || 'rgba(100, 100, 100, 0.6)',
      isDefault: !dbTag.is_custom
    }));

    // Update stores with database tags
    allTags.set(convertedTags);

    // Separate custom tags
    const custom = convertedTags.filter(tag => !tag.isDefault);
    customTags.set(custom);
  } catch (error) {
    console.error('Failed to load tags from database:', error);
    // Fallback to default tags on error
    allTags.set([...defaultTags]);
  }
}

// Create a new tag (saves to database)
export async function createTag(name: string, icon: string, color: string) {
  // Validate the name
  validateTagName(name);

  const trimmedName = name.trim();

  // Check if already being created (race condition prevention)
  if (creatingTags.has(trimmedName.toLowerCase())) {
    throw new Error(`Tag "${trimmedName}" is already being created`);
  }

  // Check if tag already exists in store
  const existingTags = get(allTags);
  if (existingTags.some(t => t.name.toLowerCase() === trimmedName.toLowerCase())) {
    throw new Error(`Tag "${trimmedName}" already exists`);
  }

  // Mark as creating
  creatingTags.add(trimmedName.toLowerCase());

  try {
    // Save to database first
    const dbTag = await dbCreateTag(trimmedName, icon, color);

    const newTag: Tag = {
      id: String(dbTag.id),
      name: dbTag.name,
      icon: dbTag.icon || icon,
      color: dbTag.color || color,
      isDefault: false
    };

    // Add to both stores
    customTags.update(tags => [...tags, newTag]);
    allTags.update(tags => [...tags, newTag]);
    return newTag;
  } catch (error) {
    console.error('Failed to create tag:', error);
    throw error;
  } finally {
    // Always remove from creating set
    creatingTags.delete(trimmedName.toLowerCase());
  }
}

// Update a tag (saves to database and broadcasts change)
export async function updateTag(tagId: string, updates: Partial<Tag>) {
  try {
    // Get the old tag data before updating
    const currentTags = get(allTags);
    const oldTag = currentTags.find(t => t.id === tagId);
    if (!oldTag) {
      throw new Error(`Tag with id ${tagId} not found`);
    }

    // Update in database
    const numericId = parseInt(tagId);
    await dbUpdateTag(numericId, updates.name!, updates.icon, updates.color);

    // Update in stores
    allTags.update(tags =>
      tags.map(tag =>
        tag.id === tagId ? { ...tag, ...updates } : tag
      )
    );

    customTags.update(tags =>
      tags.map(tag =>
        tag.id === tagId ? { ...tag, ...updates } : tag
      )
    );

    // Broadcast rename event if name changed
    if (updates.name && updates.name !== oldTag.name) {
      const event = createTagRenamedEvent(tagId, oldTag.name, updates.name);
      broadcast(event);
    }
  } catch (error) {
    console.error('Failed to update tag:', error);
    throw error;
  }
}

// Delete a tag (only custom tags can be deleted, saves to database)
export async function deleteTag(tagId: string) {
  try {
    let shouldDelete = false;

    allTags.update(tags => {
      const tag = tags.find(t => t.id === tagId);
      // Prevent deletion of default tags
      if (tag?.isDefault) {
        console.warn('Cannot delete default tag:', tagId);
        return tags;
      }
      shouldDelete = true;
      return tags.filter(tag => tag.id !== tagId);
    });

    if (shouldDelete) {
      // Delete from database
      const numericId = parseInt(tagId);
      await dbDeleteTag(numericId);

      customTags.update(tags =>
        tags.filter(tag => tag.id !== tagId)
      );
    }
  } catch (error) {
    console.error('Failed to delete tag:', error);
    throw error;
  }
}
