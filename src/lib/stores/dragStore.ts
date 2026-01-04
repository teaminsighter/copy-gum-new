// Drag State Store
// Coordinates drag state between CardsContainer and ClipboardCard
// Prevents accidental card copies during/after drag operations

import { writable, get } from 'svelte/store';

// Track if a drag operation is currently happening
export const isDragging = writable(false);

// Track if a drag just finished (used to block clicks briefly after drag ends)
export const wasDragging = writable(false);

// Drag threshold in pixels - movement less than this is considered a click
export const DRAG_THRESHOLD = 5;

// Time in ms to block clicks after drag ends
const CLICK_BLOCK_DELAY = 100;

let clickBlockTimeout: ReturnType<typeof setTimeout> | null = null;

/**
 * Start potential drag operation
 * Called on mousedown - doesn't immediately set dragging state
 */
export function startPotentialDrag() {
  // Reset wasDragging at the start of a new interaction
  wasDragging.set(false);
}

/**
 * Confirm that actual dragging has started (movement exceeded threshold)
 */
export function confirmDragging() {
  isDragging.set(true);
  wasDragging.set(true);
}

/**
 * End the drag operation
 * Keeps wasDragging=true briefly to block any click events that fire after mouseup
 */
export function endDragging() {
  isDragging.set(false);

  // Clear any existing timeout
  if (clickBlockTimeout) {
    clearTimeout(clickBlockTimeout);
  }

  // Keep wasDragging=true briefly to block clicks
  clickBlockTimeout = setTimeout(() => {
    wasDragging.set(false);
    clickBlockTimeout = null;
  }, CLICK_BLOCK_DELAY);
}

/**
 * Check if a click should be blocked (because user was dragging)
 * Use this in card click handlers
 */
export function shouldBlockClick(): boolean {
  return get(isDragging) || get(wasDragging);
}

/**
 * Reset all drag state (useful for cleanup)
 */
export function resetDragState() {
  isDragging.set(false);
  wasDragging.set(false);
  if (clickBlockTimeout) {
    clearTimeout(clickBlockTimeout);
    clickBlockTimeout = null;
  }
}
