import { test, expect } from '@playwright/test';

test.describe('Responsive Layout Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the app
    await page.goto('/');
    // Wait for app to fully load
    await page.waitForLoadState('networkidle');
    // Give Svelte time to mount
    await page.waitForTimeout(500);
  });

  test('Full panel screenshot', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const viewport = page.viewportSize();

    // Take full page screenshot
    await page.screenshot({
      path: `screenshots/${projectName}-full.png`,
      fullPage: true,
    });

    // Log viewport info
    console.log(`[${projectName}] Viewport: ${viewport?.width}x${viewport?.height}`);

    // Basic visibility checks
    const body = page.locator('body');
    await expect(body).toBeVisible();

    // Check if main app container exists
    const app = page.locator('#app');
    await expect(app).toBeVisible();
  });

  test('Panel layout measurements', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const viewport = page.viewportSize();

    // Try to find the overlay panel
    const overlayPanel = page.locator('.overlay-panel');

    if (await overlayPanel.isVisible()) {
      const panelBox = await overlayPanel.boundingBox();

      if (panelBox) {
        console.log(`[${projectName}] Panel dimensions: ${panelBox.width}x${panelBox.height}`);
        console.log(`[${projectName}] Panel position: (${panelBox.x}, ${panelBox.y})`);

        // Check if panel fits within viewport
        const fitsWidth = panelBox.width <= (viewport?.width ?? 0);
        const fitsHeight = panelBox.height <= (viewport?.height ?? 0);

        console.log(`[${projectName}] Fits width: ${fitsWidth}, Fits height: ${fitsHeight}`);

        // Take panel-specific screenshot
        await overlayPanel.screenshot({
          path: `screenshots/${projectName}-panel.png`,
        });

        // Assert panel doesn't overflow
        expect(panelBox.x).toBeGreaterThanOrEqual(0);
        expect(panelBox.y).toBeGreaterThanOrEqual(0);
        expect(panelBox.x + panelBox.width).toBeLessThanOrEqual(viewport?.width ?? 0);
        expect(panelBox.y + panelBox.height).toBeLessThanOrEqual(viewport?.height ?? 0);
      }
    }
  });

  test('Header layout check', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;

    // Check header elements
    const panelHeader = page.locator('.panel-header');

    if (await panelHeader.isVisible()) {
      const headerBox = await panelHeader.boundingBox();

      if (headerBox) {
        console.log(`[${projectName}] Header dimensions: ${headerBox.width}x${headerBox.height}`);

        // Screenshot header section
        await panelHeader.screenshot({
          path: `screenshots/${projectName}-header.png`,
        });

        // Check header height is reasonable (should be ~70px based on CSS)
        expect(headerBox.height).toBeGreaterThanOrEqual(50);
        expect(headerBox.height).toBeLessThanOrEqual(100);
      }
    }
  });

  test('Content area check', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;

    // Check content area
    const contentArea = page.locator('.content-area');

    if (await contentArea.isVisible()) {
      const contentBox = await contentArea.boundingBox();

      if (contentBox) {
        console.log(`[${projectName}] Content area: ${contentBox.width}x${contentBox.height}`);

        // Screenshot content area
        await contentArea.screenshot({
          path: `screenshots/${projectName}-content.png`,
        });
      }
    }
  });

  test('Cards container check', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;

    // Check if cards container exists and is scrollable
    const cardsContainer = page.locator('.cards-container');

    if (await cardsContainer.isVisible()) {
      const containerBox = await cardsContainer.boundingBox();

      if (containerBox) {
        console.log(`[${projectName}] Cards container: ${containerBox.width}x${containerBox.height}`);

        // Check for any card elements
        const cards = page.locator('.clipboard-card');
        const cardCount = await cards.count();
        console.log(`[${projectName}] Card count: ${cardCount}`);

        // Screenshot cards area
        await cardsContainer.screenshot({
          path: `screenshots/${projectName}-cards.png`,
        });
      }
    }
  });

  test('Categories section check', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;

    // Check categories
    const categoriesSection = page.locator('.categories-section, .categories-container');

    if (await categoriesSection.first().isVisible()) {
      const catBox = await categoriesSection.first().boundingBox();

      if (catBox) {
        console.log(`[${projectName}] Categories: ${catBox.width}x${catBox.height}`);

        // Count category pills
        const pills = page.locator('.category-pill');
        const pillCount = await pills.count();
        console.log(`[${projectName}] Category pill count: ${pillCount}`);

        // Screenshot categories
        await categoriesSection.first().screenshot({
          path: `screenshots/${projectName}-categories.png`,
        });
      }
    }
  });

  test('Check for overflow issues', async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const viewport = page.viewportSize();

    // Check if any element is overflowing the viewport
    const overflowElements = await page.evaluate(() => {
      const results: string[] = [];
      const elements = document.querySelectorAll('*');

      elements.forEach((el) => {
        const rect = el.getBoundingClientRect();
        const style = window.getComputedStyle(el);

        // Check if element extends beyond viewport
        if (rect.right > window.innerWidth + 5) {
          results.push(`${el.tagName}.${el.className} overflows right by ${rect.right - window.innerWidth}px`);
        }
        if (rect.bottom > window.innerHeight + 5 && style.position !== 'fixed') {
          // Ignore fixed positioned elements for bottom overflow
        }
        if (rect.left < -5) {
          results.push(`${el.tagName}.${el.className} overflows left by ${Math.abs(rect.left)}px`);
        }
      });

      return results;
    });

    if (overflowElements.length > 0) {
      console.log(`[${projectName}] OVERFLOW ISSUES DETECTED:`);
      overflowElements.forEach(issue => console.log(`  - ${issue}`));
    } else {
      console.log(`[${projectName}] No overflow issues detected`);
    }
  });
});
