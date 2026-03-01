<script lang="ts">
  // Main Overlay Panel Container
  // Reference: preview.html lines 31-43, 2014-2657
</script>

<div class="overlay-panel">
  <div class="panel-header">
    <slot name="header" />
  </div>

  <div class="content-area">
    <slot name="content" />
  </div>
</div>

<style>
  /* Reference: preview.html lines 31-43 */
  .overlay-panel {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    max-width: 100vw;
    height: 100vh;
    background: var(--panel-bg-color, #0a0a0a);
    box-shadow: 0 -10px 40px rgba(0, 0, 0, 0.5);
    overflow: visible;
    box-sizing: border-box;
  }


  /* Reference: preview.html lines 45-60 */
  .panel-header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    width: 100%;
    max-width: 100%;
    height: var(--header-height, 70px);
    background: transparent;
    padding: var(--header-padding-y, 15px) var(--header-padding-x, 20px);
    display: flex;
    align-items: center;
    gap: var(--header-gap, 12px);
    z-index: 10;
    box-sizing: border-box;
    overflow: hidden;
  }

  .content-area {
    position: absolute;
    top: var(--header-height, 70px);
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;
  }
</style>
