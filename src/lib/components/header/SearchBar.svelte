<script lang="ts">
  // CopyGum Expandable Search Bar
  // Reference: preview.html lines 2028-2034, 647-753, 3926-3994

  import { onMount } from 'svelte';
  import MiniTooltip from '../ui/MiniTooltip.svelte';
  import { searchQuery } from '../../stores/clipboardStore';

  let searchBtn: HTMLDivElement;
  let searchInput: HTMLInputElement;
  let expanded = false;
  let searchValue = '';

  // Sync with store
  $: searchQuery.set(searchValue);

  // Export focus function so it can be called from parent
  export function focusSearch() {
    expandSearch();
  }

  // Expand search
  function expandSearch() {
    expanded = true;
    setTimeout(() => {
      searchInput?.focus();
    }, 100);
  }

  // Collapse search
  function collapseSearch() {
    expanded = false;
    searchInput?.blur();
  }

  // Handle search button click
  function handleSearchClick(e: MouseEvent) {
    if (!expanded) {
      expandSearch();
    }
  }

  // Handle keyboard activation (Enter/Space)
  function handleKeyPress(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      if (!expanded) {
        expandSearch();
      }
    }
  }

  // Handle escape key
  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      searchValue = '';
      collapseSearch();
    }
  }

  // Handle click outside
  function handleClickOutside(e: MouseEvent) {
    if (expanded && searchBtn && !searchBtn.contains(e.target as Node)) {
      if (searchValue.trim() === '') {
        collapseSearch();
      }
    }
  }

  // Auto-expand on typing
  function handleGlobalKeyDown(e: KeyboardEvent) {
    // Handle Cmd+F / Ctrl+F
    if ((e.metaKey || e.ctrlKey) && e.key === 'f') {
      e.preventDefault();
      expandSearch();
      return;
    }

    // Skip if already expanded or typing in another input
    if (expanded) return;
    const target = e.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'SELECT' || target.tagName === 'TEXTAREA') return;
    if (e.metaKey || e.ctrlKey || e.altKey) return;

    // Check if it's a searchable character
    if (/^[a-zA-Z0-9\s]$/.test(e.key)) {
      expandSearch();
      searchValue = e.key;
      e.preventDefault();
    }
  }

  onMount(() => {
    // Add global event listeners
    document.addEventListener('click', handleClickOutside);
    document.addEventListener('keydown', handleGlobalKeyDown);

    return () => {
      document.removeEventListener('click', handleClickOutside);
      document.removeEventListener('keydown', handleGlobalKeyDown);
    };
  });
</script>

<div class="search-container">
  <div
    class="search-icon-btn tooltip-trigger"
    class:expanded
    bind:this={searchBtn}
    on:click={handleSearchClick}
    on:keypress={handleKeyPress}
    role="button"
    tabindex="0"
  >
    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="11" cy="11" r="7" stroke="rgba(247,228,121,0.85)" stroke-width="2"/>
      <path d="M16 16L21 21" stroke="rgba(247,228,121,0.85)" stroke-width="2" stroke-linecap="round"/>
    </svg>
    <input
      type="text"
      class="search-input"
      bind:this={searchInput}
      bind:value={searchValue}
      on:keydown={handleKeyDown}
      placeholder="Search clipboard..."
    />
    {#if !expanded}
      <MiniTooltip text="Search (Cmd+F)" />
    {/if}
  </div>
</div>

<style>
  /* Reference: preview.html lines 647-753 */
  .search-container {
    display: flex;
    align-items: center;
    position: relative;
    transition: all 0.3s ease;
  }

  .search-icon-btn {
    width: 40px;
    height: 40px;
    background: rgba(40, 40, 40, 0.8);
    border: 1px solid rgba(60, 60, 60, 0.5);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    position: relative;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
  }

  .search-icon-btn:hover {
    background: rgba(60, 60, 60, 0.9);
    border-color: rgba(247, 228, 121, 0.5);
  }

  .search-icon-btn svg {
    width: 20px;
    height: 20px;
    transition: transform 0.3s ease;
  }

  .search-icon-btn:hover svg {
    transform: scale(1.1);
  }

  .search-icon-btn.expanded {
    width: 300px;
    border-radius: 10px;
    justify-content: flex-start;
    padding-left: 12px;
    border-color: rgba(247, 228, 121, 0.6);
  }

  .search-input {
    position: absolute;
    left: 45px;
    width: 240px;
    background: transparent;
    border: none;
    outline: none;
    color: rgba(255, 255, 255, 0.9);
    font-size: 14px;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .search-icon-btn.expanded .search-input {
    opacity: 1;
    pointer-events: all;
  }

  .search-icon-btn.expanded svg {
    transform: scale(0.9);
    filter: drop-shadow(0 0 4px rgba(247, 228, 121, 0.3));
  }

  .search-icon-btn.expanded svg :global(circle),
  .search-icon-btn.expanded svg :global(path) {
    stroke: rgba(247, 228, 121, 1);
  }

  /* Pulse animation for search icon */
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  .search-icon-btn svg :global(circle) {
    animation: pulse 2s ease-in-out infinite;
  }

  .search-icon-btn:hover svg :global(circle),
  .search-icon-btn.expanded svg :global(circle) {
    animation: none;
  }
</style>
