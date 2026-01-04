<script lang="ts">
  // Keyboard Shortcuts Help Panel
  export let show = false;

  // Detect platform for displaying correct modifier key
  const isMac = navigator.platform.toLowerCase().includes('mac');
  const modKey = isMac ? 'Cmd' : 'Ctrl';

  // Shortcut categories
  const shortcuts = [
    {
      category: 'Window',
      items: [
        { keys: [modKey, 'Shift', 'V'], desc: 'Toggle window' },
        { keys: ['Esc'], desc: 'Close window / Clear search' },
      ]
    },
    {
      category: 'Navigation',
      items: [
        { keys: ['←', '→'], desc: 'Navigate items' },
        { keys: ['↑'], desc: 'Switch to categories' },
        { keys: ['↓'], desc: 'Switch to items' },
        { keys: [modKey, 'F'], desc: 'Focus search' },
        { keys: [modKey, 'Shift', 'F'], desc: 'Toggle filters' },
      ]
    },
    {
      category: 'Actions',
      items: [
        { keys: ['Enter'], desc: 'Copy selected item' },
        { keys: [modKey, 'P'], desc: 'Pin/unpin item' },
        { keys: [modKey, 'Delete'], desc: 'Delete item' },
        { keys: [modKey, 'Shift', 'C'], desc: 'Clear filters' },
      ]
    },
    {
      category: 'Help',
      items: [
        { keys: ['?'], desc: 'Show this help' },
      ]
    },
  ];

  function close() {
    show = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && show) {
      close();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="shortcuts-overlay" on:click={close}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="shortcuts-panel" on:click|stopPropagation>
      <div class="shortcuts-header">
        <h2>Keyboard Shortcuts</h2>
        <button class="close-btn" on:click={close} aria-label="Close shortcuts help">✕</button>
      </div>

      <div class="shortcuts-content">
        {#each shortcuts as section}
          <div class="shortcut-category">
            <h3>{section.category}</h3>
            {#each section.items as shortcut}
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
        {/each}
      </div>

      <div class="shortcuts-footer">
        <p>Press <kbd>Esc</kbd> to close this window</p>
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcuts-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(10px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
    animation: fadeIn 0.2s ease-out;
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
    background: rgba(30, 30, 35, 0.98);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 16px;
    max-width: 600px;
    width: 90%;
    max-height: 80vh;
    overflow: hidden;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    animation: slideUp 0.3s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .shortcuts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .shortcuts-header h2 {
    margin: 0;
    font-size: 20px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.95);
  }

  .close-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.7);
    font-size: 18px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.95);
    transform: scale(1.05);
  }

  .shortcuts-content {
    padding: 24px;
    overflow-y: auto;
    max-height: calc(80vh - 160px);
  }

  .shortcut-category {
    margin-bottom: 32px;
  }

  .shortcut-category:last-child {
    margin-bottom: 0;
  }

  .shortcut-category h3 {
    font-size: 14px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.7);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin: 0 0 12px 0;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .shortcut-row:last-child {
    border-bottom: none;
  }

  .shortcut-keys {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 32px;
    height: 28px;
    padding: 0 10px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .shortcut-desc {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.8);
  }

  .shortcuts-footer {
    padding: 16px 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(0, 0, 0, 0.2);
  }

  .shortcuts-footer p {
    margin: 0;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.6);
    text-align: center;
  }

  .shortcuts-footer kbd {
    font-size: 11px;
    min-width: 24px;
    height: 22px;
  }

  /* Scrollbar */
  .shortcuts-content::-webkit-scrollbar {
    width: 6px;
  }

  .shortcuts-content::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .shortcuts-content::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  .shortcuts-content::-webkit-scrollbar-thumb:hover {
    background: rgba(247, 228, 121, 0.5);
  }
</style>
