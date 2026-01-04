// Settings Store
// Manages application settings with persistence

import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface AppSettings {
  // General
  auto_start_monitoring: boolean;
  show_on_startup: boolean;
  minimize_to_tray: boolean;

  // Storage
  history_limit: number;
  auto_delete_days: number;
  save_images: boolean;
  max_image_size_mb: number;

  // Appearance
  theme: string;
  card_size: string;
  font_size: number;
  show_thumbnails: boolean;
  density: string;
  accent_color?: string;
  window_opacity: number;
  enable_blur: boolean;

  // Shortcuts
  toggle_window_shortcut: string;
  search_shortcut: string;

  // Privacy
  exclude_apps: string[];
  sensitive_keywords: string[];
  enable_analytics: boolean;

  // First-run
  hasShownOverlayInfo: boolean;
}

// Default settings (matches Rust backend)
const defaultSettings: AppSettings = {
  auto_start_monitoring: true,
  show_on_startup: false,
  minimize_to_tray: true,
  history_limit: 500,
  auto_delete_days: 0,
  save_images: true,
  max_image_size_mb: 10,
  theme: 'auto',
  card_size: 'medium',
  font_size: 14,
  show_thumbnails: true,
  density: 'comfortable',
  accent_color: undefined,
  window_opacity: 100,
  enable_blur: false,
  toggle_window_shortcut: 'CommandOrControl+Shift+V',
  search_shortcut: 'CommandOrControl+F',
  exclude_apps: [],
  sensitive_keywords: ['password', 'secret'],
  enable_analytics: false,
  hasShownOverlayInfo: false,
};

// Stores
export const settings = writable<AppSettings>(defaultSettings);
export const isLoadingSettings = writable<boolean>(false);
export const settingsError = writable<string | null>(null);

/**
 * Load settings from backend
 */
export async function loadSettings(): Promise<void> {
  isLoadingSettings.set(true);
  settingsError.set(null);

  try {
    const loadedSettings = await invoke<AppSettings>('get_settings');
    settings.set(loadedSettings);

    // Apply settings immediately
    applySettings(loadedSettings);
  } catch (e) {
    console.error('Failed to load settings:', e);
    settingsError.set(e as string);
    // Use defaults on error
    settings.set(defaultSettings);
  } finally {
    isLoadingSettings.set(false);
  }
}

/**
 * Save settings to backend
 */
export async function saveSettings(newSettings: AppSettings): Promise<void> {
  isLoadingSettings.set(true);
  settingsError.set(null);

  try {
    await invoke('save_settings', { settings: newSettings });
    settings.set(newSettings);

    // Apply settings immediately
    applySettings(newSettings);
  } catch (e) {
    console.error('Failed to save settings:', e);
    settingsError.set(e as string);
    throw e;
  } finally {
    isLoadingSettings.set(false);
  }
}

/**
 * Reset settings to defaults
 */
export async function resetSettings(): Promise<void> {
  isLoadingSettings.set(true);
  settingsError.set(null);

  try {
    const resetSettings = await invoke<AppSettings>('reset_settings');
    settings.set(resetSettings);

    // Apply settings immediately
    applySettings(resetSettings);
  } catch (e) {
    console.error('Failed to reset settings:', e);
    settingsError.set(e as string);
    throw e;
  } finally {
    isLoadingSettings.set(false);
  }
}

/**
 * Update a single setting
 */
export async function updateSetting<K extends keyof AppSettings>(
  key: K,
  value: AppSettings[K]
): Promise<void> {
  const currentSettings = get(settings);
  const newSettings = { ...currentSettings, [key]: value };
  await saveSettings(newSettings);
}

/**
 * Apply settings to the application
 */
function applySettings(appSettings: AppSettings): void {
  // Apply theme
  applyTheme(appSettings.theme);

  // Apply card size
  applyCardSize(appSettings.card_size);

  // Apply font size
  applyFontSize(appSettings.font_size);

  // Apply density
  applyDensity(appSettings.density);

  // Apply accent color
  if (appSettings.accent_color) {
    applyAccentColor(appSettings.accent_color);
  }

  // Apply window opacity
  applyWindowOpacity(appSettings.window_opacity);

  // Apply blur effect
  applyBlur(appSettings.enable_blur);
}

/**
 * Apply theme to document
 */
function applyTheme(theme: string): void {
  const root = document.documentElement;

  if (theme === 'auto') {
    // Use system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  } else {
    // Support new theme presets: light, dark, high-contrast, nord, dracula, solarized
    root.setAttribute('data-theme', theme);
  }
}

/**
 * Apply card size to document
 */
function applyCardSize(cardSize: string): void {
  const root = document.documentElement;
  root.setAttribute('data-card-size', cardSize);
}

/**
 * Apply font size to document
 */
function applyFontSize(fontSize: number): void {
  const root = document.documentElement;
  root.setAttribute('data-font-size', fontSize.toString());
}

/**
 * Apply density mode to document
 */
function applyDensity(density: string): void {
  const root = document.documentElement;
  root.setAttribute('data-density', density);
}

/**
 * Apply custom accent color to document
 */
function applyAccentColor(color: string): void {
  const root = document.documentElement;
  root.style.setProperty('--accent-color', color);
}

/**
 * Apply window opacity to document
 */
function applyWindowOpacity(opacity: number): void {
  const root = document.documentElement;
  root.style.setProperty('--window-opacity', `${opacity / 100}`);
}

/**
 * Apply blur effect to document
 */
function applyBlur(enableBlur: boolean): void {
  const root = document.documentElement;
  if (enableBlur) {
    root.classList.add('blur-enabled');
  } else {
    root.classList.remove('blur-enabled');
  }
}

/**
 * Listen for settings changes from backend
 */
export async function initSettingsStore(): Promise<void> {
  // Load initial settings
  await loadSettings();

  // Listen for settings changes from backend
  await listen<AppSettings>('settings-changed', (event) => {
    settings.set(event.payload);
    applySettings(event.payload);
  });

  // Listen for system theme changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    const currentSettings = get(settings);
    if (currentSettings.theme === 'auto') {
      applyTheme('auto');
    }
  });
}

// Auto-initialize when module loads
if (typeof window !== 'undefined') {
  initSettingsStore().catch(err => {
    console.error('Failed to initialize settings store:', err);
  });
}
