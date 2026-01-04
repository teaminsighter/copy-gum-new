<script lang="ts">
  import { advancedFilters } from '$lib/stores/clipboardStore';
  import { customCategories, categoryOrder } from '$lib/stores/categoryStore';
  import { allTags } from '$lib/stores/tagStore';
  import { get } from 'svelte/store';

  // Props
  export let isOpen = false;
  export let onAddCategory: () => void = () => {};
  export let onAddTag: () => void = () => {};

  // Local state for date inputs
  let startDate = '';
  let endDate = '';

  // Dropdown state
  let contentTypesExpanded = false;
  let categoriesExpanded = false;
  let tagsExpanded = false;

  // Available content types (must match database content_type values)
  const contentTypes = [
    { value: 'text', label: 'Text', icon: '📝' },
    { value: 'links', label: 'URL/Links', icon: '🔗' },
    { value: 'email', label: 'Email', icon: '📧' },
    { value: 'code', label: 'Code', icon: '💻' },
    { value: 'color', label: 'Color', icon: '🎨' },
    { value: 'phone', label: 'Phone', icon: '📱' },
    { value: 'number', label: 'Number', icon: '🔢' }
  ];

  // Default categories
  const defaultCategories = [
    { id: 'all', name: 'All', icon: '📌' },
    { id: 'password', name: 'Password', icon: '🔐' },
    { id: 'apikey', name: 'API Key', icon: '🔑' },
    { id: 'private', name: 'Private', icon: '🔒' },
    { id: 'email', name: 'Email', icon: '📧' },
    { id: 'phone', name: 'Phone', icon: '📱' },
    { id: 'links', name: 'Links', icon: '🔗' },
    { id: 'code', name: 'Code', icon: '💻' },
    { id: 'color', name: 'Color', icon: '🎨' },
    { id: 'image', name: 'Image', icon: '🖼️' },
    { id: 'number', name: 'Number', icon: '🔢' },
  ];

  // Reactive: Get all categories (default + custom)
  $: allCategories = [
    ...defaultCategories,
    ...$customCategories.map(cat => ({
      id: cat.id,
      name: cat.name,
      icon: cat.icon
    }))
  ];

  // Reactive: Get current filters
  $: currentFilters = $advancedFilters;
  $: activeFilterCount = getActiveFilterCount($advancedFilters);

  /**
   * Count active filters
   */
  function getActiveFilterCount(filters: typeof $advancedFilters): number {
    let count = 0;
    if (filters.pinStatus && filters.pinStatus !== 'all') count++;
    if (filters.contentTypes && filters.contentTypes.length > 0) count++;
    if (filters.categories && filters.categories.length > 0) count++;
    if (filters.tags && filters.tags.length > 0) count++;
    if (filters.dateRange) count++;
    return count;
  }

  /**
   * Update pin status filter
   */
  function handlePinStatusChange(status: 'all' | 'pinned' | 'unpinned') {
    advancedFilters.update(f => ({
      ...f,
      pinStatus: status
    }));
  }

  /**
   * Toggle content type filter
   */
  function toggleContentType(type: string) {
    advancedFilters.update(f => {
      const types = f.contentTypes || [];
      const newTypes = types.includes(type)
        ? types.filter(t => t !== type)
        : [...types, type];

      return {
        ...f,
        contentTypes: newTypes
      };
    });
  }

  /**
   * Toggle category filter
   */
  function toggleCategory(categoryId: string) {
    advancedFilters.update(f => {
      const categories = f.categories || [];
      const newCategories = categories.includes(categoryId)
        ? categories.filter(c => c !== categoryId)
        : [...categories, categoryId];

      return {
        ...f,
        categories: newCategories
      };
    });
  }

  /**
   * Toggle tag filter
   */
  function toggleTag(tagName: string) {
    advancedFilters.update(f => {
      const tags = f.tags || [];
      const newTags = tags.includes(tagName)
        ? tags.filter(t => t !== tagName)
        : [...tags, tagName];

      return {
        ...f,
        tags: newTags
      };
    });
  }

  /**
   * Update date range filter
   */
  function handleDateRangeChange() {
    if (startDate && endDate) {
      const start = new Date(startDate);
      const end = new Date(endDate);

      // Set end date to end of day
      end.setHours(23, 59, 59, 999);

      advancedFilters.update(f => ({
        ...f,
        dateRange: { start, end }
      }));
    } else {
      advancedFilters.update(f => {
        const { dateRange, ...rest } = f;
        return rest;
      });
    }
  }

  /**
   * Clear all filters
   */
  function clearAllFilters() {
    advancedFilters.set({
      pinStatus: 'all',
      contentTypes: [],
      categories: [],
      tags: []
    });
    startDate = '';
    endDate = '';
  }

  /**
   * Toggle dropdown sections (accordion - only one open at a time)
   */
  function toggleContentTypes() {
    contentTypesExpanded = !contentTypesExpanded;
    if (contentTypesExpanded) {
      categoriesExpanded = false;
      tagsExpanded = false;
    }
  }

  function toggleCategories() {
    categoriesExpanded = !categoriesExpanded;
    if (categoriesExpanded) {
      contentTypesExpanded = false;
      tagsExpanded = false;
    }
  }

  function toggleTags() {
    tagsExpanded = !tagsExpanded;
    if (tagsExpanded) {
      contentTypesExpanded = false;
      categoriesExpanded = false;
    }
  }

  /**
   * Close panel on Escape
   */
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      isOpen = false;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
  <div class="filters-sidebar">
    <!-- Header -->
    <div class="sidebar-header">
      <div class="sidebar-title">
        <span>Advanced Filters</span>
        {#if activeFilterCount > 0}
          <span class="filter-count">{activeFilterCount}</span>
        {/if}
      </div>
      <button class="close-btn" on:click={() => isOpen = false} aria-label="Close filters">
        ✕
      </button>
    </div>

    <!-- Scrollable Content -->
    <div class="sidebar-content">
      <!-- Pin Status -->
      <div class="filter-section">
        <div class="section-title">PIN STATUS</div>
        <div class="radio-group">
          <label class="radio-item" class:active={currentFilters.pinStatus === 'all'}>
            <input
              type="radio"
              name="pinStatus"
              value="all"
              checked={currentFilters.pinStatus === 'all'}
              on:change={() => handlePinStatusChange('all')}
            />
            <span>📋 All Items</span>
          </label>
          <label class="radio-item" class:active={currentFilters.pinStatus === 'pinned'}>
            <input
              type="radio"
              name="pinStatus"
              value="pinned"
              checked={currentFilters.pinStatus === 'pinned'}
              on:change={() => handlePinStatusChange('pinned')}
            />
            <span>📌 Pinned Only</span>
          </label>
          <label class="radio-item" class:active={currentFilters.pinStatus === 'unpinned'}>
            <input
              type="radio"
              name="pinStatus"
              value="unpinned"
              checked={currentFilters.pinStatus === 'unpinned'}
              on:change={() => handlePinStatusChange('unpinned')}
            />
            <span>📄 Unpinned Only</span>
          </label>
        </div>
      </div>

      <!-- Date Range -->
      <div class="filter-section">
        <div class="section-title">DATE RANGE</div>
        <div class="date-inputs">
          <div class="date-input-wrapper">
            <label for="start-date">From</label>
            <input
              id="start-date"
              type="date"
              bind:value={startDate}
              on:change={handleDateRangeChange}
              max={endDate || undefined}
            />
          </div>
          <div class="date-input-wrapper">
            <label for="end-date">To</label>
            <input
              id="end-date"
              type="date"
              bind:value={endDate}
              on:change={handleDateRangeChange}
              min={startDate || undefined}
            />
          </div>
        </div>
      </div>

      <!-- Content Types Dropdown -->
      <div class="filter-section">
        <div class="section-header" on:click={toggleContentTypes} on:keypress={(e) => e.key === 'Enter' && toggleContentTypes()} role="button" tabindex="0">
          <div class="section-title">
            CONTENT TYPES
            {#if currentFilters.contentTypes && currentFilters.contentTypes.length > 0}
              <span class="selected-count">({currentFilters.contentTypes.length})</span>
            {/if}
          </div>
          <span class="dropdown-arrow" class:expanded={contentTypesExpanded}>▼</span>
        </div>
        {#if contentTypesExpanded}
          <div class="dropdown-content">
            <div class="checkbox-list">
              {#each contentTypes as type}
                <label class="checkbox-item" class:active={currentFilters.contentTypes?.includes(type.value)}>
                  <input
                    type="checkbox"
                    checked={currentFilters.contentTypes?.includes(type.value)}
                    on:change={() => toggleContentType(type.value)}
                  />
                  <span>{type.icon} {type.label}</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Categories Dropdown -->
      <div class="filter-section">
        <div class="section-header" on:click={toggleCategories} on:keypress={(e) => e.key === 'Enter' && toggleCategories()} role="button" tabindex="0">
          <div class="section-title">
            CATEGORIES
            {#if currentFilters.categories && currentFilters.categories.length > 0}
              <span class="selected-count">({currentFilters.categories.length})</span>
            {/if}
          </div>
          <div class="header-actions">
            <button class="add-btn" on:click|stopPropagation={onAddCategory} title="Add Category">+</button>
            <span class="dropdown-arrow" class:expanded={categoriesExpanded}>▼</span>
          </div>
        </div>
        {#if categoriesExpanded}
          <div class="dropdown-content scrollable">
            <div class="checkbox-list">
              {#each allCategories as category}
                <label class="checkbox-item" class:active={currentFilters.categories?.includes(category.id)}>
                  <input
                    type="checkbox"
                    checked={currentFilters.categories?.includes(category.id)}
                    on:change={() => toggleCategory(category.id)}
                  />
                  <span>{category.icon} {category.name}</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Tags Dropdown -->
      <div class="filter-section">
        <div class="section-header" on:click={toggleTags} on:keypress={(e) => e.key === 'Enter' && toggleTags()} role="button" tabindex="0">
          <div class="section-title">
            TAGS
            {#if currentFilters.tags && currentFilters.tags.length > 0}
              <span class="selected-count">({currentFilters.tags.length})</span>
            {/if}
          </div>
          <div class="header-actions">
            <button class="add-btn" on:click|stopPropagation={onAddTag} title="Add Tag">+</button>
            <span class="dropdown-arrow" class:expanded={tagsExpanded}>▼</span>
          </div>
        </div>
        {#if tagsExpanded}
          <div class="dropdown-content scrollable">
            <div class="checkbox-list">
              {#each $allTags as tag}
                <label class="checkbox-item" class:active={currentFilters.tags?.includes(tag.name)}>
                  <input
                    type="checkbox"
                    checked={currentFilters.tags?.includes(tag.name)}
                    on:change={() => toggleTag(tag.name)}
                  />
                  <span>{tag.icon} {tag.name}</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Footer -->
    <div class="sidebar-footer">
      <button class="clear-btn" on:click={clearAllFilters} disabled={activeFilterCount === 0}>
        Clear All Filters
      </button>
    </div>
  </div>

  <!-- Overlay backdrop -->
  <div class="sidebar-overlay" on:click={() => isOpen = false} on:keypress={(e) => e.key === 'Enter' && (isOpen = false)} role="button" tabindex="0"></div>
{/if}

<style>
  /* Sidebar */
  .filters-sidebar {
    position: fixed;
    top: 70px;
    left: 0;
    width: 320px;
    height: calc(100vh - 70px);
    background: rgba(20, 20, 25, 0.98);
    backdrop-filter: blur(20px);
    border-right: 1px solid rgba(247, 228, 121, 0.3);
    box-shadow: 4px 0 20px rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    flex-direction: column;
    animation: slideInLeft 0.3s ease-out;
    box-sizing: border-box;
  }

  @keyframes slideInLeft {
    from {
      transform: translateX(-100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  /* Overlay */
  .sidebar-overlay {
    position: fixed;
    top: 70px;
    left: 320px;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 9999;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* Header */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    flex-shrink: 0;
  }

  .sidebar-title {
    font-size: 14px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.95);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .filter-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 20px;
    padding: 0 6px;
    background: #f7e479;
    color: #1e1e23;
    font-size: 11px;
    font-weight: 700;
    border-radius: 10px;
  }

  .close-btn {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.7);
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.95);
  }

  /* Content */
  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 16px;
  }

  .sidebar-content::-webkit-scrollbar {
    width: 6px;
  }

  .sidebar-content::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.2);
  }

  .sidebar-content::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  .sidebar-content::-webkit-scrollbar-thumb:hover {
    background: rgba(247, 228, 121, 0.5);
  }

  /* Filter Section */
  .filter-section {
    margin-bottom: 20px;
  }

  .filter-section:last-child {
    margin-bottom: 0;
  }

  .section-title {
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.7);
    margin-bottom: 10px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .selected-count {
    color: #f7e479;
    font-weight: 700;
  }

  /* Section Header (for dropdowns) */
  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    padding: 8px 10px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    margin-bottom: 8px;
    transition: all 0.2s;
  }

  .section-header:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(247, 228, 121, 0.3);
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .add-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(247, 228, 121, 0.15);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 4px;
    color: #f7e479;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    line-height: 1;
  }

  .add-btn:hover {
    background: rgba(247, 228, 121, 0.25);
    transform: scale(1.1);
  }

  .dropdown-arrow {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.6);
    transition: transform 0.2s;
  }

  .dropdown-arrow.expanded {
    transform: rotate(180deg);
  }

  /* Dropdown Content */
  .dropdown-content {
    animation: slideDown 0.2s ease-out;
  }

  .dropdown-content.scrollable {
    max-height: 200px;
    overflow-y: auto;
  }

  .dropdown-content.scrollable::-webkit-scrollbar {
    width: 5px;
  }

  .dropdown-content.scrollable::-webkit-scrollbar-thumb {
    background: rgba(247, 228, 121, 0.3);
    border-radius: 3px;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      max-height: 0;
    }
    to {
      opacity: 1;
      max-height: 400px;
    }
  }

  /* Radio Group */
  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .radio-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .radio-item:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(247, 228, 121, 0.3);
  }

  .radio-item.active {
    background: rgba(247, 228, 121, 0.15);
    border-color: #f7e479;
  }

  .radio-item input[type="radio"] {
    appearance: none;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-radius: 50%;
    cursor: pointer;
    position: relative;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .radio-item input[type="radio"]:checked {
    border-color: #f7e479;
  }

  .radio-item input[type="radio"]:checked::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 6px;
    height: 6px;
    background: #f7e479;
    border-radius: 50%;
  }

  .radio-item span {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.9);
  }

  /* Checkbox List */
  .checkbox-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .checkbox-item:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(247, 228, 121, 0.3);
  }

  .checkbox-item.active {
    background: rgba(247, 228, 121, 0.15);
    border-color: #f7e479;
  }

  .checkbox-item input[type="checkbox"] {
    appearance: none;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-radius: 4px;
    cursor: pointer;
    position: relative;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .checkbox-item input[type="checkbox"]:checked {
    background: #f7e479;
    border-color: #f7e479;
  }

  .checkbox-item input[type="checkbox"]:checked::after {
    content: '✓';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #1e1e23;
    font-size: 10px;
    font-weight: 700;
  }

  .checkbox-item span {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.9);
  }

  /* Date Inputs */
  .date-inputs {
    display: flex;
    gap: 8px;
  }

  .date-input-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .date-input-wrapper label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.6);
    font-weight: 500;
  }

  .date-input-wrapper input[type="date"] {
    padding: 8px 10px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.9);
    font-size: 11px;
    font-family: inherit;
    transition: all 0.2s;
  }

  .date-input-wrapper input[type="date"]:hover {
    border-color: rgba(247, 228, 121, 0.4);
  }

  .date-input-wrapper input[type="date"]:focus {
    outline: none;
    border-color: #f7e479;
    background: rgba(255, 255, 255, 0.08);
  }

  .date-input-wrapper input[type="date"]::-webkit-calendar-picker-indicator {
    filter: invert(1);
    cursor: pointer;
  }

  /* Footer */
  .sidebar-footer {
    padding: 14px 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    flex-shrink: 0;
  }

  .clear-btn {
    width: 100%;
    padding: 10px;
    background: rgba(247, 228, 121, 0.1);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 6px;
    color: #f7e479;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .clear-btn:hover:not(:disabled) {
    background: rgba(247, 228, 121, 0.2);
    transform: translateY(-1px);
  }

  .clear-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
