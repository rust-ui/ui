import { expect, Locator, Page, test } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * BUTTON COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <button> or <a> (when href provided)                                  │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │                                                             │       │
 * │   │   [Icon?]  "Button Text"  [Icon?]                           │       │
 * │   │                                                             │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌───────────────┐  Default     → bg-primary text-primary-foreground   │
 * │   │   Default     │                                                     │
 * │   └───────────────┘                                                     │
 * │                                                                         │
 * │   ┌───────────────┐  Secondary   → bg-secondary text-secondary-fg       │
 * │   │  Secondary    │                                                     │
 * │   └───────────────┘                                                     │
 * │                                                                         │
 * │   ┌───────────────┐  Outline     → border bg-background                 │
 * │   │   Outline     │                                                     │
 * │   └───────────────┘                                                     │
 * │                                                                         │
 * │   ┌───────────────┐  Ghost       → hover:bg-accent (transparent bg)     │
 * │   │    Ghost      │                                                     │
 * │   └───────────────┘                                                     │
 * │                                                                         │
 * │   ┌───────────────┐  Destructive → bg-destructive text-destructive-fg   │
 * │   │ Destructive   │                                                     │
 * │   └───────────────┘                                                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SIZES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Small (h-8):    ┌─────────┐                                           │
 * │                   │  Small  │                                           │
 * │                   └─────────┘                                           │
 * │                                                                         │
 * │   Default (h-9):  ┌───────────┐                                         │
 * │                   │  Default  │                                         │
 * │                   └───────────┘                                         │
 * │                                                                         │
 * │   Large (h-10):   ┌─────────────┐                                       │
 * │                   │    Large    │                                       │
 * │                   └─────────────┘                                       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Normal          Hover           Active          Disabled              │
 * │   ┌────────┐      ┌────────┐      ┌────────┐      ┌────────┐            │
 * │   │ Button │  →   │ Button │  →   │ Button │      │ Button │            │
 * │   └────────┘      └────────┘      └────────┘      └────────┘            │
 * │                   hover:bg-*      scale-[0.98]    opacity-50            │
 * │                                                   cursor-not-allowed    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ButtonPage extends BasePage {
  protected readonly componentName = "button";

  // Demo: Basic button
  readonly basicButton: Locator;

  // Demo: Reactive button
  readonly reactiveButton: Locator;

  // Demo: Variants
  readonly defaultButton: Locator;
  readonly secondaryButton: Locator;
  readonly outlineButton: Locator;
  readonly ghostButton: Locator;
  readonly destructiveButton: Locator;

  // Demo: Sizes
  readonly smallButton: Locator;
  readonly largeButton: Locator;

  // Demo: Disabled
  readonly disabledButton: Locator;

  // Demo: Href
  readonly dashboardLink: Locator;
  readonly externalLink: Locator;

  constructor(page: Page) {
    super(page);

    // Basic button (first "Button" on page) - scoped within preview
    this.basicButton = this.preview.getByRole("button", { name: "Button" }).first();

    // Reactive button - scoped within preview
    this.reactiveButton = this.preview
      .getByRole("button", { name: /Click Me:/ })
      .first();

    // Variants - scoped within preview
    this.defaultButton = this.preview
      .getByRole("button", { name: "Default", exact: true })
      .first();
    this.secondaryButton = this.preview
      .getByRole("button", { name: "Secondary" })
      .first();
    this.outlineButton = this.preview.getByRole("button", { name: "Outline" }).first();
    this.ghostButton = this.preview.getByRole("button", { name: "Ghost" }).first();
    this.destructiveButton = this.preview
      .getByRole("button", { name: "Destructive" })
      .first();

    // Sizes - scoped within preview
    this.smallButton = this.preview.getByRole("button", { name: "Small" });
    this.largeButton = this.preview.getByRole("button", { name: "Large" });

    // Disabled - scoped within preview
    this.disabledButton = this.preview.getByRole("button", {
      name: "Disabled",
      exact: true,
    });

    // Href - scoped within preview
    this.dashboardLink = this.preview
      .getByRole("link", { name: "Go to Dashboard" })
      .first();
    this.externalLink = this.preview.getByRole("link", { name: "External Link" });
  }

  /**
   * Click reactive button and wait for text to change.
   * Handles SSR hydration - retries if component isn't yet interactive.
   */
  async clickReactiveAndWaitFor(expectedText: string, maxRetries = 10) {
    for (let i = 0; i < maxRetries; i++) {
      const currentText = await this.reactiveButton.textContent();
      await this.reactiveButton.click();
      await this.page.waitForTimeout(100);

      const newText = await this.reactiveButton.textContent();
      if (newText?.includes(expectedText)) {
        return;
      }

      if (currentText === newText && i < maxRetries - 1) {
        await this.page.waitForTimeout(200);
        continue;
      }
    }

    await expect(this.reactiveButton).toContainText(expectedText);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Button Page", () => {
  /**
   * REACTIVE BUTTON TESTS - Click counter behavior
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────┐     ┌─────────────────────┐           │
   * │   │   Click Me: 0   🖱️  │ ──► │   Click Me: 1       │           │
   * │   └─────────────────────┘     └─────────────────────┘           │
   * │                                         │                       │
   * │                                         🖱️                      │
   * │                                         ▼                       │
   * │                               ┌─────────────────────┐           │
   * │                               │   Click Me: 2       │           │
   * │                               └─────────────────────┘           │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Reactive Button", () => {
    /**
     * TEST: Initial Count Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │         Click Me: 0                 │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                    ↑                                    │
     *   │           Initial state: count = 0                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button displays initial count of 0 on page load
     */
    test("should display initial count of 0", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("reactive-button");

      await expect(ui.reactiveButton).toContainText("Click Me: 0");
    });

    /**
     * TEST: Click Counter Increments
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────┐  🖱️   ┌───────────────────┐     │
     *   │   │   Click Me: 0    │ ────► │   Click Me: 1    │     │
     *   │   └───────────────────┘       └───────────────────┘     │
     *   │                                        │                │
     *   │                                       🖱️                │
     *   │                                        ▼                │
     *   │                               ┌───────────────────┐     │
     *   │                               │   Click Me: 2    │     │
     *   │                               └───────────────────┘     │
     *   │                                        │                │
     *   │                                       🖱️                │
     *   │                                        ▼                │
     *   │                               ┌───────────────────┐     │
     *   │                               │   Click Me: 3    │     │
     *   │                               └───────────────────┘     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each click increments the counter by 1
     */
    test("should increment count on click", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("reactive-button");

      await ui.clickReactiveAndWaitFor("Click Me: 1");
      await ui.clickReactiveAndWaitFor("Click Me: 2");
      await ui.clickReactiveAndWaitFor("Click Me: 3");
    });

    /**
     * TEST: Rapid Multiple Clicks
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   🖱️🖱️🖱️🖱️🖱️  (5 rapid clicks)                           │
     *   │        │                                                │
     *   │        ▼                                                │
     *   │   ┌───────────────────┐                                 │
     *   │   │   Click Me: 0    │ → 1 → 2 → 3 → 4 → 5             │
     *   │   └───────────────────┘                                 │
     *   │                                                         │
     *   │   Each click must register correctly, no race conditions│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Rapid clicks are handled correctly without losing count
     */
    test("should handle multiple rapid clicks", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("reactive-button");

      for (let i = 1; i <= 5; i++) {
        await ui.clickReactiveAndWaitFor(`Click Me: ${i}`);
      }
    });
  });

  /**
   * ACTIVE STATE TESTS - Press/click feedback
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Normal State          Active State (mousedown)                │
   * │   ┌─────────────┐       ┌───────────┐                           │
   * │   │   Button    │  ──►  │  Button   │  scale-[0.98]             │
   * │   └─────────────┘       └───────────┘                           │
   * │   scale: 1              scale: 0.98                             │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Active State", () => {
    /**
     * TEST: Active State Scale Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Normal State           Active State (mousedown)       │
     *   │   ┌─────────────┐       ┌───────────┐                   │
     *   │   │   Button    │  ──►  │  Button   │                   │
     *   │   └─────────────┘       └───────────┘                   │
     *   │   scale: 1              scale: 0.98                     │
     *   │                                                         │
     *   │   CSS class check: active:scale-[0.98] must be present  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has correct CSS class for press feedback
     */
    test("should have active:scale-[0.98] class", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto();

      const hasActiveScale = await ui.basicButton.evaluate((el) =>
        el.className.includes("active:scale-[0.98]"),
      );
      expect(hasActiveScale).toBe(true);
    });
  });

  /**
   * DISABLED STATE TESTS - Non-interactive button
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────┐                                       │
   * │   │     Disabled        │  ← opacity-50, cursor-not-allowed     │
   * │   └─────────────────────┘    pointer-events-none                │
   * │                                                                 │
   * │   • Cannot be clicked                                           │
   * │   • Cannot receive focus                                        │
   * │   • Visually muted (50% opacity)                                │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Disabled State", () => {
    /**
     * TEST: Disabled Button State and Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────┐                               │
     *   │   │     Disabled        │  opacity: 50%                 │
     *   │   └─────────────────────┘  cursor: not-allowed          │
     *   │            ↑                                            │
     *   │      disabled=true                                      │
     *   │      disabled:opacity-50 class applied                  │
     *   │                                                         │
     *   │   Button must:                                          │
     *   │   • Have disabled attribute                             │
     *   │   • Have disabled:opacity-50 class                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled button has correct attribute and styling
     */
    test("should be disabled with correct classes", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("disabled");

      await expect(ui.disabledButton).toBeDisabled();

      const hasDisabledClass = await ui.disabledButton.evaluate((el) =>
        el.className.includes("disabled:opacity-50"),
      );
      expect(hasDisabledClass).toBe(true);
    });
  });

  /**
   * VARIANTS TESTS - Different visual styles
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌───────────┐ ┌───────────┐ ┌───────────┐                     │
   * │   │  Default  │ │ Secondary │ │  Outline  │                     │
   * │   └───────────┘ └───────────┘ └───────────┘                     │
   * │   bg-primary    bg-secondary  border                            │
   * │                                                                 │
   * │   ┌───────────┐ ┌─────────────┐                                 │
   * │   │   Ghost   │ │ Destructive │                                 │
   * │   └───────────┘ └─────────────┘                                 │
   * │   hover:bg-*    bg-destructive                                  │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Variants", () => {
    /**
     * TEST: Button Variant CSS Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────┐  Default     → bg-primary class         │
     *   │   │  Default  │                                         │
     *   │   └───────────┘                                         │
     *   │   ┌───────────┐  Secondary   → bg-secondary class       │
     *   │   │ Secondary │                                         │
     *   │   └───────────┘                                         │
     *   │   ┌───────────┐  Outline     → border class             │
     *   │   │  Outline  │                                         │
     *   │   └───────────┘                                         │
     *   │   ┌───────────┐  Ghost       → hover:bg-accent class    │
     *   │   │   Ghost   │                                         │
     *   │   └───────────┘                                         │
     *   │   ┌───────────┐  Destructive → bg-destructive class     │
     *   │   │Destructive│                                         │
     *   │   └───────────┘                                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each variant button has its correct CSS class
     */
    test("should have correct variant classes", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("variants");

      await expect(ui.defaultButton).toHaveClass(/bg-primary/);
      await expect(ui.secondaryButton).toHaveClass(/bg-secondary/);
      await expect(ui.outlineButton).toHaveClass(/border/);
      await expect(ui.ghostButton).toHaveClass(/hover:bg-accent/);
      await expect(ui.destructiveButton).toHaveClass(/bg-destructive/);
    });
  });

  /**
   * SIZES TESTS - Height variations
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Small (h-8):     ┌─────────┐  32px height                     │
   * │                    │  Small  │                                  │
   * │                    └─────────┘                                  │
   * │                                                                 │
   * │   Default (h-9):   ┌───────────┐  36px height                   │
   * │                    │  Default  │                                │
   * │                    └───────────┘                                │
   * │                                                                 │
   * │   Large (h-10):    ┌─────────────┐  40px height                 │
   * │                    │    Large    │                              │
   * │                    └─────────────┘                              │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Sizes", () => {
    /**
     * TEST: Button Size CSS Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Small (h-8):    ┌─────────┐  height: 32px             │
     *   │                   │  Small  │  h-8 class                │
     *   │                   └─────────┘                           │
     *   │                                                         │
     *   │   Large (h-10):   ┌─────────────┐  height: 40px         │
     *   │                   │    Large    │  h-10 class           │
     *   │                   └─────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Small and large buttons have correct height classes
     */
    test("should have correct size classes", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("sizes");

      await expect(ui.smallButton).toHaveClass(/h-8/);
      await expect(ui.largeButton).toHaveClass(/h-10/);
    });
  });

  /**
   * HREF SUPPORT TESTS - Link-style buttons
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   With href prop, button renders as <a> tag:                    │
   * │                                                                 │
   * │   ┌─────────────────────┐                                       │
   * │   │  Go to Dashboard    │  ← <a href="/dashboard">              │
   * │   └─────────────────────┘                                       │
   * │                                                                 │
   * │   External links get security attributes:                       │
   * │   ┌─────────────────────┐                                       │
   * │   │  External Link  ↗   │  ← target="_blank"                    │
   * │   └─────────────────────┘    rel="noopener noreferrer"          │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Href Support", () => {
    /**
     * TEST: Button Renders as Anchor with Href
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   With href prop, button renders as <a> tag:            │
     *   │                                                         │
     *   │   ┌─────────────────────────┐                           │
     *   │   │   Go to Dashboard       │  tagName = "a"            │
     *   │   └─────────────────────────┘  href = "/dashboard"      │
     *   │                                                         │
     *   │   NOT <button>, but <a> element                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button with href prop renders as anchor element
     */
    test("should render as anchor tag with href", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto("with-href");

      const tagName = await ui.dashboardLink.evaluate((el) =>
        el.tagName.toLowerCase(),
      );
      expect(tagName).toBe("a");

      const href = await ui.dashboardLink.getAttribute("href");
      expect(href).toContain("/dashboard");
    });

    /**
     * TEST: External Link Security Attributes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   External links get security attributes:               │
     *   │                                                         │
     *   │   ┌─────────────────────────┐                           │
     *   │   │   External Link     ↗   │  target = "_blank"        │
     *   │   └─────────────────────────┘  rel = "noopener..."      │
     *   │                                                         │
     *   │   Security:                                             │
     *   │   • target="_blank" opens in new tab                    │
     *   │   • rel="noopener" prevents window.opener access        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: External links have proper security attributes
     */
    test("should have security attributes for external links", async ({
      page,
    }) => {
      const ui = new ButtonPage(page);
      await ui.goto("with-href");

      const target = await ui.externalLink.getAttribute("target");
      expect(target).toBe("_blank");

      const rel = await ui.externalLink.getAttribute("rel");
      expect(rel).toContain("noopener");
    });
  });

  /**
   * ACCESSIBILITY TESTS - Focus and touch handling
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   FOCUS RING (keyboard navigation):                             │
   * │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                          │
   * │   ┊  ┌─────────────────────────────┐ ┊ ← focus-visible:ring     │
   * │   ┊  │         Button              │ ┊                          │
   * │   ┊  └─────────────────────────────┘ ┊                          │
   * │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                          │
   * │                                                                 │
   * │   TOUCH HANDLING (mobile):                                      │
   * │   touch-manipulation  ← Optimizes touch response                │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Accessibility", () => {
    /**
     * TEST: Focus Visible Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus shows visible ring:                    │
     *   │                                                         │
     *   │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                          │
     *   │   ┊  ┌─────────────────────┐ ┊  focus-visible:ring      │
     *   │   ┊  │      Button         │ ┊  class present           │
     *   │   ┊  └─────────────────────┘ ┊                          │
     *   │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                          │
     *   │              ↑                                          │
     *   │        Focus ring visible on keyboard navigation        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has focus-visible ring class for a11y
     */
    test("should have focus-visible ring styles", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto();

      const hasFocusRing = await ui.basicButton.evaluate((el) =>
        el.className.includes("focus-visible:ring"),
      );
      expect(hasFocusRing).toBe(true);
    });

    /**
     * TEST: Touch Manipulation for Mobile
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Mobile touch optimization:                            │
     *   │                                                         │
     *   │   ┌─────────────────────┐                               │
     *   │   │      Button         │  touch-manipulation class     │
     *   │   └─────────────────────┘                               │
     *   │         👆                                              │
     *   │    Touch tap is optimized                               │
     *   │    (no 300ms delay, no double-tap zoom)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has touch-manipulation for mobile UX
     */
    test("should have touch-manipulation for mobile", async ({ page }) => {
      const ui = new ButtonPage(page);
      await ui.goto();

      const hasTouchClass = await ui.basicButton.evaluate((el) =>
        el.className.includes("touch-manipulation"),
      );
      expect(hasTouchClass).toBe(true);
    });
  });
});
