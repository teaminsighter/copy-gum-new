<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { settings, updateSetting } from '../../stores/settingsStore';
  import { showSuccess, showError } from '../../stores/toastStore';
  import { open } from '@tauri-apps/plugin-dialog';
  import { appDataDir } from '@tauri-apps/api/path';

  export let show = false;
  export let onComplete: () => void = () => {};

  let currentStep = 0;
  let imageStoragePath = '';
  let isSelectingFolder = false;

  // Detect platform
  const isMac = /Mac|iPhone|iPad|iPod/i.test(navigator.userAgent);
  const shortcutKey = isMac ? 'Cmd+Shift+V' : 'Ctrl+Shift+V';

  const steps = [
    { id: 'welcome', title: 'Welcome to CopyGum' },
    { id: 'overlay', title: 'Overlay Access' },
    { id: 'storage', title: 'Image Storage' },
    { id: 'ready', title: 'All Set!' }
  ];

  onMount(async () => {
    // Set default storage path
    try {
      const defaultPath = await appDataDir();
      imageStoragePath = defaultPath + 'images';
    } catch {
      imageStoragePath = '~/CopyGum/images';
    }
  });

  async function handleSelectFolder() {
    try {
      isSelectingFolder = true;
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Image Storage Folder'
      });

      if (selected && typeof selected === 'string') {
        imageStoragePath = selected;
      }
    } catch (err) {
      console.error('Failed to select folder:', err);
    } finally {
      isSelectingFolder = false;
    }
  }

  function nextStep() {
    if (currentStep < steps.length - 1) {
      currentStep++;
    }
  }

  function prevStep() {
    if (currentStep > 0) {
      currentStep--;
    }
  }

  async function completeSetup() {
    try {
      // Save settings
      await updateSetting('hasShownOverlayInfo', true);
      // Note: imageStoragePath would be saved to a new setting field
      // For now, we'll use the default app data directory

      showSuccess('Setup complete! Press ' + shortcutKey + ' to open CopyGum anytime.');
      show = false;
      onComplete();
    } catch (err) {
      showError('Failed to save settings');
      console.error(err);
    }
  }

  function openSystemPreferences() {
    // On macOS, we can open System Preferences for accessibility
    if (isMac) {
      // This would require a Tauri command to open system preferences
      // For now, show instructions
    }
  }
</script>

{#if show}
  <div class="wizard-backdrop" transition:fade={{ duration: 200 }}>
    <div class="wizard-container" transition:scale={{ duration: 200, start: 0.95 }}>
      <!-- Progress indicator -->
      <div class="wizard-progress">
        {#each steps as step, i}
          <div class="progress-dot" class:active={i === currentStep} class:completed={i < currentStep}></div>
        {/each}
      </div>

      <!-- Step content -->
      <div class="wizard-content">
        {#if currentStep === 0}
          <!-- Welcome -->
          <div class="step-content" transition:fade={{ duration: 150 }}>
            <div class="step-icon">
              <svg viewBox="0 0 48 48" fill="none">
                <rect x="6" y="12" width="30" height="30" rx="6" fill="#f7e479" opacity="0.3"/>
                <rect x="12" y="6" width="30" height="30" rx="6" fill="#f7e479"/>
              </svg>
            </div>
            <h2>Welcome to CopyGum</h2>
            <p>Your smart clipboard manager. Let's get you set up in just a few steps.</p>
            <div class="feature-list">
              <div class="feature-item">
                <span class="feature-check">✓</span>
                <span>Unlimited clipboard history</span>
              </div>
              <div class="feature-item">
                <span class="feature-check">✓</span>
                <span>Smart categories & search</span>
              </div>
              <div class="feature-item">
                <span class="feature-check">✓</span>
                <span>Works as an overlay</span>
              </div>
            </div>
          </div>
        {:else if currentStep === 1}
          <!-- Overlay Permission -->
          <div class="step-content" transition:fade={{ duration: 150 }}>
            <div class="step-icon overlay-icon">
              <svg viewBox="0 0 48 48" fill="none">
                <rect x="4" y="8" width="40" height="28" rx="4" stroke="#f7e479" stroke-width="2"/>
                <rect x="12" y="16" width="24" height="12" rx="2" fill="#f7e479"/>
              </svg>
            </div>
            <h2>Overlay Access</h2>
            <p>CopyGum appears as an overlay on top of your apps. Access it anytime with:</p>

            <div class="shortcut-display">
              <kbd>{isMac ? 'Cmd' : 'Ctrl'}</kbd>
              <span>+</span>
              <kbd>Shift</kbd>
              <span>+</span>
              <kbd>V</kbd>
            </div>

            {#if isMac}
              <div class="permission-note">
                <div class="note-icon">ℹ️</div>
                <div class="note-text">
                  <strong>macOS Users:</strong> If prompted, grant CopyGum accessibility permissions in System Preferences → Privacy & Security → Accessibility.
                </div>
              </div>
            {:else}
              <div class="permission-note">
                <div class="note-icon">ℹ️</div>
                <div class="note-text">
                  <strong>Windows Users:</strong> CopyGum will run as an overlay. No additional permissions needed.
                </div>
              </div>
            {/if}
          </div>
        {:else if currentStep === 2}
          <!-- Storage Configuration -->
          <div class="step-content" transition:fade={{ duration: 150 }}>
            <div class="step-icon storage-icon">
              <svg viewBox="0 0 48 48" fill="none">
                <path d="M8 12h32v24H8z" stroke="#f7e479" stroke-width="2"/>
                <path d="M8 12l16-6 16 6" stroke="#f7e479" stroke-width="2"/>
                <circle cx="24" cy="24" r="6" fill="#f7e479"/>
              </svg>
            </div>
            <h2>Image Storage</h2>
            <p>Choose where CopyGum saves copied images:</p>

            <div class="storage-selector">
              <input
                type="text"
                readonly
                value={imageStoragePath}
                class="storage-path"
              />
              <button
                class="select-folder-btn"
                on:click={handleSelectFolder}
                disabled={isSelectingFolder}
              >
                {isSelectingFolder ? 'Selecting...' : 'Browse'}
              </button>
            </div>

            <div class="storage-note">
              <p>Default location is recommended. You can change this later in Settings.</p>
            </div>
          </div>
        {:else if currentStep === 3}
          <!-- Ready -->
          <div class="step-content" transition:fade={{ duration: 150 }}>
            <div class="step-icon ready-icon">
              <svg viewBox="0 0 48 48" fill="none">
                <circle cx="24" cy="24" r="20" fill="#f7e479" opacity="0.2"/>
                <circle cx="24" cy="24" r="16" fill="#f7e479"/>
                <path d="M16 24l6 6 12-12" stroke="#000" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h2>You're All Set!</h2>
            <p>CopyGum is ready to use. Start copying and it will remember everything!</p>

            <div class="quick-tips">
              <div class="tip">
                <kbd>{isMac ? '⌘' : 'Ctrl'}+Shift+V</kbd>
                <span>Open CopyGum</span>
              </div>
              <div class="tip">
                <kbd>Enter</kbd>
                <span>Paste selected item</span>
              </div>
              <div class="tip">
                <kbd>Esc</kbd>
                <span>Close overlay</span>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Navigation -->
      <div class="wizard-nav">
        {#if currentStep > 0}
          <button class="nav-btn secondary" on:click={prevStep}>Back</button>
        {:else}
          <div></div>
        {/if}

        {#if currentStep < steps.length - 1}
          <button class="nav-btn primary" on:click={nextStep}>Continue</button>
        {:else}
          <button class="nav-btn primary" on:click={completeSetup}>Get Started</button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .wizard-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
  }

  .wizard-container {
    background: linear-gradient(135deg, #1a1a22 0%, #14141a 100%);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 20px;
    width: 90%;
    max-width: 480px;
    padding: 32px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }

  .wizard-progress {
    display: flex;
    justify-content: center;
    gap: 12px;
    margin-bottom: 32px;
  }

  .progress-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    transition: all 0.3s;
  }

  .progress-dot.active {
    background: #f7e479;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.5);
  }

  .progress-dot.completed {
    background: rgba(247, 228, 121, 0.5);
  }

  .wizard-content {
    min-height: 320px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .step-content {
    text-align: center;
    width: 100%;
  }

  .step-icon {
    width: 80px;
    height: 80px;
    margin: 0 auto 24px;
  }

  .step-icon svg {
    width: 100%;
    height: 100%;
  }

  h2 {
    font-size: 24px;
    font-weight: 700;
    color: #fff;
    margin-bottom: 12px;
  }

  p {
    color: rgba(255, 255, 255, 0.7);
    font-size: 15px;
    line-height: 1.6;
    margin-bottom: 24px;
  }

  .feature-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    text-align: left;
    background: rgba(247, 228, 121, 0.05);
    padding: 20px;
    border-radius: 12px;
  }

  .feature-item {
    display: flex;
    align-items: center;
    gap: 12px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 14px;
  }

  .feature-check {
    color: #f7e479;
    font-weight: bold;
  }

  .shortcut-display {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin: 24px 0;
  }

  .shortcut-display kbd {
    padding: 10px 16px;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 8px;
    color: #f7e479;
    font-family: monospace;
    font-size: 16px;
    font-weight: 600;
  }

  .shortcut-display span {
    color: rgba(255, 255, 255, 0.5);
    font-size: 18px;
  }

  .permission-note {
    display: flex;
    gap: 12px;
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 12px;
    padding: 16px;
    text-align: left;
  }

  .note-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .note-text {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.8);
    line-height: 1.5;
  }

  .note-text strong {
    color: #fff;
  }

  .storage-selector {
    display: flex;
    gap: 8px;
    margin-bottom: 16px;
  }

  .storage-path {
    flex: 1;
    padding: 12px 16px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 13px;
    font-family: monospace;
  }

  .select-folder-btn {
    padding: 12px 20px;
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 8px;
    color: #f7e479;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .select-folder-btn:hover:not(:disabled) {
    background: rgba(247, 228, 121, 0.25);
  }

  .select-folder-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .storage-note {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.5);
  }

  .storage-note p {
    margin: 0;
    font-size: inherit;
  }

  .quick-tips {
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: rgba(247, 228, 121, 0.05);
    padding: 20px;
    border-radius: 12px;
  }

  .tip {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.9);
  }

  .tip kbd {
    padding: 6px 12px;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    color: #f7e479;
    font-family: monospace;
    font-size: 12px;
    min-width: 100px;
    text-align: center;
  }

  .wizard-nav {
    display: flex;
    justify-content: space-between;
    margin-top: 32px;
    padding-top: 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .nav-btn {
    padding: 12px 28px;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .nav-btn.primary {
    background: #f7e479;
    color: #000;
  }

  .nav-btn.primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(247, 228, 121, 0.4);
  }

  .nav-btn.secondary {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .nav-btn.secondary:hover {
    background: rgba(255, 255, 255, 0.15);
  }
</style>
