<script lang="ts">
  // Empty State Component
  // Shown when no items exist or no search results

  export let icon: string = '📋';
  export let title: string = 'No items yet';
  export let message: string = 'Your clipboard history will appear here';
  export let actionText: string = '';
  export let onAction: (() => void) | null = null;
</script>

<div class="empty-state">
  <div class="empty-icon">{icon}</div>
  <h3 class="empty-title">{title}</h3>
  <p class="empty-message">{message}</p>

  {#if actionText && onAction}
    <button class="empty-action" on:click={onAction}>
      {actionText}
    </button>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 40px 20px;
    text-align: center;
  }

  .empty-icon {
    font-size: 64px;
    margin-bottom: 20px;
    opacity: 0.5;
    filter: grayscale(0.3);
  }

  .empty-title {
    font-size: 20px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin: 0 0 12px 0;
  }

  .empty-message {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.6);
    margin: 0 0 24px 0;
    max-width: 300px;
    line-height: 1.6;
  }

  .empty-action {
    padding: 10px 24px;
    background: rgba(247, 228, 121, 0.2);
    border: 1px solid rgba(247, 228, 121, 0.5);
    border-radius: 8px;
    color: #f7e479;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .empty-action:hover {
    background: rgba(247, 228, 121, 0.3);
    border-color: #f7e479;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.3);
  }
</style>
