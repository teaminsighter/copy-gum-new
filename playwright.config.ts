import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
    screenshot: 'on',
  },

  projects: [
    // Small laptops
    {
      name: 'laptop-small-1366x768',
      use: {
        viewport: { width: 1366, height: 768 },
        deviceScaleFactor: 1,
      },
    },
    // MacBook Air 13"
    {
      name: 'macbook-air-13-1440x900',
      use: {
        viewport: { width: 1440, height: 900 },
        deviceScaleFactor: 2,
      },
    },
    // Full HD 1080p
    {
      name: 'fullhd-1920x1080',
      use: {
        viewport: { width: 1920, height: 1080 },
        deviceScaleFactor: 1,
      },
    },
    // Windows 125% scaling on 1080p
    {
      name: 'windows-125-scaling',
      use: {
        viewport: { width: 1536, height: 864 },
        deviceScaleFactor: 1.25,
      },
    },
    // Windows 150% scaling on 1080p
    {
      name: 'windows-150-scaling',
      use: {
        viewport: { width: 1280, height: 720 },
        deviceScaleFactor: 1.5,
      },
    },
    // QHD 1440p
    {
      name: 'qhd-2560x1440',
      use: {
        viewport: { width: 2560, height: 1440 },
        deviceScaleFactor: 1,
      },
    },
    // 4K UHD
    {
      name: '4k-3840x2160',
      use: {
        viewport: { width: 3840, height: 2160 },
        deviceScaleFactor: 1,
      },
    },
    // MacBook Pro 14" (actual resolution)
    {
      name: 'macbook-pro-14',
      use: {
        viewport: { width: 1512, height: 982 },
        deviceScaleFactor: 2,
      },
    },
    // MacBook Pro 16"
    {
      name: 'macbook-pro-16',
      use: {
        viewport: { width: 1728, height: 1117 },
        deviceScaleFactor: 2,
      },
    },
    // Small Windows laptop
    {
      name: 'windows-laptop-small',
      use: {
        viewport: { width: 1280, height: 800 },
        deviceScaleFactor: 1,
      },
    },
  ],

  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: true,
    timeout: 30000,
  },
});
