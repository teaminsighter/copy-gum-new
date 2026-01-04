<script lang="ts">
  // Mini Tooltip Component
  // Reference: preview.html lines 236-274

  import { onMount } from 'svelte';

  export let text: string = '';

  let tooltipElement: HTMLDivElement;
  let tooltipStyle = '';

  onMount(() => {
    // Calculate position when tooltip is shown
    const updatePosition = () => {
      if (!tooltipElement) return;

      const trigger = tooltipElement.parentElement;
      if (!trigger) return;

      const triggerRect = trigger.getBoundingClientRect();
      const tooltipRect = tooltipElement.getBoundingClientRect();

      // Calculate position relative to viewport
      const left = triggerRect.left + (triggerRect.width / 2);
      const bottom = window.innerHeight - triggerRect.top + 8;

      tooltipStyle = `left: ${left}px; bottom: ${bottom}px;`;
    };

    // Update position on hover
    const trigger = tooltipElement.parentElement;
    if (trigger) {
      trigger.addEventListener('mouseenter', updatePosition);
      return () => trigger.removeEventListener('mouseenter', updatePosition);
    }
  });
</script>

<div class="mini-tooltip" bind:this={tooltipElement} style={tooltipStyle}>
  {text}
</div>

<style>
  /* Reference: preview.html lines 237-255 */
  .mini-tooltip {
    position: fixed;
    transform: translateX(-50%) translateY(4px);
    background: rgba(20, 20, 20, 0.95);
    border: 1px solid rgba(247, 228, 121, 0.4);
    color: #f7e479;
    padding: 5px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s ease, transform 0.2s ease;
    z-index: 99999;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
  }

  /* Arrow pointing down - Reference: preview.html lines 257-266 */
  .mini-tooltip::after {
    content: '';
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 4px solid transparent;
    border-top-color: rgba(247, 228, 121, 0.4);
  }

  /* Show on parent hover with delay - Reference: preview.html lines 268-274 */
  :global(.tooltip-trigger:hover) .mini-tooltip {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
    transition-delay: 500ms;
  }
</style>
