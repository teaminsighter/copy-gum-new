<script lang="ts">
  // Reusable Glassmorphism Effect Component
  // Reference: preview.html lines 338-369 (glass layer structure)

  export let background: string = 'rgba(50, 50, 50, 0.6)';
  export let borderColor: string = 'rgba(90, 90, 90, 0.3)';
  export let blurAmount: string = '6px';
</script>

<div class="glass-wrapper">
  <!-- Layer 1: Backdrop filter with lens distortion -->
  <div class="glass-filter" style="backdrop-filter: blur({blurAmount});"></div>

  <!-- Layer 2: Background color and border -->
  <div
    class="glass-overlay"
    style="background: {background}; border-color: {borderColor};"
  ></div>

  <!-- Layer 3: Specular highlights -->
  <div class="glass-specular"></div>

  <!-- Layer 4: Content (highest z-index) -->
  <div class="glass-content">
    <slot />
  </div>
</div>

<style>
  /* Reference: preview.html lines 338-369 */
  .glass-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .glass-filter,
  .glass-overlay,
  .glass-specular {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    pointer-events: none;
    overflow: hidden;
  }

  /* Layer 1: Backdrop blur + lens filter */
  .glass-filter {
    z-index: 0;
    filter: url(#lensFilter) saturate(115%) brightness(1.1);
  }

  /* Layer 2: Background + border */
  .glass-overlay {
    z-index: 1;
    border: 1px solid;
    transition: all 0.2s ease;
  }

  /* Layer 3: Specular highlights (shiny glass effect) */
  .glass-specular {
    z-index: 2;
    box-shadow:
      inset 1px 1px 0 rgba(255, 255, 255, 0.15),
      inset 0 0 10px rgba(255, 255, 255, 0.05);
  }

  /* Layer 4: Content */
  .glass-content {
    position: relative;
    z-index: 3;
    width: 100%;
    height: 100%;
  }
</style>
