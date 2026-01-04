<script lang="ts">
  import { settings, updateSetting } from '../../stores/settingsStore';
  import { showSuccess } from '../../stores/toastStore';

  // Theme options
  const themeOptions = [
    { value: 'auto', label: 'Auto', description: 'Follow system preferences' },
    { value: 'light', label: 'Light', description: 'Light theme' },
    { value: 'dark', label: 'Dark', description: 'Dark theme' }
  ];

  // Card size options
  const cardSizeOptions = [
    { value: 'small', label: 'Small', description: 'Compact cards (120×80)' },
    { value: 'medium', label: 'Medium', description: 'Standard cards (160×100)' },
    { value: 'large', label: 'Large', description: 'Large cards (200×120)' }
  ];

  // Font size presets
  const fontSizePresets = [
    { value: 12, label: 'Small' },
    { value: 14, label: 'Medium' },
    { value: 16, label: 'Large' },
    { value: 18, label: 'Extra Large' }
  ];

  async function handleThemeChange(theme: string) {
    await updateSetting('theme', theme);
    showSuccess(`Theme changed to ${theme}`);
  }

  async function handleCardSizeChange(size: string) {
    await updateSetting('card_size', size);
    showSuccess(`Card size changed to ${size}`);
  }

  async function handleFontSizeChange(size: number) {
    await updateSetting('font_size', size);
    showSuccess(`Font size changed to ${size}px`);
  }

  async function handleThumbnailsToggle(enabled: boolean) {
    await updateSetting('show_thumbnails', enabled);
    showSuccess(`Thumbnails ${enabled ? 'enabled' : 'disabled'}`);
  }
</script>

<div class="appearance-panel">
  <!-- Theme Section -->
  <div class="section">
    <h3 class="section-title">Theme</h3>
    <p class="section-description">
      Choose your preferred color scheme
    </p>

    <div class="theme-grid">
      {#each themeOptions as option}
        <button
          class="theme-option"
          class:selected={$settings.theme === option.value}
          on:click={() => handleThemeChange(option.value)}
        >
          <div class="theme-preview" data-theme={option.value}>
            <div class="preview-bg"></div>
            <div class="preview-text"></div>
          </div>
          <div class="option-label">{option.label}</div>
          <div class="option-description">{option.description}</div>
        </button>
      {/each}
    </div>
  </div>

  <!-- Card Size Section -->
  <div class="section">
    <h3 class="section-title">Card Size</h3>
    <p class="section-description">
      Adjust the size of clipboard item cards
    </p>

    <div class="size-grid">
      {#each cardSizeOptions as option}
        <button
          class="size-option"
          class:selected={$settings.card_size === option.value}
          on:click={() => handleCardSizeChange(option.value)}
        >
          <div class="size-preview" data-size={option.value}>
            <div class="preview-card"></div>
          </div>
          <div class="option-label">{option.label}</div>
          <div class="option-description">{option.description}</div>
        </button>
      {/each}
    </div>
  </div>

  <!-- Font Size Section -->
  <div class="section">
    <h3 class="section-title">Font Size</h3>
    <p class="section-description">
      Customize the base font size
    </p>

    <div class="font-size-controls">
      <div class="preset-buttons">
        {#each fontSizePresets as preset}
          <button
            class="preset-button"
            class:selected={$settings.font_size === preset.value}
            on:click={() => handleFontSizeChange(preset.value)}
          >
            {preset.label}
          </button>
        {/each}
      </div>

      <div class="font-size-slider">
        <input
          type="range"
          min="10"
          max="24"
          step="1"
          value={$settings.font_size}
          on:input={(e) => handleFontSizeChange(parseInt(e.currentTarget.value))}
        />
        <div class="font-size-value">{$settings.font_size}px</div>
      </div>

      <div class="font-preview">
        <p style="font-size: {$settings.font_size}px;">
          The quick brown fox jumps over the lazy dog
        </p>
      </div>
    </div>
  </div>

  <!-- Display Options Section -->
  <div class="section">
    <h3 class="section-title">Display Options</h3>
    <p class="section-description">
      Additional display preferences
    </p>

    <div class="display-options">
      <label class="toggle-option">
        <div class="toggle-label">
          <span class="label-text">Show Thumbnails</span>
          <span class="label-description">Display image previews for image items</span>
        </div>
        <div class="toggle-switch">
          <input
            type="checkbox"
            checked={$settings.show_thumbnails}
            on:change={(e) => handleThumbnailsToggle(e.currentTarget.checked)}
          />
          <span class="slider"></span>
        </div>
      </label>
    </div>
  </div>
</div>

<style>
  .appearance-panel {
    padding: 20px;
  }

  .section {
    margin-bottom: 32px;
    padding-bottom: 32px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.2);
  }

  .section:last-child {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }

  .section-title {
    font-size: 16px;
    font-weight: 600;
    color: #f7e479;
    margin: 0 0 8px 0;
  }

  .section-description {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.6);
    margin: 0 0 16px 0;
  }

  /* Theme Grid */
  .theme-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .theme-option {
    background: rgba(247, 228, 121, 0.05);
    border: 2px solid rgba(247, 228, 121, 0.2);
    border-radius: 12px;
    padding: 16px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .theme-option:hover {
    background: rgba(247, 228, 121, 0.1);
    border-color: rgba(247, 228, 121, 0.4);
    transform: translateY(-2px);
  }

  .theme-option.selected {
    background: rgba(247, 228, 121, 0.15);
    border-color: rgba(247, 228, 121, 0.8);
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .theme-preview {
    width: 100%;
    height: 60px;
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .theme-preview[data-theme="light"] .preview-bg {
    background: #ffffff;
  }

  .theme-preview[data-theme="dark"] .preview-bg {
    background: #1a1a1a;
  }

  .theme-preview[data-theme="auto"] .preview-bg {
    background: linear-gradient(90deg, #ffffff 50%, #1a1a1a 50%);
  }

  .preview-bg {
    flex: 1;
  }

  .preview-text {
    height: 12px;
    margin: 8px;
    background: rgba(247, 228, 121, 0.4);
    border-radius: 4px;
  }

  .option-label {
    font-size: 14px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
  }

  .option-description {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
    text-align: center;
  }

  /* Size Grid */
  .size-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .size-option {
    background: rgba(247, 228, 121, 0.05);
    border: 2px solid rgba(247, 228, 121, 0.2);
    border-radius: 12px;
    padding: 16px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .size-option:hover {
    background: rgba(247, 228, 121, 0.1);
    border-color: rgba(247, 228, 121, 0.4);
    transform: translateY(-2px);
  }

  .size-option.selected {
    background: rgba(247, 228, 121, 0.15);
    border-color: rgba(247, 228, 121, 0.8);
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .size-preview {
    width: 100%;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
  }

  .preview-card {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    border: 1px solid rgba(247, 228, 121, 0.5);
  }

  .size-preview[data-size="small"] .preview-card {
    width: 36px;
    height: 24px;
  }

  .size-preview[data-size="medium"] .preview-card {
    width: 48px;
    height: 30px;
  }

  .size-preview[data-size="large"] .preview-card {
    width: 60px;
    height: 36px;
  }

  /* Font Size Controls */
  .font-size-controls {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .preset-buttons {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
  }

  .preset-button {
    padding: 10px 16px;
    background: rgba(247, 228, 121, 0.05);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.8);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .preset-button:hover {
    background: rgba(247, 228, 121, 0.1);
    border-color: rgba(247, 228, 121, 0.4);
  }

  .preset-button.selected {
    background: rgba(247, 228, 121, 0.2);
    border-color: rgba(247, 228, 121, 0.6);
    color: #f7e479;
    font-weight: 600;
  }

  .font-size-slider {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 16px;
    background: rgba(247, 228, 121, 0.05);
    border-radius: 8px;
  }

  .font-size-slider input[type="range"] {
    flex: 1;
    height: 6px;
    background: rgba(247, 228, 121, 0.2);
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
  }

  .font-size-slider input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    background: #f7e479;
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  }

  .font-size-slider input[type="range"]::-moz-range-thumb {
    width: 18px;
    height: 18px;
    background: #f7e479;
    border-radius: 50%;
    cursor: pointer;
    border: none;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  }

  .font-size-value {
    min-width: 48px;
    text-align: center;
    font-size: 14px;
    font-weight: 600;
    color: #f7e479;
  }

  .font-preview {
    padding: 16px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 8px;
    border: 1px solid rgba(247, 228, 121, 0.2);
  }

  .font-preview p {
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.5;
    margin: 0;
  }

  /* Display Options */
  .display-options {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .toggle-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    background: rgba(247, 228, 121, 0.05);
    border: 1px solid rgba(247, 228, 121, 0.2);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .toggle-option:hover {
    background: rgba(247, 228, 121, 0.1);
    border-color: rgba(247, 228, 121, 0.3);
  }

  .toggle-label {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .label-text {
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.9);
  }

  .label-description {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.5);
  }

  .toggle-switch {
    position: relative;
    width: 48px;
    height: 26px;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 13px;
    transition: all 0.3s;
    cursor: pointer;
  }

  .slider::before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 3px;
    bottom: 3px;
    background: white;
    border-radius: 50%;
    transition: all 0.3s;
  }

  .toggle-switch input:checked + .slider {
    background: #f7e479;
  }

  .toggle-switch input:checked + .slider::before {
    transform: translateX(22px);
  }
</style>
