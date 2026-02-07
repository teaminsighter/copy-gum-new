<script lang="ts">
  // Tag Dropdown for Cards (Opens Upward)
  // Reference: preview.html lines 1308-1513

  import { allTags } from '../../stores/tagStore';

  export let show: boolean = false;
  export let selectedTags: string[] = [];
  export let onToggleTag: (tagName: string) => void = () => {};
  export let onEditTag: (tagName: string) => void = () => {};
  export let onDeleteTag: (tagName: string) => void = () => {};
  // Note: onCreateTag is handled by parent component, not used directly here
  export let onCreateTag: (() => void) | undefined = undefined;
  // Suppress unused warning - this prop is for parent component interface consistency
  void onCreateTag;

  // Get all tags (default + custom) from store
  $: availableTags = $allTags;

  function handleToggleTag(tagName: string, e: Event) {
    e.stopPropagation();
    onToggleTag(tagName);
  }

  function handleEdit(tagName: string, e: Event) {
    e.stopPropagation();
    onEditTag(tagName);
  }

  function handleDelete(tagName: string, e: Event) {
    e.stopPropagation();
    onDeleteTag(tagName);
  }
</script>

<div class="tag-dropdown" class:show>
  {#each availableTags as tag (tag.name)}
    <div
      class="tag-dropdown-item"
      class:selected={selectedTags.includes(tag.name)}
      on:click={(e) => handleToggleTag(tag.name, e)}
      on:keypress={(e) => e.key === 'Enter' && handleToggleTag(tag.name, e)}
      role="button"
      tabindex="0"
    >
      <span class="icon">{tag.icon}</span>
      <span>{tag.name}</span>

      <!-- Only show edit/delete for custom tags (not default) -->
      {#if !tag.isDefault}
        <div class="dropdown-item-actions">
          <div
            class="dropdown-item-edit"
            on:click={(e) => handleEdit(tag.name, e)}
            on:keypress={(e) => e.key === 'Enter' && handleEdit(tag.name, e)}
            role="button"
            tabindex="0"
            title="Edit tag"
          >
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div
            class="dropdown-item-delete"
            on:click={(e) => handleDelete(tag.name, e)}
            on:keypress={(e) => e.key === 'Enter' && handleDelete(tag.name, e)}
            role="button"
            tabindex="0"
            title="Delete tag"
          >
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 6H5H21" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  /* Reference: preview.html lines 1308-1513 */
  .tag-dropdown {
    --main-color: #f7e479;
    position: absolute;
    bottom: calc(100% + 8px); /* Opens UPWARD */
    right: 0;
    background: rgba(20, 20, 20, 0.95);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 10px;
    padding: 6px;
    min-width: 180px;
    max-height: 200px;
    overflow-y: auto;
    z-index: 10000;
    opacity: 0;
    visibility: hidden;
    transform: translateY(10px);
    transition: all 0.2s ease;
    backdrop-filter: blur(20px);
    box-shadow: 0 -10px 40px rgba(0, 0, 0, 0.5);
  }

  .tag-dropdown.show {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
  }

  .tag-dropdown::-webkit-scrollbar {
    width: 6px;
  }

  .tag-dropdown::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  .tag-dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    padding-left: 16px;
    padding-right: 32px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.85);
    position: relative;
  }

  .tag-dropdown-item::before {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    height: 0%;
    width: 2px;
    background: linear-gradient(0deg, rgba(0, 0, 0, 0) 0%, var(--main-color) 50%, rgba(0, 0, 0, 0) 100%);
    transition: height 0.3s ease;
    pointer-events: none;
  }

  .tag-dropdown-item::after {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    height: 0%;
    width: 80px;
    background: linear-gradient(90deg, rgba(247, 228, 121, 0.3) 0%, rgba(0, 0, 0, 0) 100%);
    opacity: 0;
    transition: all 0.3s ease;
    pointer-events: none;
  }

  .tag-dropdown-item:hover {
    color: var(--main-color);
  }

  .tag-dropdown-item:hover::before {
    height: 80%;
    box-shadow: 0 0 10px var(--main-color);
  }

  .tag-dropdown-item:hover::after {
    height: 80%;
    opacity: 1;
  }

  .tag-dropdown-item.selected {
    color: var(--main-color);
    font-weight: 600;
  }

  .tag-dropdown-item.selected::before {
    height: 100%;
    box-shadow: 0 0 15px var(--main-color);
  }

  .tag-dropdown-item.selected::after {
    height: 100%;
    opacity: 1;
  }

  .tag-dropdown-item .icon {
    font-size: 14px;
  }

  /* Dropdown Item Edit & Delete Icons */
  .dropdown-item-actions {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    gap: 6px;
    opacity: 0;
    transition: opacity 0.2s ease;
    z-index: 15;
    pointer-events: auto;
  }

  .tag-dropdown-item:hover .dropdown-item-actions {
    opacity: 1;
  }

  .dropdown-item-edit,
  .dropdown-item-delete {
    width: 20px;
    height: 20px;
    padding: 2px;
    cursor: pointer;
    transition: all 0.2s ease;
    pointer-events: auto;
    position: relative;
    z-index: 20;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dropdown-item-edit svg {
    width: 100%;
    height: 100%;
    stroke: var(--main-color);
    stroke-width: 2;
  }

  .dropdown-item-delete svg {
    width: 100%;
    height: 100%;
    stroke: #f56565;
    stroke-width: 2;
  }

  .dropdown-item-edit:hover {
    transform: scale(1.15);
  }

  .dropdown-item-delete:hover {
    transform: scale(1.15);
  }
</style>
