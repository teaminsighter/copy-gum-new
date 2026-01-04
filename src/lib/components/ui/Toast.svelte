<script lang="ts">
  // Toast Notification Component
  // Shows success/error/info messages with auto-dismiss

  import { onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';

  export let message: string = '';
  export let type: 'success' | 'error' | 'info' = 'success';
  export let duration: number = 3000; // Auto-dismiss after 3s
  export let onClose: () => void = () => {};

  let visible = true;

  // Auto-dismiss
  onMount(() => {
    const timeout = setTimeout(() => {
      visible = false;
      setTimeout(onClose, 300); // Wait for animation
    }, duration);

    return () => clearTimeout(timeout);
  });

  function handleClose() {
    visible = false;
    setTimeout(onClose, 300);
  }

  // Icon based on type
  const icons = {
    success: '✓',
    error: '✕',
    info: 'ℹ'
  };

  // Colors based on type
  const colors = {
    success: {
      bg: 'rgba(34, 197, 94, 0.2)',
      border: 'rgba(34, 197, 94, 0.6)',
      text: '#22c55e',
      glow: 'rgba(34, 197, 94, 0.3)'
    },
    error: {
      bg: 'rgba(239, 68, 68, 0.2)',
      border: 'rgba(239, 68, 68, 0.6)',
      text: '#ef4444',
      glow: 'rgba(239, 68, 68, 0.3)'
    },
    info: {
      bg: 'rgba(59, 130, 246, 0.2)',
      border: 'rgba(59, 130, 246, 0.6)',
      text: '#3b82f6',
      glow: 'rgba(59, 130, 246, 0.3)'
    }
  };

  $: currentColors = colors[type];
</script>

{#if visible}
  <div
    class="toast"
    transition:fly={{ y: -20, duration: 300 }}
    style="
      background: {currentColors.bg};
      border-color: {currentColors.border};
      box-shadow: 0 0 20px {currentColors.glow}, 0 8px 32px rgba(0, 0, 0, 0.5);
    "
  >
    <div class="toast-icon" style="color: {currentColors.text};">
      {icons[type]}
    </div>
    <div class="toast-message" style="color: {currentColors.text};">
      {message}
    </div>
    <button
      class="toast-close"
      on:click={handleClose}
      style="color: {currentColors.text};"
      aria-label="Close notification"
    >
      ✕
    </button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    top: 20px;
    right: 20px;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 10px;
    border: 1px solid;
    backdrop-filter: blur(20px);
    z-index: 10001;
    min-width: 280px;
    max-width: 400px;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  .toast-icon {
    font-size: 20px;
    font-weight: bold;
    flex-shrink: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toast-message {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.4;
  }

  .toast-close {
    background: none;
    border: none;
    font-size: 18px;
    cursor: pointer;
    padding: 4px;
    line-height: 1;
    opacity: 0.7;
    transition: opacity 0.2s ease;
    flex-shrink: 0;
  }

  .toast-close:hover {
    opacity: 1;
  }
</style>
