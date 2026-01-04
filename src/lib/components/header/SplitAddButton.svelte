<script lang="ts">
  // Split Add Button Component
  // Reference: preview.html lines 133-234

  export let onAddCategory: () => void = () => {};
  export let onAddTag: () => void = () => {};
</script>

<div class="split-add-btn">
  <!-- Default state: Single + icon -->
  <div class="split-add-default">
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
    </svg>
  </div>

  <!-- Expanded state: Category | Tag -->
  <div class="split-add-content">
    <!-- Left: Add Category -->
    <button class="split-add-left" on:click={onAddCategory}>
      <span class="split-icon">📝</span>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
      </svg>
    </button>

    <!-- Separator -->
    <div class="split-separator"></div>

    <!-- Right: Add Tag -->
    <button class="split-add-right" on:click={onAddTag}>
      <span class="split-icon">🏷️</span>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
      </svg>
    </button>
  </div>
</div>

<style>
  /* Reference: preview.html lines 134-153 */
  .split-add-btn {
    width: 40px;
    height: 40px;
    background: rgba(40, 40, 40, 0.8);
    border: 1px solid rgba(60, 60, 60, 0.5);
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: visible;
  }

  .split-add-btn:hover {
    width: 90px;
    border-color: rgba(247, 228, 121, 0.6);
    background: rgba(40, 40, 40, 0.9);
  }

  /* Default state - Reference: preview.html lines 155-174 */
  .split-add-default {
    position: absolute;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 1;
    transition: opacity 0.3s ease;
  }

  .split-add-default svg {
    width: 20px;
    height: 20px;
    stroke: rgba(247, 228, 121, 0.85);
    transition: all 0.2s;
  }

  .split-add-btn:hover .split-add-default {
    opacity: 0;
    pointer-events: none;
  }

  /* Expanded content - Reference: preview.html lines 176-189 */
  .split-add-content {
    display: flex;
    width: 100%;
    height: 100%;
    opacity: 0;
    transition: opacity 0.3s ease;
    pointer-events: none;
    overflow: visible;
  }

  .split-add-btn:hover .split-add-content {
    opacity: 1;
    pointer-events: all;
  }

  /* Split buttons - Reference: preview.html lines 191-228 */
  .split-add-left,
  .split-add-right {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    position: relative;
    transition: all 0.2s ease;
    overflow: visible;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .split-add-left:hover {
    background: rgba(247, 228, 121, 0.15);
  }

  .split-add-right:hover {
    background: rgba(247, 228, 121, 0.15);
  }

  .split-icon {
    font-size: 14px;
    line-height: 1;
  }

  .split-add-left svg,
  .split-add-right svg {
    width: 12px;
    height: 12px;
    stroke: rgba(247, 228, 121, 0.85);
    transition: all 0.2s;
  }

  .split-add-left:hover svg,
  .split-add-right:hover svg {
    stroke: rgba(247, 228, 121, 1);
    transform: scale(1.1);
  }

  /* Separator - Reference: preview.html lines 230-234 */
  .split-separator {
    width: 1px;
    height: 24px;
    background: rgba(247, 228, 121, 0.3);
  }
</style>
