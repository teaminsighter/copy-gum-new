# CopyGum - Modules 7-13 Implementation Plan

**Date:** 2025-11-19
**Status:** Planning Phase
**Completed Modules:** 1-6 ✅
**Remaining Modules:** 7-13 (7 modules)

---

## Module Status Overview

| Module | Name | Status | Priority | Estimated Time | Dependencies |
|--------|------|--------|----------|----------------|--------------|
| 1 | Project Setup | ✅ Complete | - | - | - |
| 2 | UI Design | ✅ Complete | - | - | Module 1 |
| 3 | Core Components | ✅ Complete | - | - | Module 2 |
| 4 | Database Schema | ✅ Complete | - | - | Module 1 |
| 5 | Clipboard Monitoring | ✅ Complete | - | - | Module 4 |
| 6 | Frontend-Backend Integration | ✅ Complete | - | - | Modules 4-5 |
| **7** | **Settings & Preferences** | 📋 Planned | P1 | 2-3h | Module 6 |
| **8** | **Search & Filter** | 📋 Planned | P2 | 1-2h | Module 6 |
| **9** | **Keyboard Shortcuts** | 📋 Planned | P2 | 2-3h | Module 6 |
| **10** | **Export/Import** | 📋 Planned | P3 | 3-4h | Module 6 |
| **11** | **Appearance Customization** | 📋 Planned | P3 | 2-3h | Module 7 |
| **12** | **Testing & Quality** | 📋 Planned | P1 | 4-6h | Modules 7-11 |
| **13** | **Build & Distribution** | 📋 Planned | P0 | 3-4h | Module 12 |

**Total Estimated Time:** 20-30 hours

---

# Module 7: Settings & Preferences

## Overview
Implement a comprehensive settings system allowing users to customize app behavior, appearance, and features.

## Priority: P1 (High)
**Why:** Users need to control app behavior (startup, monitoring, storage limits)

## Estimated Time: 2-3 hours

## Dependencies
- Module 6 (database layer must work)
- SettingsDropdown.svelte (already exists in UI)

---

## Implementation Plan

### 7.1 Backend: Settings Storage (45 min)

**Create:** `/src-tauri/src/settings.rs`

```rust
// Settings storage using serde + file system
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    // General
    pub auto_start_monitoring: bool,
    pub show_on_startup: bool,
    pub minimize_to_tray: bool,

    // Storage
    pub history_limit: i32,  // 100, 500, 1000, -1 (unlimited)
    pub auto_delete_days: i32,  // 0 (never), 7, 30, 90
    pub save_images: bool,
    pub max_image_size_mb: i32,

    // Appearance
    pub theme: String,  // "light", "dark", "auto"
    pub card_size: String,  // "small", "medium", "large"
    pub font_size: i32,  // 12, 14, 16, 18
    pub show_thumbnails: bool,

    // Shortcuts
    pub toggle_window_shortcut: String,  // "CommandOrControl+Shift+V"
    pub search_shortcut: String,  // "CommandOrControl+F"

    // Privacy
    pub exclude_apps: Vec<String>,  // Apps to not capture from
    pub sensitive_keywords: Vec<String>,  // Auto-hide items with these
    pub enable_analytics: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_start_monitoring: true,
            show_on_startup: false,
            minimize_to_tray: true,
            history_limit: 500,
            auto_delete_days: 0,
            save_images: true,
            max_image_size_mb: 10,
            theme: "auto".to_string(),
            card_size: "medium".to_string(),
            font_size: 14,
            show_thumbnails: true,
            toggle_window_shortcut: "CommandOrControl+Shift+V".to_string(),
            search_shortcut: "CommandOrControl+F".to_string(),
            exclude_apps: vec![],
            sensitive_keywords: vec!["password".to_string(), "secret".to_string()],
            enable_analytics: false,
        }
    }
}

impl AppSettings {
    pub fn load(app: &AppHandle) -> Result<Self, String> {
        let settings_path = Self::get_settings_path(app)?;

        if settings_path.exists() {
            let contents = fs::read_to_string(&settings_path)
                .map_err(|e| format!("Failed to read settings: {}", e))?;

            let settings: AppSettings = serde_json::from_str(&contents)
                .map_err(|e| format!("Failed to parse settings: {}", e))?;

            Ok(settings)
        } else {
            Ok(AppSettings::default())
        }
    }

    pub fn save(&self, app: &AppHandle) -> Result<(), String> {
        let settings_path = Self::get_settings_path(app)?;

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&settings_path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;

        Ok(())
    }

    fn get_settings_path(app: &AppHandle) -> Result<PathBuf, String> {
        let app_data = app.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        fs::create_dir_all(&app_data)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;

        Ok(app_data.join("settings.json"))
    }
}

// Tauri commands
#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    AppSettings::load(&app)
}

#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    settings.save(&app)?;

    // Emit event so other parts of app can react
    let _ = app.emit("settings-changed", &settings);

    Ok(())
}

#[tauri::command]
pub async fn reset_settings(app: AppHandle) -> Result<AppSettings, String> {
    let default = AppSettings::default();
    default.save(&app)?;
    Ok(default)
}
```

**Update:** `/src-tauri/src/main.rs`
```rust
mod settings;

fn main() {
    tauri::Builder::default()
        // ... existing plugins
        .invoke_handler(tauri::generate_handler![
            // ... existing commands
            settings::get_settings,
            settings::save_settings,
            settings::reset_settings,
        ])
        // ... rest of setup
}
```

**Update:** `/src-tauri/Cargo.toml`
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### 7.2 Frontend: Settings Store (30 min)

**Create:** `/src/lib/stores/settingsStore.ts`

```typescript
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

  // Shortcuts
  toggle_window_shortcut: string;
  search_shortcut: string;

  // Privacy
  exclude_apps: string[];
  sensitive_keywords: string[];
  enable_analytics: boolean;
}

// Default settings
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
  toggle_window_shortcut: 'CommandOrControl+Shift+V',
  search_shortcut: 'CommandOrControl+F',
  exclude_apps: [],
  sensitive_keywords: ['password', 'secret'],
  enable_analytics: false,
};

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

    // Apply theme immediately
    applyTheme(loadedSettings.theme);
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

    // Apply theme immediately
    applyTheme(newSettings.theme);

    console.log('✅ Settings saved');
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

    applyTheme(resetSettings.theme);

    console.log('✅ Settings reset to defaults');
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
 * Apply theme to document
 */
function applyTheme(theme: string): void {
  const root = document.documentElement;

  if (theme === 'auto') {
    // Use system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  } else {
    root.setAttribute('data-theme', theme);
  }
}

/**
 * Listen for settings changes from backend
 */
export async function initSettingsStore(): Promise<void> {
  // Load initial settings
  await loadSettings();

  // Listen for settings changes
  await listen<AppSettings>('settings-changed', (event) => {
    settings.set(event.payload);
    applyTheme(event.payload.theme);
  });

  // Listen for system theme changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    const currentSettings = get(settings);
    if (currentSettings.theme === 'auto') {
      applyTheme('auto');
    }
  });
}

// Auto-initialize
if (typeof window !== 'undefined') {
  initSettingsStore().catch(err => {
    console.error('Failed to initialize settings store:', err);
  });
}
```

---

### 7.3 Frontend: Update SettingsDropdown (45 min)

**Update:** `/src/lib/components/header/SettingsDropdown.svelte`

Connect the existing UI to the settings store:

```svelte
<script lang="ts">
  import { settings, saveSettings, resetSettings } from '$lib/stores/settingsStore';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let localSettings = $settings;

  // Watch for external changes
  $: localSettings = $settings;

  async function handleSave() {
    try {
      await saveSettings(localSettings);
      dispatch('close');
    } catch (e) {
      alert('Failed to save settings: ' + e);
    }
  }

  async function handleReset() {
    if (confirm('Reset all settings to defaults?')) {
      try {
        const defaults = await resetSettings();
        localSettings = defaults;
      } catch (e) {
        alert('Failed to reset settings: ' + e);
      }
    }
  }
</script>

<div class="settings-panel">
  <!-- General Settings -->
  <section>
    <h3>General</h3>

    <label>
      <input type="checkbox" bind:checked={localSettings.auto_start_monitoring} />
      Auto-start clipboard monitoring
    </label>

    <label>
      <input type="checkbox" bind:checked={localSettings.show_on_startup} />
      Show window on startup
    </label>

    <label>
      <input type="checkbox" bind:checked={localSettings.minimize_to_tray} />
      Minimize to system tray
    </label>
  </section>

  <!-- Storage Settings -->
  <section>
    <h3>Storage</h3>

    <label>
      History Limit
      <select bind:value={localSettings.history_limit}>
        <option value={100}>100 items</option>
        <option value={500}>500 items</option>
        <option value={1000}>1,000 items</option>
        <option value={-1}>Unlimited</option>
      </select>
    </label>

    <label>
      Auto-delete items older than
      <select bind:value={localSettings.auto_delete_days}>
        <option value={0}>Never</option>
        <option value={7}>7 days</option>
        <option value={30}>30 days</option>
        <option value={90}>90 days</option>
      </select>
    </label>

    <label>
      <input type="checkbox" bind:checked={localSettings.save_images} />
      Save clipboard images
    </label>

    <label>
      Max image size (MB)
      <input type="number" bind:value={localSettings.max_image_size_mb} min="1" max="100" />
    </label>
  </section>

  <!-- Appearance Settings -->
  <section>
    <h3>Appearance</h3>

    <label>
      Theme
      <select bind:value={localSettings.theme}>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
        <option value="auto">Auto (System)</option>
      </select>
    </label>

    <label>
      Card Size
      <select bind:value={localSettings.card_size}>
        <option value="small">Small</option>
        <option value="medium">Medium</option>
        <option value="large">Large</option>
      </select>
    </label>

    <label>
      Font Size
      <select bind:value={localSettings.font_size}>
        <option value={12}>12px</option>
        <option value={14}>14px</option>
        <option value={16}>16px</option>
        <option value={18}>18px</option>
      </select>
    </label>

    <label>
      <input type="checkbox" bind:checked={localSettings.show_thumbnails} />
      Show image thumbnails
    </label>
  </section>

  <!-- Privacy Settings -->
  <section>
    <h3>Privacy</h3>

    <label>
      Exclude Apps (comma-separated)
      <input
        type="text"
        value={localSettings.exclude_apps.join(', ')}
        on:change={(e) => localSettings.exclude_apps = e.currentTarget.value.split(',').map(s => s.trim())}
        placeholder="Keychain Access, 1Password"
      />
    </label>

    <label>
      <input type="checkbox" bind:checked={localSettings.enable_analytics} />
      Enable anonymous analytics
    </label>
  </section>

  <!-- Actions -->
  <div class="settings-actions">
    <button on:click={handleReset} class="btn-secondary">Reset to Defaults</button>
    <button on:click={handleSave} class="btn-primary">Save Settings</button>
  </div>
</div>
```

---

### 7.4 Apply Settings in App (30 min)

**Update:** `/src/lib/stores/clipboardStore.ts`

Apply settings when monitoring starts:

```typescript
import { settings } from './settingsStore';
import { get } from 'svelte/store';

export async function initClipboardStore(): Promise<void> {
  const appSettings = get(settings);

  // Load initial items with history limit
  await loadClipboardItems(appSettings.history_limit > 0 ? appSettings.history_limit : 100);

  // Auto-start monitoring if enabled
  if (appSettings.auto_start_monitoring) {
    await startClipboardMonitoring();
  }
}
```

**Apply theme CSS variables:**

Update `/src/app.css` or create `/src/lib/styles/themes.css`:

```css
:root[data-theme="light"] {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --text-primary: #1a1a1a;
  --text-secondary: #666666;
  --accent: #007aff;
  /* ... more variables */
}

:root[data-theme="dark"] {
  --bg-primary: #1a1a1a;
  --bg-secondary: #2d2d2d;
  --text-primary: #ffffff;
  --text-secondary: #aaaaaa;
  --accent: #0a84ff;
  /* ... more variables */
}

/* Card sizes */
.card-small {
  padding: 8px;
  font-size: 12px;
}

.card-medium {
  padding: 12px;
  font-size: 14px;
}

.card-large {
  padding: 16px;
  font-size: 16px;
}
```

---

## Testing Checklist

### 7.5 Testing (30 min)

- [ ] Settings load on app startup
- [ ] Settings persist after app restart
- [ ] Theme changes apply immediately
- [ ] History limit works (test with 10 items, set limit to 5)
- [ ] Auto-delete works (set to 1 day, verify old items deleted)
- [ ] Excluded apps don't capture clipboard
- [ ] Card size changes affect UI
- [ ] Font size changes affect UI
- [ ] Reset to defaults works
- [ ] Settings validation (can't set negative values)

---

## Deliverables

- [x] Backend settings module (`settings.rs`)
- [x] Frontend settings store (`settingsStore.ts`)
- [x] Updated SettingsDropdown UI
- [x] Theme system working
- [x] Settings persistence working
- [ ] Tests written
- [ ] Documentation updated

---

# Module 8: Search & Filter Enhancement

## Overview
Enhance search functionality with advanced filters, debouncing, and better UX.

## Priority: P2 (Medium)

## Estimated Time: 1-2 hours

## Dependencies
- Module 6 (database search already implemented)
- Module 7 (settings for search behavior)

---

## Implementation Plan

### 8.1 Debounced Search (20 min)

**Update:** `/src/lib/components/header/SearchBar.svelte`

```svelte
<script lang="ts">
  import { searchQuery } from '$lib/stores/clipboardStore';
  import { debounce } from '$lib/utils/debounce';

  let inputValue = '';

  // Debounce search by 300ms
  const debouncedSearch = debounce((value: string) => {
    searchQuery.set(value);
  }, 300);

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    inputValue = target.value;
    debouncedSearch(inputValue);
  }

  function handleClear() {
    inputValue = '';
    searchQuery.set('');
  }
</script>

<div class="search-bar">
  <input
    type="text"
    placeholder="Search clipboard..."
    value={inputValue}
    on:input={handleInput}
  />
  {#if inputValue}
    <button on:click={handleClear} class="clear-btn">×</button>
  {/if}
</div>
```

**Create:** `/src/lib/utils/debounce.ts`

```typescript
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;

  return function executedFunction(...args: Parameters<T>) {
    const later = () => {
      timeout = null;
      func(...args);
    };

    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}
```

---

### 8.2 Advanced Filters (40 min)

**Update:** `/src/lib/stores/clipboardStore.ts`

Add advanced filter options:

```typescript
export const filterOptions = writable({
  categories: [] as string[],
  dateRange: {
    start: null as Date | null,
    end: null as Date | null,
  },
  isPinned: null as boolean | null,
  contentTypes: [] as string[],
  tags: [] as number[],
});

// Enhanced filtered items
export const filteredItems = derived(
  [clipboardItems, selectedCategory, searchQuery, filterOptions],
  ([$items, $category, $search, $filters]) => {
    let filtered = $items;

    // Filter by category
    if ($category && $category !== 'all') {
      filtered = filtered.filter(item => item.category === $category);
    }

    // Filter by multiple categories (advanced)
    if ($filters.categories.length > 0) {
      filtered = filtered.filter(item =>
        $filters.categories.includes(item.category)
      );
    }

    // Filter by date range
    if ($filters.dateRange.start || $filters.dateRange.end) {
      filtered = filtered.filter(item => {
        const itemDate = new Date(item.timestamp);
        const afterStart = !$filters.dateRange.start || itemDate >= $filters.dateRange.start;
        const beforeEnd = !$filters.dateRange.end || itemDate <= $filters.dateRange.end;
        return afterStart && beforeEnd;
      });
    }

    // Filter by pinned status
    if ($filters.isPinned !== null) {
      filtered = filtered.filter(item => item.is_pinned === $filters.isPinned);
    }

    // Filter by content types
    if ($filters.contentTypes.length > 0) {
      filtered = filtered.filter(item =>
        $filters.contentTypes.includes(item.content_type)
      );
    }

    // Filter by search (client-side for instant feedback)
    if ($search) {
      const query = $search.toLowerCase();
      filtered = filtered.filter(item =>
        item.content.toLowerCase().includes(query)
      );
    }

    return filtered;
  }
);
```

**Create:** `/src/lib/components/filters/AdvancedFilters.svelte`

```svelte
<script lang="ts">
  import { filterOptions } from '$lib/stores/clipboardStore';

  let showFilters = false;

  function toggleFilters() {
    showFilters = !showFilters;
  }

  function clearFilters() {
    filterOptions.set({
      categories: [],
      dateRange: { start: null, end: null },
      isPinned: null,
      contentTypes: [],
      tags: [],
    });
  }
</script>

<div class="advanced-filters">
  <button on:click={toggleFilters} class="filter-toggle">
    <span>Filters</span>
    {#if $filterOptions.categories.length > 0 || $filterOptions.contentTypes.length > 0}
      <span class="filter-badge">{$filterOptions.categories.length + $filterOptions.contentTypes.length}</span>
    {/if}
  </button>

  {#if showFilters}
    <div class="filter-panel">
      <!-- Date Range -->
      <div class="filter-section">
        <h4>Date Range</h4>
        <input
          type="date"
          bind:value={$filterOptions.dateRange.start}
          placeholder="From"
        />
        <input
          type="date"
          bind:value={$filterOptions.dateRange.end}
          placeholder="To"
        />
      </div>

      <!-- Pin Status -->
      <div class="filter-section">
        <h4>Pin Status</h4>
        <label>
          <input
            type="radio"
            name="pinned"
            value={null}
            checked={$filterOptions.isPinned === null}
            on:change={() => $filterOptions.isPinned = null}
          />
          All
        </label>
        <label>
          <input
            type="radio"
            name="pinned"
            value={true}
            checked={$filterOptions.isPinned === true}
            on:change={() => $filterOptions.isPinned = true}
          />
          Pinned Only
        </label>
        <label>
          <input
            type="radio"
            name="pinned"
            value={false}
            checked={$filterOptions.isPinned === false}
            on:change={() => $filterOptions.isPinned = false}
          />
          Unpinned Only
        </label>
      </div>

      <!-- Content Types -->
      <div class="filter-section">
        <h4>Content Type</h4>
        {#each ['text', 'url', 'email', 'code', 'color', 'phone'] as type}
          <label>
            <input
              type="checkbox"
              value={type}
              checked={$filterOptions.contentTypes.includes(type)}
              on:change={(e) => {
                if (e.currentTarget.checked) {
                  $filterOptions.contentTypes = [...$filterOptions.contentTypes, type];
                } else {
                  $filterOptions.contentTypes = $filterOptions.contentTypes.filter(t => t !== type);
                }
              }}
            />
            {type}
          </label>
        {/each}
      </div>

      <button on:click={clearFilters} class="clear-filters">Clear All Filters</button>
    </div>
  {/if}
</div>
```

---

### 8.3 Search Keyboard Shortcut (20 min)

**Update:** `/src/routes/+page.svelte` or main app component

```typescript
import { onMount } from 'svelte';

onMount(() => {
  const handleKeydown = (e: KeyboardEvent) => {
    // Cmd+F or Ctrl+F to focus search
    if ((e.metaKey || e.ctrlKey) && e.key === 'f') {
      e.preventDefault();
      const searchInput = document.querySelector('input[type="text"]') as HTMLInputElement;
      searchInput?.focus();
    }
  };

  window.addEventListener('keydown', handleKeydown);

  return () => {
    window.removeEventListener('keydown', handleKeydown);
  };
});
```

---

## Testing Checklist

- [ ] Search debouncing works (type fast, only searches after 300ms)
- [ ] Search clears properly
- [ ] Advanced filters work individually
- [ ] Advanced filters work in combination
- [ ] Date range filtering accurate
- [ ] Pin status filter works
- [ ] Content type filter works
- [ ] Cmd+F focuses search input
- [ ] Filter badge shows correct count

---

# Module 9: Keyboard Shortcuts

## Overview
Implement comprehensive keyboard navigation and customizable shortcuts.

## Priority: P2 (Medium)

## Estimated Time: 2-3 hours

## Dependencies
- Module 6 (clipboard store)
- Module 7 (settings for custom shortcuts)

---

## Implementation Plan

### 9.1 Keyboard Navigation System (60 min)

**Create:** `/src/lib/utils/keyboard.ts`

```typescript
export type KeyboardShortcut = {
  key: string;
  ctrlKey?: boolean;
  metaKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
  action: () => void;
  description: string;
};

export class KeyboardManager {
  private shortcuts: Map<string, KeyboardShortcut> = new Map();

  register(id: string, shortcut: KeyboardShortcut) {
    this.shortcuts.set(id, shortcut);
  }

  unregister(id: string) {
    this.shortcuts.delete(id);
  }

  handleKeydown(e: KeyboardEvent): boolean {
    for (const [, shortcut] of this.shortcuts) {
      const keyMatch = e.key.toLowerCase() === shortcut.key.toLowerCase();
      const ctrlMatch = !shortcut.ctrlKey || e.ctrlKey === shortcut.ctrlKey;
      const metaMatch = !shortcut.metaKey || e.metaKey === shortcut.metaKey;
      const shiftMatch = !shortcut.shiftKey || e.shiftKey === shortcut.shiftKey;
      const altMatch = !shortcut.altKey || e.altKey === shortcut.altKey;

      if (keyMatch && ctrlMatch && metaMatch && shiftMatch && altMatch) {
        e.preventDefault();
        shortcut.action();
        return true;
      }
    }
    return false;
  }

  getAll(): Map<string, KeyboardShortcut> {
    return this.shortcuts;
  }
}

export const keyboardManager = new KeyboardManager();
```

---

### 9.2 Implement App-Wide Shortcuts (60 min)

**Create:** `/src/lib/composables/useKeyboard.ts`

```typescript
import { onMount, onDestroy } from 'svelte';
import { keyboardManager, type KeyboardShortcut } from '$lib/utils/keyboard';
import { goto } from '$app/navigation';
import {
  searchQuery,
  deleteItem,
  togglePin,
  clipboardItems
} from '$lib/stores/clipboardStore';

export function useKeyboardShortcuts() {
  let selectedIndex = 0;

  onMount(() => {
    // Register global shortcuts
    keyboardManager.register('search', {
      key: 'f',
      metaKey: true,
      action: () => {
        const searchInput = document.querySelector('input[type="text"]') as HTMLInputElement;
        searchInput?.focus();
      },
      description: 'Focus search'
    });

    keyboardManager.register('escape', {
      key: 'Escape',
      action: () => {
        // Close window or clear search
        const searchInput = document.querySelector('input[type="text"]') as HTMLInputElement;
        if (searchInput?.value) {
          searchQuery.set('');
          searchInput.value = '';
        } else {
          // Hide window
          window.__TAURI__?.window.getCurrent().hide();
        }
      },
      description: 'Close window or clear search'
    });

    keyboardManager.register('selectAll', {
      key: 'a',
      metaKey: true,
      action: () => {
        // Select all items logic
        console.log('Select all items');
      },
      description: 'Select all items'
    });

    keyboardManager.register('arrowDown', {
      key: 'ArrowDown',
      action: () => {
        // Navigate down
        const items = document.querySelectorAll('.clipboard-card');
        selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
        items[selectedIndex]?.scrollIntoView({ block: 'nearest' });
        items[selectedIndex]?.classList.add('keyboard-selected');
      },
      description: 'Navigate down'
    });

    keyboardManager.register('arrowUp', {
      key: 'ArrowUp',
      action: () => {
        // Navigate up
        const items = document.querySelectorAll('.clipboard-card');
        selectedIndex = Math.max(selectedIndex - 1, 0);
        items[selectedIndex]?.scrollIntoView({ block: 'nearest' });
        items[selectedIndex]?.classList.add('keyboard-selected');
      },
      description: 'Navigate up'
    });

    keyboardManager.register('enter', {
      key: 'Enter',
      action: async () => {
        // Copy selected item and close
        const items = clipboardItems;
        // Implementation needed
      },
      description: 'Copy selected item'
    });

    keyboardManager.register('delete', {
      key: 'Backspace',
      metaKey: true,
      action: () => {
        // Delete selected item
        console.log('Delete selected item');
      },
      description: 'Delete selected item'
    });

    keyboardManager.register('pin', {
      key: 'p',
      metaKey: true,
      action: () => {
        // Pin/unpin selected item
        console.log('Toggle pin');
      },
      description: 'Pin/unpin selected item'
    });

    // Global keydown listener
    const handleKeydown = (e: KeyboardEvent) => {
      // Don't intercept if typing in input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        // Allow Escape and Cmd+A even in inputs
        if (e.key !== 'Escape' && !(e.key === 'a' && e.metaKey)) {
          return;
        }
      }

      keyboardManager.handleKeydown(e);
    };

    window.addEventListener('keydown', handleKeydown);

    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });

  onDestroy(() => {
    // Cleanup shortcuts
    keyboardManager.unregister('search');
    keyboardManager.unregister('escape');
    // ... unregister all
  });
}
```

---

### 9.3 Shortcuts Help Panel (40 min)

**Create:** `/src/lib/components/help/ShortcutsHelp.svelte`

```svelte
<script lang="ts">
  import { keyboardManager } from '$lib/utils/keyboard';

  let showHelp = false;

  function toggleHelp() {
    showHelp = !showHelp;
  }

  // Register ? shortcut to open help
  import { onMount } from 'svelte';
  onMount(() => {
    keyboardManager.register('help', {
      key: '?',
      shiftKey: true,
      action: toggleHelp,
      description: 'Show keyboard shortcuts'
    });
  });

  const shortcuts = [
    { keys: ['Cmd/Ctrl', 'Shift', 'V'], desc: 'Toggle window' },
    { keys: ['Cmd/Ctrl', 'F'], desc: 'Focus search' },
    { keys: ['↑', '↓'], desc: 'Navigate items' },
    { keys: ['Enter'], desc: 'Copy selected item' },
    { keys: ['Cmd/Ctrl', 'P'], desc: 'Pin/unpin item' },
    { keys: ['Cmd/Ctrl', 'Backspace'], desc: 'Delete item' },
    { keys: ['Cmd/Ctrl', 'A'], desc: 'Select all' },
    { keys: ['Esc'], desc: 'Close window' },
    { keys: ['?'], desc: 'Show this help' },
  ];
</script>

{#if showHelp}
  <div class="shortcuts-help-overlay" on:click={toggleHelp}>
    <div class="shortcuts-help-panel" on:click|stopPropagation>
      <h2>Keyboard Shortcuts</h2>

      <div class="shortcuts-list">
        {#each shortcuts as shortcut}
          <div class="shortcut-row">
            <div class="shortcut-keys">
              {#each shortcut.keys as key}
                <kbd>{key}</kbd>
              {/each}
            </div>
            <div class="shortcut-desc">{shortcut.desc}</div>
          </div>
        {/each}
      </div>

      <button on:click={toggleHelp}>Close</button>
    </div>
  </div>
{/if}

<style>
  .shortcuts-help-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .shortcuts-help-panel {
    background: var(--bg-primary);
    padding: 24px;
    border-radius: 12px;
    max-width: 500px;
    width: 90%;
  }

  kbd {
    background: var(--bg-secondary);
    padding: 4px 8px;
    border-radius: 4px;
    font-family: monospace;
    margin-right: 4px;
  }

  .shortcut-row {
    display: flex;
    justify-content: space-between;
    padding: 8px 0;
    border-bottom: 1px solid var(--border-color);
  }
</style>
```

---

## Testing Checklist

- [ ] Cmd+F focuses search
- [ ] Arrow keys navigate items
- [ ] Enter copies and closes
- [ ] Cmd+P pins item
- [ ] Cmd+Backspace deletes
- [ ] Escape closes window
- [ ] ? shows help panel
- [ ] Shortcuts don't trigger while typing
- [ ] Visual feedback for selected item

---

**(Continuing with Modules 10-13 in next section...)**

---

# Module 10: Export/Import

## Overview
Allow users to export clipboard history for backup or sharing, and import data from other sources.

## Priority: P3 (Low-Medium)

## Estimated Time: 3-4 hours

## Dependencies
- Module 6 (database access)
- Module 7 (settings for export preferences)

---

## Implementation Plan

### 10.1 Backend: Export Functionality (90 min)

**Create:** `/src-tauri/src/export.rs`

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    version: String,
    export_date: String,
    items: Vec<ExportItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportItem {
    content: String,
    content_type: String,
    category: String,
    timestamp: i64,
    is_pinned: bool,
    tags: Vec<String>,
    // Images as base64 if needed
    image_data: Option<String>,
}

#[tauri::command]
pub async fn export_to_json(
    app: AppHandle,
    file_path: String,
    include_images: bool,
) -> Result<String, String> {
    // Query all clipboard items from database via frontend
    // (Since we use frontend-only SQL, we need to coordinate)

    // For now, return path where file should be saved
    let export_path = PathBuf::from(file_path);

    // Frontend will fetch data and send it back to save
    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn save_export_file(
    file_path: String,
    data: String,
) -> Result<(), String> {
    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to save export file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn import_from_json(
    file_path: String,
) -> Result<String, String> {
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read import file: {}", e))?;

    // Return contents to frontend for processing
    Ok(contents)
}

#[tauri::command]
pub async fn export_to_csv(
    app: AppHandle,
    file_path: String,
) -> Result<(), String> {
    // CSV export for spreadsheet compatibility
    // Frontend will prepare CSV data and call save_export_file
    Ok(())
}
```

**Update:** `/src-tauri/src/main.rs`
```rust
mod export;

.invoke_handler(tauri::generate_handler![
    // ... existing
    export::export_to_json,
    export::save_export_file,
    export::import_from_json,
    export::export_to_csv,
])
```

---

### 10.2 Frontend: Export Service (60 min)

**Create:** `/src/lib/services/export.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { getClipboardItems } from './database';
import type { ClipboardItem } from './database';

export interface ExportData {
  version: string;
  export_date: string;
  items: ClipboardItem[];
}

/**
 * Export clipboard history to JSON
 */
export async function exportToJSON(includeImages: boolean = false): Promise<void> {
  try {
    // Get all clipboard items
    const items = await getClipboardItems(10000); // Get all

    // Prepare export data
    const exportData: ExportData = {
      version: '1.0',
      export_date: new Date().toISOString(),
      items: items.map(item => ({
        ...item,
        // Optionally exclude images to reduce file size
        image_path: includeImages ? item.image_path : undefined,
        image_thumbnail: includeImages ? item.image_thumbnail : undefined,
      })),
    };

    // Show save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${Date.now()}.json`,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) return; // User cancelled

    // Save file
    const jsonData = JSON.stringify(exportData, null, 2);
    await invoke('save_export_file', {
      filePath,
      data: jsonData
    });

    console.log('✅ Exported to:', filePath);
  } catch (error) {
    console.error('Failed to export:', error);
    throw error;
  }
}

/**
 * Export clipboard history to CSV
 */
export async function exportToCSV(): Promise<void> {
  try {
    const items = await getClipboardItems(10000);

    // Convert to CSV
    const headers = ['Content', 'Type', 'Category', 'Date', 'Pinned', 'Tags'];
    const rows = items.map(item => [
      `"${item.content.replace(/"/g, '""')}"`, // Escape quotes
      item.content_type,
      item.category,
      new Date(item.timestamp).toISOString(),
      item.is_pinned ? 'Yes' : 'No',
      item.tags || '',
    ]);

    const csvData = [
      headers.join(','),
      ...rows.map(row => row.join(','))
    ].join('\n');

    // Show save dialog
    const filePath = await save({
      defaultPath: `copygum-export-${Date.now()}.csv`,
      filters: [{
        name: 'CSV',
        extensions: ['csv']
      }]
    });

    if (!filePath) return;

    await invoke('save_export_file', {
      filePath,
      data: csvData
    });

    console.log('✅ Exported to CSV:', filePath);
  } catch (error) {
    console.error('Failed to export CSV:', error);
    throw error;
  }
}

/**
 * Import clipboard history from JSON
 */
export async function importFromJSON(): Promise<number> {
  try {
    // Show open dialog
    const filePath = await open({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (!filePath) return 0; // User cancelled

    // Read file
    const contents = await invoke<string>('import_from_json', { filePath });
    const importData: ExportData = JSON.parse(contents);

    // Validate version
    if (importData.version !== '1.0') {
      throw new Error('Unsupported import file version');
    }

    // Import items (using database service)
    let imported = 0;
    for (const item of importData.items) {
      try {
        await saveClipboardItem(
          item.content,
          item.content_type,
          item.category
        );
        imported++;
      } catch (e) {
        console.error('Failed to import item:', e);
      }
    }

    console.log(`✅ Imported ${imported} items`);
    return imported;
  } catch (error) {
    console.error('Failed to import:', error);
    throw error;
  }
}

/**
 * Create a backup of the entire database
 */
export async function createBackup(): Promise<void> {
  try {
    // Export everything including settings
    const items = await getClipboardItems(10000);

    const backup = {
      version: '1.0',
      backup_date: new Date().toISOString(),
      items,
      // Add settings if available
      settings: {}, // TODO: Get from settings store
    };

    const filePath = await save({
      defaultPath: `copygum-backup-${Date.now()}.json`,
      filters: [{
        name: 'Backup',
        extensions: ['json']
      }]
    });

    if (!filePath) return;

    await invoke('save_export_file', {
      filePath,
      data: JSON.stringify(backup, null, 2)
    });

    console.log('✅ Backup created:', filePath);
  } catch (error) {
    console.error('Failed to create backup:', error);
    throw error;
  }
}
```

---

### 10.3 UI: Export/Import Panel (60 min)

**Create:** `/src/lib/components/export/ExportPanel.svelte`

```svelte
<script lang="ts">
  import { exportToJSON, exportToCSV, importFromJSON, createBackup } from '$lib/services/export';
  import { loadClipboardItems } from '$lib/stores/clipboardStore';

  let isExporting = false;
  let isImporting = false;
  let message = '';

  async function handleExportJSON() {
    isExporting = true;
    message = '';
    try {
      await exportToJSON(false); // Don't include images by default
      message = '✅ Exported successfully!';
    } catch (e) {
      message = `❌ Export failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }

  async function handleExportCSV() {
    isExporting = true;
    message = '';
    try {
      await exportToCSV();
      message = '✅ Exported to CSV successfully!';
    } catch (e) {
      message = `❌ Export failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }

  async function handleImport() {
    isImporting = true;
    message = '';
    try {
      const count = await importFromJSON();
      message = `✅ Imported ${count} items!`;
      // Reload items to show imported data
      await loadClipboardItems();
    } catch (e) {
      message = `❌ Import failed: ${e}`;
    } finally {
      isImporting = false;
    }
  }

  async function handleBackup() {
    isExporting = true;
    message = '';
    try {
      await createBackup();
      message = '✅ Backup created successfully!';
    } catch (e) {
      message = `❌ Backup failed: ${e}`;
    } finally {
      isExporting = false;
    }
  }
</script>

<div class="export-panel">
  <h2>Export & Import</h2>

  <section>
    <h3>Export</h3>
    <p>Save your clipboard history to a file</p>

    <div class="button-group">
      <button on:click={handleExportJSON} disabled={isExporting}>
        {isExporting ? 'Exporting...' : 'Export to JSON'}
      </button>

      <button on:click={handleExportCSV} disabled={isExporting}>
        {isExporting ? 'Exporting...' : 'Export to CSV'}
      </button>

      <button on:click={handleBackup} disabled={isExporting} class="btn-primary">
        {isExporting ? 'Creating...' : 'Create Backup'}
      </button>
    </div>
  </section>

  <section>
    <h3>Import</h3>
    <p>Import clipboard history from a JSON file</p>

    <button on:click={handleImport} disabled={isImporting}>
      {isImporting ? 'Importing...' : 'Import from JSON'}
    </button>
  </section>

  {#if message}
    <div class="message" class:success={message.includes('✅')} class:error={message.includes('❌')}>
      {message}
    </div>
  {/if}
</div>

<style>
  .export-panel {
    padding: 24px;
  }

  section {
    margin-bottom: 32px;
    padding: 16px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .button-group {
    display: flex;
    gap: 12px;
    margin-top: 12px;
  }

  .message {
    padding: 12px;
    border-radius: 6px;
    margin-top: 16px;
  }

  .message.success {
    background: #e6f7e6;
    color: #2d6a2d;
  }

  .message.error {
    background: #ffe6e6;
    color: #c62828;
  }
</style>
```

---

### 10.4 Add to Settings/Menu (20 min)

Add export/import option to settings dropdown or create a dedicated menu item.

---

## Testing Checklist

- [ ] Export to JSON creates valid file
- [ ] Export to CSV opens in spreadsheet
- [ ] Import from JSON works
- [ ] Import handles duplicates gracefully
- [ ] Backup includes all data
- [ ] Large exports complete successfully (1000+ items)
- [ ] File dialogs work on all platforms
- [ ] Progress indication during export/import

---

# Module 11: Appearance Customization

## Overview
Allow users to customize the app's look and feel beyond basic themes.

## Priority: P3 (Low-Medium)

## Estimated Time: 2-3 hours

## Dependencies
- Module 7 (settings system)

---

## Implementation Plan

### 11.1 Theme System Enhancement (60 min)

**Update:** `/src/lib/styles/themes.css`

Add more theme options and custom theme support:

```css
/* Light Theme */
:root[data-theme="light"] {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --bg-tertiary: #eeeeee;
  --text-primary: #1a1a1a;
  --text-secondary: #666666;
  --text-tertiary: #999999;
  --accent: #007aff;
  --accent-hover: #0051d5;
  --border-color: #dddddd;
  --shadow: rgba(0, 0, 0, 0.1);
  --success: #34c759;
  --error: #ff3b30;
  --warning: #ff9500;
}

/* Dark Theme */
:root[data-theme="dark"] {
  --bg-primary: #1a1a1a;
  --bg-secondary: #2d2d2d;
  --bg-tertiary: #3a3a3a;
  --text-primary: #ffffff;
  --text-secondary: #aaaaaa;
  --text-tertiary: #888888;
  --accent: #0a84ff;
  --accent-hover: #409cff;
  --border-color: #444444;
  --shadow: rgba(0, 0, 0, 0.3);
  --success: #32d74b;
  --error: #ff453a;
  --warning: #ff9f0a;
}

/* High Contrast Theme */
:root[data-theme="high-contrast"] {
  --bg-primary: #000000;
  --bg-secondary: #1a1a1a;
  --bg-tertiary: #2d2d2d;
  --text-primary: #ffffff;
  --text-secondary: #ffffff;
  --text-tertiary: #cccccc;
  --accent: #00d4ff;
  --accent-hover: #00a3cc;
  --border-color: #ffffff;
  --shadow: rgba(255, 255, 255, 0.2);
  --success: #00ff00;
  --error: #ff0000;
  --warning: #ffff00;
}

/* Card Size Variations */
.card-size-small {
  --card-padding: 8px;
  --card-gap: 6px;
  --font-size-base: 12px;
  --font-size-label: 10px;
}

.card-size-medium {
  --card-padding: 12px;
  --card-gap: 8px;
  --font-size-base: 14px;
  --font-size-label: 12px;
}

.card-size-large {
  --card-padding: 16px;
  --card-gap: 12px;
  --font-size-base: 16px;
  --font-size-label: 14px;
}

/* Font Size Scaling */
:root[data-font-size="12"] { font-size: 12px; }
:root[data-font-size="14"] { font-size: 14px; }
:root[data-font-size="16"] { font-size: 16px; }
:root[data-font-size="18"] { font-size: 18px; }
```

---

### 11.2 Custom Theme Creator (60 min)

**Create:** `/src/lib/components/appearance/ThemeCreator.svelte`

```svelte
<script lang="ts">
  import { settings, updateSetting } from '$lib/stores/settingsStore';

  interface CustomTheme {
    name: string;
    colors: {
      bgPrimary: string;
      bgSecondary: string;
      textPrimary: string;
      textSecondary: string;
      accent: string;
    };
  }

  let customTheme: CustomTheme = {
    name: 'My Theme',
    colors: {
      bgPrimary: '#ffffff',
      bgSecondary: '#f5f5f5',
      textPrimary: '#1a1a1a',
      textSecondary: '#666666',
      accent: '#007aff',
    }
  };

  function applyCustomTheme() {
    const root = document.documentElement;
    root.style.setProperty('--bg-primary', customTheme.colors.bgPrimary);
    root.style.setProperty('--bg-secondary', customTheme.colors.bgSecondary);
    root.style.setProperty('--text-primary', customTheme.colors.textPrimary);
    root.style.setProperty('--text-secondary', customTheme.colors.textSecondary);
    root.style.setProperty('--accent', customTheme.colors.accent);
  }

  function saveCustomTheme() {
    // Save to settings
    localStorage.setItem('customTheme', JSON.stringify(customTheme));
    applyCustomTheme();
  }

  function loadCustomTheme() {
    const saved = localStorage.getItem('customTheme');
    if (saved) {
      customTheme = JSON.parse(saved);
      applyCustomTheme();
    }
  }

  // Load on mount
  import { onMount } from 'svelte';
  onMount(loadCustomTheme);
</script>

<div class="theme-creator">
  <h3>Custom Theme</h3>

  <div class="color-inputs">
    <label>
      Theme Name
      <input type="text" bind:value={customTheme.name} />
    </label>

    <label>
      Primary Background
      <input type="color" bind:value={customTheme.colors.bgPrimary} />
      <input type="text" bind:value={customTheme.colors.bgPrimary} />
    </label>

    <label>
      Secondary Background
      <input type="color" bind:value={customTheme.colors.bgSecondary} />
      <input type="text" bind:value={customTheme.colors.bgSecondary} />
    </label>

    <label>
      Primary Text
      <input type="color" bind:value={customTheme.colors.textPrimary} />
      <input type="text" bind:value={customTheme.colors.textPrimary} />
    </label>

    <label>
      Secondary Text
      <input type="color" bind:value={customTheme.colors.textSecondary} />
      <input type="text" bind:value={customTheme.colors.textSecondary} />
    </label>

    <label>
      Accent Color
      <input type="color" bind:value={customTheme.colors.accent} />
      <input type="text" bind:value={customTheme.colors.accent} />
    </label>
  </div>

  <div class="preview">
    <h4>Preview</h4>
    <div class="preview-card" style="
      background: {customTheme.colors.bgPrimary};
      border: 1px solid {customTheme.colors.bgSecondary};
      color: {customTheme.colors.textPrimary};
    ">
      <p style="color: {customTheme.colors.textPrimary}">Sample text</p>
      <p style="color: {customTheme.colors.textSecondary}">Secondary text</p>
      <button style="background: {customTheme.colors.accent}; color: white;">
        Accent Button
      </button>
    </div>
  </div>

  <div class="actions">
    <button on:click={applyCustomTheme}>Preview</button>
    <button on:click={saveCustomTheme} class="btn-primary">Save Theme</button>
  </div>
</div>
```

---

### 11.3 UI Density Options (40 min)

Add compact/comfortable/spacious modes:

```typescript
// In settingsStore.ts
export interface AppSettings {
  // ... existing
  ui_density: 'compact' | 'comfortable' | 'spacious';
}

// Apply density
function applyUIden(density: string) {
  const root = document.documentElement;
  root.setAttribute('data-density', density);
}
```

```css
/* In themes.css */
:root[data-density="compact"] {
  --spacing-xs: 2px;
  --spacing-sm: 4px;
  --spacing-md: 8px;
  --spacing-lg: 12px;
  --spacing-xl: 16px;
}

:root[data-density="comfortable"] {
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 12px;
  --spacing-lg: 16px;
  --spacing-xl: 24px;
}

:root[data-density="spacious"] {
  --spacing-xs: 6px;
  --spacing-sm: 12px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;
}
```

---

## Testing Checklist

- [ ] Theme switching works instantly
- [ ] Custom themes persist across restarts
- [ ] Color picker works correctly
- [ ] Preview shows accurate colors
- [ ] Card size changes affect all cards
- [ ] Font size changes are readable
- [ ] UI density affects spacing correctly
- [ ] High contrast mode is accessible

---

# Module 12: Testing & Quality

## Overview
Comprehensive testing suite to ensure app reliability and quality.

## Priority: P1 (High - before release)

## Estimated Time: 4-6 hours

## Dependencies
- All previous modules (7-11)

---

## Implementation Plan

### 12.1 Unit Tests Setup (60 min)

**Install testing dependencies:**
```bash
npm install -D vitest @testing-library/svelte @testing-library/jest-dom
```

**Create:** `vitest.config.ts`
```typescript
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  test: {
    globals: true,
    environment: 'jsdom',
  },
});
```

---

### 12.2 Database Service Tests (60 min)

**Create:** `/src/lib/services/__tests__/database.test.ts`

```typescript
import { describe, it, expect, beforeEach } from 'vitest';
import { getClipboardItems, saveClipboardItem } from '../database';

describe('Database Service', () => {
  beforeEach(async () => {
    // Clear test database
  });

  it('should save and retrieve clipboard items', async () => {
    const id = await saveClipboardItem('test content', 'text', 'text');
    expect(id).toBeGreaterThan(0);

    const items = await getClipboardItems();
    expect(items).toHaveLength(1);
    expect(items[0].content).toBe('test content');
  });

  it('should filter by category', async () => {
    await saveClipboardItem('https://example.com', 'url', 'link');
    await saveClipboardItem('test@example.com', 'email', 'email');

    const links = await getClipboardItems(100, 'link');
    expect(links).toHaveLength(1);
    expect(links[0].category).toBe('link');
  });

  // More tests...
});
```

---

### 12.3 Component Tests (90 min)

**Create:** `/src/lib/components/__tests__/ClipboardCard.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import ClipboardCard from '../cards/ClipboardCard.svelte';

describe('ClipboardCard', () => {
  it('renders content correctly', () => {
    const { getByText } = render(ClipboardCard, {
      props: {
        item: {
          id: 1,
          content: 'Test content',
          content_type: 'text',
          category: 'text',
          timestamp: Date.now(),
          is_pinned: false,
          is_deleted: false,
        }
      }
    });

    expect(getByText('Test content')).toBeInTheDocument();
  });

  it('handles pin toggle', async () => {
    const { component, getByRole } = render(ClipboardCard, {
      props: { /* ... */ }
    });

    const pinButton = getByRole('button', { name: /pin/i });
    await fireEvent.click(pinButton);

    // Verify event emitted
  });

  // More tests...
});
```

---

### 12.4 Integration Tests (90 min)

Test full user flows:

```typescript
describe('Clipboard Capture Flow', () => {
  it('should capture, save, and display clipboard item', async () => {
    // 1. Simulate clipboard change
    // 2. Verify event emitted
    // 3. Verify database save
    // 4. Verify UI update
  });
});
```

---

### 12.5 Manual Testing Checklist (60 min)

Create comprehensive manual test plan:

**Create:** `MANUAL-TESTING.md`

```markdown
# Manual Testing Checklist

## Clipboard Monitoring
- [ ] Copy plain text → appears in app
- [ ] Copy URL → categorized as link
- [ ] Copy email → categorized as email
- [ ] Copy code → categorized as code
- [ ] Copy image → saved with thumbnail
- [ ] Multiple rapid copies handled correctly
- [ ] Empty clipboard handled gracefully

## UI Interactions
- [ ] Pin item → moves to top
- [ ] Unpin item → returns to chronological order
- [ ] Delete item → removes from list
- [ ] Click item → copies to clipboard
- [ ] Select multiple → bulk actions work
- [ ] Search filters correctly
- [ ] Category filter works

## Settings
- [ ] Theme changes apply immediately
- [ ] Settings persist after restart
- [ ] History limit enforced
- [ ] Auto-delete works

## Performance
- [ ] App starts in < 2 seconds
- [ ] UI responds in < 100ms
- [ ] 1000+ items scroll smoothly
- [ ] Memory usage < 100MB idle
- [ ] No memory leaks after 1 hour

## Edge Cases
- [ ] Very long text (100k chars)
- [ ] Special characters (emoji, unicode)
- [ ] Binary data handled
- [ ] Database corruption recovery
- [ ] Network interruption handling
```

---

## Testing Checklist

- [ ] Unit tests written for core functions
- [ ] Component tests cover main UI elements
- [ ] Integration tests verify user flows
- [ ] Manual testing checklist completed
- [ ] Performance benchmarks met
- [ ] No memory leaks detected
- [ ] Error handling tested
- [ ] Edge cases covered

---

# Module 13: Build & Distribution

## Overview
Prepare application for production release across all platforms.

## Priority: P0 (Critical - for release)

## Estimated Time: 3-4 hours

## Dependencies
- Module 12 (all tests passing)

---

## Implementation Plan

### 13.1 Build Configuration (60 min)

**Update:** `/src-tauri/tauri.conf.json`

```json
{
  "productName": "CopyGum",
  "version": "1.0.0",
  "identifier": "com.copygum.app",
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "msi", "appimage"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "macOS": {
      "minimumSystemVersion": "10.15"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    },
    "linux": {
      "deb": {
        "depends": []
      }
    }
  }
}
```

---

### 13.2 Icons & Assets (30 min)

Create app icons for all platforms:

- macOS: icon.icns (512x512, 256x256, 128x128, etc.)
- Windows: icon.ico (256x256, 128x128, 64x64, 48x48, 32x32, 16x16)
- Linux: PNG files (512x512, 256x256, 128x128, 64x64, 32x32)

Use `tauri icon` command to generate all sizes from a single 1024x1024 PNG.

---

### 13.3 Code Signing (60 min)

**macOS:**
```bash
# Get Apple Developer certificate
# Update tauri.conf.json with certificate info
```

**Windows:**
```bash
# Get code signing certificate
# Configure in tauri.conf.json
```

---

### 13.4 Build Scripts (30 min)

**Update:** `package.json`

```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "tauri:build:mac": "tauri build --target universal-apple-darwin",
    "tauri:build:windows": "tauri build --target x86_64-pc-windows-msvc",
    "tauri:build:linux": "tauri build --target x86_64-unknown-linux-gnu",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint .",
    "format": "prettier --write ."
  }
}
```

---

### 13.5 Auto-Update Setup (60 min)

**Install updater:**
```bash
npm install @tauri-apps/plugin-updater
```

**Create:** `/src/lib/services/updater.ts`

```typescript
import { check } from '@tauri-apps/plugin-updater';
import { ask } from '@tauri-apps/plugin-dialog';

export async function checkForUpdates(): Promise<void> {
  try {
    const update = await check();

    if (update) {
      const yes = await ask(
        `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
        {
          title: 'Update Available',
          okLabel: 'Update',
          cancelLabel: 'Later'
        }
      );

      if (yes) {
        await update.downloadAndInstall();
        // App will restart automatically
      }
    }
  } catch (error) {
    console.error('Update check failed:', error);
  }
}
```

---

### 13.6 Release Process (30 min)

**Create:** `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [macos-latest, windows-latest, ubuntu-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: npm ci

      - name: Build
        run: npm run tauri:build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}-build
          path: src-tauri/target/release/bundle/
```

---

## Distribution Checklist

- [ ] App icons created for all platforms
- [ ] Code signing configured (macOS, Windows)
- [ ] Build scripts tested on all platforms
- [ ] Auto-update mechanism working
- [ ] Release notes prepared
- [ ] GitHub releases automated
- [ ] Download page created
- [ ] Documentation published

---

# Summary & Timeline

## Module Priority Order

**Phase 1: Core Features (P1)**
1. Module 7: Settings & Preferences (2-3h)
2. Module 12: Testing & Quality (4-6h)

**Phase 2: Enhanced Features (P2)**
3. Module 8: Search & Filter (1-2h)
4. Module 9: Keyboard Shortcuts (2-3h)

**Phase 3: Additional Features (P3)**
5. Module 11: Appearance (2-3h)
6. Module 10: Export/Import (3-4h)

**Phase 4: Release (P0)**
7. Module 13: Build & Distribution (3-4h)

---

## Total Estimated Time: 20-30 hours

**Breakdown:**
- Module 7: 2-3h
- Module 8: 1-2h
- Module 9: 2-3h
- Module 10: 3-4h
- Module 11: 2-3h
- Module 12: 4-6h
- Module 13: 3-4h

---

## Next Immediate Steps

1. ✅ Complete Module 6 testing (end-to-end clipboard capture)
2. Begin Module 7 (Settings & Preferences)
3. Run comprehensive tests (Module 12)
4. Implement remaining features based on priority
5. Build and release (Module 13)

---

**Created:** 2025-11-19
**Status:** Planning Complete
**Ready to Begin:** Module 7 (after Module 6 testing)
