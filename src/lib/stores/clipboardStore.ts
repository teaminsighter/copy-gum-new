// Clipboard Store - Manages clipboard items with database integration
// Connects UI to Rust backend via database service

import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ClipboardItem } from '../services/database';
import {
  getClipboardItems,
  saveClipboardItem,
  saveImageClipboardItem,
  deleteClipboardItem as deleteItemFromDb,
  togglePin as togglePinInDb,
  searchClipboard as searchInDb,
  updateItemCategory as updateItemCategoryInDb
} from '../services/database';
import { settings } from './settingsStore';

// ============================================
// STORE STATE
// ============================================

export const clipboardItems = writable<ClipboardItem[]>([]);
export const isLoading = writable<boolean>(false);
export const error = writable<string | null>(null);
export const isMonitoring = writable<boolean>(false);

// Filters
export const selectedCategory = writable<string | null>(null);
export const searchQuery = writable<string>('');

// Advanced filters
export interface AdvancedFilters {
  dateRange?: { start: Date; end: Date };
  pinStatus?: 'all' | 'pinned' | 'unpinned';
  contentTypes?: string[]; // ['text', 'url', 'code', etc.]
  categories?: string[]; // Filter by category IDs
  tags?: string[]; // Filter by tag names
}

export const advancedFilters = writable<AdvancedFilters>({
  pinStatus: 'all',
  contentTypes: [],
  categories: [],
  tags: []
});

// Debounced search query (updates after 300ms of inactivity)
export const debouncedSearchQuery = writable<string>('');

let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

// Subscribe to searchQuery and debounce it
searchQuery.subscribe(value => {
  if (searchDebounceTimer) {
    clearTimeout(searchDebounceTimer);
  }

  searchDebounceTimer = setTimeout(() => {
    debouncedSearchQuery.set(value);
  }, 300);
});

// Derived store: filtered items based on all filters
export const filteredItems = derived(
  [clipboardItems, selectedCategory, debouncedSearchQuery, advancedFilters],
  ([$items, $category, $search, $filters]) => {
    let filtered = $items;

    // Filter by category
    if ($category && $category !== 'all') {
      filtered = filtered.filter(item => item.category === $category);
    }

    // Filter by search (client-side for instant feedback)
    if ($search) {
      const query = $search.toLowerCase();
      filtered = filtered.filter(item =>
        item.content.toLowerCase().includes(query) ||
        (item.app_name && item.app_name.toLowerCase().includes(query)) ||
        (item.category && item.category.toLowerCase().includes(query)) ||
        (item.content_type && item.content_type.toLowerCase().includes(query))
      );
    }

    // Filter by pin status
    if ($filters.pinStatus === 'pinned') {
      filtered = filtered.filter(item => item.is_pinned);
    } else if ($filters.pinStatus === 'unpinned') {
      filtered = filtered.filter(item => !item.is_pinned);
    }

    // Filter by content types
    if ($filters.contentTypes && $filters.contentTypes.length > 0) {
      filtered = filtered.filter(item =>
        $filters.contentTypes!.includes(item.content_type)
      );
    }

    // Filter by categories (from advanced filters panel)
    if ($filters.categories && $filters.categories.length > 0) {
      filtered = filtered.filter(item =>
        $filters.categories!.includes(item.category)
      );
    }

    // Filter by tags
    if ($filters.tags && $filters.tags.length > 0) {
      filtered = filtered.filter(item => {
        // tag_names is a comma-separated string from the database VIEW
        if (!item.tag_names) return false;
        const itemTags = item.tag_names.split(',').map(t => t.trim());
        // Item must have at least one of the selected tags
        return $filters.tags!.some(tag => itemTags.includes(tag));
      });
    }

    // Filter by date range
    if ($filters.dateRange) {
      const { start, end } = $filters.dateRange;
      filtered = filtered.filter(item => {
        const itemDate = new Date(item.created_at);
        return itemDate >= start && itemDate <= end;
      });
    }

    return filtered;
  }
);

// ============================================
// QUEUE PROCESSING
// ============================================

/**
 * Process clipboard events from queue one at a time
 * This prevents duplicate saves when multiple event listeners fire
 */
async function processQueue() {
  const state = getMonitorState();

  if (state.isProcessing || state.processingQueue.length === 0) {
    return;
  }

  state.isProcessing = true;

  // Batch process all items
  const processedContent = new Set<string>();
  let hasNewItems = false;
  const updatedItemData = new Map<number, { contentType: string; category: string }>();

  while (state.processingQueue.length > 0) {
    const event = state.processingQueue.shift()!;

    // Use image path or content as unique identifier
    const uniqueKey = event.isImage ? event.imagePath! : event.content;

    // Skip if we already processed this content in this batch
    if (processedContent.has(uniqueKey)) {
      continue;
    }

    try {
      let result: { id: number; isNew: boolean };

      if (event.isImage && event.imagePath) {
        // Save image item
        result = await saveImageClipboardItem(
          event.imagePath,
          event.thumbnailPath || '',
          event.imageWidth || 0,
          event.imageHeight || 0,
          event.imageSize || 0,
          event.dominantColor || null,
          event.sourceAppName,
          event.sourceAppIcon,
          event.sourceBundleId
        );
      } else {
        // Save text item
        result = await saveClipboardItem(
          event.content,
          event.contentType,
          event.category,
          event.sourceAppName,
          event.sourceAppIcon,
          event.sourceBundleId
        );
      }

      // Track if we inserted any new items
      if (result.isNew) {
        hasNewItems = true;
      } else {
        // Track updated items with their new detection data
        updatedItemData.set(result.id, {
          contentType: event.contentType,
          category: event.category
        });
      }

      processedContent.add(uniqueKey);

      // Clear remaining duplicates from queue (same content or image path)
      state.processingQueue = state.processingQueue.filter(item => {
        const itemKey = item.isImage ? item.imagePath : item.content;
        return itemKey !== uniqueKey;
      });
    } catch (e) {
      console.error('Failed to save clipboard item:', e);
    }
  }

  // Reload if new items were added OR update timestamps and re-sort if duplicates were updated
  if (hasNewItems) {
    // New items added - full reload from database
    await loadClipboardItems();
  } else if (updatedItemData.size > 0) {
    // Duplicates updated - update timestamps, category, and content_type in memory
    const now = Date.now();
    clipboardItems.update(items => {
      const updatedItems = items.map(item => {
        const updateInfo = updatedItemData.get(item.id!);
        if (updateInfo) {
          return {
            ...item,
            timestamp: now,
            content_type: updateInfo.contentType,
            category: updateInfo.category
          };
        }
        return item;
      });

      // Re-sort: pinned first, then by timestamp
      return updatedItems.sort((a, b) => {
        if (a.is_pinned !== b.is_pinned) {
          return b.is_pinned ? 1 : -1;
        }
        return (b.timestamp || 0) - (a.timestamp || 0);
      });
    });
  }

  state.isProcessing = false;
}

// ============================================
// LOAD CLIPBOARD ITEMS
// ============================================

/**
 * Load clipboard items from database
 * Uses history_limit from settings (-1 = unlimited)
 * @param category - Filter by specific category (optional)
 */
export async function loadClipboardItems(
  category?: string
): Promise<void> {
  // Get limit from settings (-1 means unlimited, use 10000 as practical max)
  const currentSettings = get(settings);
  const limit = currentSettings.history_limit === -1 ? 10000 : currentSettings.history_limit;

  isLoading.set(true);
  error.set(null);

  try {
    const items = await getClipboardItems(limit, category);
    clipboardItems.set(items);
  } catch (e) {
    console.error('Failed to load clipboard items:', e);
    error.set(e as string);
  } finally {
    isLoading.set(false);
  }
}

// ============================================
// CLIPBOARD MONITORING
// ============================================

// Queue item type for clipboard events
interface ClipboardQueueItem {
  content: string;
  contentType: string;
  category: string;
  sourceAppName?: string;
  sourceAppIcon?: string;
  sourceBundleId?: string;
  // Image-specific fields
  isImage: boolean;
  imagePath?: string;
  thumbnailPath?: string;
  imageWidth?: number;
  imageHeight?: number;
  imageSize?: number;
  dominantColor?: string;
}

// Use window object for truly global state (survives HMR reloads)
declare global {
  interface Window {
    __clipboardMonitor?: {
      eventUnlisten: (() => void) | null;
      isInitialized: boolean;
      isProcessing: boolean;
      processingQueue: ClipboardQueueItem[];
    };
  }
}

// Initialize global state if not exists
if (typeof window !== 'undefined' && !window.__clipboardMonitor) {
  window.__clipboardMonitor = {
    eventUnlisten: null,
    isInitialized: false,
    isProcessing: false,
    processingQueue: []
  };
}

// Helper to get monitor state (safe for SSR)
const getMonitorState = () => {
  if (typeof window === 'undefined') {
    return {
      eventUnlisten: null,
      isInitialized: false,
      isProcessing: false,
      processingQueue: []
    };
  }
  return window.__clipboardMonitor!;
};

/**
 * Start clipboard monitoring
 * - Starts backend clipboard capture
 * - Listens for clipboard-changed events from Rust backend
 */
export async function startClipboardMonitoring(): Promise<void> {
  const state = getMonitorState();

  // Guard: prevent multiple event listeners (check this FIRST, before isMonitoring)
  if (state.eventUnlisten !== null) {
    return;
  }

  if (get(isMonitoring)) {
    return;
  }

  try {
    // Listen for clipboard-changed events from Rust backend
    const unlisten = await listen<{
      content: string;
      contentType: string;
      category: string;
      isImage: boolean;
      imagePath?: string;
      thumbnailPath?: string;
      imageWidth?: number;
      imageHeight?: number;
      imageSize?: number;
      dominantColor?: string;
      sourceAppName?: string;
      sourceAppIcon?: string;
      sourceBundleId?: string;
    }>('clipboard-changed', async (event) => {
      const currentState = getMonitorState();

      // Use image path or content as unique key for duplicate detection
      const uniqueKey = event.payload.isImage ? event.payload.imagePath : event.payload.content;

      // Check if this exact content/image is already in the queue (prevent duplicate events)
      const isDuplicate = currentState.processingQueue.some(item => {
        const itemKey = item.isImage ? item.imagePath : item.content;
        return itemKey === uniqueKey;
      });

      if (isDuplicate) {
        return;
      }

      // Add to queue with all fields including image data
      currentState.processingQueue.push({
        content: event.payload.content,
        contentType: event.payload.contentType,
        category: event.payload.category,
        sourceAppName: event.payload.sourceAppName,
        sourceAppIcon: event.payload.sourceAppIcon,
        sourceBundleId: event.payload.sourceBundleId,
        isImage: event.payload.isImage,
        imagePath: event.payload.imagePath,
        thumbnailPath: event.payload.thumbnailPath,
        imageWidth: event.payload.imageWidth,
        imageHeight: event.payload.imageHeight,
        imageSize: event.payload.imageSize,
        dominantColor: event.payload.dominantColor
      });

      // Start processing if not already processing
      if (!currentState.isProcessing) {
        processQueue();
      }
    });

    state.eventUnlisten = unlisten;

    // Start backend monitoring (Rust clipboard watcher)
    await invoke('start_clipboard_monitoring');
    isMonitoring.set(true);
  } catch (e) {
    console.error('Failed to start clipboard monitoring:', e);
    error.set(`Failed to start monitoring: ${e}`);
  }
}

/**
 * Stop clipboard monitoring
 */
export async function stopClipboardMonitoring(): Promise<void> {
  if (!get(isMonitoring)) return;

  const state = getMonitorState();

  try {
    // Stop backend monitoring
    await invoke('stop_clipboard_monitoring');
    isMonitoring.set(false);

    // Stop listening for events (with error handling)
    if (state.eventUnlisten) {
      try {
        state.eventUnlisten();
      } catch (e) {
        console.warn('Event unlisten failed (might already be cleaned up):', e);
      }
      state.eventUnlisten = null;
    }

    // Reset initialization flag
    state.isInitialized = false;
  } catch (e) {
    console.error('Failed to stop clipboard monitoring:', e);
  }
}

/**
 * Check if monitoring is active
 */
export async function checkMonitoringStatus(): Promise<boolean> {
  try {
    const status = await invoke<boolean>('is_clipboard_monitoring');
    isMonitoring.set(status);
    return status;
  } catch (e) {
    console.error('Failed to check monitoring status:', e);
    return false;
  }
}

// ============================================
// ITEM OPERATIONS
// ============================================

/**
 * Toggle pin status of an item
 */
export async function togglePin(itemId: number, pinned: boolean): Promise<void> {
  try {
    await togglePinInDb(itemId, pinned);

    // Update local state optimistically
    clipboardItems.update(items =>
      items.map(item =>
        item.id === itemId ? { ...item, is_pinned: pinned } : item
      )
    );
  } catch (e) {
    console.error('Failed to toggle pin:', e);
    throw e;
  }
}

/**
 * Delete an item (soft delete)
 */
export async function deleteItem(itemId: number): Promise<void> {
  try {
    await deleteItemFromDb(itemId);

    // Remove from local state
    clipboardItems.update(items =>
      items.filter(item => item.id !== itemId)
    );
  } catch (e) {
    console.error('Failed to delete item:', e);
    throw e;
  }
}

/**
 * Update category of a clipboard item
 */
export async function updateItemCategory(itemId: number, category: string): Promise<void> {
  try {
    // Update in database first
    await updateItemCategoryInDb(itemId, category);

    // Update local state to keep UI in sync
    clipboardItems.update(items =>
      items.map(item =>
        item.id === itemId ? { ...item, category } : item
      )
    );
  } catch (e) {
    console.error('Failed to update item category:', e);
    throw e;
  }
}

/**
 * Delete multiple items at once
 */
export async function deleteMultipleItems(itemIds: number[]): Promise<void> {
  try {
    // Delete each item individually using database service
    for (const id of itemIds) {
      await deleteItemFromDb(id);
    }

    // Remove from local state
    clipboardItems.update(items =>
      items.filter(item => !itemIds.includes(item.id!))
    );
  } catch (e) {
    console.error('Failed to delete multiple items:', e);
    throw e;
  }
}

/**
 * Pin multiple items at once
 */
export async function pinMultipleItems(itemIds: number[]): Promise<void> {
  try {
    // Pin each item individually using database service
    for (const id of itemIds) {
      await togglePinInDb(id, true);
    }

    // Update local state
    clipboardItems.update(items =>
      items.map(item =>
        itemIds.includes(item.id!) ? { ...item, is_pinned: true } : item
      )
    );
  } catch (e) {
    console.error('Failed to pin multiple items:', e);
    throw e;
  }
}

// ============================================
// SEARCH
// ============================================

/**
 * Search clipboard items in database
 * For instant client-side filtering, use the searchQuery writable store instead
 */
export async function searchClipboard(query: string): Promise<void> {
  if (!query.trim()) {
    // If empty search, reload all items
    await loadClipboardItems();
    return;
  }

  isLoading.set(true);
  error.set(null);

  try {
    const results = await searchInDb(query);
    clipboardItems.set(results);
  } catch (e) {
    console.error('Failed to search:', e);
    error.set(e as string);
  } finally {
    isLoading.set(false);
  }
}

// ============================================
// INITIALIZATION
// ============================================

/**
 * Initialize clipboard store
 * - Load initial items
 * - Start monitoring (if auto_start_monitoring setting is enabled)
 */
export async function initClipboardStore(): Promise<void> {
  const state = getMonitorState();

  // Always load items (even if already initialized)
  await loadClipboardItems();

  // Check auto_start_monitoring setting
  const currentSettings = get(settings);
  const shouldAutoStart = currentSettings.auto_start_monitoring;

  // Only start monitoring if auto_start_monitoring is enabled and not already monitoring
  if (shouldAutoStart && (!state.isInitialized || !get(isMonitoring))) {
    state.isInitialized = true;
    await startClipboardMonitoring();
  } else {
    state.isInitialized = true;
  }
}

// NOTE: Auto-initialization removed to prevent multiple event listeners
// Initialize manually from App.svelte onMount() instead
