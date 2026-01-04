// Event Bus for Broadcasting Real-time Changes
// Enables smooth propagation of tag/category renames across all cards

import { writable } from 'svelte/store';

// Event types for tag operations
export type TagRenamedEvent = {
  type: 'TAG_RENAMED';
  oldName: string;
  newName: string;
  tagId: string;
  timestamp: number;
};

// Event types for category operations
export type CategoryRenamedEvent = {
  type: 'CATEGORY_RENAMED';
  categoryId: string;
  oldName: string;
  newName: string;
  timestamp: number;
};

// Union type for all possible events
export type AppEvent = TagRenamedEvent | CategoryRenamedEvent;

// Event bus store - emits events to all subscribers
export const eventBus = writable<AppEvent | null>(null);

/**
 * Broadcast an event to all listening components
 * Event is cleared after 50ms to prevent duplicate processing
 */
export function broadcast(event: AppEvent) {
  eventBus.set(event);

  // Clear event after broadcasting to prevent re-processing
  setTimeout(() => eventBus.set(null), 50);
}

/**
 * Create a tag renamed event
 */
export function createTagRenamedEvent(
  tagId: string,
  oldName: string,
  newName: string
): TagRenamedEvent {
  return {
    type: 'TAG_RENAMED',
    tagId,
    oldName,
    newName,
    timestamp: Date.now()
  };
}

/**
 * Create a category renamed event
 */
export function createCategoryRenamedEvent(
  categoryId: string,
  oldName: string,
  newName: string
): CategoryRenamedEvent {
  return {
    type: 'CATEGORY_RENAMED',
    categoryId,
    oldName,
    newName,
    timestamp: Date.now()
  };
}
