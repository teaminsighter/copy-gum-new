// Navigation Store - Manages keyboard navigation state
import { writable } from 'svelte/store';

// Focus layer: 'categories' or 'cards'
export const focusLayer = writable<'categories' | 'cards'>('cards');

// Card navigation
export const selectedCardIndex = writable<number>(-1); // -1 means no card selected
export const totalCards = writable<number>(0);

// Category navigation
export const selectedCategoryIndex = writable<number>(-1); // -1 means no category selected
export const totalCategories = writable<number>(0);

// Card functions
export function selectCard(index: number) {
  selectedCardIndex.set(index);
}

export function selectNextCard() {
  selectedCardIndex.update(current => {
    let total = 0;
    totalCards.subscribe(t => total = t)();
    if (total === 0) return current;
    return current < total - 1 ? current + 1 : current;
  });
}

export function selectPreviousCard() {
  selectedCardIndex.update(current => {
    return current > 0 ? current - 1 : current;
  });
}

export function clearCardSelection() {
  selectedCardIndex.set(-1);
}

// Category functions
export function selectCategory(index: number) {
  selectedCategoryIndex.set(index);
}

export function selectNextCategory() {
  selectedCategoryIndex.update(current => {
    let total = 0;
    totalCategories.subscribe(t => total = t)();
    if (total === 0) return current;
    return current < total - 1 ? current + 1 : current;
  });
}

export function selectPreviousCategory() {
  selectedCategoryIndex.update(current => {
    return current > 0 ? current - 1 : current;
  });
}

export function clearCategorySelection() {
  selectedCategoryIndex.set(-1);
}

// Layer switching
export function switchToCategories() {
  focusLayer.set('categories');
  // If no category selected, select first one
  selectedCategoryIndex.update(current => current === -1 ? 0 : current);
}

export function switchToCards() {
  focusLayer.set('cards');
  // If no card selected, select first one
  selectedCardIndex.update(current => current === -1 ? 0 : current);
}

// Clear all selections
export function clearAllSelections() {
  selectedCardIndex.set(-1);
  selectedCategoryIndex.set(-1);
  focusLayer.set('cards');
}
