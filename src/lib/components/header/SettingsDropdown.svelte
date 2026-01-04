<script lang="ts">
  // Settings Dropdown Component - Professional Version
  // Reorganized with Preferences, Storage, Account, Help tabs

  import { showSuccess, showError } from '../../stores/toastStore';
  import { settings, updateSetting, resetSettings, isLoadingSettings } from '../../stores/settingsStore';
  import { get } from 'svelte/store';
  import { exportToJSON, exportToCSV, importFromJSON, clearAllHistory } from '../../services/exportService';
  import { filteredItems } from '../../stores/clipboardStore';

  export let show: boolean = false;

  // Tab state
  let activePanel: 'preferences' | 'storage' | 'account' | 'help' = 'preferences';

  // Map settings to local UI format
  $: historyLimitUI = $settings.history_limit === -1 ? 'unlimited' :
                       $settings.history_limit === 50 ? '50' :
                       $settings.history_limit === 100 ? '100' : '500';

  $: autoClearUI = $settings.auto_delete_days === 0 ? 'never' :
                   $settings.auto_delete_days === 7 ? '7days' :
                   $settings.auto_delete_days === 30 ? '30days' : '90days';

  // Storage settings (not yet implemented in backend - for UI only)
  let storageType: 'local' | 'cloud' = 'local';
  let storagePath = '~/Documents/CopyGum';
  let cloudApiKey = '';
  let isConnecting = false;
  let cloudStatus: 'connected' | 'disconnected' | 'error' = 'disconnected';

  // Account settings (not yet implemented - for UI only)
  let userEmail = '';
  let licenseKey = '';
  let licenseStatus: 'trial' | 'active' | 'expired' | 'verifying' = 'trial';
  let isVerifying = false;
  let trialDaysRemaining = 15;
  let isTrialExpired = trialDaysRemaining <= 0;

  // Update trial expired status when days change
  $: isTrialExpired = trialDaysRemaining <= 0;
  $: if (isTrialExpired && licenseStatus === 'trial') {
    licenseStatus = 'expired';
  }

  function switchPanel(panel: typeof activePanel) {
    activePanel = panel;
  }

  // Handle settings changes
  async function handleToggleSetting(key: 'auto_start_monitoring' | 'show_on_startup' | 'minimize_to_tray' | 'save_images' | 'show_thumbnails' | 'enable_analytics', value: boolean) {
    try {
      await updateSetting(key, value);
      showSuccess('Setting updated');
    } catch (e) {
      showError('Failed to update setting');
      console.error(e);
    }
  }

  async function handleHistoryLimitChange(value: typeof historyLimitUI) {
    const numValue = value === 'unlimited' ? -1 :
                     value === '50' ? 50 :
                     value === '100' ? 100 : 500;
    try {
      await updateSetting('history_limit', numValue);
      showSuccess('History limit updated');
    } catch (e) {
      showError('Failed to update history limit');
      console.error(e);
    }
  }

  async function handleAutoClearChange(value: typeof autoClearUI) {
    const numValue = value === 'never' ? 0 :
                     value === '7days' ? 7 :
                     value === '30days' ? 30 : 90;
    try {
      await updateSetting('auto_delete_days', numValue);
      showSuccess('Auto-delete updated');
    } catch (e) {
      showError('Failed to update auto-delete');
      console.error(e);
    }
  }

  async function handleThemeChange(theme: string) {
    try {
      await updateSetting('theme', theme);
      showSuccess('Theme updated');
    } catch (e) {
      showError('Failed to update theme');
      console.error(e);
    }
  }

  async function handleCardSizeChange(size: string) {
    try {
      await updateSetting('card_size', size);
      showSuccess('Card size updated');
    } catch (e) {
      showError('Failed to update card size');
      console.error(e);
    }
  }

  async function handleFontSizeChange(size: number) {
    try {
      await updateSetting('font_size', size);
      showSuccess('Font size updated');
    } catch (e) {
      showError('Failed to update font size');
      console.error(e);
    }
  }

  async function handleDensityChange(density: string) {
    try {
      await updateSetting('density', density);
      showSuccess('Density updated');
    } catch (e) {
      showError('Failed to update density');
      console.error(e);
    }
  }

  async function handleAccentColorChange(color: string) {
    try {
      // Validate hex color format
      if (!/^#[0-9A-Fa-f]{6}$/.test(color)) {
        showError('Invalid color format. Use #RRGGBB');
        return;
      }
      await updateSetting('accent_color', color);
      showSuccess('Accent color updated');
    } catch (e) {
      showError('Failed to update accent color');
      console.error(e);
    }
  }

  async function handleResetAccentColor() {
    try {
      await updateSetting('accent_color', undefined);
      showSuccess('Accent color reset to default');
    } catch (e) {
      showError('Failed to reset accent color');
      console.error(e);
    }
  }

  async function handleOpacityChange(opacity: number) {
    try {
      await updateSetting('window_opacity', opacity);
      // No success message for slider - too noisy
    } catch (e) {
      showError('Failed to update opacity');
      console.error(e);
    }
  }

  async function handleResetSettings() {
    try {
      await resetSettings();
      showSuccess('Settings reset to defaults');
    } catch (e) {
      showError('Failed to reset settings');
      console.error(e);
    }
  }

  import { showInfo } from '../../stores/toastStore';
  import { loadClipboardItems } from '../../stores/clipboardStore';
  import { loadCategoriesFromDatabase } from '../../stores/categoryStore';

  // Coming Soon handlers - features not yet implemented
  async function handleChooseFolder() {
    showInfo('Folder selection coming soon!');
  }

  async function handleConnectCloud() {
    showInfo('Cloud sync coming soon!');
  }

  async function handleVerifyLicense() {
    showInfo('License activation coming soon!');
  }

  async function handlePurchaseLicense() {
    showInfo('Premium features coming soon!');
  }

  // Export/Import handlers
  let isExporting = false;
  let isImporting = false;

  async function handleExportJSON() {
    try {
      isExporting = true;
      await exportToJSON(true); // Include images
      showSuccess('Data exported successfully!');
    } catch (err) {
      showError('Failed to export data');
      console.error(err);
    } finally {
      isExporting = false;
    }
  }

  async function handleExportCSV() {
    try {
      isExporting = true;
      await exportToCSV();
      showSuccess('Data exported to CSV!');
    } catch (err) {
      showError('Failed to export CSV');
      console.error(err);
    } finally {
      isExporting = false;
    }
  }

  async function handleImportJSON() {
    try {
      isImporting = true;
      const count = await importFromJSON();
      if (count > 0) {
        showSuccess(`Imported ${count} items!`);
        // Refresh the stores instead of full page reload
        await loadClipboardItems();
        await loadCategoriesFromDatabase();
      }
    } catch (err) {
      showError('Failed to import data');
      console.error(err);
    } finally {
      isImporting = false;
    }
  }

  async function handleClearHistory() {
    if (confirm('Clear all unpinned history? This cannot be undone!')) {
      try {
        await clearAllHistory();
        showSuccess('History cleared!');
        // Refresh the stores instead of full page reload
        await loadClipboardItems();
        await loadCategoriesFromDatabase();
      } catch (err) {
        showError('Failed to clear history');
        console.error(err);
      }
    }
  }
</script>

<div class="settings-dropdown" class:show>
  <!-- Navigation Tabs -->
  <div class="settings-nav">
    <button
      class="settings-nav-btn"
      class:active={activePanel === 'preferences'}
      on:click={() => switchPanel('preferences')}
    >
      Preferences
    </button>
    <button
      class="settings-nav-btn"
      class:active={activePanel === 'storage'}
      on:click={() => switchPanel('storage')}
    >
      Storage
    </button>
    <button
      class="settings-nav-btn"
      class:active={activePanel === 'account'}
      on:click={() => switchPanel('account')}
    >
      Account
    </button>
    <button
      class="settings-nav-btn"
      class:active={activePanel === 'help'}
      on:click={() => switchPanel('help')}
    >
      Help
    </button>
  </div>

  <!-- Content Area -->
  <div class="settings-content">
    <!-- Preferences Panel -->
    <div class="settings-panel" class:active={activePanel === 'preferences'}>
      <!-- General Settings -->
      <div class="settings-section">
        <div class="settings-section-title">General</div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Auto-Start Monitoring</div>
            <div class="toggle-description">Start clipboard monitoring on app launch</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.auto_start_monitoring}
              on:change={(e) => handleToggleSetting('auto_start_monitoring', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Show on Startup</div>
            <div class="toggle-description">Show window when app starts</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.show_on_startup}
              on:change={(e) => handleToggleSetting('show_on_startup', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Minimize to Tray</div>
            <div class="toggle-description">Hide to system tray when closed</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.minimize_to_tray}
              on:change={(e) => handleToggleSetting('minimize_to_tray', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Save Images</div>
            <div class="toggle-description">Store clipboard images in history</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.save_images}
              on:change={(e) => handleToggleSetting('save_images', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <!-- Clipboard History -->
      <div class="settings-section">
        <div class="settings-section-title">Clipboard History</div>

        <div class="setting-group">
          <div class="setting-group-label">Maximum Items</div>
          <div class="setting-group-description">How many clipboard items to keep</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={historyLimitUI === '50'}>
              <input
                type="radio"
                name="history"
                value="50"
                checked={historyLimitUI === '50'}
                on:change={() => handleHistoryLimitChange('50')}
                disabled={$isLoadingSettings}
              />
              <span>50</span>
            </label>
            <label class="radio-compact-item" class:active={historyLimitUI === '100'}>
              <input
                type="radio"
                name="history"
                value="100"
                checked={historyLimitUI === '100'}
                on:change={() => handleHistoryLimitChange('100')}
                disabled={$isLoadingSettings}
              />
              <span>100</span>
            </label>
            <label class="radio-compact-item" class:active={historyLimitUI === '500'}>
              <input
                type="radio"
                name="history"
                value="500"
                checked={historyLimitUI === '500'}
                on:change={() => handleHistoryLimitChange('500')}
                disabled={$isLoadingSettings}
              />
              <span>500</span>
            </label>
            <label class="radio-compact-item" class:active={historyLimitUI === 'unlimited'}>
              <input
                type="radio"
                name="history"
                value="unlimited"
                checked={historyLimitUI === 'unlimited'}
                on:change={() => handleHistoryLimitChange('unlimited')}
                disabled={$isLoadingSettings}
              />
              <span>∞</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Auto-Clear After</div>
          <div class="setting-group-description">Automatically delete old items</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={autoClearUI === 'never'}>
              <input
                type="radio"
                name="autoclear"
                value="never"
                checked={autoClearUI === 'never'}
                on:change={() => handleAutoClearChange('never')}
                disabled={$isLoadingSettings}
              />
              <span>Never</span>
            </label>
            <label class="radio-compact-item" class:active={autoClearUI === '7days'}>
              <input
                type="radio"
                name="autoclear"
                value="7days"
                checked={autoClearUI === '7days'}
                on:change={() => handleAutoClearChange('7days')}
                disabled={$isLoadingSettings}
              />
              <span>7d</span>
            </label>
            <label class="radio-compact-item" class:active={autoClearUI === '30days'}>
              <input
                type="radio"
                name="autoclear"
                value="30days"
                checked={autoClearUI === '30days'}
                on:change={() => handleAutoClearChange('30days')}
                disabled={$isLoadingSettings}
              />
              <span>30d</span>
            </label>
            <label class="radio-compact-item" class:active={autoClearUI === '90days'}>
              <input
                type="radio"
                name="autoclear"
                value="90days"
                checked={autoClearUI === '90days'}
                on:change={() => handleAutoClearChange('90days')}
                disabled={$isLoadingSettings}
              />
              <span>90d</span>
            </label>
          </div>
        </div>
      </div>

      <!-- Appearance Settings -->
      <div class="settings-section">
        <div class="settings-section-title">Appearance</div>

        <div class="setting-group">
          <div class="setting-group-label">Theme</div>
          <div class="setting-group-description">Choose your preferred color scheme</div>
          <div class="theme-grid">
            <label class="theme-card" class:active={$settings.theme === 'auto'}>
              <input
                type="radio"
                name="theme"
                value="auto"
                checked={$settings.theme === 'auto'}
                on:change={() => handleThemeChange('auto')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview auto"></div>
              <span class="theme-name">Auto</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'light'}>
              <input
                type="radio"
                name="theme"
                value="light"
                checked={$settings.theme === 'light'}
                on:change={() => handleThemeChange('light')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview light"></div>
              <span class="theme-name">Light</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'dark'}>
              <input
                type="radio"
                name="theme"
                value="dark"
                checked={$settings.theme === 'dark'}
                on:change={() => handleThemeChange('dark')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview dark"></div>
              <span class="theme-name">Dark</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'high-contrast'}>
              <input
                type="radio"
                name="theme"
                value="high-contrast"
                checked={$settings.theme === 'high-contrast'}
                on:change={() => handleThemeChange('high-contrast')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview high-contrast"></div>
              <span class="theme-name">High Contrast</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'nord'}>
              <input
                type="radio"
                name="theme"
                value="nord"
                checked={$settings.theme === 'nord'}
                on:change={() => handleThemeChange('nord')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview nord"></div>
              <span class="theme-name">Nord</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'dracula'}>
              <input
                type="radio"
                name="theme"
                value="dracula"
                checked={$settings.theme === 'dracula'}
                on:change={() => handleThemeChange('dracula')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview dracula"></div>
              <span class="theme-name">Dracula</span>
            </label>
            <label class="theme-card" class:active={$settings.theme === 'solarized'}>
              <input
                type="radio"
                name="theme"
                value="solarized"
                checked={$settings.theme === 'solarized'}
                on:change={() => handleThemeChange('solarized')}
                disabled={$isLoadingSettings}
              />
              <div class="theme-preview solarized"></div>
              <span class="theme-name">Solarized</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Card Size</div>
          <div class="setting-group-description">Size of clipboard item cards</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={$settings.card_size === 'small'}>
              <input
                type="radio"
                name="cardsize"
                value="small"
                checked={$settings.card_size === 'small'}
                on:change={() => handleCardSizeChange('small')}
                disabled={$isLoadingSettings}
              />
              <span>Small</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.card_size === 'medium'}>
              <input
                type="radio"
                name="cardsize"
                value="medium"
                checked={$settings.card_size === 'medium'}
                on:change={() => handleCardSizeChange('medium')}
                disabled={$isLoadingSettings}
              />
              <span>Medium</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.card_size === 'large'}>
              <input
                type="radio"
                name="cardsize"
                value="large"
                checked={$settings.card_size === 'large'}
                on:change={() => handleCardSizeChange('large')}
                disabled={$isLoadingSettings}
              />
              <span>Large</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Font Size</div>
          <div class="setting-group-description">Base font size for the interface</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={$settings.font_size === 12}>
              <input
                type="radio"
                name="fontsize"
                value="12"
                checked={$settings.font_size === 12}
                on:change={() => handleFontSizeChange(12)}
                disabled={$isLoadingSettings}
              />
              <span>12px</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.font_size === 14}>
              <input
                type="radio"
                name="fontsize"
                value="14"
                checked={$settings.font_size === 14}
                on:change={() => handleFontSizeChange(14)}
                disabled={$isLoadingSettings}
              />
              <span>14px</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.font_size === 16}>
              <input
                type="radio"
                name="fontsize"
                value="16"
                checked={$settings.font_size === 16}
                on:change={() => handleFontSizeChange(16)}
                disabled={$isLoadingSettings}
              />
              <span>16px</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.font_size === 18}>
              <input
                type="radio"
                name="fontsize"
                value="18"
                checked={$settings.font_size === 18}
                on:change={() => handleFontSizeChange(18)}
                disabled={$isLoadingSettings}
              />
              <span>18px</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Density</div>
          <div class="setting-group-description">Spacing and padding throughout the interface</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={$settings.density === 'compact'}>
              <input
                type="radio"
                name="density"
                value="compact"
                checked={$settings.density === 'compact'}
                on:change={() => handleDensityChange('compact')}
                disabled={$isLoadingSettings}
              />
              <span>Compact</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.density === 'comfortable'}>
              <input
                type="radio"
                name="density"
                value="comfortable"
                checked={$settings.density === 'comfortable'}
                on:change={() => handleDensityChange('comfortable')}
                disabled={$isLoadingSettings}
              />
              <span>Comfortable</span>
            </label>
            <label class="radio-compact-item" class:active={$settings.density === 'spacious'}>
              <input
                type="radio"
                name="density"
                value="spacious"
                checked={$settings.density === 'spacious'}
                on:change={() => handleDensityChange('spacious')}
                disabled={$isLoadingSettings}
              />
              <span>Spacious</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Accent Color</div>
          <div class="setting-group-description">Customize the highlight color (optional)</div>
          <div class="color-picker-row">
            <input
              type="color"
              value={$settings.accent_color || '#f7e479'}
              on:change={(e) => handleAccentColorChange(e.currentTarget.value)}
              disabled={$isLoadingSettings}
              class="color-input"
            />
            <input
              type="text"
              value={$settings.accent_color || '#f7e479'}
              on:change={(e) => handleAccentColorChange(e.currentTarget.value)}
              disabled={$isLoadingSettings}
              class="color-text-input"
              placeholder="#f7e479"
              maxlength="7"
            />
            {#if $settings.accent_color}
              <button
                class="reset-color-btn"
                on:click={handleResetAccentColor}
                disabled={$isLoadingSettings}
                title="Reset to default"
              >
                Reset
              </button>
            {/if}
          </div>
        </div>

        <div class="setting-group">
          <div class="setting-group-label">Window Opacity</div>
          <div class="setting-group-description">Transparency level: {$settings.window_opacity}%</div>
          <div class="slider-container">
            <input
              type="range"
              min="70"
              max="100"
              value={$settings.window_opacity}
              on:input={(e) => handleOpacityChange(parseInt(e.currentTarget.value))}
              disabled={$isLoadingSettings}
              class="opacity-slider"
            />
            <div class="slider-labels">
              <span>70%</span>
              <span>85%</span>
              <span>100%</span>
            </div>
          </div>
        </div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Background Blur</div>
            <div class="toggle-description">Enable backdrop blur effect</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.enable_blur}
              on:change={(e) => handleToggleSetting('enable_blur', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>

        <div class="toggle-row">
          <div>
            <div class="toggle-label">Show Thumbnails</div>
            <div class="toggle-description">Display image previews in history</div>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              checked={$settings.show_thumbnails}
              on:change={(e) => handleToggleSetting('show_thumbnails', e.currentTarget.checked)}
              disabled={$isLoadingSettings}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- Storage Panel -->
    <div class="settings-panel" class:active={activePanel === 'storage'}>
      <div class="settings-section">
        <div class="settings-section-title">Storage Type</div>

        <div class="setting-group">
          <div class="setting-group-label">Choose Storage Location</div>
          <div class="setting-group-description">Where to store your clipboard history</div>
          <div class="radio-compact">
            <label class="radio-compact-item" class:active={storageType === 'local'}>
              <input type="radio" name="storage" value="local" bind:group={storageType} />
              <span>💾 Local</span>
            </label>
            <label class="radio-compact-item" class:active={storageType === 'cloud'}>
              <input type="radio" name="storage" value="cloud" bind:group={storageType} />
              <span>☁️ Cloud</span>
            </label>
          </div>
        </div>
      </div>

      {#if storageType === 'local'}
        <div class="settings-section">
          <div class="settings-section-title">Local Storage Settings</div>
          <div class="settings-description">
            Select a folder for local storage:
          </div>
          <button class="storage-button" on:click={handleChooseFolder}>
            📁 Choose Folder
          </button>
          <div class="storage-path" on:click={handleChooseFolder} role="button" tabindex="0" on:keypress={(e) => e.key === 'Enter' && handleChooseFolder()}>
            📂 {storagePath}
          </div>
        </div>
      {:else}
        <div class="settings-section">
          <div class="settings-section-title">Cloud Storage Settings</div>
          <div class="settings-description">
            Connect to Cloud Storage:
          </div>
          <input
            type="text"
            placeholder="Enter API Key"
            bind:value={cloudApiKey}
            class="settings-input"
            style="margin-bottom: 10px;"
          />
          <button
            class="storage-button"
            class:loading={isConnecting}
            on:click={handleConnectCloud}
            disabled={isConnecting}
          >
            {#if isConnecting}
              <span class="spinner"></span> Connecting...
            {:else}
              🔗 Connect
            {/if}
          </button>
          <div class="status-row">
            <span class="status-label">Status:</span>
            <span class="status-indicator">
              <span class="status-dot status-{cloudStatus}"></span>
              {#if cloudStatus === 'connected'}
                Connected
              {:else if cloudStatus === 'error'}
                Connection Failed
              {:else}
                Not Connected
              {/if}
            </span>
          </div>
        </div>
      {/if}

      <!-- Data Management Section -->
      <div class="settings-section">
        <div class="settings-section-title">Export & Import</div>
        <div class="settings-description">
          Backup and restore your clipboard history
        </div>

        <div class="button-grid">
          <button
            class="action-button export-button"
            class:loading={isExporting}
            on:click={handleExportJSON}
            disabled={isExporting || isImporting}
          >
            {#if isExporting}
              <span class="spinner"></span> Exporting...
            {:else}
              📥 Export JSON
            {/if}
          </button>

          <button
            class="action-button export-button"
            class:loading={isExporting}
            on:click={handleExportCSV}
            disabled={isExporting || isImporting}
          >
            {#if isExporting}
              <span class="spinner"></span> Exporting...
            {:else}
              📊 Export CSV
            {/if}
          </button>

          <button
            class="action-button import-button"
            class:loading={isImporting}
            on:click={handleImportJSON}
            disabled={isExporting || isImporting}
          >
            {#if isImporting}
              <span class="spinner"></span> Importing...
            {:else}
              📤 Import JSON
            {/if}
          </button>

          <button
            class="action-button danger-button"
            on:click={handleClearHistory}
            disabled={isExporting || isImporting}
          >
            🗑️ Clear History
          </button>
        </div>
      </div>
    </div>

    <!-- Account Panel -->
    <div class="settings-panel" class:active={activePanel === 'account'}>
      <!-- License Status Card -->
      <div class="settings-section">
        <div class="settings-section-title">License Status</div>

        {#if licenseStatus === 'trial'}
          <!-- Trial Active -->
          <div class="license-status-card trial">
            <div class="license-status-header">
              <div class="license-status-icon">🎉</div>
              <div class="license-status-info">
                <div class="license-status-title">Free Trial Active</div>
                <div class="license-status-subtitle">
                  {trialDaysRemaining} {trialDaysRemaining === 1 ? 'day' : 'days'} remaining
                </div>
              </div>
            </div>
            <div class="trial-progress-bar">
              <div class="trial-progress-fill" style="width: {(trialDaysRemaining / 15) * 100}%"></div>
            </div>
            <div class="license-status-description">
              You're using CopyGum with full features during the trial period.
            </div>
          </div>
        {:else if licenseStatus === 'expired'}
          <!-- Trial Expired -->
          <div class="license-status-card expired">
            <div class="license-status-header">
              <div class="license-status-icon">⚠️</div>
              <div class="license-status-info">
                <div class="license-status-title">Trial Expired</div>
                <div class="license-status-subtitle">Purchase a license to continue</div>
              </div>
            </div>
            <div class="license-status-description">
              Your 15-day trial has ended. Purchase a license to unlock all features.
            </div>
          </div>
        {:else if licenseStatus === 'active'}
          <!-- License Active -->
          <div class="license-status-card active">
            <div class="license-status-header">
              <div class="license-status-icon">✅</div>
              <div class="license-status-info">
                <div class="license-status-title">License Activated</div>
                <div class="license-status-subtitle">Thank you for your purchase!</div>
              </div>
            </div>
            <div class="license-status-description">
              You have full access to all CopyGum features.
            </div>
          </div>
        {/if}
      </div>

      {#if licenseStatus !== 'active'}
        <!-- Purchase Section -->
        <div class="settings-section">
          <div class="settings-section-title">Purchase License</div>
          <div class="settings-description">
            Enter your email to purchase a lifetime license for CopyGum
          </div>

          <input
            type="email"
            placeholder="your.email@example.com"
            bind:value={userEmail}
            class="settings-input"
            style="margin-bottom: 12px;"
          />

          <button
            class="purchase-button"
            on:click={handlePurchaseLicense}
          >
            💳 Purchase License - $29
          </button>

          <div class="purchase-benefits">
            <div class="benefit-item">✓ Lifetime access</div>
            <div class="benefit-item">✓ All features unlocked</div>
            <div class="benefit-item">✓ Free updates forever</div>
            <div class="benefit-item">✓ Priority support</div>
          </div>
        </div>
      {/if}

      <!-- Activate License Section -->
      <div class="settings-section">
        <div class="settings-section-title">
          {licenseStatus === 'active' ? 'Your License Key' : 'Already Purchased?'}
        </div>
        <div class="settings-description">
          {licenseStatus === 'active'
            ? 'Your license key is activated and ready to use'
            : 'Enter your license key received after purchase'}
        </div>

        <input
          type="text"
          placeholder="XXXX-XXXX-XXXX-XXXX"
          bind:value={licenseKey}
          class="settings-input license-input"
          disabled={licenseStatus === 'active'}
        />

        {#if licenseStatus !== 'active'}
          <button
            class="storage-button"
            class:loading={isVerifying}
            on:click={handleVerifyLicense}
            disabled={isVerifying}
            style="margin-top: 10px;"
          >
            {#if isVerifying}
              <span class="spinner"></span> Verifying...
            {:else}
              🔑 Activate License
            {/if}
          </button>
        {/if}
      </div>
    </div>

    <!-- Help Panel -->
    <div class="settings-panel" class:active={activePanel === 'help'}>
      <div class="settings-section">
        <div class="settings-section-title">Keyboard Shortcuts</div>
        <div class="shortcuts-list">
          <div class="shortcut-item">
            <span>Open/Close Panel</span>
            <kbd>Cmd+Shift+V</kbd>
          </div>
          <div class="shortcut-item">
            <span>Search Items</span>
            <kbd>Cmd+F</kbd>
          </div>
          <div class="shortcut-item">
            <span>Switch to Categories</span>
            <kbd>↑</kbd>
          </div>
          <div class="shortcut-item">
            <span>Switch to Cards</span>
            <kbd>↓</kbd>
          </div>
          <div class="shortcut-item">
            <span>Navigate Left</span>
            <kbd>←</kbd>
          </div>
          <div class="shortcut-item">
            <span>Navigate Right</span>
            <kbd>→</kbd>
          </div>
          <div class="shortcut-item">
            <span>Copy/Activate Item</span>
            <kbd>Enter</kbd>
          </div>
          <div class="shortcut-item">
            <span>Delete Item</span>
            <kbd>Delete</kbd>
          </div>
        </div>

        <div class="settings-section-title" style="margin-top: 24px;">How to Use</div>
        <div class="how-to-use">
          <p><kbd class="inline-kbd">Cmd+Shift+V</kbd> to open CopyGum</p>
          <p>Your clipboard history appears as cards</p>
          <p>Use <kbd class="inline-kbd">↑</kbd> <kbd class="inline-kbd">↓</kbd> to switch between categories and cards</p>
          <p>Use <kbd class="inline-kbd">←</kbd> <kbd class="inline-kbd">→</kbd> to navigate within each layer</p>
          <p>Press <kbd class="inline-kbd">Enter</kbd> to copy items or activate categories</p>
          <p>Click any card to copy it instantly</p>
        </div>
      </div>

      <div class="settings-section">
        <div class="about-section">
          <div class="about-logo">CopyGum</div>
          <div class="about-version">Version 1.0.0</div>
          <div class="about-description">
            A modern clipboard manager<br/>
            built with Tauri + Svelte
          </div>
          <div class="about-links">
            <a href="#" class="about-link">Website</a>
            <span class="about-divider">•</span>
            <a href="#" class="about-link">GitHub</a>
            <span class="about-divider">•</span>
            <a href="#" class="about-link">Support</a>
          </div>
        </div>
      </div>
    </div>
  </div>

</div>

<style>
  /* Container */
  .settings-dropdown {
    position: fixed;
    top: calc(100vh - 480px + 80px);
    right: 20px;
    width: 360px;
    max-height: calc(100vh - (100vh - 480px + 80px) - 20px);
    background: rgba(20, 20, 20, 0.95);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 12px;
    opacity: 0;
    visibility: hidden;
    transform: translateY(-10px);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 1000;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-dropdown.show {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
  }

  /* Navigation */
  .settings-nav {
    display: flex;
    gap: 6px;
    padding: 16px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.2);
  }

  .settings-nav-btn {
    flex: 1;
    padding: 10px 8px;
    background: rgba(247, 228, 121, 0.08);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.7);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    text-align: center;
  }

  .settings-nav-btn:hover {
    background: rgba(247, 228, 121, 0.15);
    color: rgba(247, 228, 121, 0.9);
    transform: translateY(-1px);
    border-color: rgba(247, 228, 121, 0.4);
  }

  .settings-nav-btn.active {
    background: rgba(247, 228, 121, 0.25);
    border-color: rgba(247, 228, 121, 0.6);
    color: #f7e479;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.2);
  }

  /* Content */
  .settings-content {
    padding: 20px;
    max-height: 420px;
    overflow-y: auto;
    flex: 1;
  }

  .settings-content::-webkit-scrollbar {
    width: 6px;
  }

  .settings-content::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  .settings-panel {
    display: none;
    animation: fadeIn 0.2s ease;
  }

  .settings-panel.active {
    display: block;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .settings-section {
    margin-bottom: 28px;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .settings-section-title {
    font-size: 14px;
    font-weight: 700;
    color: #f7e479;
    margin-bottom: 8px;
    letter-spacing: 0.5px;
    text-transform: uppercase;
  }

  .settings-description {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
    margin-bottom: 12px;
    line-height: 1.5;
  }

  /* Input Fields */
  .settings-input {
    width: 100%;
    padding: 10px 12px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 13px;
    transition: all 0.2s;
    outline: none;
    box-sizing: border-box;
  }

  .settings-input:focus {
    border-color: rgba(247, 228, 121, 0.6);
    box-shadow: 0 0 0 3px rgba(247, 228, 121, 0.1), 0 0 12px rgba(247, 228, 121, 0.2);
  }

  .settings-input:hover {
    border-color: rgba(247, 228, 121, 0.4);
  }

  .license-input {
    font-family: 'Courier New', monospace;
    letter-spacing: 1px;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
  }

  .input-with-button .settings-input {
    flex: 1;
  }

  .verify-button {
    padding: 10px 20px;
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    color: #f7e479;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .verify-button:hover {
    background: rgba(247, 228, 121, 0.25);
    transform: translateY(-1px);
  }

  /* Radio Container */
  .radio-container {
    --main-color: #f7e479;
    display: flex;
    flex-direction: column;
    position: relative;
    padding-left: 0.5rem;
    margin-bottom: 12px;
    min-height: calc(2.5rem * var(--total-radio)); /* Increased from 2rem */
  }

  .radio-container input {
    cursor: pointer;
    appearance: none;
    height: 2.5rem; /* Increased from 2rem */
  }

  .radio-container label {
    cursor: pointer;
    color: rgba(255, 255, 255, 0.8);
    font-size: 13px;
    font-weight: 500;
    position: relative;
    top: calc(-2.5rem * var(--total-radio)); /* Increased from 2rem */
    left: 1.8rem; /* Increased from 1.5rem */
    height: 2.5rem; /* Increased from 2rem */
    display: flex;
    align-items: center;
    transition: all 0.2s;
    padding-right: 8px; /* Add padding to prevent overflow */
  }

  .radio-container input:checked + label {
    color: var(--main-color);
    font-weight: 600;
  }

  .glider-container {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: linear-gradient(0deg, rgba(0, 0, 0, 0) 0%, rgba(27, 27, 27, 1) 50%, rgba(0, 0, 0, 0) 100%);
    width: 1px;
    pointer-events: none;
  }

  .glider {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translate(-50%, 0);
    width: 0.5rem;
    height: 2.5rem; /* Increased from 2rem */
    background: var(--main-color);
    border-radius: 1rem;
    transition: top 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 0 8px rgba(247, 228, 121, 0.5);
  }

  .radio-container input:nth-of-type(1):checked ~ .glider-container .glider { top: 0; }
  .radio-container input:nth-of-type(2):checked ~ .glider-container .glider { top: 2.5rem; } /* Changed from 2rem */
  .radio-container input:nth-of-type(3):checked ~ .glider-container .glider { top: 5rem; } /* Changed from 4rem */
  .radio-container input:nth-of-type(4):checked ~ .glider-container .glider { top: 7.5rem; } /* Changed from 6rem */

  /* Toggle Switch */
  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .toggle-row:last-child {
    border-bottom: none;
  }

  .toggle-label {
    font-size: 13px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 4px;
  }

  .toggle-description {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 26px;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(255, 255, 255, 0.2);
    transition: 0.3s;
    border-radius: 26px;
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  .toggle-slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.3s;
    border-radius: 50%;
  }

  .toggle input:checked + .toggle-slider {
    background-color: #f7e479;
    border-color: #f7e479;
  }

  .toggle input:checked + .toggle-slider:before {
    transform: translateX(22px);
  }

  /* Preview Box */
  .preview-box {
    margin-top: 12px;
    padding: 10px 12px;
    background: rgba(247, 228, 121, 0.05);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 6px;
    font-size: 12px;
  }

  .preview-label {
    color: rgba(255, 255, 255, 0.6);
    margin-right: 8px;
  }

  .preview-content {
    color: rgba(255, 255, 255, 0.9);
    font-family: 'Courier New', monospace;
  }

  /* Storage Details */
  .storage-details {
    margin-top: 16px;
    padding: 16px;
    background: rgba(247, 228, 121, 0.05);
    border-radius: 8px;
    border: 1px solid rgba(247, 228, 121, 0.2);
  }

  .storage-button {
    width: 100%;
    padding: 11px;
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    color: #f7e479;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
  }

  .storage-button:hover:not(:disabled) {
    background: rgba(247, 228, 121, 0.25);
    transform: translateY(-1px);
  }

  .storage-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .storage-button.loading {
    cursor: wait;
  }

  .storage-path {
    margin-top: 10px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.7);
    padding: 8px 10px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .storage-path:hover {
    background: rgba(0, 0, 0, 0.4);
    color: #f7e479;
  }

  /* Status */
  .status-row {
    margin-top: 10px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-label {
    color: rgba(255, 255, 255, 0.6);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    color: rgba(255, 255, 255, 0.9);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: inline-block;
  }

  .status-dot.status-connected {
    background: #22c55e;
    box-shadow: 0 0 8px rgba(34, 197, 94, 0.5);
  }

  .status-dot.status-disconnected {
    background: #ef4444;
    box-shadow: 0 0 8px rgba(239, 68, 68, 0.5);
  }

  .status-dot.status-error {
    background: #f59e0b;
    box-shadow: 0 0 8px rgba(245, 158, 11, 0.5);
  }

  .status-dot.status-inactive {
    background: #6b7280;
  }

  /* Spinner */
  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(247, 228, 121, 0.3);
    border-top-color: #f7e479;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    display: inline-block;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Shortcuts */
  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    background: rgba(247, 228, 121, 0.05);
    border-radius: 6px;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.85);
  }

  kbd {
    padding: 5px 10px;
    background: rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 11px;
    color: #f7e479;
    font-weight: 600;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .inline-kbd {
    display: inline;
    padding: 2px 6px;
    margin: 0 2px;
  }

  /* How to Use */
  .how-to-use {
    color: rgba(255, 255, 255, 0.75);
    font-size: 13px;
    line-height: 1.8;
  }

  .how-to-use p {
    margin: 8px 0;
  }

  /* About */
  .about-section {
    text-align: center;
    padding: 20px 0;
  }

  .about-logo {
    font-size: 28px;
    font-weight: 700;
    color: #f7e479;
    margin-bottom: 8px;
    text-shadow: 0 0 20px rgba(247, 228, 121, 0.3);
  }

  .about-version {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.6);
    margin-bottom: 16px;
  }

  .about-description {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.6;
    margin-bottom: 20px;
  }

  .about-links {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 12px;
  }

  .about-link {
    color: #f7e479;
    text-decoration: none;
    transition: all 0.2s;
  }

  .about-link:hover {
    color: #fff;
    text-decoration: underline;
  }

  .about-divider {
    color: rgba(255, 255, 255, 0.4);
  }

  /* Action Buttons */
  .settings-actions {
    display: flex;
    gap: 10px;
    padding: 16px;
    border-top: 1px solid rgba(247, 228, 121, 0.2);
    background: rgba(0, 0, 0, 0.3);
  }

  .action-btn {
    flex: 1;
    padding: 10px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid transparent;
  }

  .action-btn.primary {
    background: rgba(247, 228, 121, 0.2);
    border-color: rgba(247, 228, 121, 0.5);
    color: #f7e479;
  }

  .action-btn.primary:hover {
    background: rgba(247, 228, 121, 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .action-btn.secondary {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.8);
  }

  .action-btn.secondary:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-1px);
  }

  /* Setting Group */
  .setting-group {
    margin-bottom: 20px;
  }

  .setting-group:last-child {
    margin-bottom: 0;
  }

  .setting-group-label {
    font-size: 13px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 6px;
  }

  .setting-group-description {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
    margin-bottom: 10px;
    line-height: 1.4;
  }

  /* Theme Grid */
  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(90px, 1fr));
    gap: 12px;
    margin-top: 8px;
  }

  .theme-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 12px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .theme-card input {
    display: none;
  }

  .theme-card:hover {
    background: rgba(255, 255, 255, 0.06);
    border-color: rgba(247, 228, 121, 0.3);
    transform: translateY(-2px);
  }

  .theme-card.active {
    background: rgba(247, 228, 121, 0.15);
    border-color: rgba(247, 228, 121, 0.6);
    box-shadow: 0 0 16px rgba(247, 228, 121, 0.2);
  }

  .theme-preview {
    width: 100%;
    height: 50px;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    position: relative;
    overflow: hidden;
  }

  .theme-preview::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 50%;
    bottom: 0;
  }

  .theme-preview::after {
    content: '';
    position: absolute;
    top: 0;
    left: 50%;
    right: 0;
    bottom: 0;
  }

  /* Theme Preview Colors */
  .theme-preview.auto::before { background: #ffffff; }
  .theme-preview.auto::after { background: #1a1a1a; }

  .theme-preview.light::before { background: #ffffff; }
  .theme-preview.light::after { background: #f5f5f5; }

  .theme-preview.dark::before { background: #1a1a1a; }
  .theme-preview.dark::after { background: #2a2a2a; }

  .theme-preview.high-contrast::before { background: #000000; }
  .theme-preview.high-contrast::after { background: #00d4ff; }

  .theme-preview.nord::before { background: #2e3440; }
  .theme-preview.nord::after { background: #88c0d0; }

  .theme-preview.dracula::before { background: #282a36; }
  .theme-preview.dracula::after { background: #ff79c6; }

  .theme-preview.solarized::before { background: #002b36; }
  .theme-preview.solarized::after { background: #268bd2; }

  .theme-name {
    font-size: 12px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.7);
    text-align: center;
  }

  .theme-card.active .theme-name {
    color: #f7e479;
    font-weight: 600;
  }

  /* Color Picker */
  .color-picker-row {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-top: 8px;
  }

  .color-input {
    width: 50px;
    height: 40px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    cursor: pointer;
    background: transparent;
    transition: all 0.2s;
  }

  .color-input:hover {
    border-color: rgba(247, 228, 121, 0.4);
    box-shadow: 0 0 8px rgba(247, 228, 121, 0.2);
  }

  .color-input::-webkit-color-swatch-wrapper {
    padding: 0;
  }

  .color-input::-webkit-color-swatch {
    border: none;
    border-radius: 5px;
  }

  .color-text-input {
    flex: 1;
    padding: 10px 12px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 13px;
    font-family: 'Monaco', 'Menlo', monospace;
    transition: all 0.2s;
  }

  .color-text-input:focus {
    outline: none;
    border-color: rgba(247, 228, 121, 0.5);
    background: rgba(255, 255, 255, 0.08);
  }

  .color-text-input::placeholder {
    color: rgba(255, 255, 255, 0.3);
  }

  .reset-color-btn {
    padding: 10px 16px;
    background: rgba(255, 59, 48, 0.15);
    border: 1px solid rgba(255, 59, 48, 0.3);
    border-radius: 6px;
    color: #ff3b30;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .reset-color-btn:hover {
    background: rgba(255, 59, 48, 0.25);
    border-color: rgba(255, 59, 48, 0.5);
    transform: translateY(-1px);
  }

  .reset-color-btn:active {
    transform: translateY(0);
  }

  .reset-color-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Opacity Slider */
  .slider-container {
    margin-top: 12px;
  }

  .opacity-slider {
    width: 100%;
    height: 6px;
    background: linear-gradient(to right,
      rgba(247, 228, 121, 0.1) 0%,
      rgba(247, 228, 121, 0.3) 100%
    );
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
    cursor: pointer;
  }

  .opacity-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    background: #f7e479;
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    transition: all 0.2s;
  }

  .opacity-slider::-webkit-slider-thumb:hover {
    transform: scale(1.2);
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.5);
  }

  .opacity-slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
    background: #f7e479;
    border: none;
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    transition: all 0.2s;
  }

  .opacity-slider::-moz-range-thumb:hover {
    transform: scale(1.2);
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.5);
  }

  .slider-labels {
    display: flex;
    justify-content: space-between;
    margin-top: 6px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.4);
  }

  /* Compact Radio Buttons */
  .radio-compact {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .radio-compact-item {
    flex: 1;
    min-width: 60px;
    padding: 10px 12px;
    background: rgba(247, 228, 121, 0.08);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 6px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s;
    color: rgba(255, 255, 255, 0.7);
    font-size: 13px;
    font-weight: 500;
  }

  .radio-compact-item input {
    display: none;
  }

  .radio-compact-item:hover {
    background: rgba(247, 228, 121, 0.15);
    border-color: rgba(247, 228, 121, 0.4);
    color: rgba(255, 255, 255, 0.9);
  }

  .radio-compact-item.active {
    background: rgba(247, 228, 121, 0.25);
    border-color: rgba(247, 228, 121, 0.6);
    color: #f7e479;
    font-weight: 600;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.2);
  }

  /* License Status Card */
  .license-status-card {
    padding: 20px;
    border-radius: 8px;
    border: 1px solid;
    margin-bottom: 8px;
  }

  .license-status-card.trial {
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(99, 102, 241, 0.1) 100%);
    border-color: rgba(59, 130, 246, 0.3);
  }

  .license-status-card.expired {
    background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(220, 38, 38, 0.1) 100%);
    border-color: rgba(239, 68, 68, 0.3);
  }

  .license-status-card.active {
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.1) 0%, rgba(22, 163, 74, 0.1) 100%);
    border-color: rgba(34, 197, 94, 0.3);
  }

  .license-status-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }

  .license-status-icon {
    font-size: 32px;
    line-height: 1;
  }

  .license-status-info {
    flex: 1;
  }

  .license-status-title {
    font-size: 16px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.95);
    margin-bottom: 4px;
  }

  .license-status-subtitle {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.7);
  }

  .license-status-description {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.65);
    line-height: 1.5;
  }

  /* Trial Progress Bar */
  .trial-progress-bar {
    width: 100%;
    height: 6px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 12px;
  }

  .trial-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6 0%, #6366f1 100%);
    border-radius: 3px;
    transition: width 0.3s ease;
    box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
  }

  /* Purchase Button */
  .purchase-button {
    width: 100%;
    padding: 14px;
    background: linear-gradient(135deg, #f7e479 0%, #f59e0b 100%);
    border: none;
    border-radius: 8px;
    color: #000;
    font-size: 14px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .purchase-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(247, 228, 121, 0.4);
  }

  .purchase-button:active {
    transform: translateY(0);
  }

  /* Purchase Benefits */
  .purchase-benefits {
    margin-top: 16px;
    padding: 16px;
    background: rgba(247, 228, 121, 0.05);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 8px;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  .benefit-item {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
    font-weight: 500;
  }

  /* Disabled Input */
  .settings-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background: rgba(0, 0, 0, 0.2);
  }
  /* Export/Import Buttons */
  .button-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    margin-top: 12px;
  }

  .action-button {
    padding: 10px 16px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.9);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
  }

  .action-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(247, 228, 121, 0.4);
  }

  .action-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .action-button.loading {
    position: relative;
  }

  .export-button:hover:not(:disabled) {
    border-color: rgba(76, 175, 80, 0.5);
    background: rgba(76, 175, 80, 0.1);
  }

  .import-button:hover:not(:disabled) {
    border-color: rgba(33, 150, 243, 0.5);
    background: rgba(33, 150, 243, 0.1);
  }

  .danger-button:hover:not(:disabled) {
    border-color: rgba(244, 67, 54, 0.5);
    background: rgba(244, 67, 54, 0.1);
  }
</style>
