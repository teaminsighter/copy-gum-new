import { describe, it, expect } from 'vitest';
import { defaultSettings, type AppSettings } from '../types/settings';

describe('defaultSettings', () => {
  describe('General settings', () => {
    it('has auto start monitoring enabled by default', () => {
      expect(defaultSettings.autoStartMonitoring).toBe(true);
    });

    it('does not show on startup by default', () => {
      expect(defaultSettings.showOnStartup).toBe(false);
    });

    it('minimizes to tray by default', () => {
      expect(defaultSettings.minimizeToTray).toBe(true);
    });

    it('centers window by default', () => {
      expect(defaultSettings.windowPosition).toBe('center');
    });
  });

  describe('Storage settings', () => {
    it('has history limit of 500 by default', () => {
      expect(defaultSettings.historyLimit).toBe(500);
    });

    it('never auto-deletes by default (0 days)', () => {
      expect(defaultSettings.autoDeleteDays).toBe(0);
    });

    it('saves images by default', () => {
      expect(defaultSettings.saveImages).toBe(true);
    });

    it('has max image size of 10MB by default', () => {
      expect(defaultSettings.maxImageSizeMb).toBe(10);
    });
  });

  describe('Appearance settings', () => {
    it('uses auto theme by default', () => {
      expect(defaultSettings.theme).toBe('auto');
    });

    it('uses medium card size by default', () => {
      expect(defaultSettings.cardSize).toBe('medium');
    });

    it('uses 14px font size by default', () => {
      expect(defaultSettings.fontSize).toBe(14);
    });

    it('shows thumbnails by default', () => {
      expect(defaultSettings.showThumbnails).toBe(true);
    });
  });

  describe('Shortcuts', () => {
    it('has CommandOrControl+Shift+V as toggle shortcut', () => {
      expect(defaultSettings.toggleWindowShortcut).toBe('CommandOrControl+Shift+V');
    });

    it('has CommandOrControl+F as search shortcut', () => {
      expect(defaultSettings.searchShortcut).toBe('CommandOrControl+F');
    });
  });

  describe('Privacy settings', () => {
    it('has no excluded apps by default', () => {
      expect(defaultSettings.excludeApps).toEqual([]);
    });

    it('has default sensitive keywords', () => {
      expect(defaultSettings.sensitiveKeywords).toContain('password');
      expect(defaultSettings.sensitiveKeywords).toContain('secret');
      expect(defaultSettings.sensitiveKeywords).toContain('api_key');
      expect(defaultSettings.sensitiveKeywords).toContain('token');
    });

    it('has analytics disabled by default', () => {
      expect(defaultSettings.enableAnalytics).toBe(false);
    });
  });

  describe('Type compliance', () => {
    it('defaultSettings conforms to AppSettings type', () => {
      const settings: AppSettings = defaultSettings;

      // Type checks are compile-time, but we can verify required properties exist
      expect(settings.autoStartMonitoring).toBeDefined();
      expect(settings.showOnStartup).toBeDefined();
      expect(settings.minimizeToTray).toBeDefined();
      expect(settings.historyLimit).toBeDefined();
      expect(settings.autoDeleteDays).toBeDefined();
      expect(settings.saveImages).toBeDefined();
      expect(settings.maxImageSizeMb).toBeDefined();
      expect(settings.theme).toBeDefined();
      expect(settings.cardSize).toBeDefined();
      expect(settings.fontSize).toBeDefined();
      expect(settings.showThumbnails).toBeDefined();
      expect(settings.toggleWindowShortcut).toBeDefined();
      expect(settings.searchShortcut).toBeDefined();
      expect(settings.excludeApps).toBeDefined();
      expect(settings.sensitiveKeywords).toBeDefined();
      expect(settings.enableAnalytics).toBeDefined();
    });
  });
});
