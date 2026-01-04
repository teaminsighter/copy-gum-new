// Settings Service
// Frontend service to interact with Rust settings backend

import { invoke } from '@tauri-apps/api/core';
import type { AppSettings } from '../types/settings';

/**
 * Convert camelCase (TS) to snake_case (Rust)
 */
function toSnakeCase(obj: any): any {
  const converted: any = {};
  for (const key in obj) {
    const snakeKey = key.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
    converted[snakeKey] = obj[key];
  }
  return converted;
}

/**
 * Convert snake_case (Rust) to camelCase (TS)
 */
function toCamelCase(obj: any): any {
  const converted: any = {};
  for (const key in obj) {
    const camelKey = key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
    converted[camelKey] = obj[key];
  }
  return converted;
}

/**
 * Get settings from backend
 */
export async function getSettings(): Promise<AppSettings> {
  try {
    const rustSettings = await invoke<any>('get_settings');
    return toCamelCase(rustSettings);
  } catch (error) {
    console.error('Failed to get settings:', error);
    throw error;
  }
}

/**
 * Save settings to backend
 */
export async function saveSettings(settings: AppSettings): Promise<void> {
  try {
    const rustSettings = toSnakeCase(settings);
    await invoke('save_settings', { settings: rustSettings });
    console.log(' Settings saved successfully');
  } catch (error) {
    console.error('Failed to save settings:', error);
    throw error;
  }
}

/**
 * Reset settings to defaults
 */
export async function resetSettings(): Promise<AppSettings> {
  try {
    const rustSettings = await invoke<any>('reset_settings');
    console.log('= Settings reset to defaults');
    return toCamelCase(rustSettings);
  } catch (error) {
    console.error('Failed to reset settings:', error);
    throw error;
  }
}
