<script lang="ts">
  // Category Dropdown for Cards
  // Reference: preview.html lines 1144-1307, 2678-2750

  import { categoryCustomizations, customCategories } from '../../stores/categoryStore';

  export let show: boolean = false;
  export let currentCategory: string = 'text';
  export let onSelectCategory: (categoryId: string) => void = () => {};
  export let onEditCategory: (categoryId: string, isCustom: boolean) => void = () => {};
  export let onDeleteCategory: (categoryId: string) => void = () => {};

  const defaultCategories = [
    { id: 'all', icon: '📌', name: 'All', isCustom: false },
    { id: 'password', icon: '🔐', name: 'Password', isCustom: false },
    { id: 'apikey', icon: '🔑', name: 'API Key', isCustom: false },
    { id: 'private', icon: '🔒', name: 'Private', isCustom: false },
    { id: 'email', icon: '📧', name: 'Email', isCustom: false },
    { id: 'phone', icon: '📱', name: 'Phone', isCustom: false },
    { id: 'links', icon: '🔗', name: 'Links', isCustom: false },
    { id: 'code', icon: '💻', name: 'Code', isCustom: false },
    { id: 'color', icon: '🎨', name: 'Color', isCustom: false },
    { id: 'image', icon: '🖼️', name: 'Image', isCustom: false },
    { id: 'number', icon: '🔢', name: 'Number', isCustom: false },
    { id: 'text', icon: '📝', name: 'Text', isCustom: false },
  ];

  // Merge default categories with custom icons, then add custom categories
  $: categories = [
    ...defaultCategories.map(cat => ({
      ...cat,
      icon: $categoryCustomizations[cat.id]?.icon || cat.icon,
      isCustom: false // Ensure default categories have isCustom: false
    })),
    ...$customCategories.map(cat => ({
      ...cat,
      isCustom: true // Ensure custom categories have isCustom: true
    }))
  ];

  function handleSelect(categoryId: string, e: Event) {
    e.stopPropagation();
    onSelectCategory(categoryId);
  }

  function handleEdit(categoryId: string, isCustom: boolean, e: Event) {
    e.stopPropagation();
    onEditCategory(categoryId, isCustom);
  }

  function handleDelete(categoryId: string, e: Event) {
    e.stopPropagation();
    onDeleteCategory(categoryId);
  }
</script>

<div class="category-dropdown" class:show>
  {#each categories as category (category.id)}
    <div
      class="category-dropdown-item"
      class:active={currentCategory === category.id}
      on:click={(e) => handleSelect(category.id, e)}
      on:keypress={(e) => e.key === 'Enter' && handleSelect(category.id, e)}
      role="button"
      tabindex="0"
    >
      <span class="icon">{category.icon}</span>
      <span>{category.name}</span>

      <!-- Show actions only for custom categories -->
      {#if category.isCustom}
        <div class="dropdown-item-actions">
          <!-- Edit icon - only for custom categories -->
          <div
            class="dropdown-item-edit"
            on:click={(e) => handleEdit(category.id, category.isCustom, e)}
            on:keypress={(e) => e.key === 'Enter' && handleEdit(category.id, category.isCustom, e)}
            role="button"
            tabindex="0"
            title="Edit category"
          >
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>

          <!-- Delete icon -->
          <div
            class="dropdown-item-delete"
            on:click={(e) => handleDelete(category.id, e)}
            on:keypress={(e) => e.key === 'Enter' && handleDelete(category.id, e)}
            role="button"
            tabindex="0"
            title="Delete category"
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
  /* Reference: preview.html lines 1144-1307 */
  .category-dropdown {
    --main-color: #f7e479;
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    background: rgba(20, 20, 20, 0.95);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 10px;
    padding: 6px;
    min-width: 180px;
    max-height: 220px;
    overflow-y: auto;
    z-index: 10000;
    opacity: 0;
    visibility: hidden;
    transform: translateY(-10px);
    transition: all 0.2s ease;
    backdrop-filter: blur(20px);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
  }

  .category-dropdown.show {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
  }

  .category-dropdown::-webkit-scrollbar {
    width: 6px;
  }

  .category-dropdown::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  .category-dropdown-item {
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

  .category-dropdown-item::before {
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

  .category-dropdown-item::after {
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

  .category-dropdown-item:hover {
    color: var(--main-color);
  }

  .category-dropdown-item:hover::before {
    height: 80%;
    box-shadow: 0 0 10px var(--main-color);
  }

  .category-dropdown-item:hover::after {
    height: 80%;
    opacity: 1;
  }

  .category-dropdown-item.active {
    color: var(--main-color);
    font-weight: 600;
  }

  .category-dropdown-item.active::before {
    height: 100%;
    box-shadow: 0 0 15px var(--main-color);
  }

  .category-dropdown-item.active::after {
    height: 100%;
    opacity: 1;
  }

  .category-dropdown-item .icon {
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

  .category-dropdown-item:hover .dropdown-item-actions {
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

  /* Create New Category */
  .dropdown-create-new {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    padding-left: 16px;
    margin-top: 4px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-size: 12px;
    color: rgba(247, 228, 121, 0.9);
    font-weight: 500;
    position: relative;
    border-top: 1px solid rgba(247, 228, 121, 0.15);
  }

  .dropdown-create-new::before {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    height: 0%;
    width: 2px;
    background: linear-gradient(0deg, rgba(0, 0, 0, 0) 0%, #f7e479 50%, rgba(0, 0, 0, 0) 100%);
    transition: height 0.3s ease;
  }

  .dropdown-create-new:hover::before {
    height: 80%;
    box-shadow: 0 0 10px #f7e479;
  }

  /* Create Input */
  .dropdown-create-input {
    display: flex;
    gap: 6px;
    padding: 8px 12px;
    margin-top: 4px;
    border-top: 1px solid rgba(247, 228, 121, 0.15);
  }

  .dropdown-create-input input {
    flex: 1;
    background: rgba(30, 30, 30, 0.9);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    padding: 6px 10px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 12px;
    outline: none;
  }

  .dropdown-create-input input:focus {
    border-color: rgba(247, 228, 121, 0.6);
  }

  .dropdown-create-input button {
    background: rgba(247, 228, 121, 0.2);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 6px;
    padding: 6px 12px;
    color: rgba(247, 228, 121, 0.95);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .dropdown-create-input button:hover {
    background: rgba(247, 228, 121, 0.3);
    border-color: rgba(247, 228, 121, 0.6);
  }
</style>
