// Toast Store - Manages toast notifications
import { writable } from 'svelte/store';

export interface ToastMessage {
  id: number;
  message: string;
  type: 'success' | 'error' | 'info';
  duration?: number;
}

export const toasts = writable<ToastMessage[]>([]);

let nextId = 0;

export function showToast(
  message: string,
  type: 'success' | 'error' | 'info' = 'success',
  duration: number = 3000
) {
  const id = nextId++;
  const toast: ToastMessage = { id, message, type, duration };

  toasts.update(t => [...t, toast]);

  // Auto-remove after duration
  setTimeout(() => {
    removeToast(id);
  }, duration + 300); // Add time for animation
}

export function removeToast(id: number) {
  toasts.update(t => t.filter(toast => toast.id !== id));
}

// Convenience functions
export function showSuccess(message: string, duration?: number) {
  showToast(message, 'success', duration);
}

export function showError(message: string, duration?: number) {
  showToast(message, 'error', duration);
}

export function showInfo(message: string, duration?: number) {
  showToast(message, 'info', duration);
}
