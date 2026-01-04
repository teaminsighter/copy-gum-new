// Settings Types
// TypeScript interfaces matching the Rust AppSettings struct

export interface AppSettings {
  // General settings
  autoStartMonitoring: boolean;
  showOnStartup: boolean;
  minimizeToTray: boolean;
  windowPosition: string; // 'center' | 'remember' | 'top-right'

  // Storage settings
  historyLimit: number;      // 100, 500, 1000, -1 (unlimited)
  autoDeleteDays: number;    // 0 (never), 7, 30, 90
  saveImages: boolean;
  maxImageSizeMb: number;

  // Appearance settings
  theme: 'light' | 'dark' | 'auto';
  cardSize: 'small' | 'medium' | 'large';
  fontSize: number;          // 12, 14, 16, 18
  showThumbnails: boolean;

  // Keyboard shortcuts
  toggleWindowShortcut: string;  // 'CommandOrControl+Shift+V'
  searchShortcut: string;        // 'CommandOrControl+F'

  // Privacy settings
  excludeApps: string[];         // Apps to not capture from
  sensitiveKeywords: string[];   // Auto-hide items with these
  enableAnalytics: boolean;
}

// Default settings matching Rust defaults
export const defaultSettings: AppSettings = {
  // General defaults
  autoStartMonitoring: true,
  showOnStartup: false,
  minimizeToTray: true,
  windowPosition: 'center',

  // Storage defaults
  historyLimit: 500,
  autoDeleteDays: 0,
  saveImages: true,
  maxImageSizeMb: 10,

  // Appearance defaults
  theme: 'auto',
  cardSize: 'medium',
  fontSize: 14,
  showThumbnails: true,

  // Shortcut defaults
  toggleWindowShortcut: 'CommandOrControl+Shift+V',
  searchShortcut: 'CommandOrControl+F',

  // Privacy defaults
  excludeApps: [],
  sensitiveKeywords: ['password', 'secret', 'api_key', 'token'],
  enableAnalytics: false,
};
