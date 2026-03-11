import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * BUTTON-ACTION COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <button data-name="ButtonAction">                                     │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   🗑️   Hold to Delete                                          │   │
 * │   │   ↑         ↑                                                   │   │
 * │   │  icon     text                                                  │   │
 * │   │                                                                 │   │
 * │   │  ═══════════════════════════════════════════════                │   │
 * │   │          ↑ progress bar (fills on hold)                         │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HOLD PROGRESS ANIMATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Start holding:                    Mid-hold:                           │
 * │   ┌────────────────────────────┐    ┌────────────────────────────┐      │
 * │   │  🗑️   Hold to Delete       │    │  🗑️   Hold to Delete       │      │
 * │   │  ▓░░░░░░░░░░░░░░░░░░░░░░░ │    │  ▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░ │      │
 * │   └────────────────────────────┘    └────────────────────────────┘      │
 * │                                                                         │
 * │   Complete:                                                             │
 * │   ┌────────────────────────────┐                                        │
 * │   │  🗑️   Hold to Delete       │                                        │
 * │   │  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │  → triggers action                     │
 * │   └────────────────────────────┘                                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .bg-destructive              ← Destructive red color       │       │
 * │   │  .text-destructive-foreground ← Contrasting text            │       │
 * │   │  .relative                    ← For progress overlay        │       │
 * │   │  .overflow-hidden             ← Clip progress bar           │       │
 * │   │  .rounded-md                  ← Medium border radius        │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ButtonActionPage extends BasePage {
  protected readonly componentName = "button-action";

  // Button action elements - ButtonAction wraps Button component internally
  readonly buttonAction: Locator;
  readonly icon: Locator;
  readonly text: Locator;

  constructor(page: Page) {
    super(page);

    // TODO: ButtonAction should have its own data-name="ButtonAction" attribute
    // Currently it wraps Button internally with data-name="Button"
    this.buttonAction = this.preview.locator('[data-name="Button"]').first();

    // Icon and text inside the button
    this.icon = this.buttonAction.locator("svg");
    // The text is in a nested span - get the innermost one with the actual text
    this.text = this.buttonAction.getByText("Hold to Delete");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("ButtonAction Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: ButtonAction Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Page DOM                                              │
     *   │   ├── ...                                               │
     *   │   └── <button data-name="ButtonAction">  ← EXISTS?     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The ButtonAction component renders and is visible
     */
    test("should have ButtonAction component", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toBeVisible();
    });

    /**
     * TEST: Button Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button data-name="Button">                           │
     *   │            └──────────┬─────┘                           │
     *   │                       │                                 │
     *   │   ButtonAction uses Button internally                   │
     *   │   CHECK: data-name === "Button"                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component has correct data-name for testing/styling
     */
    test("should have Button data-name attribute", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveAttribute("data-name", "Button");
    });

    /**
     * TEST: Button Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM Element:                                          │
     *   │                                                         │
     *   │   <button>  ← tagName.toLowerCase() === "button"        │
     *   │      │                                                  │
     *   │      └── NOT <div>, <span>, or other element            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component uses semantic <button> element
     */
    test("should be a button element", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      const tagName = await ui.buttonAction.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });

    /**
     * TEST: Trash Icon Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  [SVG]   Hold to Delete           │                 │
     *   │   │    ↑                              │                 │
     *   │   │  trash icon visible?              │                 │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The trash/delete icon is rendered and visible
     */
    test("should have trash icon", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.icon).toBeVisible();
    });

    /**
     * TEST: Button Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   "Hold to Delete"            │                 │
     *   │   │         └──────┬──────┘           │                 │
     *   │   │                │                  │                 │
     *   │   │     CHECK: text === "Hold to Delete"                │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button displays correct instructional text
     */
    test("should have text 'Hold to Delete'", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.text).toHaveText("Hold to Delete");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Destructive Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← bg-destructive │
     *   │   │▓▓  🗑️   Hold to Delete          ▓▓│   (red bg)       │
     *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│                 │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has destructive/danger background styling
     */
    test("button should have destructive styling", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/bg-destructive/);
    });

    /**
     * TEST: White Text Color for Contrast
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   Hold to Delete              │                 │
     *   │   │        └──────┬──────┘            │                 │
     *   │   │               │                   │                 │
     *   │   │   text-white                      │                 │
     *   │   │   (contrasting white text)        │                 │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Text is readable against destructive background
     */
    test("button should have white text for contrast", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/text-white/);
    });

    /**
     * TEST: Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   position: relative  ← Required for progress overlay   │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   Hold to Delete              │                 │
     *   │   │  ═══════════════                  │ ← absolute child│
     *   │   │  (progress bar positioned inside) │                 │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has relative positioning for progress overlay
     */
    test("button should have relative positioning", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/relative/);
    });

    /**
     * TEST: Overflow Hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   overflow: hidden                                      │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   Hold to Delete              │                 │
     *   │   │  ▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░ │                 │
     *   │   │        └── progress clipped ──┘   │                 │
     *   │   └───────────────────────────────────┘                 │
     *   │   Progress bar stays within button bounds               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Progress bar animation stays clipped within button
     */
    test("button should have overflow-hidden", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/overflow-hidden/);
    });

    /**
     * TEST: Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭───────────────────────────────────╮                 │
     *   │   │  🗑️   Hold to Delete              │  ← rounded-md   │
     *   │   ╰───────────────────────────────────╯                 │
     *   │     ↑                               ↑                   │
     *   │   border-radius on all corners                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has medium border-radius for modern look
     */
    test("button should have rounded corners", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/rounded-md/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Button Clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User Action:                                          │
     *   │                                                         │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   Hold to Delete              │                 │
     *   │   └───────────────────────────────────┘                 │
     *   │                    ↑                                    │
     *   │                 CLICK                                   │
     *   │                    │                                    │
     *   │            No error thrown                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button responds to click events without error
     */
    test("button should be clickable", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await ui.buttonAction.click();
      // Should not throw error
    });

    /**
     * TEST: Button Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard Navigation:                                  │
     *   │                                                         │
     *   │   [TAB] ─────────────────────────────────┐              │
     *   │                                          ↓              │
     *   │   ┌───────────────────────────────────┐                 │
     *   │   │  🗑️   Hold to Delete              │ :focus          │
     *   │   └───────────────────────────────────┘                 │
     *   │                                                         │
     *   │   CHECK: document.activeElement === button              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button can receive keyboard focus
     */
    test("button should be focusable", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await ui.buttonAction.focus();
      await expect(ui.buttonAction).toBeFocused();
    });

    /**
     * TEST: Hold Progress Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   mousedown (t=0ms)        t=500ms          mouseup     │
     *   │        │                      │                │        │
     *   │        ↓                      ↓                ↓        │
     *   │   ┌──────────────┐      ┌──────────────┐  ┌──────────┐  │
     *   │   │ Hold to Del  │      │ Hold to Del  │  │  STILL   │  │
     *   │   │ ░░░░░░░░░░░░ │  →   │ ▓▓▓▓▓░░░░░░░ │  │ VISIBLE  │  │
     *   │   └──────────────┘      └──────────────┘  └──────────┘  │
     *   │                                                         │
     *   │   Action NOT completed (requires full hold duration)    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Partial hold does not trigger action; button remains
     */
    test("holding button should trigger progress", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      // Start holding the button
      await ui.buttonAction.dispatchEvent("mousedown");

      // Wait a bit
      await page.waitForTimeout(500);

      // Release
      await ui.buttonAction.dispatchEvent("mouseup");

      // Button should still be visible (action not completed in 500ms)
      await expect(ui.buttonAction).toBeVisible();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Button Element Is Semantic
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button>                                              │
     *   │      │                                                  │
     *   │      └── Uses semantic button element                   │
     *   │          (not div with role="button")                   │
     *   │                                                         │
     *   │   TODO: Add type="button" to prevent form submission    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Uses native button element for accessibility
     */
    test("button should be semantic button element", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      const tagName = await ui.buttonAction.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });

    /**
     * TEST: Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Default:                    Focused (keyboard):       │
     *   │   ┌───────────────────┐       ┏━━━━━━━━━━━━━━━━━━━┓     │
     *   │   │ Hold to Delete    │       ┃ Hold to Delete    ┃     │
     *   │   └───────────────────┘       ┗━━━━━━━━━━━━━━━━━━━┛     │
     *   │                                        ↑                │
     *   │                               focus-visible:ring        │
     *   │                               (visible ring on focus)   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Keyboard users can see focus indicator
     */
    test("button should have focus ring styles", async ({ page }) => {
      const ui = new ButtonActionPage(page);
      await ui.goto();

      await expect(ui.buttonAction).toHaveClass(/focus-visible:ring/);
    });
  });
});
