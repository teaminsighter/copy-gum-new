<script lang="ts">
  // Skeleton Card - Loading placeholder
  // Animated shimmer effect while content loads
</script>

<div class="skeleton-card">
  <div class="glass-overlay"></div>
  <div class="glass-content">
    <!-- Header -->
    <div class="skeleton-header">
      <div class="skeleton-icon"></div>
      <div class="skeleton-text skeleton-text-short"></div>
    </div>

    <!-- Content -->
    <div class="skeleton-body">
      <div class="skeleton-text skeleton-text-long"></div>
      <div class="skeleton-text skeleton-text-medium"></div>
      <div class="skeleton-text skeleton-text-short"></div>
    </div>

    <!-- Footer -->
    <div class="skeleton-footer">
      <div class="skeleton-tag"></div>
      <div class="skeleton-tag"></div>
    </div>
  </div>
</div>

<style>
  .skeleton-card {
    flex-shrink: 0;
    width: 288px;
    height: 95%;
    border-radius: 12px;
    position: relative;
    overflow: hidden;
  }

  .glass-overlay {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    background: rgba(30, 30, 30, 0.4);
    border: 1px solid rgba(60, 60, 60, 0.5);
    backdrop-filter: blur(20px);
  }

  .glass-content {
    position: relative;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 16px;
    gap: 16px;
  }

  .skeleton-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .skeleton-icon {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
    animation: shimmer 2s infinite;
  }

  .skeleton-text {
    height: 12px;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.1);
    animation: shimmer 2s infinite;
  }

  .skeleton-text-short {
    width: 60px;
  }

  .skeleton-text-medium {
    width: 120px;
  }

  .skeleton-text-long {
    width: 100%;
  }

  .skeleton-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .skeleton-footer {
    display: flex;
    gap: 8px;
  }

  .skeleton-tag {
    width: 60px;
    height: 24px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.1);
    animation: shimmer 2s infinite;
  }

  @keyframes shimmer {
    0% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
    100% {
      opacity: 0.6;
    }
  }
</style>
