import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * THEME-TOGGLE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <button data-name="ThemeToggle">                                      │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │                    ┌─────────┐                                  │   │
 * │   │                    │  ☀️/🌙  │  ← Sun or Moon icon              │   │
 * │   │                    │         │                                  │   │
 * │   │                    └─────────┘                                  │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * THEME STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Light Mode:                    Dark Mode:                             │
 * │   ┌─────────────┐                ┌─────────────┐                        │
 * │   │             │                │░░░░░░░░░░░░░│                        │
 * │   │     ☀️      │  ──click──>   │     🌙      │                        │
 * │   │             │                │░░░░░░░░░░░░░│                        │
 * │   └─────────────┘                └─────────────┘                        │
 * │   (light background)             (dark background)                      │
 * │                                                                         │
 * │   document.documentElement.classList.contains("dark")                   │
 * │   false                          true                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ICON TRANSITION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Light theme: Sun icon visible    Dark theme: Moon icon visible        │
 * │   ┌───────────┐                    ┌───────────┐                        │
 * │   │    ☀️     │   ←───toggle───>   │    🌙     │                        │
 * │   │  visible  │                    │  visible  │                        │
 * │   └───────────┘                    └───────────┘                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .size-9         ← 36px size                                │       │
 * │   │  .inline-flex    ← Inline flex display                      │       │
 * │   │  .items-center   ← Center vertically                        │       │
 * │   │  .justify-center ← Center horizontally                      │       │
 * │   │  .rounded-md     ← Medium border radius                     │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ThemeTogglePage extends BasePage {
  protected readonly componentName = "theme-toggle";

  // Theme toggle elements
  readonly themeToggle: Locator;
  readonly sunIcon: Locator;
  readonly moonIcon: Locator;

  constructor(page: Page) {
    super(page);

    // Main theme toggle button - scoped within preview
    this.themeToggle = this.preview.locator('[data-name="ThemeToggle"]').first();

    // Icons - one for light mode, one for dark mode
    this.sunIcon = this.themeToggle.locator("svg").first();
    this.moonIcon = this.themeToggle.locator("svg").last();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("ThemeToggle Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have ThemeToggle button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Page                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │     [ThemeToggle] <── visible? ✓                  │  │
     *   │  │     ┌─────────┐                                   │  │
     *   │  │     │  icon   │                                   │  │
     *   │  │     └─────────┘                                   │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ThemeToggle button is visible on the page
     */
    test("should have ThemeToggle button", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toBeVisible();
    });

    /**
     * TEST: Should have ThemeToggle data-name attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <button data-name="ThemeToggle"> <── attribute check   │
     *   │       ↓                                                 │
     *   │  data-name === "ThemeToggle" ✓                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ThemeToggle has correct data-name attribute
     */
    test("should have ThemeToggle data-name attribute", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveAttribute("data-name", "ThemeToggle");
    });

    /**
     * TEST: Should be a button element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]                                          │
     *   │       ↓                                                 │
     *   │  tagName.toLowerCase() === "button" ✓                   │
     *   │                                                         │
     *   │  <button>  ← semantic HTML element                      │
     *   │    └─ icon                                              │
     *   │  </button>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ThemeToggle uses semantic button element
     */
    test("should be a button element", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      const tagName = await ui.themeToggle.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });

    /**
     * TEST: Should have icon(s)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]                                          │
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │                                                 │    │
     *   │  │     [svg] ← count >= 1 ✓                        │    │
     *   │  │      ☀️  or  🌙                                  │    │
     *   │  │                                                 │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ThemeToggle contains at least one SVG icon
     */
    test("should have icon(s)", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      const icons = ui.themeToggle.locator("svg");
      const count = await icons.count();
      expect(count).toBeGreaterThanOrEqual(1);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Toggle should have size classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]  class="... size-9 ..."                  │
     *   │                                                         │
     *   │       ┌─────────┐                                       │
     *   │       │         │  ← 36px x 36px (size-9)               │
     *   │       │  icon   │                                       │
     *   │       │         │                                       │
     *   │       └─────────┘                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle button has correct size class
     */
    test("toggle should have size classes", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveClass(/size-9/);
    });

    /**
     * TEST: Toggle should have inline-flex
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]  class="... inline-flex ..."             │
     *   │                                                         │
     *   │       ┌─────────┐                                       │
     *   │       │  icon   │  ← display: inline-flex               │
     *   │       └─────────┘                                       │
     *   │                                                         │
     *   │  (inline element with flex capabilities)                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle uses inline-flex display
     */
    test("toggle should have inline-flex", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveClass(/inline-flex/);
    });

    /**
     * TEST: Toggle should have items-center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]  class="... items-center ..."            │
     *   │                                                         │
     *   │       ┌─────────┐                                       │
     *   │       │    ·    │                                       │
     *   │       │  icon ──│── vertically centered ✓               │
     *   │       │    ·    │                                       │
     *   │       └─────────┘                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is vertically centered inside button
     */
    test("toggle should have items-center", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveClass(/items-center/);
    });

    /**
     * TEST: Toggle should have justify-center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]  class="... justify-center ..."          │
     *   │                                                         │
     *   │       ┌─────────┐                                       │
     *   │       │  ·  ·   │                                       │
     *   │       │  icon   │── horizontally centered ✓             │
     *   │       │  ·  ·   │                                       │
     *   │       └─────────┘                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is horizontally centered inside button
     */
    test("toggle should have justify-center", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveClass(/justify-center/);
    });

    /**
     * TEST: Toggle should have rounded corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]  class="... rounded-md ..."              │
     *   │                                                         │
     *   │       ╭─────────╮                                       │
     *   │       │         │  ← border-radius: medium              │
     *   │       │  icon   │                                       │
     *   │       │         │                                       │
     *   │       ╰─────────╯                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle has medium rounded corners
     */
    test("toggle should have rounded corners", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveClass(/rounded-md/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Toggle should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]                                          │
     *   │       ┌─────────┐                                       │
     *   │       │  icon   │  ← click                              │
     *   │       └─────────┘                                       │
     *   │            ↓                                            │
     *   │     no error thrown ✓                                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle button responds to click events
     */
    test("toggle should be clickable", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await ui.themeToggle.click();
      // Should not throw error
    });

    /**
     * TEST: Toggle should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ThemeToggle]                                          │
     *   │       ┌─────────┐                                       │
     *   │       │  icon   │  ← focus()                            │
     *   │       └─────────┘                                       │
     *   │            ↓                                            │
     *   │     isFocused() === true ✓                              │
     *   │                                                         │
     *   │  (keyboard navigation support)                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle can receive keyboard focus
     */
    test("toggle should be focusable", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await ui.themeToggle.focus();
      await expect(ui.themeToggle).toBeFocused();
    });

    /**
     * TEST: Clicking toggle should change theme
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  BEFORE:                 AFTER:                         │
     *   │  ┌───────────────┐       ┌───────────────┐              │
     *   │  │               │       │░░░░░░░░░░░░░░░│              │
     *   │  │     ☀️        │ click │     🌙        │              │
     *   │  │               │ ───→  │░░░░░░░░░░░░░░░│              │
     *   │  └───────────────┘       └───────────────┘              │
     *   │  (light theme)           (dark theme)                   │
     *   │                                                         │
     *   │  document.documentElement.classList.contains("dark")    │
     *   │  false          ───→     true  (or vice versa) ✓        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking toggle switches between light/dark themes
     */
    test("clicking toggle should change theme", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      // Get initial theme
      const initialTheme = await page.evaluate(() =>
        document.documentElement.classList.contains("dark")
      );

      // Click toggle
      await ui.themeToggle.click();

      // Wait for theme to change
      await page.waitForTimeout(100);

      // Check theme changed
      const newTheme = await page.evaluate(() =>
        document.documentElement.classList.contains("dark")
      );

      expect(newTheme).not.toBe(initialTheme);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Should have aria-label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <button aria-label="Toggle theme"> <── attribute check │
     *   │       ↓                                                 │
     *   │  aria-label is truthy ✓                                 │
     *   │                                                         │
     *   │  Screen reader:                                         │
     *   │  "Toggle theme, button"                                 │
     *   │                                                         │
     *   │  (accessible name for assistive technology)             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle has aria-label for screen readers
     */
    test("should have aria-label", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      const ariaLabel = await ui.themeToggle.getAttribute("aria-label");
      expect(ariaLabel).toBeTruthy();
    });

    /**
     * TEST: Should have type=button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <button type="button"> <── attribute check             │
     *   │       ↓                                                 │
     *   │  type === "button" ✓                                    │
     *   │                                                         │
     *   │  (prevents form submission in form contexts)            │
     *   │  (explicit button behavior)                             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toggle has explicit type="button" attribute
     */
    test("should have type=button", async ({ page }) => {
      const ui = new ThemeTogglePage(page);
      await ui.goto();

      await expect(ui.themeToggle).toHaveAttribute("type", "button");
    });
  });
});
