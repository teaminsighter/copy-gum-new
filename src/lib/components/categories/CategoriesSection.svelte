<script lang="ts">
  // Categories Section with Horizontal Scrolling
  // Reference: preview.html lines 2038-2254, 277-582

  import { onMount } from 'svelte';
  import CategoryPill from './CategoryPill.svelte';
  import CategoryEditPanel from './CategoryEditPanel.svelte';
  import { categoryCustomizations, customCategories, categoryOrder, updateCategoryIcon, updateCategoryColor, updateCustomCategory, reorderCategories } from '../../stores/categoryStore';
  import { selectedCategory } from '../../stores/clipboardStore';
  import { clickOutside } from '../../utils/clickOutside';
  import { selectedCategoryIndex, totalCategories, focusLayer } from '../../stores/navigationStore';

  // Default categories from database schema
  const defaultCategories = [
    { id: 'all', icon: '📌', name: 'All', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'password', icon: '🔐', name: 'Password', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'apikey', icon: '🔑', name: 'API Key', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'private', icon: '🔒', name: 'Private', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'email', icon: '📧', name: 'Email', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'phone', icon: '📱', name: 'Phone', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'links', icon: '🔗', name: 'Links', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'code', icon: '💻', name: 'Code', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'color', icon: '🎨', name: 'Color', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'image', icon: '🖼️', name: 'Image', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'number', icon: '🔢', name: 'Number', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
    { id: 'text', icon: '📝', name: 'Text', color: 'rgba(50, 50, 50, 0.6)', isCustom: false },
  ];

  // Build categories array respecting the order from categoryOrder store
  $: categories = $categoryOrder
    .map(id => {
      // Try to find in default categories
      const defaultCat = defaultCategories.find(c => c.id === id);
      if (defaultCat) {
        return {
          ...defaultCat,
          icon: $categoryCustomizations[id]?.icon || defaultCat.icon,
          color: $categoryCustomizations[id]?.color || defaultCat.color,
          isCustom: false
        };
      }

      // Try to find in custom categories
      const customCat = $customCategories.find(c => c.id === id);
      if (customCat) {
        return {
          ...customCat,
          color: 'rgba(50, 50, 50, 0.6)',
          isCustom: true
        };
      }

      return null;
    })
    .filter(cat => cat !== null) as Array<{
      id: string;
      icon: string;
      name: string;
      color: string;
      isCustom: boolean;
    }>;

  // Update arrows when categories change
  $: if (categories && categoriesContainer) {
    setTimeout(() => updateArrows(), 0);
  }

  // Update total categories count for navigation
  $: totalCategories.set(categories.length);

  // Sync activeCategory with store (so pill highlighting stays correct)
  $: activeCategory = $selectedCategory || 'all';
  let categoriesContainer: HTMLDivElement;
  let showLeftArrow = false;
  let showRightArrow = false;

  // Edit panel state
  let showEditPanel = false;
  let editingCategory: typeof categories[0] | null = null;
  let panelLeft = 0;
  let panelTop = 45;

  // Drag-to-reorder state
  let draggedIndex: number | null = null;
  let dropTargetIndex: number | null = null;
  let isDraggingCategory = false;
  let dragStartX = 0;
  let dragStartTime = 0;
  let dragThreshold = 5; // px movement threshold to start drag

  // Momentum scrolling variables
  let isDragging = false;
  let lastMouseX = 0;
  let lastMoveTime = 0;
  let velocitySamples: Array<{ velocity: number; time: number }> = [];
  let momentumAnimation: number | null = null;

  function handleCategoryClick(categoryId: string) {
    selectedCategory.set(categoryId === 'all' ? null : categoryId);
  }

  // Export function to activate category by index (for keyboard navigation)
  export function activateSelectedCategory() {
    if ($selectedCategoryIndex >= 0 && $selectedCategoryIndex < categories.length) {
      const category = categories[$selectedCategoryIndex];
      handleCategoryClick(category.id);
    }
  }

  function handleEditCategory(categoryId: string, event?: MouseEvent) {
    const cat = categories.find(c => c.id === categoryId);
    if (cat) {
      editingCategory = cat;

      // Calculate position based on clicked element
      if (event) {
        const target = (event.target as HTMLElement).closest('.category-pill') as HTMLElement;
        if (target) {
          const pillRect = target.getBoundingClientRect();

          // Position below the pill using viewport coordinates
          // Panel will be positioned with position: fixed
          panelTop = pillRect.bottom + 8; // 8px gap below pill
          panelLeft = pillRect.left;

          // Ensure panel doesn't overflow right edge of screen
          const panelWidth = 288; // Width of edit panel
          if (panelLeft + panelWidth > window.innerWidth - 20) {
            panelLeft = window.innerWidth - panelWidth - 20;
          }

          // Ensure panel doesn't overflow left edge
          if (panelLeft < 20) {
            panelLeft = 20;
          }
        }
      }

      showEditPanel = true;
    }
  }

  async function handleSaveCategory(name: string, icon: string, color: string) {
    if (editingCategory) {
      // Check if this is a custom category or default category
      const isCustomCategory = $customCategories.some(cat => cat.id === editingCategory?.id);

      if (isCustomCategory) {
        // Update custom category (name and icon only, no color)
        await updateCustomCategory(editingCategory.id, { name, icon });
      } else {
        // Update default category (icon only, no name or color changes)
        if (icon !== editingCategory.icon) {
          updateCategoryIcon(editingCategory.id, icon);
        }
      }
    }
    showEditPanel = false;
    editingCategory = null;
  }

  function handleCloseEditPanel() {
    showEditPanel = false;
    editingCategory = null;
  }

  // Manual drag and drop handlers (like preview.html)
  function handlePillMouseDown(index: number, e: MouseEvent) {
    // Don't drag the "all" category (index 0)
    if (index === 0) {
      return;
    }

    draggedIndex = index;
    dragStartX = e.clientX;
    dragStartTime = Date.now();
    isDraggingCategory = false;

    // Add document-level listeners for drag detection
    document.addEventListener('mousemove', detectCategoryDrag);
    document.addEventListener('mouseup', endCategoryDrag);
  }

  function detectCategoryDrag(e: MouseEvent) {
    if (draggedIndex === null) {
      return;
    }

    const deltaX = Math.abs(e.clientX - dragStartX);
    const deltaTime = Date.now() - dragStartTime;

    // Start drag if moved more than threshold or held for 200ms
    if ((deltaX > dragThreshold || deltaTime > 200) && !isDraggingCategory) {
      isDraggingCategory = true;

      // Prevent momentum scrolling while dragging
      if (categoriesContainer) {
        categoriesContainer.style.cursor = 'grabbing';
      }
    }

    if (isDraggingCategory) {
      // Calculate which pill we're hovering over
      updateDropTarget(e.clientX);
    }
  }

  function updateDropTarget(mouseX: number) {
    if (!categoriesContainer || draggedIndex === null) return;

    const pills = categoriesContainer.querySelectorAll('.category-pill');
    let closestIndex: number | null = null;
    let closestDistance = Infinity;

    pills.forEach((pill, index) => {
      // Skip the "all" category and the dragged pill
      if (index === 0 || index === draggedIndex) return;

      const rect = pill.getBoundingClientRect();
      const pillCenterX = rect.left + rect.width / 2;
      const distance = Math.abs(mouseX - pillCenterX);

      if (distance < closestDistance) {
        closestDistance = distance;
        closestIndex = index;
      }
    });

    dropTargetIndex = closestIndex;
  }

  function endCategoryDrag() {
    document.removeEventListener('mousemove', detectCategoryDrag);
    document.removeEventListener('mouseup', endCategoryDrag);

    if (isDraggingCategory && draggedIndex !== null && dropTargetIndex !== null && draggedIndex !== dropTargetIndex) {
      // Use the store function to reorder - this will trigger reactive recalculation
      reorderCategories(draggedIndex, dropTargetIndex);
    }

    // Cleanup
    if (categoriesContainer) {
      categoriesContainer.style.cursor = 'grab';
    }
    draggedIndex = null;
    dropTargetIndex = null;
    isDraggingCategory = false;
  }

  function updateArrows() {
    if (!categoriesContainer) return;

    const { scrollLeft, scrollWidth, clientWidth } = categoriesContainer;
    showLeftArrow = scrollLeft > 10;
    showRightArrow = scrollLeft < scrollWidth - clientWidth - 10;
  }

  function scrollLeft() {
    if (categoriesContainer) {
      categoriesContainer.scrollBy({ left: -200, behavior: 'smooth' });
    }
  }

  function scrollRight() {
    if (categoriesContainer) {
      categoriesContainer.scrollBy({ left: 200, behavior: 'smooth' });
    }
  }

  // Momentum scrolling - Reference: preview.html lines 3995-4097
  function handleMouseDown(e: MouseEvent) {
    // Don't interfere with category pill clicks
    if ((e.target as HTMLElement).closest('.category-pill')) {
      return;
    }

    // Prevent text selection during drag
    e.preventDefault();
    if (document.body) {
      document.body.style.userSelect = 'none';
    }

    isDragging = true;
    if (categoriesContainer) {
      categoriesContainer.style.cursor = 'grabbing';
    }
    lastMouseX = e.clientX;
    lastMoveTime = Date.now();
    velocitySamples = [];

    if (momentumAnimation) {
      cancelAnimationFrame(momentumAnimation);
      momentumAnimation = null;
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !categoriesContainer) return;
    e.preventDefault();

    const currentX = e.clientX;
    const currentTime = Date.now();
    const deltaX = currentX - lastMouseX;
    const deltaTime = currentTime - lastMoveTime;

    categoriesContainer.scrollLeft -= deltaX;

    if (deltaTime > 0) {
      velocitySamples.push({
        velocity: deltaX / deltaTime,
        time: currentTime
      });
      velocitySamples = velocitySamples.filter(
        sample => currentTime - sample.time < 80
      );
    }

    lastMouseX = currentX;
    lastMoveTime = currentTime;
  }

  function handleMouseUp() {
    if (!isDragging) return;
    isDragging = false;
    if (categoriesContainer) {
      categoriesContainer.style.cursor = 'grab';
    }

    // Re-enable text selection
    if (document.body) {
      document.body.style.userSelect = '';
    }

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
  }

  function handleMouseLeave() {
    if (isDragging) {
      handleMouseUp();
    }
  }

  function applyMomentum(initialVelocity: number) {
    let velocity = initialVelocity * 16;

    function animate() {
      if (!categoriesContainer) return;

      categoriesContainer.scrollLeft -= velocity;
      velocity *= 0.92; // Friction

      if (Math.abs(velocity) > 0.3) {
        momentumAnimation = requestAnimationFrame(animate);
      } else {
        momentumAnimation = null;
      }
    }

    animate();
  }

  // Scroll to selected category for keyboard navigation
  export function scrollToSelected() {
    if (!categoriesContainer || $selectedCategoryIndex < 0) return;

    const pills = categoriesContainer.querySelectorAll('.category-pill');
    const selectedPill = pills[$selectedCategoryIndex] as HTMLElement;

    if (selectedPill) {
      selectedPill.scrollIntoView({
        behavior: 'smooth',
        block: 'nearest',
        inline: 'center'
      });
    }
  }

  onMount(() => {
    updateArrows();

    if (categoriesContainer) {
      categoriesContainer.addEventListener('scroll', updateArrows);
      return () => {
        categoriesContainer.removeEventListener('scroll', updateArrows);
      };
    }
  });
</script>

<div class="categories-section">
  <!-- Left Arrow -->
  <div
    class="category-scroll-arrow left"
    class:visible={showLeftArrow}
    on:click={scrollLeft}
    on:keypress={(e) => e.key === 'Enter' && scrollLeft()}
    role="button"
    tabindex="0"
  >
    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M15 18L9 12L15 6" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </div>

  <!-- Right Arrow -->
  <div
    class="category-scroll-arrow right"
    class:visible={showRightArrow}
    on:click={scrollRight}
    on:keypress={(e) => e.key === 'Enter' && scrollRight()}
    role="button"
    tabindex="0"
  >
    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M9 18L15 12L9 6" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </div>

  <!-- Categories Container -->
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div
    class="categories-container"
    bind:this={categoriesContainer}
    on:mousedown={handleMouseDown}
    on:mousemove={handleMouseMove}
    on:mouseup={handleMouseUp}
    on:mouseleave={handleMouseLeave}
    role="region"
    aria-label="Category filter"
  >
    {#each categories as category, index (category.id)}
      <CategoryPill
        category={category.id}
        icon={category.icon}
        name={category.name}
        active={activeCategory === category.id}
        isDragging={draggedIndex === index}
        isDropTarget={dropTargetIndex === index}
        isCustom={category.isCustom}
        isSelected={$focusLayer === 'categories' && $selectedCategoryIndex === index}
        onClick={() => handleCategoryClick(category.id)}
        onEdit={(id, event) => handleEditCategory(id, event)}
        onMouseDown={(e) => handlePillMouseDown(index, e)}
      />
    {/each}
  </div>

  <!-- Edit Panel (positioned absolutely over categories) -->
  {#if editingCategory}
    <div class="category-edit-wrapper" style="top: {panelTop}px; left: {panelLeft}px;" use:clickOutside={handleCloseEditPanel}>
      <CategoryEditPanel
        show={showEditPanel}
        categoryName={editingCategory.name}
        categoryIcon={editingCategory.icon}
        categoryColor={editingCategory.color}
        showColorTab={false}
        isNameEditable={editingCategory.isCustom}
        onSave={handleSaveCategory}
        onClose={handleCloseEditPanel}
      />
    </div>
  {/if}
</div>

<style>
  /* Reference: preview.html lines 277-582 */
  .categories-section {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    margin: 0 20px;
    overflow: visible;
    position: relative;
    max-width: calc(100% - 40px);
  }

  .categories-container {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    overflow-y: visible;
    cursor: grab;
    scroll-behavior: smooth;
    flex: 1;
    min-width: 0;
    padding: 4px 0;
    box-sizing: border-box;
    -ms-overflow-style: none;
    scrollbar-width: none;

    /* Prevent text selection during drag */
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
  }

  .categories-container::-webkit-scrollbar {
    display: none;
  }

  .categories-container:active {
    cursor: grabbing;
    user-select: none !important;
  }

  /* Category scroll arrows */
  .category-scroll-arrow {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 10;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    transition: all 0.2s ease;
    opacity: 0;
    pointer-events: none;
  }

  .category-scroll-arrow.visible {
    opacity: 1;
    pointer-events: all;
  }

  .category-scroll-arrow:hover {
    background: rgba(255, 255, 255, 0.25);
    border-color: rgba(255, 255, 255, 0.4);
    transform: translateY(-50%) scale(1.1);
  }

  .category-scroll-arrow.left {
    left: 0;
    background: linear-gradient(90deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0.05) 100%);
  }

  .category-scroll-arrow.right {
    right: 0;
    background: linear-gradient(270deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0.05) 100%);
  }

  .category-scroll-arrow svg {
    width: 20px;
    height: 20px;
    stroke: rgba(255, 255, 255, 0.9);
    stroke-width: 2.5;
  }

  /* Category Edit Panel Wrapper */
  .category-edit-wrapper {
    position: fixed;
    z-index: 15000;
    pointer-events: none;
  }

  .category-edit-wrapper :global(.edit-panel) {
    pointer-events: auto;
    position: relative;
    top: 0;
    left: 0;
  }
</style>
