<script lang="ts">
  // CopyGum Logo/Branding
  // Reference: preview.html lines 2018-2025
</script>

<div class="logo-section">
  <div class="brand-name">
    <span class="brand-letter">C</span>
    <span class="brand-text">opy</span>
    <span class="brand-letter">G</span>
    <span class="brand-text">um</span>
  </div>
</div>

<style>
  .logo-section {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .brand-name {
    font-size: 20px;
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 2px;
    user-select: none;
    transition: transform 0.3s ease;
  }

  /* Bounce animation on hover - Reference: preview.html lines 633-638 */
  .logo-section:hover .brand-name {
    animation: bounce 0.6s ease-in-out;
  }

  @keyframes bounce {
    0%, 100% { transform: translateY(0) scale(1); }
    25% { transform: translateY(-5px) scale(1.1); }
    50% { transform: translateY(-2px) scale(1.05); }
    75% { transform: translateY(-4px) scale(1.08); }
  }

  .brand-letter {
    color: #f7e479;
    font-size: 24px;
  }

  .brand-text {
    color: rgba(255, 255, 255, 0.9);
  }
</style>
