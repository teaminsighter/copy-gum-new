<script lang="ts">
  // Single Category Pill Component
  // Reference: preview.html lines 2055-2063, 311-463

  export let category: string;
  export let icon: string;
  export let name: string;
  export let active: boolean = false;
  export let isDragging: boolean = false;
  export let isDropTarget: boolean = false;
  export let isCustom: boolean = false; // New prop to check if category is custom
  export let isSelected: boolean = false; // Keyboard navigation selection
  export let onClick: () => void = () => {};
  export let onEdit: (category: string, event?: MouseEvent) => void = () => {};
  export let onMouseDown: (event: MouseEvent) => void = () => {};

  function handleEditClick(e: MouseEvent) {
    e.stopPropagation();
    onEdit(category, e);
  }

  function handleMouseDown(e: MouseEvent) {
    // Don't drag if clicking edit icon
    if ((e.target as HTMLElement).closest('.category-edit')) {
      return;
    }

    onMouseDown(e);
  }
</script>

<div
  class="category-pill"
  class:active
  class:custom={isCustom}
  class:dragging={isDragging}
  class:drop-target={isDropTarget}
  class:selected={isSelected}
  data-category={category}
  on:click={onClick}
  on:keypress={(e) => e.key === 'Enter' && onClick()}
  on:mousedown={handleMouseDown}
  role="button"
  tabindex="0"
>
  <div class="glass-filter"></div>
  <div class="glass-overlay"></div>
  <div class="glass-specular"></div>
  <div class="glass-content">
    <span class="category-icon">{icon}</span>
    <span class="category-name">{name}</span>
    {#if isCustom}
      <div
        class="category-edit"
        on:click={handleEditClick}
        on:keypress={(e) => { if (e.key === 'Enter') handleEditClick(e as any); }}
        role="button"
        tabindex="0"
      >
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
    {/if}
  </div>
</div>

<style>
  /* Reference: preview.html lines 311-463 */
  .category-pill {
    flex-shrink: 0;
    height: 36px;
    border-radius: 10px;
    position: relative;
    overflow: visible;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;

    /* Prevent text selection during drag */
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
  }

  .category-pill .glass-filter,
  .category-pill .glass-overlay,
  .category-pill .glass-specular {
    overflow: hidden;
  }

  .category-pill:hover {
    transform: translateY(-1px);
  }

  .category-pill:hover .glass-overlay {
    border: 1.5px solid rgba(247, 228, 121, 0.7);
    box-shadow: 0 0 6px rgba(247, 228, 121, 0.2),
                inset 0 0 5px rgba(247, 228, 121, 0.05);
  }

  .category-pill .glass-filter,
  .category-pill .glass-overlay,
  .category-pill .glass-specular {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    pointer-events: none;
  }

  .category-pill .glass-filter {
    z-index: 0;
    backdrop-filter: blur(6px);
    filter: url(#lensFilter) saturate(115%) brightness(1.1);
  }

  .category-pill .glass-overlay {
    z-index: 1;
    background: rgba(50, 50, 50, 0.6);
    border: 1px solid rgba(90, 90, 90, 0.3);
  }

  .category-pill .glass-specular {
    z-index: 2;
    box-shadow: inset 1px 1px 0 rgba(255, 255, 255, 0.15),
                inset 0 0 10px rgba(255, 255, 255, 0.05);
  }

  .category-pill .glass-content {
    position: relative;
    z-index: 3;
    padding: 0 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    height: 100%;
    font-size: 13px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.85);
    transition: padding 0.2s ease, color 0.2s ease, text-shadow 0.2s ease;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    pointer-events: none;
  }

  /* Only expand custom categories on hover (they have edit icon) */
  .category-pill.custom:hover .glass-content {
    padding-right: 40px;
  }

  .category-pill.active .glass-overlay {
    background: linear-gradient(135deg, rgba(247, 228, 121, 0.3) 0%, rgba(247, 228, 121, 0.4) 100%);
    border-color: rgba(247, 228, 121, 0.8);
    box-shadow: 0 2px 6px rgba(247, 228, 121, 0.2);
  }

  .category-pill.active .glass-content {
    color: white;
  }

  .category-icon {
    font-size: 12px;
    line-height: 1;
    pointer-events: all;
  }

  .category-name {
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    pointer-events: all;
  }

  /* Edit Icon - Reference: preview.html lines 439-462 */
  .category-edit {
    position: absolute;
    right: 12px;
    width: 16px;
    height: 16px;
    opacity: 0;
    transition: opacity 0.15s ease;
    pointer-events: none;
    cursor: pointer;
  }

  .category-pill:hover .category-edit {
    opacity: 0.7;
    pointer-events: all;
  }

  .category-edit:hover {
    opacity: 1;
  }

  .category-edit svg {
    width: 16px;
    height: 16px;
    stroke: rgba(255, 255, 255, 0.8);
  }

  /* Drag states - Reference: preview.html lines 395-437 */
  .category-pill.dragging {
    opacity: 0.4;
    transform: scale(0.95) translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    cursor: grabbing !important;
    z-index: 1000;
    transition: none;
  }

  .category-pill.drop-target {
    border-left: 3px solid #667eea;
    padding-left: 5px;
  }

  .category-pill.drop-target .glass-overlay {
    border-color: rgba(102, 126, 234, 0.8);
    background: rgba(102, 126, 234, 0.2);
  }

  /* Keyboard navigation selection - blue border like cards */
  .category-pill.selected .glass-overlay {
    border: 2px solid rgba(59, 130, 246, 0.9);
    box-shadow: 0 0 24px rgba(59, 130, 246, 0.5),
                inset 0 0 20px rgba(59, 130, 246, 0.2);
  }

  .category-pill.selected .glass-content {
    color: white;
  }
</style>
