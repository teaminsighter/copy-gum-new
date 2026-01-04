<script lang="ts">
  // Toast Container - Displays all active toasts
  import { toasts, removeToast } from '../../stores/toastStore';
  import Toast from './Toast.svelte';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <Toast
      message={toast.message}
      type={toast.type}
      duration={toast.duration || 3000}
      onClose={() => removeToast(toast.id)}
    />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 0;
    right: 0;
    z-index: 10001;
    pointer-events: none;
  }

  .toast-container :global(.toast) {
    pointer-events: auto;
    margin-bottom: 12px;
  }
</style>
