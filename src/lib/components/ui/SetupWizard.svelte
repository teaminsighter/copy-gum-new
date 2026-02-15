<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { updateSetting } from '../../stores/settingsStore';
  import { showSuccess } from '../../stores/toastStore';

  export let show = false;
  export let onComplete: () => void = () => {};

  let currentStep = 0;

  // Detect platform
  const isMac = /Mac|iPhone|iPad|iPod/i.test(navigator.userAgent);

  const steps = [
    { id: 'welcome', title: 'Welcome' },
    { id: 'shortcut', title: 'Shortcut' },
    { id: 'ready', title: 'Done' }
  ];

  function nextStep() {
    if (currentStep < steps.length - 1) {
      currentStep++;
    }
  }

  function prevStep() {
    if (currentStep > 0) {
      currentStep--;
    }
  }

  async function completeSetup() {
    try {
      await updateSetting('hasShownOverlayInfo', true);
      showSuccess('Setup complete!');
      show = false;
      onComplete();
    } catch (err) {
      console.error(err);
      show = false;
    }
  }

  async function skipSetup() {
    try {
      await updateSetting('hasShownOverlayInfo', true);
    } catch (err) {
      console.error(err);
    }
    show = false;
    onComplete();
  }
</script>

{#if show}
  <div class="wizard-backdrop" transition:fade={{ duration: 150 }}>
    <div class="wizard-container" transition:scale={{ duration: 150, start: 0.95 }}>
      <!-- Navigation at TOP (so it's above the Dock) -->
      <div class="wizard-nav">
        {#if currentStep > 0}
          <button class="nav-btn secondary" on:click={prevStep}>Back</button>
        {:else}
          <button class="nav-btn text" on:click={skipSetup}>Skip</button>
        {/if}

        <!-- Progress dots in center -->
        <div class="wizard-progress">
          {#each steps as step, i}
            <button
              class="progress-dot"
              class:active={i === currentStep}
              class:completed={i < currentStep}
              on:click={() => currentStep = i}
              aria-label="Go to {step.title} step"
            ></button>
          {/each}
        </div>

        {#if currentStep < steps.length - 1}
          <button class="nav-btn primary" on:click={nextStep}>Next</button>
        {:else}
          <button class="nav-btn primary" on:click={completeSetup}>Get Started</button>
        {/if}
      </div>

      <!-- Content -->
      <div class="wizard-content">
        {#if currentStep === 0}
          <div class="step" transition:fade={{ duration: 100 }}>
            <div class="icon">
              <svg viewBox="0 0 32 32" fill="none">
                <rect x="4" y="6" width="18" height="18" rx="3" fill="#f7e479" opacity="0.3"/>
                <rect x="10" y="8" width="18" height="18" rx="3" fill="#f7e479"/>
              </svg>
            </div>
            <h2>Welcome to CopyGum</h2>
            <p>Your smart clipboard manager. Access your copy history instantly.</p>
          </div>
        {:else if currentStep === 1}
          <div class="step" transition:fade={{ duration: 100 }}>
            <div class="shortcut-box">
              <kbd>{isMac ? 'Cmd' : 'Ctrl'}</kbd>
              <span>+</span>
              <kbd>Shift</kbd>
              <span>+</span>
              <kbd>V</kbd>
            </div>
            <p>Press this shortcut anywhere to toggle CopyGum</p>
            <div class="hint">
              {#if isMac}
                Grant accessibility permission if prompted
              {:else}
                Works anywhere, no extra permissions needed
              {/if}
            </div>
          </div>
        {:else}
          <div class="step" transition:fade={{ duration: 100 }}>
            <div class="icon ready">
              <svg viewBox="0 0 32 32" fill="none">
                <circle cx="16" cy="16" r="14" fill="#f7e479"/>
                <path d="M10 16l4 4 8-8" stroke="#000" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h2>You're all set!</h2>
            <p>Start copying - CopyGum remembers everything.</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .wizard-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
  }

  .wizard-container {
    position: relative;
    background: linear-gradient(135deg, #1e1e26 0%, #16161c 100%);
    border: 1px solid rgba(247, 228, 121, 0.3);
    border-radius: 16px;
    width: 340px;
    padding: 20px 24px 24px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
  }

  /* Navigation at TOP */
  .wizard-nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .wizard-progress {
    display: flex;
    justify-content: center;
    gap: 8px;
  }

  .progress-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    border: none;
    padding: 0;
    cursor: pointer;
    transition: all 0.2s;
  }

  .progress-dot.active {
    background: #f7e479;
    transform: scale(1.2);
  }

  .progress-dot.completed {
    background: rgba(247, 228, 121, 0.5);
  }

  .wizard-content {
    min-height: 180px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .step {
    text-align: center;
    width: 100%;
  }

  .icon {
    width: 56px;
    height: 56px;
    margin: 0 auto 16px;
  }

  .icon svg {
    width: 100%;
    height: 100%;
  }

  h2 {
    font-size: 20px;
    font-weight: 700;
    color: #fff;
    margin: 0 0 8px;
  }

  p {
    color: rgba(255, 255, 255, 0.7);
    font-size: 14px;
    line-height: 1.5;
    margin: 0;
  }

  .shortcut-box {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    margin-bottom: 16px;
  }

  .shortcut-box kbd {
    padding: 8px 14px;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(247, 228, 121, 0.4);
    border-radius: 6px;
    color: #f7e479;
    font-family: system-ui, sans-serif;
    font-size: 14px;
    font-weight: 600;
  }

  .shortcut-box span {
    color: rgba(255, 255, 255, 0.4);
    font-size: 16px;
  }

  .hint {
    margin-top: 12px;
    padding: 10px 14px;
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 8px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.7);
  }

  .nav-btn {
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
    min-width: 70px;
  }

  .nav-btn.primary {
    background: #f7e479;
    color: #000;
  }

  .nav-btn.primary:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(247, 228, 121, 0.3);
  }

  .nav-btn.secondary {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
  }

  .nav-btn.secondary:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .nav-btn.text {
    background: transparent;
    color: rgba(255, 255, 255, 0.5);
  }

  .nav-btn.text:hover {
    color: rgba(255, 255, 255, 0.8);
  }
</style>
