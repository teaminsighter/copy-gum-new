<script lang="ts">
  // Confirmation Modal Component
  // Generic modal for confirming dangerous actions

  import { fade, scale } from 'svelte/transition';

  export let show: boolean = false;
  export let title: string = 'Confirm Action';
  export let message: string = 'Are you sure you want to proceed?';
  export let confirmText: string = 'Confirm';
  export let cancelText: string = 'Cancel';
  export let type: 'danger' | 'warning' | 'info' = 'danger';
  export let onConfirm: () => void = () => {};
  export let onCancel: () => void = () => {};

  // Colors based on type
  const colors = {
    danger: {
      border: 'rgba(239, 68, 68, 0.6)',
      button: '#ef4444',
      glow: 'rgba(239, 68, 68, 0.3)'
    },
    warning: {
      border: 'rgba(251, 191, 36, 0.6)',
      button: '#fbbf24',
      glow: 'rgba(251, 191, 36, 0.3)'
    },
    info: {
      border: 'rgba(59, 130, 246, 0.6)',
      button: '#3b82f6',
      glow: 'rgba(59, 130, 246, 0.3)'
    }
  };

  $: currentColors = colors[type];

  function handleConfirm() {
    onConfirm();
  }

  function handleCancel() {
    onCancel();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleCancel();
    } else if (e.key === 'Enter') {
      handleConfirm();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if show}
  <!-- Backdrop -->
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="modal-backdrop" transition:fade={{ duration: 200 }} on:click={handleCancel}>
  </div>

  <!-- Modal -->
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div
    class="modal"
    transition:scale={{ duration: 200, start: 0.95 }}
    on:click|stopPropagation
    style="
      border-color: {currentColors.border};
      box-shadow: 0 0 30px {currentColors.glow}, 0 20px 60px rgba(0, 0, 0, 0.6);
    "
  >
    <div class="modal-header">
      <h3>{title}</h3>
    </div>

    <div class="modal-body">
      <p>{message}</p>
    </div>

    <div class="modal-footer">
      <button class="btn btn-cancel" on:click={handleCancel}>
        {cancelText}
      </button>
      <button
        class="btn btn-confirm"
        on:click={handleConfirm}
        style="
          background: {currentColors.button};
          border-color: {currentColors.button};
          box-shadow: 0 0 12px {currentColors.glow};
        "
      >
        {confirmText}
      </button>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    z-index: 10000;
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: rgba(20, 20, 20, 0.98);
    border: 1px solid;
    border-radius: 12px;
    backdrop-filter: blur(20px);
    z-index: 10001;
    width: 90%;
    max-width: 400px;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  .modal-header {
    padding: 20px 24px 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .modal-header h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: #fff;
  }

  .modal-body {
    padding: 20px 24px;
  }

  .modal-body p {
    margin: 0;
    font-size: 14px;
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.85);
  }

  .modal-footer {
    padding: 12px 24px 20px;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  .btn {
    padding: 10px 20px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid;
  }

  .btn-cancel {
    background: transparent;
    border-color: rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.85);
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .btn-confirm {
    color: #fff;
  }

  .btn-confirm:hover {
    transform: translateY(-1px);
    filter: brightness(1.1);
  }
</style>
