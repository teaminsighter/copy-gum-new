<script lang="ts">
  // CopyGum Main Application
  // Reference: Development Plan Phase 2

  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import SvgFilters from './lib/components/ui/SvgFilters.svelte';
  import OverlayPanel from './lib/components/core/OverlayPanel.svelte';
  import LogoSection from './lib/components/header/LogoSection.svelte';
  import SearchBar from './lib/components/header/SearchBar.svelte';
  import CategoriesSection from './lib/components/categories/CategoriesSection.svelte';
  import SplitAddButton from './lib/components/header/SplitAddButton.svelte';
  import SettingsButton from './lib/components/header/SettingsButton.svelte';
  import SettingsDropdown from './lib/components/header/SettingsDropdown.svelte';
  import AdvancedFiltersPanel from './lib/components/header/AdvancedFiltersPanel.svelte';
  import ShortcutsHelp from './lib/components/help/ShortcutsHelp.svelte';
  import CardsContainer from './lib/components/cards/CardsContainer.svelte';
  import CategoryEditPanel from './lib/components/categories/CategoryEditPanel.svelte';
  import ToastContainer from './lib/components/ui/ToastContainer.svelte';
  import SetupWizard from './lib/components/ui/SetupWizard.svelte';
  import { createCustomCategory, loadCategoriesFromDatabase } from './lib/stores/categoryStore';
  import { ensureDefaultCategories } from './lib/services/database';
  import { createTag, loadTagsFromDatabase } from './lib/stores/tagStore';
  import { showSuccess, showError } from './lib/stores/toastStore';
  import { clickOutside } from './lib/utils/clickOutside';
  import { initClipboardStore, stopClipboardMonitoring, advancedFilters } from './lib/stores/clipboardStore';
  import { settings } from './lib/stores/settingsStore';

  // Card size mappings (width in pixels)
  const cardSizeMap: Record<string, number> = {
    small: 240,
    medium: 288,
    large: 340
  };

  // Apply CSS custom properties based on settings
  $: if (typeof document !== 'undefined') {
    document.documentElement.style.setProperty('--card-font-size', `${$settings.font_size}px`);
    document.documentElement.style.setProperty('--card-width', `${cardSizeMap[$settings.card_size] || 288}px`);
  }

  let visible = true; // Set to true for development/testing
  let showSettings = false;
  let showFilters = false;
  let showHelp = false;
  let showCategoryEditPanel = false;
  let showTagEditPanel = false;
  let showSetupWizard = false;
  let editName = '';
  let editIcon = '';
  let editColor = '';

  // Component references for keyboard navigation
  let categoriesSectionRef: any;
  let cardsContainerRef: any;

  // Wire up the component references when they're mounted
  $: if (categoriesSectionRef && cardsContainerRef) {
    cardsContainerRef.setCategoriesRef(categoriesSectionRef);
  }

  // Reactive: Count active filters
  $: activeFilterCount = getActiveFilterCount($advancedFilters);

  function getActiveFilterCount(filters: typeof $advancedFilters): number {
    let count = 0;
    if (filters.pinStatus && filters.pinStatus !== 'all') count++;
    if (filters.contentTypes && filters.contentTypes.length > 0) count++;
    if (filters.dateRange) count++;
    return count;
  }

  function handleKeydown(event: KeyboardEvent) {
    // ? - Toggle help panel
    if (event.key === '?' && event.shiftKey) {
      event.preventDefault();
      showHelp = !showHelp;
      return;
    }

    // Cmd+Shift+F - Toggle filters panel
    if ((event.metaKey || event.ctrlKey) && event.shiftKey && event.key === 'F') {
      event.preventDefault();
      toggleFilters();
      return;
    }

    // Cmd+Shift+C - Clear all filters
    if ((event.metaKey || event.ctrlKey) && event.shiftKey && event.key === 'C') {
      event.preventDefault();
      clearAllFilters();
      return;
    }

    // Escape key handling
    if (event.key === 'Escape') {
      if (showHelp) {
        showHelp = false;
      } else if (showSettings) {
        showSettings = false;
      } else if (showFilters) {
        showFilters = false;
      } else {
        invoke('hide_window').catch(err => {
          console.error('Failed to hide window:', err);
        });
      }
    }
  }

  function clearAllFilters() {
    advancedFilters.set({
      pinStatus: 'all',
      contentTypes: []
    });
    showSuccess('All filters cleared');
  }

  function toggleSettings() {
    showSettings = !showSettings;
  }

  function handleCloseSettings() {
    showSettings = false;
  }

  function toggleFilters() {
    showFilters = !showFilters;
  }

  function handleCloseFilters() {
    showFilters = false;
  }

  function handleAddCategory() {
    // TODO: implement category creation panel
    showCategoryEditPanel = true;
  }

  function handleAddTag() {
    // Close category panel if open
    showCategoryEditPanel = false;
    editName = '';
    editIcon = '🏷️'; // Default icon for new tag
    editColor = 'rgba(50, 50, 50, 0.6)';
    showTagEditPanel = true;
  }

  async function handleSaveCategoryEdit(name: string, icon: string, color: string) {
    if (name.trim()) {
      try {
        // Create category with icon and color (now async)
        await createCustomCategory(name, icon, color);
        showSuccess(`Category "${name}" created!`);
      } catch (error) {
        console.error('Failed to create category:', error);
        showError('Failed to create category');
      }
    } else {
      showError('Category name cannot be empty');
    }
    showCategoryEditPanel = false;
  }

  function handleSaveTagEdit(name: string, icon: string, color: string) {
    if (name.trim()) {
      createTag(name, icon, color);
      showSuccess(`Tag "${name}" created!`);
    } else {
      showError('Tag name cannot be empty');
    }
    showTagEditPanel = false;
  }

  function handleCloseCategoryEdit() {
    showCategoryEditPanel = false;
  }

  function handleCloseTagEdit() {
    showTagEditPanel = false;
  }

  onMount(async () => {
    // Ensure all default categories exist in database (handles migration)
    await ensureDefaultCategories();

    // Load categories from database
    await loadCategoriesFromDatabase();

    // Load tags from database
    await loadTagsFromDatabase();

    // Initialize clipboard store (single initialization point)
    await initClipboardStore();

    // Show setup wizard on first run
    const currentSettings = $settings;
    if (!currentSettings.hasShownOverlayInfo) {
      showSetupWizard = true;
    }
  });

  onDestroy(async () => {
    // Cleanup: stop monitoring and remove event listener
    await stopClipboardMonitoring();
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main>
  <!-- SVG Filters (hidden, used by CSS filter property) -->
  <SvgFilters />

  <!-- Main Overlay Panel -->
  <OverlayPanel {visible}>
    <svelte:fragment slot="header">
      <LogoSection />

      <SearchBar />

      <!-- Filter Button -->
      <div class="filter-button-wrapper">
        <button class="filter-btn" on:click={toggleFilters} class:active={showFilters || activeFilterCount > 0}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M2 3h12M4 8h8M6 13h4" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          {#if activeFilterCount > 0}
            <span class="filter-badge">{activeFilterCount}</span>
          {/if}
        </button>

        <!-- Advanced Filters Panel -->
        {#if showFilters}
          <div use:clickOutside={handleCloseFilters}>
            <AdvancedFiltersPanel
              bind:isOpen={showFilters}
              onAddCategory={handleAddCategory}
              onAddTag={handleAddTag}
            />
          </div>
        {/if}
      </div>

      <CategoriesSection bind:this={categoriesSectionRef} />

      <!-- Header Actions (right side) -->
      <div class="header-actions">
        <SplitAddButton
          onAddCategory={handleAddCategory}
          onAddTag={handleAddTag}
        />
        <SettingsButton onClick={toggleSettings} />

        <!-- Global Category Edit Panel (for adding new categories) -->
        {#if showCategoryEditPanel}
          <div class="global-edit-panel" use:clickOutside={handleCloseCategoryEdit}>
            <CategoryEditPanel
              show={showCategoryEditPanel}
              categoryName={editName}
              categoryIcon={editIcon}
              categoryColor={editColor}
              showColorTab={false}
              isNameEditable={true}
              onSave={handleSaveCategoryEdit}
              onClose={handleCloseCategoryEdit}
            />
          </div>
        {/if}

        <!-- Global Tag Edit Panel (for adding new tags) -->
        {#if showTagEditPanel}
          <div class="global-edit-panel" use:clickOutside={handleCloseTagEdit}>
            <CategoryEditPanel
              show={showTagEditPanel}
              categoryName={editName}
              categoryIcon={editIcon}
              categoryColor={editColor}
              showColorTab={true}
              isNameEditable={true}
              panelTitle="Tag name"
              onSave={handleSaveTagEdit}
              onClose={handleCloseTagEdit}
            />
          </div>
        {/if}
      </div>
    </svelte:fragment>

    <svelte:fragment slot="content">
      <CardsContainer bind:this={cardsContainerRef} />
    </svelte:fragment>
  </OverlayPanel>

  <!-- Settings Dropdown -->
  {#if showSettings}
    <div class="settings-dropdown-wrapper" use:clickOutside={handleCloseSettings}>
      <SettingsDropdown show={showSettings} />
    </div>
  {/if}

  <!-- Keyboard Shortcuts Help -->
  <ShortcutsHelp bind:show={showHelp} />

  <!-- Toast Notifications -->
  <ToastContainer />

  <!-- Setup Wizard (first run) -->
  <SetupWizard bind:show={showSetupWizard} />
</main>

<style>
  main {
    width: 100%;
    height: 100%;
  }

  /* Filter Button */
  .filter-button-wrapper {
    position: relative;
    margin-left: 8px;
  }

  .filter-btn {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    height: 32px;
    padding: 0 10px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.8);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .filter-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(247, 228, 121, 0.4);
    color: rgba(255, 255, 255, 0.95);
  }

  .filter-btn.active {
    background: rgba(247, 228, 121, 0.15);
    border-color: #f7e479;
    color: #f7e479;
  }

  .filter-badge {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 16px;
    height: 16px;
    padding: 0 4px;
    background: #f7e479;
    color: #1e1e23;
    font-size: 10px;
    font-weight: 700;
    border-radius: 8px;
  }

  /* Header Actions - Reference: preview.html lines 86-93 */
  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-left: auto;
    flex-shrink: 1;
    overflow: visible;
    position: relative;
  }

  /* Global Edit Panel Container */
  .global-edit-panel {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 8px;
    width: 288px;
    max-height: 380px;
    z-index: 10000;
    pointer-events: none;
  }

  .global-edit-panel :global(.edit-panel) {
    pointer-events: auto;
  }

  /* Settings Dropdown Wrapper */
  .settings-dropdown-wrapper {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 999;
    pointer-events: none;
  }

  .settings-dropdown-wrapper :global(.settings-dropdown) {
    pointer-events: auto;
  }
</style>
