<script lang="ts">
  // Category Edit Panel Component
  // Reference: preview.html lines 1515-1724

  import { onMount, tick } from 'svelte';

  export let show: boolean = false;
  export let categoryName: string = '';
  export let categoryIcon: string = '';
  export let categoryColor: string = '';
  export let onSave: (name: string, icon: string, color: string) => void = () => {};
  export let onClose: () => void = () => {};
  export let showColorTab: boolean = true; // Control whether to show color tab
  export let isNameEditable: boolean = true; // Control whether name can be edited
  export let panelTitle: string = 'Category name'; // Customizable panel title

  let activeTab: 'icon' | 'color' = 'icon';
  let editName = categoryName;
  let selectedIcon = categoryIcon;
  let selectedColor = categoryColor;
  let panelElement: HTMLDivElement;
  let inputElement: HTMLInputElement;

  // Track the last prop values to only update when panel first opens
  let lastCategoryName = '';
  let lastCategoryIcon = '';
  let lastCategoryColor = '';

  // Available icons for categories
  const icons = [
    '📌', '🔐', '🔑', '🔒', '📧', '📱', '🔗', '💻',
    '🎨', '🖼️', '🔢', '📝', '📁', '📅', '💡', '⭐',
    '🔥', '💼', '👤', '🌐', '⚙️', '🎯', '🎬', '🎵',
    '📷', '🎮', '🏠', '✈️', '🚗', '🍕', '☕', '📚',
    '⚡', '🌙', '☀️', '🌈', '💎', '🎁', '🏆', '💰',
    '📊', '💾', '🔍', '📌', '🗂️', '📎', '🖇️', '📝'
  ];

  // Available colors for category backgrounds
  const colors = [
    'rgba(50, 50, 50, 0.6)',           // Dark gray (default)
    'rgba(91, 127, 237, 0.6)',         // Blue
    'rgba(102, 126, 234, 0.6)',        // Purple-blue
    'rgba(139, 79, 155, 0.6)',         // Purple
    'rgba(255, 87, 51, 0.6)',          // Orange-red
    'rgba(255, 165, 0, 0.6)',          // Orange
    'rgba(247, 228, 121, 0.6)',        // Yellow/Gold
    'rgba(34, 197, 94, 0.6)',          // Green
    'rgba(16, 185, 129, 0.6)',         // Teal
    'rgba(59, 130, 246, 0.6)',         // Sky blue
    'rgba(236, 72, 153, 0.6)',         // Pink
    'rgba(239, 68, 68, 0.6)',          // Red
  ];

  function switchTab(tab: 'icon' | 'color', event: MouseEvent) {
    event.stopPropagation();
    activeTab = tab;
  }

  function selectIcon(icon: string, event: MouseEvent) {
    event.stopPropagation();
    selectedIcon = icon;
  }

  function selectColor(color: string, event: MouseEvent) {
    event.stopPropagation();
    selectedColor = color;
  }

  function handleSave(event: MouseEvent) {
    event.stopPropagation();
    onSave(editName, selectedIcon, selectedColor);
    onClose();
  }

  function handleInputClick(event: MouseEvent) {
    event.stopPropagation();
  }

  // Update local state ONLY when props change AND panel opens (not continuously)
  $: {
    if (show && (
      categoryName !== lastCategoryName ||
      categoryIcon !== lastCategoryIcon ||
      categoryColor !== lastCategoryColor
    )) {
      // Only update if the props actually changed (panel opened with new category)
      editName = categoryName;
      selectedIcon = categoryIcon;
      selectedColor = categoryColor;

      // Remember these values to prevent re-triggering
      lastCategoryName = categoryName;
      lastCategoryIcon = categoryIcon;
      lastCategoryColor = categoryColor;

      // Auto-focus and select input when panel opens and name is editable
      if (isNameEditable) {
        tick().then(() => {
          inputElement?.focus();
          inputElement?.select();
        });
      }
    }
  }
</script>

<div bind:this={panelElement} class="edit-panel" class:show>
  <!-- Header with Name Input and Save Button -->
  <div class="edit-panel-header">
    <input
      bind:this={inputElement}
      type="text"
      class="edit-panel-input"
      bind:value={editName}
      placeholder={panelTitle}
      readonly={!isNameEditable}
      on:click={handleInputClick}
    />
    <button class="edit-panel-save-btn" on:click={handleSave}>
      Save
    </button>
  </div>

  <!-- Tabs: Icon | Color -->
  <div class="edit-panel-tabs">
    <button
      class="edit-panel-tab"
      class:active={activeTab === 'icon'}
      on:click={(e) => switchTab('icon', e)}
    >
      Icon
    </button>
    {#if showColorTab}
      <button
        class="edit-panel-tab"
        class:active={activeTab === 'color'}
        on:click={(e) => switchTab('color', e)}
      >
        Color
      </button>
    {/if}
  </div>

  <!-- Content Area -->
  <div class="edit-panel-content">
    <!-- Icon Tab Content -->
    {#if activeTab === 'icon'}
      <div class="edit-tab-content active">
        <div class="edit-panel-icon-grid">
          {#each icons as icon}
            <button
              class="edit-panel-icon-item"
              class:selected={selectedIcon === icon}
              on:click={(e) => selectIcon(icon, e)}
            >
              {icon}
            </button>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Color Tab Content -->
    {#if showColorTab && activeTab === 'color'}
      <div class="edit-tab-content active">
        <div class="edit-panel-color-grid">
          {#each colors as color}
            <button
              class="edit-panel-color-item"
              class:selected={selectedColor === color}
              style="background: {color};"
              on:click={(e) => selectColor(color, e)}
              aria-label="Select color {color}"
            >
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  /* Reference: preview.html lines 1515-1543 */
  .edit-panel {
    position: relative;
    width: 288px;
    max-height: 320px;
    background: rgba(20, 20, 20, 0.98);
    border-radius: 12px;
    backdrop-filter: blur(20px);
    z-index: 15000;
    display: none;
    flex-direction: column;
    overflow: hidden;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6), inset 0 0 0 1px rgba(247, 228, 121, 0.4);
    box-sizing: border-box;
    opacity: 0;
    transform: scale(0.95);
    transition: opacity 0.2s ease-out, transform 0.2s ease-out;
  }

  .edit-panel.show {
    display: flex;
    opacity: 1;
    transform: scale(1);
    pointer-events: auto;
    visibility: visible;
  }

  /* Header - Reference: preview.html lines 1544-1592 */
  .edit-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px 10px 10px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.2);
    gap: 8px;
  }

  .edit-panel-input {
    flex: 1;
    background: rgba(40, 40, 40, 0.8);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    padding: 8px 12px;
    color: #fff;
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: all 0.2s ease;
  }

  .edit-panel-input:focus {
    border-color: rgba(247, 228, 121, 0.6);
    box-shadow: 0 0 0 3px rgba(247, 228, 121, 0.1);
  }

  .edit-panel-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .edit-panel-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background: rgba(30, 30, 30, 0.8);
  }

  .edit-panel-save-btn {
    background: rgba(247, 228, 121, 0.2);
    border: 1px solid rgba(247, 228, 121, 0.5);
    color: #f7e479;
    padding: 8px 18px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .edit-panel-save-btn:hover {
    background: rgba(247, 228, 121, 0.3);
    border-color: #f7e479;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.3);
  }

  /* Tabs - Reference: preview.html lines 1594-1619 */
  .edit-panel-tabs {
    display: flex;
    padding: 0 12px 0 10px;
    gap: 6px;
    border-bottom: 1px solid rgba(247, 228, 121, 0.1);
  }

  .edit-panel-tab {
    padding: 8px 16px;
    font-size: 12px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    transition: all 0.2s ease;
    border-bottom: 2px solid transparent;
    position: relative;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
  }

  .edit-panel-tab:hover {
    color: rgba(247, 228, 121, 0.8);
  }

  .edit-panel-tab.active {
    color: #f7e479;
    border-bottom-color: #f7e479;
  }

  /* Content - Reference: preview.html lines 1621-1647 */
  .edit-panel-content {
    flex: 1;
    padding: 10px 12px 10px 10px;
    overflow-y: auto;
    overflow-x: hidden;
    min-height: 0;
    scroll-behavior: smooth;
  }

  .edit-panel-content::-webkit-scrollbar {
    width: 6px;
  }

  .edit-panel-content::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .edit-panel-content::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
    transition: background 0.2s ease;
  }

  .edit-panel-content::-webkit-scrollbar-thumb:hover {
    background: rgba(247, 228, 121, 0.5);
  }

  /* Icon Grid - Reference: preview.html lines 1649-1678 */
  .edit-panel-icon-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 8px;
    overflow-y: auto;
    overflow-x: hidden;
    max-height: 200px;
    padding-right: 4px;
    scroll-behavior: smooth;
  }

  .edit-panel-icon-grid::-webkit-scrollbar {
    width: 6px;
  }

  .edit-panel-icon-grid::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .edit-panel-icon-grid::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
    transition: background 0.2s ease;
  }

  .edit-panel-icon-grid::-webkit-scrollbar-thumb:hover {
    background: rgba(247, 228, 121, 0.5);
  }

  .edit-panel-icon-item {
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    background: rgba(40, 40, 40, 0.6);
    border: 2px solid rgba(247, 228, 121, 0.2);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .edit-panel-icon-item:hover {
    background: rgba(60, 60, 60, 0.8);
    border-color: rgba(247, 228, 121, 0.6);
    transform: scale(1.05);
  }

  .edit-panel-icon-item.selected {
    background: rgba(247, 228, 121, 0.2);
    border-color: #f7e479;
    box-shadow: 0 0 12px rgba(247, 228, 121, 0.3);
  }

  /* Color Grid - Reference: preview.html lines 1680-1716 */
  .edit-panel-color-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 8px;
  }

  .edit-panel-color-item {
    aspect-ratio: 1;
    border: 3px solid rgba(247, 228, 121, 0.2);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
  }

  .edit-panel-color-item:hover {
    transform: scale(1.08);
    border-color: rgba(247, 228, 121, 0.5);
  }

  .edit-panel-color-item.selected {
    border-color: #f7e479;
    box-shadow: 0 0 16px rgba(247, 228, 121, 0.4);
    transform: scale(1.1);
  }

  .edit-panel-color-item.selected::after {
    content: '✓';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #fff;
    font-size: 18px;
    font-weight: bold;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  }

  /* Tab Content - Reference: preview.html lines 1718-1724 */
  .edit-tab-content {
    display: none;
  }

  .edit-tab-content.active {
    display: block;
  }
</style>
