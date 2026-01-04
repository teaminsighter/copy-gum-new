// Keyboard Store
// Manages keyboard navigation state for clipboard items

import { writable, derived, get } from 'svelte/store';
import { filteredItems } from './clipboardStore';

// Currently selected item index
export const selectedItemIndex = writable<number>(-1);

// Currently selected item
export const selectedItem = derived(
  [selectedItemIndex, filteredItems],
  ([$index, $items]) => {
    if ($index >= 0 && $index < $items.length) {
      return $items[$index];
    }
    return null;
  }
);

/**
 * Navigate down to next item
 */
export function navigateDown(): void {
  const items = get(filteredItems);
  selectedItemIndex.update(i => {
    const newIndex = Math.min(i + 1, items.length - 1);
    // If no item selected yet, select first
    if (i === -1 && items.length > 0) return 0;
    return newIndex;
  });
}

/**
 * Navigate up to previous item
 */
export function navigateUp(): void {
  selectedItemIndex.update(i => {
    if (i === -1) return -1; // Don't navigate if nothing selected
    return Math.max(i - 1, 0);
  });
}

/**
 * Reset selection (deselect all)
 */
export function resetSelection(): void {
  selectedItemIndex.set(-1);
}

/**
 * Select first item
 */
export function selectFirst(): void {
  const items = get(filteredItems);
  if (items.length > 0) {
    selectedItemIndex.set(0);
  }
}

/**
 * Select last item
 */
export function selectLast(): void {
  const items = get(filteredItems);
  if (items.length > 0) {
    selectedItemIndex.set(items.length - 1);
  }
}

/**
 * Select item by ID
 */
export function selectItemById(id: number): void {
  const items = get(filteredItems);
  const index = items.findIndex(item => item.id === id);
  if (index !== -1) {
    selectedItemIndex.set(index);
  }
}

/**
 * Get selected item index
 */
export function getSelectedIndex(): number {
  return get(selectedItemIndex);
}

/**
 * Check if an index is selected
 */
export function isIndexSelected(index: number): boolean {
  return get(selectedItemIndex) === index;
}
