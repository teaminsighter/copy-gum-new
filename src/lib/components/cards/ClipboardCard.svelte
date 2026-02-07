<script lang="ts">
  // Clipboard Card Component
  // Reference: preview.html lines 2663-2949, 792-1062

  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import CategoryDropdown from './CategoryDropdown.svelte';
  import TagDropdown from './TagDropdown.svelte';
  import CategoryEditPanel from '../categories/CategoryEditPanel.svelte';
  import ConfirmModal from '../ui/ConfirmModal.svelte';
  import {
    updateCategoryIcon,
    updateCategoryColor,
    createCustomCategory,
    updateCustomCategory,
    deleteCustomCategory,
    customCategories
  } from '../../stores/categoryStore';
  import { allTags, updateTag, deleteTag } from '../../stores/tagStore';
  import { showSuccess, showError } from '../../stores/toastStore';
  import { clickOutside } from '../../utils/clickOutside';
  import { addTagToItemByName, removeTagFromItemByName } from '../../services/database';
  import { loadClipboardItems, updateItemCategory, togglePin, deleteItem } from '../../stores/clipboardStore';
  import { eventBus } from '../../events/eventBus';
  import { shouldBlockClick } from '../../stores/dragStore';

  // Custom event to close all dropdowns globally
  const CLOSE_DROPDOWNS_EVENT = 'closeAllDropdowns';

  export let itemId: number; // Database ID for this clipboard item
  export let appIcon: string = '💻';
  export let appBundleId: string | undefined = undefined;
  export let appName: string = 'Unknown';

  // System icon (fetched from macOS)
  let systemIconUrl: string | null = null;
  let iconLoading = false;
  export let category: string = 'text'; // Category ID (not label)

  // Map category ID to label
  const categoryMap: Record<string, string> = {
    all: 'All',
    password: 'Password',
    apikey: 'API Key',
    private: 'Private',
    email: 'Email',
    phone: 'Phone',
    links: 'Links',
    code: 'Code',
    color: 'Color',
    image: 'Image',
    number: 'Number',
    text: 'Text',
  };

  // Compute category label reactively from category ID
  // This will update when category changes OR when customCategories store updates
  $: categoryLabel = (() => {
    // Check if it's a custom category
    const customCat = $customCategories.find(c => c.id === category);
    if (customCat) {
      return customCat.name;
    }
    // Otherwise use the category map
    return categoryMap[category] || category;
  })();
  export let content: string = '';
  export let timestamp: string = 'Just now';
  export let charCount: number = 0;
  export let isPinned: boolean = false;
  export let tags: string[] = [];
  export let customBg: string = ''; // For color/image cards
  export let isLightBg: boolean = false;
  export let isSelected: boolean = false; // Keyboard navigation selection
  export let imageUrl: string = ''; // For image cards (converted Tauri asset URL)
  export let imagePath: string = ''; // Original file path for copying back to clipboard
  export let imageSize: string = ''; // For image cards (e.g., "1920 × 1080 • PNG")
  export let fileSize: string = ''; // For image cards (e.g., "2.4 MB")
  export let showThumbnails: boolean = true; // Whether to show image thumbnails or placeholders

  // Check if this is an image card (only show as image if thumbnails are enabled)
  $: isImageCard = category === 'image' && imageUrl && showThumbnails;

  // Check if this is a color card
  $: isColorCard = category === 'color' && content;

  // Check if content is short (for center alignment)
  $: isShortContent = content.length <= 50;

  let showCategoryDropdown = false;
  let showTagDropdown = false;
  let showEditPanel = false;
  let showDeleteConfirm = false;
  let editType: 'category' | 'tag' = 'category';
  let editName = '';
  let editIcon = '';
  let editColor = '';
  let editingCategoryId = ''; // Track which category is being edited
  let editingTagName = ''; // Track which tag is being edited
  let editingIsCustomCategory = false; // Track if editing a custom category
  let cardElement: HTMLDivElement;
  let isCopying = false; // Animation state for copy action

  async function handlePin() {
    const newPinned = !isPinned;
    try {
      await togglePin(itemId, newPinned);
      isPinned = newPinned;
      showSuccess(newPinned ? 'Pinned!' : 'Unpinned');
    } catch (err) {
      console.error('Failed to toggle pin:', err);
      showError('Failed to toggle pin');
    }
  }

  function handleDelete() {
    showDeleteConfirm = true;
  }

  async function confirmDelete() {
    showDeleteConfirm = false;
    try {
      await deleteItem(itemId);
      showSuccess('Item deleted');
    } catch (err) {
      console.error('Failed to delete item:', err);
      showError('Failed to delete item');
    }
  }

  function cancelDelete() {
    showDeleteConfirm = false;
  }

  async function handleCardClick() {
    // Check if user was dragging - if so, don't copy
    if (shouldBlockClick()) {
      return;
    }

    // Close all open panels/dropdowns first
    showCategoryDropdown = false;
    showTagDropdown = false;
    showEditPanel = false;

    try {
      // Trigger animation
      isCopying = true;

      // For image cards, copy the actual image to clipboard
      if (category === 'image' && imagePath) {
        await invoke('copy_image_to_clipboard', { imagePath });
        showSuccess('Image copied to clipboard!');
      } else {
        // For text/other cards, copy the text content
        await writeText(content);
        showSuccess('Copied to clipboard!');
      }

      // Auto-close panel after copy (with small delay for animation)
      setTimeout(() => {
        isCopying = false;
        invoke('hide_window').catch(err => {
          console.error('Failed to hide window:', err);
        });
      }, 200);

    } catch (err) {
      console.error('Failed to copy to clipboard:', err);
      showError('Failed to copy to clipboard');
      isCopying = false;
    }
  }

  // App icon is not editable - it represents the source platform
  // Removed handleAppIconClick function

  function handleCategoryClick(e: Event) {
    e.stopPropagation();
    // Broadcast to close all other dropdowns
    window.dispatchEvent(new CustomEvent(CLOSE_DROPDOWNS_EVENT, { detail: { source: cardElement } }));
    showCategoryDropdown = !showCategoryDropdown;
    if (showCategoryDropdown) {
      showTagDropdown = false;
    }
  }

  async function handleSelectCategory(categoryId: string) {
    // Close dropdown first
    showCategoryDropdown = false;

    // Use the store function which updates both database and store
    try {
      await updateItemCategory(itemId, categoryId);
      showSuccess('Category updated');
    } catch (err: any) {
      console.error('Failed to save category:', err);

      // Check if error is due to missing category
      if (err?.message?.includes('does not exist in the database')) {
        // Try to find the category in customCategories store
        const customCat = $customCategories.find(c => c.id === categoryId);

        if (customCat) {
          try {
            // Create the category in the database
            await createCustomCategory(customCat.name, customCat.icon);

            // Retry the update
            await updateItemCategory(itemId, categoryId);
            showSuccess('Category updated');
            return;
          } catch (createErr) {
            console.error('Failed to create category:', createErr);
          }
        }
      }

      showError('Failed to update category');
    }
  }

  function handleEditCategory(categoryId: string, isCustom: boolean) {
    showCategoryDropdown = false;

    // Track which category is being edited
    editingCategoryId = categoryId;
    editingIsCustomCategory = isCustom;
    editType = 'category';

    if (isCustom) {
      // For custom categories, get data from customCategories store
      const customCat = $customCategories.find(c => c.id === categoryId);
      editName = customCat?.name || categoryId;
      editIcon = customCat?.icon || '📁';
    } else {
      // For default categories
      editName = categoryMap[categoryId] || categoryId;

      // Get default icon from category ID
      const categoryIcons: Record<string, string> = {
        all: '📌',
        password: '🔐',
        apikey: '🔑',
        private: '🔒',
        email: '📧',
        phone: '📱',
        links: '🔗',
        code: '💻',
        color: '🎨',
        image: '🖼️',
        number: '🔢',
        text: '📝',
      };
      editIcon = categoryIcons[categoryId] || '📌';
    }

    editColor = customBg || 'rgba(50, 50, 50, 0.6)';
    showEditPanel = true;
  }

  async function handleDeleteCategory(categoryId: string) {
    showCategoryDropdown = false;

    try {
      await deleteCustomCategory(categoryId);

      // If the deleted category was selected, switch to 'text'
      if (category === categoryId) {
        // Update category in database
        await updateItemCategory(itemId, 'text');
      }

      showSuccess(`Category "${categoryId}" deleted`);
    } catch (error) {
      console.error('Failed to delete category:', error);
      showError('Failed to delete category');
    }
  }

  function handleAddTagClick(e: Event) {
    e.stopPropagation();
    // Broadcast to close all other dropdowns
    window.dispatchEvent(new CustomEvent(CLOSE_DROPDOWNS_EVENT, { detail: { source: cardElement } }));
    showTagDropdown = !showTagDropdown;
    if (showTagDropdown) {
      showCategoryDropdown = false;
    }
  }

  async function handleToggleTag(tagName: string) {
    const wasSelected = tags.includes(tagName);

    if (wasSelected) {
      // Remove tag locally first (optimistic update)
      tags = tags.filter(t => t !== tagName);

      // Save to database
      try {
        await removeTagFromItemByName(itemId, tagName);
        showSuccess(`Tag "${tagName}" removed`);
        // No need to reload all items - optimistic update is sufficient
      } catch (err) {
        console.error('Failed to remove tag:', err);
        // Rollback local change
        tags = [...tags, tagName];
        showError('Failed to remove tag');
      }
    } else {
      // Add tag locally first (optimistic update)
      tags = [...tags, tagName];

      // Save to database
      try {
        await addTagToItemByName(itemId, tagName);
        showSuccess(`Tag "${tagName}" added`);
        // No need to reload all items - optimistic update is sufficient
      } catch (err) {
        console.error('Failed to add tag:', err);
        // Rollback local change
        tags = tags.filter(t => t !== tagName);
        showError('Failed to add tag');
      }
    }

    showTagDropdown = false; // Close dropdown after operation
  }

  async function handleRemoveTag(tagName: string, e: Event) {
    e.stopPropagation();

    // Remove tag locally first (optimistic update)
    const previousTags = [...tags];
    tags = tags.filter(t => t !== tagName);

    // Save to database
    try {
      await removeTagFromItemByName(itemId, tagName);
      // Reload items from database to get updated tag_names
      await loadClipboardItems();
    } catch (err) {
      console.error('Failed to remove tag:', err);
      // Rollback local change
      tags = previousTags;
      showError('Failed to remove tag');
    }
  }

  function handleEditTag(tagName: string) {
    showTagDropdown = false;

    // Find tag in allTags store
    const tag = $allTags.find(t => t.name === tagName);
    if (!tag) {
      console.error('Tag not found:', tagName);
      return;
    }

    // Track which tag is being edited
    editingTagName = tagName;
    editType = 'tag';
    editName = tag.name;
    editIcon = tag.icon;
    editColor = tag.color;

    showEditPanel = true;
  }

  function handleDeleteTag(tagName: string) {
    showTagDropdown = false;

    // Find the tag to get its ID
    const tag = $allTags.find(t => t.name === tagName);
    if (!tag) {
      return;
    }

    // Check if it's a default tag
    if (tag.isDefault) {
      return;
    }

    // Delete the tag from the store immediately
    deleteTag(tag.id);

    // Remove tag from this card if it has it
    if (tags.includes(tagName)) {
      tags = tags.filter(t => t !== tagName);
    }

    showSuccess(`Tag "${tagName}" deleted`);
  }

  function handleCreateTag() {
    showTagDropdown = false;

    // Open edit panel for creating a new tag
    editType = 'tag';
    editName = '';
    editIcon = '🏷️'; // Default tag icon
    editColor = 'rgba(50, 50, 50, 0.6)'; // Default color
    editingTagName = ''; // No existing tag being edited

    showEditPanel = true;
  }

  // Get tag color from allTags store
  function getTagColor(tagName: string): string {
    // Find tag in allTags store
    const tag = $allTags.find(t => t.name === tagName);
    if (tag) return tag.color;

    // Fallback to default color
    return 'rgba(50, 50, 50, 0.6)';
  }

  async function handleSaveEdit(name: string, icon: string, color: string) {
    try {
    if (editType === 'category') {
      // Check if editing app icon (display only - source app icons are auto-detected)
      if (editingCategoryId === 'app-icon') {
        appIcon = icon;
        // Note: App icons are auto-detected from the source application
        // and stored when the clipboard item is created. This only updates display.
      }
      // Check if we're creating a new category (editingCategoryId is empty)
      else if (!editingCategoryId || editingCategoryId === '') {
        // Create the new category (now async)
        const newCategory = await createCustomCategory(name, icon);

        // Select the newly created category for this card
        category = newCategory.id;

        // Save to database
        await updateItemCategory(itemId, category);

        // Reload items to ensure UI stays in sync
        await loadClipboardItems();
      } else {
        // Editing existing category
        const isCustomCategory = $customCategories.some(c => c.id === editingCategoryId);

        if (isCustomCategory) {
          // For custom categories, update the category itself (name and icon)
          await updateCustomCategory(editingCategoryId, { name, icon });

          // categoryLabel will update automatically via reactive statement
          // because customCategories store will be updated
        } else {
          // For default categories, update the icon globally (saves to database)
          await updateCategoryIcon(editingCategoryId, icon);

          // Update category color if changed (saves to database)
          if (color && color !== 'rgba(50, 50, 50, 0.6)') {
            await updateCategoryColor(editingCategoryId, color);
            customBg = color; // Also update this card's background
          }
        }
      }

    } else if (editType === 'tag') {
      // Find the tag being edited in allTags store
      const tag = $allTags.find(t => t.name === editingTagName);
      if (!tag) {
        console.error('Tag not found:', editingTagName);
        showEditPanel = false;
        return;
      }

      // Update tag globally (saves to database and broadcasts event)
      await updateTag(tag.id, { name, icon, color });

      // Local update will happen via event bus automatically
      // But also update immediately for instant feedback on this card
      const tagIndex = tags.findIndex(t => t === editingTagName);
      if (tagIndex !== -1 && name !== editingTagName) {
        tags = [...tags.slice(0, tagIndex), name, ...tags.slice(tagIndex + 1)];
      }
    }

    // Show success message
    showEditPanel = false;
    showSuccess(editType === 'tag' ? 'Tag updated' : 'Category updated');

    } catch (error) {
      console.error('Failed to save edit:', error);
      showError('Failed to save changes');
    }
  }

  function handleCloseEditPanel() {
    showEditPanel = false;
  }

  // Listen for global close event
  function handleCloseDropdowns(e: Event) {
    const customEvent = e as CustomEvent;
    // Only close if event came from a different card
    if (customEvent.detail?.source !== cardElement) {
      showCategoryDropdown = false;
      showTagDropdown = false;
    }
  }

  onMount(() => {
    window.addEventListener(CLOSE_DROPDOWNS_EVENT, handleCloseDropdowns);

    // Fetch system icon if bundle ID is available
    if (appBundleId) {
      iconLoading = true;
      invoke<string>('get_app_icon_data', {
        bundleId: appBundleId,
        appName: appName
      }).then(iconData => {
        // Check if it's a data URL (system icon) or emoji
        if (iconData.startsWith('data:image/')) {
          systemIconUrl = iconData;
        }
        iconLoading = false;
      }).catch(() => {
        iconLoading = false;
      });
    }

    // Subscribe to event bus for real-time tag/category renames
    const unsubscribe = eventBus.subscribe(event => {
      if (!event) return;

      // Handle tag renamed events
      if (event.type === 'TAG_RENAMED') {
        const tagIndex = tags.findIndex(t => t === event.oldName);
        if (tagIndex !== -1) {
          // Replace old tag name with new tag name
          tags = [
            ...tags.slice(0, tagIndex),
            event.newName,
            ...tags.slice(tagIndex + 1)
          ];
        }
      }

      // Handle category renamed events
      if (event.type === 'CATEGORY_RENAMED') {
        // Check if this card uses the renamed category (by old ID or old name)
        if (category === event.categoryId || category === event.oldName) {
          // Update the internal category variable to the new name
          // This is important because category ID = category name for custom categories
          category = event.newName;
        }
      }
    });

    // Cleanup function
    return () => {
      unsubscribe();
    };
  });

  onDestroy(() => {
    window.removeEventListener(CLOSE_DROPDOWNS_EVENT, handleCloseDropdowns);
  });
</script>

<div
  bind:this={cardElement}
  class="clipboard-card"
  class:has-custom-bg={customBg || isColorCard}
  class:light-bg={isLightBg}
  class:copying={isCopying}
  class:selected={isSelected}
  class:image-card={isImageCard}
  class:color-card={isColorCard}
  style={isImageCard ? `--card-bg-image: url(${imageUrl}); background-image: url(${imageUrl}); background-size: cover; background-position: center;` : (isColorCard ? `--card-color: ${content}; background: ${content};` : (customBg ? `background: ${customBg};` : ''))}
  on:click={handleCardClick}
  on:keypress={(e) => e.key === 'Enter' && handleCardClick()}
  role="button"
  tabindex="0"
>
  <!-- Blurred background removed - using card background-image instead -->
  <div class="glass-filter"></div>
  <div class="glass-overlay"></div>
  <div class="glass-specular"></div>
  <div class="glass-content">
    <!-- Card Header -->
    <div class="card-header">
      <div class="header-line-1">
        <div class="app-info">
          <div class="app-icon">
            {#if systemIconUrl}
              <img src={systemIconUrl} alt={appName} class="system-icon" />
            {:else}
              {appIcon}
            {/if}
          </div>
          <div class="app-label" style="position: relative;">
            <span class="app-name">{appName}</span>
            <span class="separator">|</span>
            <span
              class="category-label"
              on:click={handleCategoryClick}
              on:keypress={(e) => e.key === 'Enter' && handleCategoryClick(e)}
              role="button"
              tabindex="0"
              style="cursor: pointer;"
            >
              {categoryLabel}
            </span>

            <!-- Category Dropdown -->
            {#if showCategoryDropdown}
              <div use:clickOutside={() => showCategoryDropdown = false}>
                <CategoryDropdown
                  show={showCategoryDropdown}
                  currentCategory={category}
                  onSelectCategory={handleSelectCategory}
                  onEditCategory={handleEditCategory}
                  onDeleteCategory={handleDeleteCategory}
                />
              </div>
            {/if}
          </div>
        </div>
      </div>
      <div class="header-line-2">
        <span class="timestamp">{timestamp}</span>
        <div class="card-actions">
          <button class="pin-btn" class:pinned={isPinned} on:click|stopPropagation={handlePin}>
            📌
          </button>
          <button class="delete-btn" on:click|stopPropagation={handleDelete}>
            🗑️
          </button>
        </div>
      </div>
    </div>

    <!-- Card Content -->
    <div class="card-content">
      <!-- Content Preview -->
      {#if isImageCard}
        <!-- Image Card Content with Thumbnail -->
        <div class="image-preview-container">
          <img src={imageUrl} alt={content} class="image-preview" />
        </div>
        <div class="image-metadata">
          {content}
          {#if imageSize || fileSize}
            <div class="image-size">
              {#if imageSize && fileSize}
                {imageSize} • {fileSize}
              {:else if imageSize}
                {imageSize}
              {:else if fileSize}
                {fileSize}
              {/if}
            </div>
          {/if}
        </div>
      {:else if category === 'image' && !isImageCard}
        <!-- Image Card Content without Thumbnail (placeholder for old entries or thumbnails disabled) -->
        <div class="image-placeholder">
          <span class="placeholder-icon">🖼️</span>
          <span class="placeholder-text">Image</span>
          {#if imageSize || fileSize}
            <span class="placeholder-size">
              {#if imageSize && fileSize}
                {imageSize} • {fileSize}
              {:else if imageSize}
                {imageSize}
              {:else if fileSize}
                {fileSize}
              {/if}
            </span>
          {/if}
        </div>
      {:else if isColorCard}
        <!-- Color Card Content - just show hex code centered -->
        <div class="color-card-content">
          <div class="color-code">{content}</div>
        </div>
      {:else}
        <!-- Regular Content -->
        <div class="content-preview" class:short={isShortContent}>{content}</div>
      {/if}
    </div>

    <!-- Card Footer -->
    <div class="card-footer">
      <div class="footer-tags">
        {#each tags as tag}
          {#key $allTags}
            <span
              class="tag"
              style="
                background: {getTagColor(tag)};
                border-color: {getTagColor(tag).replace('0.6)', '0.5)')};
                box-shadow: 0 0 8px {getTagColor(tag).replace('0.6)', '0.3)')};
              "
              on:click={(e) => handleRemoveTag(tag, e)}
              on:keypress={(e) => e.key === 'Enter' && handleRemoveTag(tag, e)}
              role="button"
              tabindex="0"
              title="Click to remove"
            >
              {tag}
            </span>
          {/key}
        {/each}
      </div>
      <div class="footer-right">
        <span class="char-count">
          {#if category === 'image' && (imageSize || fileSize)}
            {#if imageSize && fileSize}
              {imageSize} • {fileSize}
            {:else if imageSize}
              {imageSize}
            {:else if fileSize}
              {fileSize}
            {/if}
          {:else}
            {charCount} chars
          {/if}
        </span>
        <div class="add-tag-btn">
        <button on:click={handleAddTagClick} aria-label="Add tag">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </button>

        <!-- Tag Dropdown -->
        {#if showTagDropdown}
          <div use:clickOutside={() => showTagDropdown = false}>
            <TagDropdown
              show={showTagDropdown}
              selectedTags={tags}
              onToggleTag={handleToggleTag}
              onEditTag={handleEditTag}
              onDeleteTag={handleDeleteTag}
              onCreateTag={handleCreateTag}
            />
          </div>
        {/if}
        </div>
      </div>
    </div>

    <!-- Edit Panel -->
    {#if showEditPanel}
      <div class="card-edit-wrapper" use:clickOutside={handleCloseEditPanel}>
        <CategoryEditPanel
          show={showEditPanel}
          categoryName={editName}
          categoryIcon={editIcon}
          categoryColor={editColor}
          showColorTab={editType === 'tag'}
          isNameEditable={editType === 'tag' || (editType === 'category' && editingIsCustomCategory)}
          panelTitle={editType === 'tag' ? 'Tag name' : 'Category name'}
          onSave={handleSaveEdit}
          onClose={handleCloseEditPanel}
        />
      </div>
    {/if}
  </div>
</div>

<!-- Delete Confirmation Modal -->
<ConfirmModal
  show={showDeleteConfirm}
  title="Delete Item"
  message="Are you sure you want to delete this clipboard item? This action cannot be undone."
  confirmText="Delete"
  cancelText="Cancel"
  type="danger"
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>

<style>
  /* Reference: preview.html lines 792-1062 */
  .clipboard-card {
    flex-shrink: 0;
    width: var(--card-width, 288px);
    height: 95%;
    border-radius: 12px;
    position: relative;
    overflow: visible;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .clipboard-card:hover {
    transform: translateY(-2px);
  }

  /* Regular cards - hover glow */
  .clipboard-card:not(.has-custom-bg):hover .glass-overlay {
    border: 1.5px solid rgba(247, 228, 121, 0.8);
    box-shadow: 0 0 8px rgba(247, 228, 121, 0.25),
                inset 0 0 8px rgba(247, 228, 121, 0.05);
  }

  /* Color/Image cards - NO glow on hover */
  .clipboard-card.has-custom-bg:hover .glass-overlay {
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  /* Color/Image cards - NO glow on selected */
  .clipboard-card.has-custom-bg.selected .glass-overlay {
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  /* Color/Image cards - NO glow on copying */
  .clipboard-card.has-custom-bg.copying .glass-overlay {
    border: 1px solid rgba(255, 255, 255, 0.3);
  }

  /* Keyboard Selection - Regular cards only */
  .clipboard-card:not(.has-custom-bg).selected .glass-overlay {
    border: 2px solid rgba(59, 130, 246, 0.9);
    box-shadow: 0 0 24px rgba(59, 130, 246, 0.5),
                inset 0 0 20px rgba(59, 130, 246, 0.2);
  }

  /* Copy Animation */
  .clipboard-card.copying {
    animation: copyPulse 0.6s ease-out;
  }

  /* Copy Animation - Regular cards only */
  .clipboard-card:not(.has-custom-bg).copying .glass-overlay {
    border: 1.5px solid rgba(34, 197, 94, 0.9);
    box-shadow: 0 0 20px rgba(34, 197, 94, 0.4),
                inset 0 0 15px rgba(34, 197, 94, 0.2);
  }

  @keyframes copyPulse {
    0% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.02);
    }
    100% {
      transform: scale(1);
    }
  }

  .clipboard-card .glass-filter,
  .clipboard-card .glass-overlay,
  .clipboard-card .glass-specular {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    pointer-events: none;
    overflow: hidden;
  }

  /* Glass filter - Regular cards */
  .clipboard-card:not(.has-custom-bg) .glass-filter {
    z-index: 0;
    backdrop-filter: blur(8px);
    filter: url(#lensFilter) saturate(110%) brightness(1.05);
  }

  /* NO backdrop blur for color/image cards */
  .clipboard-card.has-custom-bg .glass-filter {
    z-index: 0;
  }

  /* Glass overlay - Regular cards */
  .clipboard-card:not(.has-custom-bg) .glass-overlay {
    z-index: 1;
    background: linear-gradient(135deg, rgba(40, 40, 40, 0.85) 0%, rgba(25, 25, 25, 0.9) 100%);
    border: 1px solid rgba(80, 80, 80, 0.3);
  }

  /* Minimal border for color/image cards */
  .clipboard-card.has-custom-bg .glass-overlay {
    z-index: 1;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  /* No overlay - using individual backgrounds for elements */

  /* Light background color cards - use dark text for readability */
  .clipboard-card.light-bg .glass-content {
    color: rgba(0, 0, 0, 0.85);
    text-shadow: none;
  }

  .clipboard-card.light-bg .app-name,
  .clipboard-card.light-bg .content-preview,
  .clipboard-card.light-bg .timestamp,
  .clipboard-card.light-bg .char-count {
    color: rgba(0, 0, 0, 0.75);
    text-shadow: none;
  }

  .clipboard-card.light-bg .category-label {
    color: rgba(0, 0, 0, 0.85);
    text-shadow: none;
  }

  .clipboard-card.light-bg .separator {
    color: rgba(0, 0, 0, 0.5);
  }

  .clipboard-card.light-bg .app-icon {
    filter: drop-shadow(0 1px 2px rgba(255, 255, 255, 0.5));
  }

  .clipboard-card.light-bg .pin-btn,
  .clipboard-card.light-bg .delete-btn {
    color: rgba(0, 0, 0, 0.5);
    filter: grayscale(1) drop-shadow(0 1px 2px rgba(255, 255, 255, 0.5));
  }

  .clipboard-card.light-bg .pin-btn:hover,
  .clipboard-card.light-bg .delete-btn:hover {
    color: rgba(0, 0, 0, 0.8);
  }

  .clipboard-card.light-bg .pin-btn.pinned {
    color: #b8860b;
    filter: grayscale(0) drop-shadow(0 1px 2px rgba(255, 255, 255, 0.5));
  }

  .clipboard-card.light-bg .card-header {
    border-bottom-color: rgba(0, 0, 0, 0.15);
  }

  .clipboard-card.light-bg .tag {
    background: rgba(0, 0, 0, 0.15);
    border-color: rgba(0, 0, 0, 0.25);
    color: rgba(0, 0, 0, 0.85);
  }

  .clipboard-card.light-bg .add-tag-btn button {
    background: rgba(0, 0, 0, 0.1);
    border-color: rgba(0, 0, 0, 0.3);
    color: rgba(0, 0, 0, 0.6);
  }

  .clipboard-card.light-bg .add-tag-btn button:hover {
    background: rgba(0, 0, 0, 0.2);
    border-color: rgba(0, 0, 0, 0.5);
    color: rgba(0, 0, 0, 0.9);
  }

  .clipboard-card .glass-content {
    text-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
  }

  /* Inner specular glow - Regular cards only */
  .clipboard-card:not(.has-custom-bg) .glass-specular {
    z-index: 2;
    box-shadow: inset 1px 1px 0 rgba(255, 255, 255, 0.1),
                inset 0 0 20px rgba(255, 255, 255, 0.05);
  }

  /* NO inner glow for color/image cards */
  .clipboard-card.has-custom-bg .glass-specular {
    z-index: 2;
  }

  .clipboard-card .glass-content {
    position: relative;
    z-index: 3;
    padding: 20px;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: visible;
  }

  .card-header {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 20px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(102, 126, 234, 0.2);
    overflow: visible;
  }

  /* Fully opaque dark gradient header for color/image cards - no color/image showing through */
  .clipboard-card.has-custom-bg .card-header {
    background: linear-gradient(135deg, rgb(40, 40, 40) 0%, rgb(25, 25, 25) 100%);
    margin: -20px -20px 20px -20px;
    padding: 16px 20px 12px 20px;
    border-radius: 12px 12px 0 0;
    border-bottom: 1px solid rgba(102, 126, 234, 0.2);
  }

  /* Override light-bg text colors for header since header is dark */
  .clipboard-card.has-custom-bg .card-header .category-label {
    color: rgba(247, 228, 121, 0.95);
  }

  .clipboard-card.has-custom-bg .card-header .app-name,
  .clipboard-card.has-custom-bg .card-header .app-label {
    color: rgba(255, 255, 255, 0.85);
  }

  .clipboard-card.has-custom-bg .card-header .separator {
    color: rgba(255, 255, 255, 0.3);
  }

  .clipboard-card.has-custom-bg .card-header .timestamp {
    color: rgba(255, 255, 255, 0.45);
  }

  .clipboard-card.has-custom-bg .card-header .pin-btn,
  .clipboard-card.light-bg.has-custom-bg .card-header .pin-btn {
    color: rgba(255, 255, 255, 0.25) !important;
    filter: grayscale(1) !important;
  }

  .clipboard-card.has-custom-bg .card-header .pin-btn.pinned,
  .clipboard-card.light-bg.has-custom-bg .card-header .pin-btn.pinned {
    color: #f7e479 !important;
    filter: grayscale(0) !important;
  }

  .clipboard-card.has-custom-bg .card-header .delete-btn,
  .clipboard-card.light-bg.has-custom-bg .card-header .delete-btn {
    color: rgba(255, 255, 255, 0.25) !important;
    filter: grayscale(1) !important;
  }


  .header-line-1 {
    display: flex;
    justify-content: space-between;
    align-items: center;
    overflow: visible;
  }

  .header-line-2 {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.45);
  }

  .char-count {
    color: rgba(255, 255, 255, 0.4);
    font-weight: 500;
  }

  .app-info {
    display: flex;
    align-items: center;
    gap: 10px;
    flex: 1;
    min-width: 0;
    overflow: visible;
  }

  .app-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    flex-shrink: 0;
  }

  .app-icon .system-icon {
    width: 24px;
    height: 24px;
    object-fit: contain;
    border-radius: 4px;
  }

  /* Stronger shadow for app icon on color/image cards */
  .clipboard-card.has-custom-bg .app-icon {
    filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.8));
  }

  .app-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .app-name {
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 120px;
  }

  /* Color/image cards now have dark header, so use normal text styles */
  .clipboard-card.has-custom-bg .app-label {
    color: rgba(255, 255, 255, 0.7);
  }

  .clipboard-card.has-custom-bg .app-name {
    color: rgba(255, 255, 255, 0.85);
  }

  .app-name {
    color: rgba(255, 255, 255, 0.85);
  }

  .category-label {
    color: rgba(247, 228, 121, 0.95);
    font-weight: 600;
    font-size: 15px;
  }

  .separator {
    color: rgba(255, 255, 255, 0.3);
  }

  .card-actions {
    display: flex;
    gap: 4px;
    align-items: center;
    flex-shrink: 0;
  }

  .pin-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.25);
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
    padding: 4px;
    filter: grayscale(1);
  }

  .pin-btn:hover {
    color: rgba(255, 255, 255, 0.5);
    transform: scale(1.15);
    filter: grayscale(0.5);
  }

  .pin-btn.pinned {
    color: #f7e479;
    filter: grayscale(0);
    text-shadow: 0 0 4px rgba(247, 228, 121, 0.3);
  }

  .pin-btn.pinned:hover {
    color: #f7e479;
    transform: scale(1.2) rotate(15deg);
    text-shadow: 0 0 6px rgba(247, 228, 121, 0.4);
  }

  .delete-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.25);
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
    padding: 4px;
    filter: grayscale(1);
  }

  .delete-btn:hover {
    color: #f7e479;
    transform: scale(1.15);
    filter: grayscale(0);
    text-shadow: 0 0 8px rgba(247, 228, 121, 0.6);
  }

  .card-content {
    flex: 1;
    overflow: hidden;
    margin-bottom: 16px;
  }

  .content-preview {
    color: rgba(255, 255, 255, 0.95);
    font-size: var(--card-font-size, 14px);
    line-height: 1.6;
    word-wrap: break-word;
    overflow-wrap: break-word;
    display: -webkit-box;
    -webkit-line-clamp: 10;
    line-clamp: 10;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  /* Center short content (less than 50 chars) - for single words, color codes, numbers */
  .content-preview.short {
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    height: 100%;
    -webkit-line-clamp: unset;
    line-clamp: unset;
    -webkit-box-orient: unset;
  }

  /* Image Card Styles */
  .image-preview-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 20px 0;
    position: relative;
  }

  .image-preview {
    max-width: 100%;
    max-height: 180px;
    object-fit: contain;
    border-radius: 8px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  }

  .image-metadata {
    font-size: 13px;
    font-weight: 600;
    text-align: center;
    color: rgba(255, 255, 255, 0.9);
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.6);
    margin-top: 8px;
  }

  .image-size {
    font-size: 11px;
    opacity: 0.8;
    margin-top: 4px;
    font-weight: 500;
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    margin-top: auto;
  }

  .footer-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    flex: 1;
  }

  .footer-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .footer-right .char-count {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.4);
    font-weight: 500;
  }

  .tag {
    padding: 4px 10px;
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 12px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.95);
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    transition: all 0.2s;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }

  .tag:hover {
    background: rgba(247, 228, 121, 0.25);
    backdrop-filter: blur(15px);
    -webkit-backdrop-filter: blur(15px);
    border-color: rgba(247, 228, 121, 0.5);
    transform: scale(1.05);
  }

  .add-tag-btn {
    position: relative;
    flex-shrink: 0;
  }

  .add-tag-btn button {
    width: 24px;
    height: 24px;
    background: rgba(0, 0, 0, 0.3);
    border: 1.5px solid rgba(255, 255, 255, 0.4);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    color: rgba(255, 255, 255, 0.7);
  }

  .add-tag-btn button:hover {
    background: rgba(247, 228, 121, 0.15);
    border-color: rgba(247, 228, 121, 0.7);
    color: rgba(247, 228, 121, 1);
    transform: scale(1.1);
  }

  .add-tag-btn button svg {
    width: 12px;
    height: 12px;
  }

  /* Card Edit Panel Wrapper */
  .card-edit-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 10000;
    pointer-events: none;
  }

  .card-edit-wrapper :global(.edit-panel) {
    pointer-events: auto;
  }

  /* Keyboard Selection State */
  .clipboard-card.selected {
    outline: 3px solid #f7e479;
    outline-offset: 3px;
    box-shadow:
      0 0 0 6px rgba(247, 228, 121, 0.15),
      0 8px 32px rgba(247, 228, 121, 0.3),
      inset 0 0 0 1px rgba(247, 228, 121, 0.4);
  }

  .clipboard-card.selected::before {
    content: '';
    position: absolute;
    top: -3px;
    left: -3px;
    right: -3px;
    bottom: -3px;
    border-radius: 14px;
    background: linear-gradient(135deg, rgba(247, 228, 121, 0.1), rgba(247, 228, 121, 0.05));
    pointer-events: none;
    z-index: -1;
  }

  /* Image Card - Blurred Background Effect */
  .image-card {
    overflow: hidden;
    background-color: rgba(30, 30, 30, 0.95); /* Fallback if image fails to load */
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
  }

  /* Blurred pseudo-element like the demo */
  .image-card::before {
    content: '';
    position: absolute;
    inset: 0;
    background: inherit;
    filter: blur(20px);
    opacity: 0.5;
    z-index: 0;
  }

  .image-card .glass-overlay {
    background: linear-gradient(180deg, rgba(0, 0, 0, 0.4) 0%, rgba(0, 0, 0, 0.6) 100%);
    border: 2px solid rgba(255, 255, 255, 0.2);
  }

  .image-card .glass-content {
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.9), 0 0 4px rgba(0, 0, 0, 0.8);
  }

  .image-card .card-header {
    border-bottom-color: rgba(255, 255, 255, 0.2);
  }

  .image-card .header-line-2 {
    color: rgba(255, 255, 255, 0.8);
  }

  .image-card .timestamp,
  .image-card .char-count {
    color: rgba(255, 255, 255, 0.8);
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.8);
  }

  .image-card .app-name,
  .image-card .category-label {
    color: rgba(255, 255, 255, 0.95);
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.9);
  }

  /* Color Card Styles */
  .color-card .glass-overlay {
    background: transparent;
    border: 2px solid rgba(255, 255, 255, 0.3);
  }

  .color-card .glass-specular {
    box-shadow: inset 2px 2px 0 rgba(255, 255, 255, 0.2),
                inset 0 0 20px rgba(255, 255, 255, 0.08);
  }

  .color-card-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  .color-code {
    font-size: 14px;
    font-weight: 700;
    font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
    text-shadow: 0 2px 10px rgba(0, 0, 0, 0.6);
    color: rgba(255, 255, 255, 0.95);
    text-align: center;
    text-transform: uppercase;
  }

  /* Light background color cards */
  .color-card.light-bg .color-code {
    color: rgba(0, 0, 0, 0.85);
    text-shadow: 0 2px 10px rgba(255, 255, 255, 0.6);
  }

  .color-card.light-bg .glass-overlay {
    border-color: rgba(0, 0, 0, 0.15);
  }

  .color-card.light-bg .glass-specular {
    box-shadow: inset 2px 2px 0 rgba(255, 255, 255, 0.4),
                inset 0 0 20px rgba(255, 255, 255, 0.15);
  }

  /* Image Placeholder (when thumbnails are disabled) */
  .image-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 8px;
    color: rgba(255, 255, 255, 0.7);
  }

  .placeholder-icon {
    font-size: 48px;
    opacity: 0.6;
  }

  .placeholder-text {
    font-size: 14px;
    font-weight: 500;
  }

  .placeholder-size {
    font-size: 11px;
    opacity: 0.5;
  }
</style>
