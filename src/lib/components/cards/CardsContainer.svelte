<script lang="ts">
  // Cards Container with Horizontal Scrolling
  // Reference: preview.html lines 2660-2661, 764-790

  import { onMount } from 'svelte';
  import ClipboardCard from './ClipboardCard.svelte';
  import SkeletonCard from './SkeletonCard.svelte';
  import EmptyState from '../ui/EmptyState.svelte';
  import {
    selectedCardIndex,
    totalCards,
    selectNextCard,
    selectPreviousCard,
    selectedCategoryIndex,
    selectNextCategory,
    selectPreviousCategory,
    focusLayer,
    switchToCategories,
    switchToCards
  } from '../../stores/navigationStore';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { showSuccess, showError } from '../../stores/toastStore';
  import {
    filteredItems,
    isLoading as storeLoading,
    error as storeError,
    selectedCategory,
    searchQuery,
    togglePin,
    deleteItem
  } from '../../stores/clipboardStore';
  import { settings } from '../../stores/settingsStore';
  import {
    isDragging as dragStoreIsDragging,
    startPotentialDrag,
    confirmDragging,
    endDragging,
    DRAG_THRESHOLD
  } from '../../stores/dragStore';

  // Subscribe to clipboard store
  $: items = $filteredItems;
  $: loading = $storeLoading;
  $: errorMessage = $storeError;

  // Helper to convert local file paths to Tauri asset URLs
  function getImageUrl(imagePath: string | undefined): string {
    if (!imagePath) {
      return '';
    }
    // Convert local file path to Tauri asset URL
    const url = convertFileSrc(imagePath);
    return url;
  }

  // Helper to format file size
  function formatFileSize(bytes: number | undefined): string {
    if (!bytes) return '';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  // Helper to format image dimensions
  function formatImageSize(width: number | undefined, height: number | undefined): string {
    if (!width || !height) return '';
    return `${width} × ${height}`;
  }

  // Helper to detect if a hex color is light (for text contrast)
  function isLightColor(hexColor: string | undefined): boolean {
    if (!hexColor || !hexColor.startsWith('#')) return false;

    // Remove # and parse hex
    let hex = hexColor.slice(1);

    // Handle 3-character hex (#RGB)
    if (hex.length === 3) {
      hex = hex[0] + hex[0] + hex[1] + hex[1] + hex[2] + hex[2];
    }

    // Parse RGB values
    const r = parseInt(hex.substr(0, 2), 16);
    const g = parseInt(hex.substr(2, 2), 16);
    const b = parseInt(hex.substr(4, 2), 16);

    // Calculate relative luminance (WCAG formula)
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

    // Return true if luminance is above 0.5 (light color)
    return luminance > 0.5;
  }

  // REMOVED: Sample data (now loaded from database via clipboardStore)

  let cardsContainer: HTMLDivElement;

  // Helper function to format Unix timestamp to relative time
  function formatTimestamp(timestamp: number): string {
    const now = Date.now();
    const diff = now - timestamp;
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (seconds < 60) return 'just now';
    if (minutes < 60) return `${minutes} min ago`;
    if (hours < 24) return `${hours} hour${hours > 1 ? 's' : ''} ago`;
    if (days < 7) return `${days} day${days > 1 ? 's' : ''} ago`;
    return new Date(timestamp).toLocaleDateString();
  }

  // Export a function to allow parent to pass reference to CategoriesSection
  let categoriesSectionRef: any = null;
  export function setCategoriesRef(ref: any) {
    categoriesSectionRef = ref;
  }

  // Momentum scrolling variables
  let isDragging = false;          // Local drag state for scroll handling
  let isPotentialDrag = false;     // Tracking mousedown but not yet confirmed drag
  let dragStartX = 0;              // Where the drag started (for threshold)
  let lastMouseX = 0;
  let lastMoveTime = 0;
  let velocitySamples: Array<{ velocity: number; time: number }> = [];
  let momentumAnimation: number | null = null;

  function handleMouseDown(e: MouseEvent) {
    // Don't start drag if clicking on a card's interactive elements
    const target = e.target as HTMLElement;
    if (target.closest('button') || target.closest('.category-label') || target.closest('.tag') || target.closest('.add-tag-btn')) {
      return;
    }

    // Prevent text selection during potential drag
    e.preventDefault();
    if (document.body) {
      document.body.style.userSelect = 'none';
    }

    // Start potential drag - don't confirm yet until movement exceeds threshold
    isPotentialDrag = true;
    dragStartX = e.clientX;
    lastMouseX = e.clientX;
    lastMoveTime = Date.now();
    velocitySamples = [];

    // Notify drag store that a potential drag started
    startPotentialDrag();

    if (momentumAnimation) {
      cancelAnimationFrame(momentumAnimation);
      momentumAnimation = null;
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isPotentialDrag || !cardsContainer) return;

    const currentX = e.clientX;
    const currentTime = Date.now();
    const totalMovement = Math.abs(currentX - dragStartX);

    // Check if movement exceeds drag threshold
    if (!isDragging && totalMovement > DRAG_THRESHOLD) {
      // Confirm this is a real drag, not a click
      isDragging = true;
      confirmDragging(); // Update global store
      if (cardsContainer) {
        cardsContainer.style.cursor = 'grabbing';
      }
    }

    // Only scroll if we've confirmed it's a drag
    if (isDragging) {
      e.preventDefault();
      const deltaX = currentX - lastMouseX;
      const deltaTime = currentTime - lastMoveTime;

      cardsContainer.scrollLeft -= deltaX;

      if (deltaTime > 0) {
        velocitySamples.push({
          velocity: deltaX / deltaTime,
          time: currentTime
        });
        velocitySamples = velocitySamples.filter(
          sample => currentTime - sample.time < 80
        );
      }
    }

    lastMouseX = currentX;
    lastMoveTime = currentTime;
  }

  function handleMouseUp() {
    // Re-enable text selection
    if (document.body) {
      document.body.style.userSelect = '';
    }

    // If this was a confirmed drag, end it properly
    if (isDragging) {
      isDragging = false;
      isPotentialDrag = false;
      endDragging(); // Update global store - blocks clicks briefly

      if (cardsContainer) {
        cardsContainer.style.cursor = 'grab';
      }

      // Apply momentum
      let velocity = 0;
      if (velocitySamples.length > 0) {
        const totalVelocity = velocitySamples.reduce(
          (sum, sample) => sum + sample.velocity, 0
        );
        velocity = totalVelocity / velocitySamples.length;
      }

      if (Math.abs(velocity) > 0.1) {
        applyMomentum(velocity);
      }
    } else {
      // Was a potential drag that never exceeded threshold (i.e., a click)
      isPotentialDrag = false;
      // Don't call endDragging() - let the click go through normally
    }
  }

  function handleMouseLeave() {
    if (isDragging || isPotentialDrag) {
      handleMouseUp();
    }
  }

  function applyMomentum(initialVelocity: number) {
    let velocity = initialVelocity * 16;

    function animate() {
      if (!cardsContainer) return;

      cardsContainer.scrollLeft -= velocity;
      velocity *= 0.92; // Friction

      if (Math.abs(velocity) > 0.3) {
        momentumAnimation = requestAnimationFrame(animate);
      } else {
        momentumAnimation = null;
      }
    }

    animate();
  }

  // Keyboard navigation with 2D support
  function handleKeydown(e: KeyboardEvent) {
    // Skip if typing in input field
    const target = e.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;

    const currentLayer = $focusLayer;

    switch (e.key) {
      case 'ArrowUp':
        e.preventDefault();
        // Switch from cards to categories
        if (currentLayer === 'cards') {
          switchToCategories();
          if (categoriesSectionRef?.scrollToSelected) {
            setTimeout(() => categoriesSectionRef.scrollToSelected(), 50);
          }
        }
        break;

      case 'ArrowDown':
        e.preventDefault();
        // Switch from categories to cards
        if (currentLayer === 'categories') {
          switchToCards();
          scrollToSelected();
        }
        break;

      case 'ArrowRight':
        e.preventDefault();
        if (currentLayer === 'categories') {
          // Navigate categories
          selectNextCategory();
          if (categoriesSectionRef?.scrollToSelected) {
            setTimeout(() => categoriesSectionRef.scrollToSelected(), 50);
          }
        } else {
          // Navigate cards
          selectNextCard();
          scrollToSelected();
        }
        break;

      case 'ArrowLeft':
        e.preventDefault();
        if (currentLayer === 'categories') {
          // Navigate categories
          selectPreviousCategory();
          if (categoriesSectionRef?.scrollToSelected) {
            setTimeout(() => categoriesSectionRef.scrollToSelected(), 50);
          }
        } else {
          // Navigate cards
          selectPreviousCard();
          scrollToSelected();
        }
        break;

      case 'Enter':
        e.preventDefault();
        if (currentLayer === 'cards') {
          copySelectedCard();
        } else {
          // Activate selected category (filter by it)
          if (categoriesSectionRef?.activateSelectedCategory) {
            categoriesSectionRef.activateSelectedCategory();
          }
        }
        break;

      case 'Delete':
      case 'Backspace':
        // Only delete with Cmd/Ctrl modifier to prevent accidental deletes
        if ((e.metaKey || e.ctrlKey) && currentLayer === 'cards') {
          e.preventDefault();
          deleteSelectedCard();
        }
        break;

      case 'p':
      case 'P':
        // Cmd+P to toggle pin
        if ((e.metaKey || e.ctrlKey) && currentLayer === 'cards') {
          e.preventDefault();
          togglePinSelectedCard();
        }
        break;
    }
  }

  function scrollToSelected() {
    if (!cardsContainer || $selectedCardIndex < 0) return;

    const cards = cardsContainer.querySelectorAll('.clipboard-card, .skeleton-card');
    const selectedCard = cards[$selectedCardIndex] as HTMLElement;

    if (selectedCard) {
      selectedCard.scrollIntoView({
        behavior: 'smooth',
        block: 'nearest',
        inline: 'center'
      });
    }
  }

  async function copySelectedCard() {
    if ($selectedCardIndex < 0 || $selectedCardIndex >= items.length) return;

    const card = items[$selectedCardIndex];
    try {
      // For image cards, copy the actual image to clipboard
      if (card.category === 'image' && card.image_path) {
        await invoke('copy_image_to_clipboard', { imagePath: card.image_path });
        showSuccess('Image copied to clipboard!');
      } else {
        // For text/other cards, copy the text content
        await writeText(card.content);
        showSuccess('Copied to clipboard!');
      }

      // Auto-close panel after copy (with small delay for feedback)
      setTimeout(() => {
        invoke('hide_window').catch(err => {
          console.error('Failed to hide window:', err);
        });
      }, 200);
    } catch (err) {
      showError('Failed to copy');
    }
  }

  async function togglePinSelectedCard() {
    if ($selectedCardIndex < 0 || $selectedCardIndex >= items.length) return;

    const card = items[$selectedCardIndex];
    if (!card || card.id === undefined) return;

    try {
      await togglePin(card.id, !card.is_pinned);
      showSuccess(card.is_pinned ? 'Unpinned' : 'Pinned');
    } catch (err) {
      showError('Failed to toggle pin');
    }
  }

  async function deleteSelectedCard() {
    if ($selectedCardIndex < 0 || $selectedCardIndex >= items.length) return;

    const card = items[$selectedCardIndex];
    if (!card || card.id === undefined) return;

    try {
      await deleteItem(card.id);
      showSuccess('Item deleted');
      // Keep selection at same index (next item will move into this position)
    } catch (err) {
      showError('Failed to delete');
    }
  }

  onMount(() => {
    // Items are already loaded by clipboardStore auto-initialization
    // Update total cards count when items change
    const unsubscribe = filteredItems.subscribe(currentItems => {
      totalCards.set(currentItems.length);
    });

    return () => {
      if (momentumAnimation) {
        cancelAnimationFrame(momentumAnimation);
      }
      unsubscribe();
    };
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
  class="cards-container"
  bind:this={cardsContainer}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseLeave}
  role="region"
  aria-label="Clipboard items"
>
  {#if loading}
    <!-- Show skeleton cards while loading -->
    {#each Array(5) as _}
      <SkeletonCard />
    {/each}
  {:else if errorMessage}
    <!-- Show error state -->
    <EmptyState
      icon="⚠️"
      title="Error loading clipboard"
      message={errorMessage}
    />
  {:else if items.length === 0}
    <!-- Show empty state when no items -->
    <EmptyState
      icon="📋"
      title="No clipboard history yet"
      message="Copy something to get started. Your clipboard items will appear here."
    />
  {:else}
    <!-- Show actual cards from database -->
    {#each items as item, index (item.id)}
      <ClipboardCard
        itemId={item.id}
        appIcon={item.app_icon || '📋'}
        appName={item.app_name || 'Unknown'}
        category={item.category}
        content={item.content}
        timestamp={formatTimestamp(item.timestamp)}
        charCount={item.content.length}
        isPinned={item.is_pinned}
        tags={item.tag_names ? item.tag_names.split(',').filter(t => t.trim()) : []}
        customBg={item.category === 'image' ? (item.image_dominant_color || '') : ''}
        isLightBg={item.category === 'color' ? isLightColor(item.content) : false}
        isSelected={$selectedCardIndex === index}
        imageUrl={getImageUrl(item.image_path)}
        imagePath={item.image_path || ''}
        imageSize={formatImageSize(item.image_width, item.image_height)}
        fileSize={formatFileSize(item.image_size)}
        showThumbnails={$settings.show_thumbnails}
      />
    {/each}
  {/if}
</div>

<style>
  /* Reference: preview.html lines 764-790 */
  .cards-container {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    height: 100%;
    padding: 4px 20px 0 20px;
    overflow-x: auto;
    overflow-y: visible;
    cursor: grab;
    user-select: none;
    width: 100%;
    max-width: 100%;
    box-sizing: border-box;
  }

  .cards-container::-webkit-scrollbar {
    display: none;
  }

  .cards-container {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  .cards-container:active {
    cursor: grabbing;
    user-select: none !important;
    -webkit-user-select: none !important;
  }
</style>
