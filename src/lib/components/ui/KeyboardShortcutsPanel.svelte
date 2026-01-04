<script lang="ts">
  import { keyboardManager } from '../../utils/keyboardManager';
  import { onMount } from 'svelte';

  export let show: boolean = false;

  let shortcutsByCategory: Record<string, any[]> = {
    navigation: [],
    actions: [],
    search: [],
    global: []
  };

  onMount(() => {
    shortcutsByCategory = keyboardManager.getShortcutsByCategory();
  });

  function getCategoryTitle(category: string): string {
    const titles: Record<string, string> = {
      global: 'Global Shortcuts',
      navigation: 'Navigation',
      actions: 'Actions',
      search: 'Search'
    };
    return titles[category] || category;
  }

  function handleClose() {
    show = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' || e.key === '?') {
      handleClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if show}
  <div class="shortcuts-overlay" on:click={handleClose} role="button" tabindex="-1">
    <div class="shortcuts-panel" on:click|stopPropagation role="dialog" aria-label="Keyboard Shortcuts">
      <div class="shortcuts-header">
        <h2>Keyboard Shortcuts</h2>
        <button class="close-button" on:click={handleClose} aria-label="Close">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
            <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>
      </div>

      <div class="shortcuts-content">
        {#each Object.entries(shortcutsByCategory) as [category, shortcuts]}
          {#if shortcuts.length > 0}
            <div class="shortcuts-category">
              <h3 class="category-title">{getCategoryTitle(category)}</h3>
              <div class="shortcuts-list">
                {#each shortcuts as shortcut}
                  <div class="shortcut-row">
                    <span class="shortcut-description">{shortcut.description}</span>
                    <kbd class="shortcut-keys">{keyboardManager.getShortcutLabel(shortcut)}</kbd>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      </div>

      <div class="shortcuts-footer">
        <span class="hint">Press <kbd>?</kbd> or <kbd>Esc</kbd> to close</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcuts-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .shortcuts-panel {
    width: 90%;
    max-width: 700px;
    max-height: 80vh;
    background: rgba(30, 30, 30, 0.95);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 16px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .shortcuts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 24px 28px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.2);
  }

  .shortcuts-header h2 {
    margin: 0;
    font-size: 24px;
    font-weight: 700;
    color: #f7e479;
    text-shadow: 0 0 20px rgba(247, 228, 121, 0.3);
  }

  .close-button {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(247, 228, 121, 0.1);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 8px;
    color: rgba(247, 228, 121, 0.9);
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background: rgba(247, 228, 121, 0.2);
    border-color: rgba(247, 228, 121, 0.6);
    transform: scale(1.1);
  }

  .shortcuts-content {
    flex: 1;
    overflow-y: auto;
    padding: 20px 28px;
  }

  .shortcuts-content::-webkit-scrollbar {
    width: 8px;
  }

  .shortcuts-content::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 4px;
  }

  .shortcuts-content::-webkit-scrollbar-thumb:hover {
    background: rgba(247, 228, 121, 0.5);
  }

  .shortcuts-category {
    margin-bottom: 32px;
  }

  .shortcuts-category:last-child {
    margin-bottom: 0;
  }

  .category-title {
    font-size: 16px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin: 0 0 16px 0;
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: rgba(247, 228, 121, 0.05);
    border: 1px solid rgba(247, 228, 121, 0.1);
    border-radius: 8px;
    transition: all 0.2s;
  }

  .shortcut-row:hover {
    background: rgba(247, 228, 121, 0.1);
    border-color: rgba(247, 228, 121, 0.3);
    transform: translateX(4px);
  }

  .shortcut-description {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.85);
  }

  .shortcut-keys {
    padding: 6px 12px;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 6px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 12px;
    font-weight: 600;
    color: #f7e479;
    white-space: nowrap;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .shortcuts-footer {
    padding: 16px 28px;
    border-top: 1px solid rgba(247, 228, 121, 0.2);
    text-align: center;
  }

  .hint {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.6);
  }

  .hint kbd {
    display: inline-block;
    padding: 3px 8px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 4px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 11px;
    color: #f7e479;
    margin: 0 4px;
  }
</style>
