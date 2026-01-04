# Module 6: Frontend-Backend Integration
# Connecting Svelte UI to Rust Backend

**Date:** 2025-11-19
**Priority:** CRITICAL
**Status:** 🔄 In Progress

---

## 📋 Executive Summary

Module 6 connects the completed Svelte UI (Phase 2) with the working Rust backend (Modules 1-5). This makes CopyGum fully functional by replacing mock data with real database calls and enabling real-time clipboard capture.

**Goal:** Transform CopyGum from a UI prototype into a working application.

---

## 🎯 Module Objectives

### Primary Goals:
1. Replace all mock/sample data with real Tauri invoke calls
2. Connect clipboard items display to database
3. Integrate clipboard monitoring with UI updates
4. Enable real-time updates when clipboard changes
5. Add proper error handling and loading states
6. Make all CRUD operations functional

### Success Criteria:
- ✅ Clipboard items load from database
- ✅ New clipboard copies appear in UI immediately
- ✅ Categories and tags work with real data
- ✅ Search, filter, pin, delete operations work
- ✅ No console errors
- ✅ Smooth user experience with loading states

---

## 📦 Implementation Plan

### **Phase 1: Clipboard Items Store Integration** (Steps 1-3)

#### Step 1: Update clipboardStore.ts - Load Items
**File:** `src/lib/stores/clipboardStore.ts`

**Current State:** Uses mock data from `mockClipboardData`

**Changes Needed:**
```typescript
import { invoke } from '@tauri-apps/api/core';

// Remove mock data import
// import { mockClipboardData } from './mockData';

interface ClipboardItem {
  id: number;
  content: string;
  content_type: string;
  category: string;
  timestamp: string;
  is_pinned: boolean;
  is_favorite: boolean;
  is_deleted: boolean;
  app_name?: string;
  app_icon?: string;
  is_image: boolean;
  image_path?: string;
  image_thumbnail?: string;
  image_width?: number;
  image_height?: number;
  image_size?: number;
  image_dominant_color?: string;
  tags?: string[];
}

// Create writable store
export const clipboardItems = writable<ClipboardItem[]>([]);
export const isLoading = writable(true);
export const error = writable<string | null>(null);

// Load items from backend
export async function loadClipboardItems(
  limit?: number,
  category?: string,
  search?: string
) {
  isLoading.set(true);
  error.set(null);

  try {
    const items = await invoke<ClipboardItem[]>('get_clipboard_items', {
      limit: limit || 100,
      offset: 0,
      category: category || null,
      tags: null,
      searchQuery: search || null
    });

    clipboardItems.set(items);
  } catch (e) {
    console.error('Failed to load clipboard items:', e);
    error.set(e as string);
  } finally {
    isLoading.set(false);
  }
}

// Initialize on app start
loadClipboardItems();
```

**Estimated Time:** 30 minutes

---

#### Step 2: Add Real-time Clipboard Monitoring
**File:** `src/lib/stores/clipboardStore.ts`

**Add monitoring control:**
```typescript
import { listen } from '@tauri-apps/api/event';

let monitoringEnabled = false;
let unlistenFn: (() => void) | null = null;

export async function startClipboardMonitoring() {
  if (monitoringEnabled) return;

  try {
    // Start backend monitoring
    await invoke('start_clipboard_monitoring');

    // Listen for clipboard events (optional - if you emit events)
    // unlistenFn = await listen('clipboard-changed', () => {
    //   loadClipboardItems(); // Reload items
    // });

    // Poll for new items every 2 seconds as fallback
    const pollInterval = setInterval(() => {
      loadClipboardItems();
    }, 2000);

    monitoringEnabled = true;
    console.log('✅ Clipboard monitoring started');
  } catch (e) {
    console.error('Failed to start monitoring:', e);
    error.set(e as string);
  }
}

export async function stopClipboardMonitoring() {
  if (!monitoringEnabled) return;

  try {
    await invoke('stop_clipboard_monitoring');

    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }

    monitoringEnabled = false;
    console.log('⏹️ Clipboard monitoring stopped');
  } catch (e) {
    console.error('Failed to stop monitoring:', e);
  }
}

// Auto-start monitoring on app load
startClipboardMonitoring();
```

**Estimated Time:** 45 minutes

---

#### Step 3: Implement Item Operations
**File:** `src/lib/stores/clipboardStore.ts`

**Add CRUD operations:**
```typescript
// Pin/Unpin item
export async function togglePin(itemId: number, pinned: boolean) {
  try {
    await invoke('toggle_pin', { id: itemId, pinned });

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

// Delete item (soft delete)
export async function deleteItem(itemId: number) {
  try {
    await invoke('delete_clipboard_item', { id: itemId });

    // Remove from local state
    clipboardItems.update(items =>
      items.filter(item => item.id !== itemId)
    );
  } catch (e) {
    console.error('Failed to delete item:', e);
    throw e;
  }
}

// Delete multiple items
export async function deleteMultipleItems(itemIds: number[]) {
  try {
    await invoke('delete_multiple_items', { ids: itemIds });

    // Remove from local state
    clipboardItems.update(items =>
      items.filter(item => !itemIds.includes(item.id))
    );
  } catch (e) {
    console.error('Failed to delete multiple items:', e);
    throw e;
  }
}

// Pin multiple items
export async function pinMultipleItems(itemIds: number[]) {
  try {
    await invoke('pin_multiple_items', { ids: itemIds });

    // Update local state
    clipboardItems.update(items =>
      items.map(item =>
        itemIds.includes(item.id) ? { ...item, is_pinned: true } : item
      )
    );
  } catch (e) {
    console.error('Failed to pin multiple items:', e);
    throw e;
  }
}

// Search items
export async function searchClipboard(query: string) {
  isLoading.set(true);

  try {
    const results = await invoke<ClipboardItem[]>('search_clipboard', { query });
    clipboardItems.set(results);
  } catch (e) {
    console.error('Failed to search:', e);
    error.set(e as string);
  } finally {
    isLoading.set(false);
  }
}
```

**Estimated Time:** 1 hour

---

### **Phase 2: Categories Store Integration** (Steps 4-5)

#### Step 4: Update categoryStore.ts
**File:** `src/lib/stores/categoryStore.ts`

**Current State:** Uses mock categories

**Changes Needed:**
```typescript
import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

interface Category {
  id: number;
  name: string;
  icon: string;
  color?: string;
  is_custom: boolean;
  order_index: number;
  item_count?: number;
}

export const categories = writable<Category[]>([]);
export const isLoading = writable(false);

// Load categories
export async function loadCategories() {
  isLoading.set(true);

  try {
    const cats = await invoke<Category[]>('get_categories');
    categories.set(cats);
  } catch (e) {
    console.error('Failed to load categories:', e);
  } finally {
    isLoading.set(false);
  }
}

// Create custom category
export async function createCustomCategory(
  name: string,
  icon: string,
  color: string
) {
  try {
    const newCat = await invoke<Category>('create_custom_category', {
      name,
      icon,
      color
    });

    categories.update(cats => [...cats, newCat]);
    return newCat;
  } catch (e) {
    console.error('Failed to create category:', e);
    throw e;
  }
}

// Update category
export async function updateCategory(
  id: number,
  name: string,
  icon: string,
  color: string
) {
  try {
    await invoke('update_category', { id, name, icon, color });

    categories.update(cats =>
      cats.map(cat =>
        cat.id === id ? { ...cat, name, icon, color } : cat
      )
    );
  } catch (e) {
    console.error('Failed to update category:', e);
    throw e;
  }
}

// Delete category
export async function deleteCategory(id: number) {
  try {
    await invoke('delete_category', { id });

    categories.update(cats => cats.filter(cat => cat.id !== id));
  } catch (e) {
    console.error('Failed to delete category:', e);
    throw e;
  }
}

// Reorder categories
export async function reorderCategories(categoryIds: number[]) {
  try {
    await invoke('reorder_categories', { categoryIds });

    // Update local order
    categories.update(cats => {
      const ordered = categoryIds
        .map(id => cats.find(cat => cat.id === id))
        .filter(Boolean) as Category[];
      return ordered;
    });
  } catch (e) {
    console.error('Failed to reorder categories:', e);
    throw e;
  }
}

// Initialize
loadCategories();
```

**Estimated Time:** 1 hour

---

#### Step 5: Update Category UI Components
**Files:**
- `src/lib/components/categories/CategoriesSection.svelte`
- `src/lib/components/categories/CategoryEditPanel.svelte`

**Changes:**
- Remove mock data references
- Add loading states
- Show error messages
- Use real store functions

**Estimated Time:** 45 minutes

---

### **Phase 3: Tags Store Integration** (Steps 6-7)

#### Step 6: Update tagStore.ts
**File:** `src/lib/stores/tagStore.ts`

**Similar to categories, implement:**
```typescript
// Load tags
export async function loadTags()

// Create tag
export async function createTag(name: string, icon: string, color: string)

// Update tag
export async function updateTag(id: number, name: string, icon: string, color: string)

// Delete tag
export async function deleteTag(id: number)

// Add tag to item
export async function addTagToItem(itemId: number, tagId: number)

// Remove tag from item
export async function removeTagFromItem(itemId: number, tagId: number)

// Get tags for specific item
export async function getTagsForItem(itemId: number)
```

**Estimated Time:** 1 hour

---

#### Step 7: Update Tag UI Components
**Files:**
- `src/lib/components/tags/` (various tag components)

**Changes:**
- Connect to real tag operations
- Add loading states
- Handle errors

**Estimated Time:** 45 minutes

---

### **Phase 4: Main UI Integration** (Steps 8-10)

#### Step 8: Update CardsContainer.svelte
**File:** `src/lib/components/cards/CardsContainer.svelte`

**Changes Needed:**
```svelte
<script lang="ts">
  import { clipboardItems, isLoading, loadClipboardItems } from '$lib/stores/clipboardStore';
  import { onMount } from 'svelte';

  // Subscribe to clipboard items
  $: items = $clipboardItems;
  $: loading = $isLoading;

  onMount(() => {
    // Items already loaded by store initialization
    // Just ensure monitoring is running
  });

  // Filter by selected category
  function filterByCategory(category: string) {
    loadClipboardItems(100, category);
  }

  // Search
  function handleSearch(query: string) {
    if (query) {
      searchClipboard(query);
    } else {
      loadClipboardItems();
    }
  }
</script>

{#if loading}
  <div class="loading">Loading clipboard items...</div>
{:else if items.length === 0}
  <div class="empty">
    <p>No clipboard items yet</p>
    <p>Copy something to get started!</p>
  </div>
{:else}
  <!-- Render items -->
  {#each items as item (item.id)}
    <ClipboardCard {item} />
  {/each}
{/if}
```

**Estimated Time:** 1 hour

---

#### Step 9: Update ClipboardCard.svelte
**File:** `src/lib/components/cards/ClipboardCard.svelte`

**Changes:**
- Use real pin/delete operations
- Handle image paths properly (convert to asset URL)
- Add loading states for operations

**Estimated Time:** 45 minutes

---

#### Step 10: Add Error Handling & Loading States
**Files:** Various components

**Add:**
- Loading spinners
- Error toasts
- Retry mechanisms
- Optimistic UI updates

**Estimated Time:** 1 hour

---

## 🧪 Testing Checklist

### Functionality Tests:

**Clipboard Items:**
- [ ] Items load from database on app start
- [ ] New clipboard copies appear in UI (within 2 seconds)
- [ ] Pin/unpin works and persists
- [ ] Delete removes items
- [ ] Multi-select operations work
- [ ] Search filters items correctly

**Categories:**
- [ ] Categories load from database
- [ ] Create custom category works
- [ ] Edit category saves changes
- [ ] Delete category works
- [ ] Filter by category shows correct items
- [ ] Category reordering persists

**Tags:**
- [ ] Tags load from database
- [ ] Create tag works
- [ ] Add/remove tags on items works
- [ ] Filter by tag works
- [ ] Tag editing saves changes

**Real-time Updates:**
- [ ] Copy text → appears in UI within 2 seconds
- [ ] Copy image → appears with thumbnail
- [ ] Copy URL → detected as link category
- [ ] Copy color → detected as color category

---

## ⚠️ Known Issues & Considerations

1. **Polling vs Events:**
   - Currently using polling (check every 2 seconds)
   - Could optimize with Tauri events in future

2. **Image Paths:**
   - Need to convert filesystem paths to `asset://` URLs
   - Use Tauri's convertFileSrc() helper

3. **Performance:**
   - Loading 100 items at once might be slow
   - Consider implementing virtual scrolling later (Module 9)

4. **Error Recovery:**
   - Need clear error messages
   - Add retry buttons for failed operations

---

## 📊 Progress Tracking

| Step | Task | Status | Time |
|------|------|--------|------|
| 1 | Update clipboardStore - Load Items | 📋 Pending | 30min |
| 2 | Add Real-time Monitoring | 📋 Pending | 45min |
| 3 | Implement Item Operations | 📋 Pending | 1h |
| 4 | Update categoryStore | 📋 Pending | 1h |
| 5 | Update Category UI | 📋 Pending | 45min |
| 6 | Update tagStore | 📋 Pending | 1h |
| 7 | Update Tag UI | 📋 Pending | 45min |
| 8 | Update CardsContainer | 📋 Pending | 1h |
| 9 | Update ClipboardCard | 📋 Pending | 45min |
| 10 | Error Handling & Loading | 📋 Pending | 1h |

**Total Estimated Time:** 8-9 hours
**Actual Time:** TBD

---

## 🎯 Success Metrics

- ✅ Zero console errors
- ✅ All mock data removed
- ✅ Real-time clipboard capture working (<2s latency)
- ✅ All CRUD operations functional
- ✅ Smooth user experience with loading states
- ✅ App feels responsive and polished

---

**Module Start Date:** 2025-11-19
**Developer:** Claude Code Assistant
**Status:** Ready to Begin - Waiting for App Build to Complete
