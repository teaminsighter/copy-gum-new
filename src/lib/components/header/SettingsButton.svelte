<script lang="ts">
  // Settings Button Component
  // Reference: preview.html (settings icon button)

  export let onClick: () => void = () => {};
</script>

<button class="icon-btn" on:click={onClick}>
  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" />
    <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
  </svg>
</button>

<style>
  /* Reference: preview.html lines 95-130 */
  .icon-btn {
    width: 40px;
    height: 40px;
    background: rgba(40, 40, 40, 0.8);
    border: 1px solid rgba(60, 60, 60, 0.5);
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    overflow: visible;
    padding: 0;
  }

  .icon-btn:hover {
    background: rgba(60, 60, 60, 0.9);
    border-color: rgba(247, 228, 121, 0.5);
    transform: translateY(-2px);
  }

  .icon-btn svg {
    width: 20px;
    height: 20px;
    stroke: rgba(247, 228, 121, 0.85);
    transition: all 0.2s;
  }

  .icon-btn:hover svg {
    stroke: rgba(247, 228, 121, 1);
    transform: scale(1.1);
  }

  .icon-btn:active {
    transform: translateY(0);
  }
</style>
