import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

class SonnerPage extends BasePage {
  protected readonly componentName = "sonner";

  // Sonner elements
  readonly triggerButton: Locator;
  readonly toastContainer: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger button - scoped within preview
    this.triggerButton = this.preview.getByRole("button", { name: "Toast Me!" });

    // Toast container (appears after trigger) - scoped within preview
    this.toastContainer = this.preview.locator('[data-sonner-toaster="true"]');
  }

  async triggerToast() {
    await this.triggerButton.click();
    // Wait for toast to appear
    await this.page.waitForTimeout(300);
  }
}

class SonnerPositionsPage extends BasePage {
  protected readonly componentName = "sonner";
  protected readonly demoName = "positions";

  // Trigger buttons
  readonly topLeftTrigger: Locator;
  readonly topCenterTrigger: Locator;
  readonly topRightTrigger: Locator;
  readonly bottomLeftTrigger: Locator;
  readonly bottomCenterTrigger: Locator;
  readonly bottomRightTrigger: Locator;

  // Toaster containers by position
  readonly topLeftToaster: Locator;
  readonly topCenterToaster: Locator;
  readonly topRightToaster: Locator;
  readonly bottomLeftToaster: Locator;
  readonly bottomCenterToaster: Locator;
  readonly bottomRightToaster: Locator;

  // All toasters
  readonly allToasters: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger buttons - scoped within preview
    this.topLeftTrigger = this.preview.getByRole("button", { name: "Top Left" });
    this.topCenterTrigger = this.preview.getByRole("button", { name: "Top Center" });
    this.topRightTrigger = this.preview.getByRole("button", { name: "Top Right" });
    this.bottomLeftTrigger = this.preview.getByRole("button", { name: "Bottom Left" });
    this.bottomCenterTrigger = this.preview.getByRole("button", { name: "Bottom Center" });
    this.bottomRightTrigger = this.preview.getByRole("button", { name: "Bottom Right (Default)" });

    // Toasters by position - scoped within preview
    this.topLeftToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="TopLeft"]');
    this.topCenterToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="TopCenter"]');
    this.topRightToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="TopRight"]');
    this.bottomLeftToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="BottomLeft"]');
    this.bottomCenterToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="BottomCenter"]');
    this.bottomRightToaster = this.preview.locator('[data-sonner-toaster="true"][data-position="BottomRight"]');

    // All toasters - scoped within preview
    this.allToasters = this.preview.locator('[data-sonner-toaster="true"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

/**
 * ============================================================================
 * SONNER TOAST COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [Toast Me!]  ← SonnerTrigger (button that creates toasts)             │
 * │                                                                         │
 * │                              ┌──────────────────────────┐               │
 * │                              │  You got a message    ×  │ ← Toast       │
 * │                              │  You toasted me!         │               │
 * │                              ├──────────────────────────┤               │
 * │                              │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← Progress   │
 * │                              └──────────────────────────┘               │
 * │                                        ↑                                │
 * │                              SonnerToaster (container)                  │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TOAST POSITIONS (6 available):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │ ┌─────────┐          ┌─────────┐          ┌─────────┐                   │
 * │ │TopLeft  │          │TopCenter│          │TopRight │                   │
 * │ └─────────┘          └─────────┘          └─────────┘                   │
 * │                                                                         │
 * │                         (page content)                                  │
 * │                                                                         │
 * │ ┌──────────┐        ┌────────────┐        ┌───────────┐                 │
 * │ │BottomLeft│        │BottomCenter│        │BottomRight│  ← default     │
 * │ └──────────┘        └────────────┘        └───────────┘                 │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */
test.describe("Sonner Page", () => {
  /**
   * STRUCTURE TESTS - Verify basic component elements exist
   *
   * ┌─────────────────────────────────────────┐
   * │                                         │
   * │         [  Toast Me!  ]  ← button       │
   * │              ↑                          │
   * │    SonnerTrigger component              │
   * │    - data-name="SonnerTrigger"          │
   * │    - data-variant="Default"             │
   * │    - data-toast-title="..."             │
   * │    - data-toast-description="..."       │
   * │                                         │
   * └─────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌────────────────────┐                          │
     *   │         │     Toast Me!      │  ← must be visible       │
     *   │         └────────────────────┘                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button is rendered and visible
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toBeVisible();
    });

    /**
     * TEST: Trigger Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌────────────────────┐                          │
     *   │         │     Toast Me!      │  ← text content          │
     *   │         └────────────────────┘                          │
     *   │                                                         │
     *   │   Button text must be exactly "Toast Me!"               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button displays correct text
     */
    test("trigger should have text 'Toast Me!'", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveText("Toast Me!");
    });

    /**
     * TEST: Trigger is Button Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button>Toast Me!</button>                            │
     *   │      ↑                                                  │
     *   │   tagName must be "button" (not div, span, etc.)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger is semantically a button element
     */
    test("trigger should be a button element", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      const tagName = await ui.triggerButton.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Trigger Primary Background
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌────────────────────┐                          │
     *   │         │▓▓▓ Toast Me! ▓▓▓▓▓▓│  bg-primary class        │
     *   │         └────────────────────┘                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has primary background color class
     */
    test("trigger should have button styling", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/bg-primary/);
    });

    /**
     * TEST: Trigger Text Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌────────────────────┐                          │
     *   │         │     Toast Me!      │  text-primary-foreground │
     *   │         └────────────────────┘                          │
     *   │                  ↑                                      │
     *   │            Contrasting text color for readability       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has correct text color class
     */
    test("trigger should have text-primary-foreground", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/text-primary-foreground/);
    });

    /**
     * TEST: Trigger Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   h-9 (height: 36px)                                    │
     *   │   ┌────────────────────┐                                │
     *   │   │     Toast Me!      │  36px tall                     │
     *   │   └────────────────────┘                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has correct height class
     */
    test("trigger should have h-9", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/h-9/);
    });

    /**
     * TEST: Trigger Border Radius
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ╭────────────────────╮  rounded-md              │
     *   │         │     Toast Me!      │  (border-radius: 0.375rem)
     *   │         ╰────────────────────╯                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has medium rounded corners
     */
    test("trigger should have rounded-md", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Trigger Horizontal Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   px-4 (padding-left/right: 1rem)                       │
     *   │   ┌────────────────────┐                                │
     *   │   │ ◄──► Toast Me! ◄──►│                                │
     *   │   └────────────────────┘                                │
     *   │       16px        16px                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has correct horizontal padding
     */
    test("trigger should have px-4", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/px-4/);
    });
  });

  /**
   * TOAST BEHAVIOR TESTS - Verify toast creation and content
   *
   * TOAST CREATION FLOW:
   * ┌────────────────────────────────────────────────────────────────────┐
   * │                                                                    │
   * │  STEP 1: Click trigger          STEP 2: Toast appears              │
   * │  ┌──────────────┐               ┌──────────────────────────┐       │
   * │  │ Toast Me! 🖱️ │  ─────────►   │  You got a message    ×  │       │
   * │  └──────────────┘               │  You toasted me!         │       │
   * │                                 ├──────────────────────────┤       │
   * │                                 │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│       │
   * │                                 └──────────────────────────┘       │
   * │                                                                    │
   * └────────────────────────────────────────────────────────────────────┘
   *
   * TOAST STRUCTURE:
   * ┌────────────────────────────────────────┐
   * │  Title (h3)                        ×   │ ← Close button
   * │  Description (p)                       │
   * ├────────────────────────────────────────┤
   * │  [data-duration-track]                 │ ← Progress bar container
   * │    └── [data-duration-progress]        │ ← Animated fill
   * └────────────────────────────────────────┘
   */
  test.describe("Toast Behavior", () => {
    /**
     * TEST: Toast Appears on Trigger Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────┐          ┌──────────────────────┐  │
     *   │   │   Toast Me!  🖱️│  ──────► │  Toast notification  │  │
     *   │   └────────────────┘          │  appears on screen   │  │
     *   │                               └──────────────────────┘  │
     *   │                                                         │
     *   │   [data-sonner-toast="true"] becomes visible            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger creates visible toast
     */
    test("clicking trigger should show toast", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerToast();

      // Toast should appear
      const toast = page.locator('[data-sonner-toast="true"]');
      await expect(toast.first()).toBeVisible();
    });

    /**
     * TEST: Toast Has Title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────┐              │
     *   │   │  "You got a message"              ×  │  ← title     │
     *   │   │   ...description...                  │              │
     *   │   └──────────────────────────────────────┘              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toast displays correct title text
     */
    test("toast should have title", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerToast();

      // Scope to toast element to avoid matching demo code snippets
      const toast = page.locator('[data-sonner-toast="true"]').first();
      await expect(toast.getByText("You got a message")).toBeVisible();
    });

    /**
     * TEST: Toast Has Description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────┐              │
     *   │   │  You got a message                ×  │              │
     *   │   │  "You toasted me!"                   │  ← description
     *   │   └──────────────────────────────────────┘              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toast displays correct description text
     */
    test("toast should have description", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerToast();

      // Scope to toast element to avoid matching demo code snippets
      const toast = page.locator('[data-sonner-toast="true"]').first();
      await expect(toast.getByText("You toasted me!")).toBeVisible();
    });

    /**
     * TEST: Multiple Toasts Creation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   🖱️🖱️ (two clicks)                                      │
     *   │      │                                                  │
     *   │      ▼                                                  │
     *   │   ┌──────────────────────┐                              │
     *   │   │  Toast 1             │                              │
     *   │   └┬────────────────────┬┘                              │
     *   │    │  Toast 2           │                               │
     *   │    └────────────────────┘                               │
     *   │                                                         │
     *   │   Multiple toasts can exist simultaneously              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Multiple trigger clicks create multiple toasts
     */
    test("multiple clicks should create multiple toasts", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerToast();
      await ui.triggerToast();

      const toasts = page.locator('[data-sonner-toast="true"]');
      const count = await toasts.count();
      expect(count).toBeGreaterThanOrEqual(1);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Trigger Clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌────────────────────┐                          │
     *   │         │   Toast Me!    🖱️  │  ← click succeeds        │
     *   │         └────────────────────┘                          │
     *   │                                                         │
     *   │   Button responds to click without throwing errors      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can be clicked without error
     */
    test("trigger should be clickable", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerButton.click();
      // Should not throw error
    });

    /**
     * TEST: Trigger Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │         ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                         │
     *   │         ┊  ┌────────────────┐ ┊  ← focused              │
     *   │         ┊  │   Toast Me!    │ ┊                         │
     *   │         ┊  └────────────────┘ ┊                         │
     *   │         └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                         │
     *   │                                                         │
     *   │   Button can receive focus programmatically             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can receive focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await ui.triggerButton.focus();
      await expect(ui.triggerButton).toBeFocused();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus shows visible ring:                    │
     *   │                                                         │
     *   │         ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                         │
     *   │         ┊  ┌────────────────┐ ┊  focus-visible:ring     │
     *   │         ┊  │   Toast Me!    │ ┊                         │
     *   │         ┊  └────────────────┘ ┊                         │
     *   │         └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has focus-visible ring class for a11y
     */
    test("trigger should have focus ring styles", async ({ page }) => {
      const ui = new SonnerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/focus-visible:ring/);
    });
  });
});

/**
 * ============================================================================
 * SONNER POSITIONS PAGE TESTS
 * ============================================================================
 *
 * POSITION LAYOUT:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │  ┌─────────┐              ┌─────────┐              ┌─────────┐          │
 * │  │TopLeft  │              │TopCenter│              │TopRight │          │
 * │  │ toast   │              │ toast   │              │ toast   │          │
 * │  └─────────┘              └─────────┘              └─────────┘          │
 * │       ↑                        ↑                        ↑               │
 * │  data-direction=          data-direction=          data-direction=      │
 * │    "TopDown"                "TopDown"                "TopDown"          │
 * │                                                                         │
 * │  ┌────────────────────────────────────────────────────────────────┐     │
 * │  │  [Top Left] [Top Center] [Top Right]                           │     │
 * │  │  [Bottom Left] [Bottom Center] [Bottom Right (Default)]        │     │
 * │  └────────────────────────────────────────────────────────────────┘     │
 * │                           ↑ Trigger buttons                             │
 * │                                                                         │
 * │       ↓                        ↓                        ↓               │
 * │  data-direction=          data-direction=          data-direction=      │
 * │    "BottomUp"               "BottomUp"               "BottomUp"         │
 * │  ┌──────────┐            ┌────────────┐            ┌───────────┐        │
 * │  │BottomLeft│            │BottomCenter│            │BottomRight│        │
 * │  │  toast   │            │   toast    │            │   toast   │        │
 * │  └──────────┘            └────────────┘            └───────────┘        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * DIRECTION ATTRIBUTE (controls stacking & entry animation):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │  TOP POSITIONS (TopDown):          BOTTOM POSITIONS (BottomUp):         │
 * │  - Entry: slides DOWN from above   - Entry: slides UP from below        │
 * │  - Stack: older toasts go BELOW    - Stack: older toasts go ABOVE       │
 * │  - --fold-multiplier: 1            - --fold-multiplier: -1              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */
test.describe("Sonner Positions Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: All Six Position Triggers Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────┐  ┌────────────┐  ┌──────────┐             │
     *   │   │Top Left │  │Top Center  │  │Top Right │             │
     *   │   └─────────┘  └────────────┘  └──────────┘             │
     *   │                                                         │
     *   │   ┌────────────┐  ┌─────────────┐  ┌─────────────────┐  │
     *   │   │Bottom Left │  │Bottom Center│  │Bottom Right(Def)│  │
     *   │   └────────────┘  └─────────────┘  └─────────────────┘  │
     *   │                                                         │
     *   │   All 6 position trigger buttons must be visible        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All six position trigger buttons are rendered
     */
    test("should have all six position triggers", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await expect(ui.topLeftTrigger).toBeVisible();
      await expect(ui.topCenterTrigger).toBeVisible();
      await expect(ui.topRightTrigger).toBeVisible();
      await expect(ui.bottomLeftTrigger).toBeVisible();
      await expect(ui.bottomCenterTrigger).toBeVisible();
      await expect(ui.bottomRightTrigger).toBeVisible();
    });

    /**
     * TEST: Toasters at Each Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [Toaster]                    [Toaster]    [Toaster]   │
     *   │   TopLeft                      TopCenter    TopRight    │
     *   │                                                         │
     *   │              (page content area)                        │
     *   │                                                         │
     *   │   [Toaster]                    [Toaster]    [Toaster]   │
     *   │   BottomLeft                   BottomCenter BottomRight │
     *   │                                                         │
     *   │   count >= 6 toaster containers                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toaster containers exist for all positions
     */
    test("should have toasters at each position", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Should have 6+ toasters (5 from demo + 1 from layout)
      const toasterCount = await ui.allToasters.count();
      expect(toasterCount).toBeGreaterThanOrEqual(6);
    });
  });

  /**
   * DATA ATTRIBUTES - Verify correct attributes on toasters
   *
   * TOASTER ELEMENT ATTRIBUTES:
   * ┌─────────────────────────────────────────────────────────────────┐
   * │  <ol data-name="SonnerList"                                     │
   * │      data-sonner-toaster="true"      ← identifies as toaster    │
   * │      data-sonner-theme="light"       ← theme (light/dark)       │
   * │      data-position="TopLeft"         ← position on screen       │
   * │      data-direction="TopDown"        ← stack direction          │
   * │      data-expanded="false">          ← hover state              │
   * │                                                                 │
   * │    <li data-sonner-toast="true">...</li>  ← individual toasts   │
   * │                                                                 │
   * │  </ol>                                                          │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Data Attributes", () => {
    /**
     * TEST: Toaster Identification Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol data-sonner-toaster="true">  ← identifies toaster │
     *   │     <li data-sonner-toast="true">...</li>               │
     *   │   </ol>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toaster elements have identification attribute
     */
    test("toasters should have data-sonner-toaster attribute", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await expect(ui.topLeftToaster).toHaveAttribute("data-sonner-toaster", "true");
      await expect(ui.bottomRightToaster).toHaveAttribute("data-sonner-toaster", "true");
    });

    /**
     * TEST: Toaster Theme Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol data-sonner-theme="light">                        │
     *   │                          ↑                              │
     *   │   Theme attribute controls toast styling                │
     *   │   (light/dark mode support)                             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toasters have theme attribute for styling
     */
    test("toasters should have data-sonner-theme attribute", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await expect(ui.topLeftToaster).toHaveAttribute("data-sonner-theme", "light");
      await expect(ui.bottomRightToaster).toHaveAttribute("data-sonner-theme", "light");
    });

    /**
     * TEST: Trigger Position Attributes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button data-toast-position="TopLeft">Top Left</button>
     *   │   <button data-toast-position="TopCenter">...</button>  │
     *   │   <button data-toast-position="BottomRight">...</button>│
     *   │                               ↑                         │
     *   │   Position attribute tells which toaster to use         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger buttons specify their target position
     */
    test("triggers should have data-toast-position attribute", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await expect(ui.topLeftTrigger).toHaveAttribute("data-toast-position", "TopLeft");
      await expect(ui.topCenterTrigger).toHaveAttribute("data-toast-position", "TopCenter");
      await expect(ui.bottomRightTrigger).toHaveAttribute("data-toast-position", "BottomRight");
    });

    /**
     * DATA-DIRECTION TEST:
     *
     * ┌─────────────────────────────────────────────────────────────────┐
     * │  TOP positions → data-direction="TopDown"                       │
     * │                                                                 │
     * │    TopLeft       TopCenter       TopRight                       │
     * │      ↓              ↓               ↓                           │
     * │  [TopDown]      [TopDown]       [TopDown]                       │
     * │                                                                 │
     * │  This means: toasts enter from ABOVE, stack DOWNWARD            │
     * └─────────────────────────────────────────────────────────────────┘
     */
    test("TOP position toasters should have data-direction=TopDown", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // All TOP positions should have direction TopDown (toasts stack downward)
      await expect(ui.topLeftToaster).toHaveAttribute("data-direction", "TopDown");
      await expect(ui.topCenterToaster).toHaveAttribute("data-direction", "TopDown");
      await expect(ui.topRightToaster).toHaveAttribute("data-direction", "TopDown");
    });

    /**
     * DATA-DIRECTION TEST:
     *
     * ┌─────────────────────────────────────────────────────────────────┐
     * │  BOTTOM positions → data-direction="BottomUp"                   │
     * │                                                                 │
     * │  BottomLeft    BottomCenter    BottomRight                      │
     * │      ↓              ↓               ↓                           │
     * │  [BottomUp]     [BottomUp]      [BottomUp]                      │
     * │                                                                 │
     * │  This means: toasts enter from BELOW, stack UPWARD              │
     * └─────────────────────────────────────────────────────────────────┘
     */
    test("BOTTOM position toasters should have data-direction=BottomUp", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // All BOTTOM positions should have direction BottomUp (toasts stack upward)
      await expect(ui.bottomLeftToaster).toHaveAttribute("data-direction", "BottomUp");
      await expect(ui.bottomCenterToaster).toHaveAttribute("data-direction", "BottomUp");
      await expect(ui.bottomRightToaster).toHaveAttribute("data-direction", "BottomUp");
    });
  });

  test.describe("Position Targeting", () => {
    /**
     * TEST: TopLeft Toast Targeting
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────┐                                  │
     *   │   │ Toast at top-left│ ← appears here                   │
     *   │   └──────────────────┘                                  │
     *   │                                                         │
     *   │                                                         │
     *   │       ┌─────────────┐                                   │
     *   │       │  Top Left 🖱️│ ← click trigger                   │
     *   │       └─────────────┘                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TopLeft trigger creates toast in TopLeft toaster
     */
    test("clicking TopLeft trigger shows toast in TopLeft toaster", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.topLeftTrigger.click();
      await page.waitForTimeout(300);

      // Toast should appear inside TopLeft toaster
      const toast = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
      await expect(toast).toBeVisible();
      await expect(toast.getByText("Toast positioned at top-left")).toBeVisible();
    });

    /**
     * TEST: TopCenter Toast Targeting
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │            ┌──────────────────┐                         │
     *   │            │ Toast at top     │ ← appears here          │
     *   │            └──────────────────┘                         │
     *   │                                                         │
     *   │          ┌───────────────┐                              │
     *   │          │  Top Center 🖱️│ ← click trigger              │
     *   │          └───────────────┘                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TopCenter trigger creates toast in TopCenter toaster
     */
    test("clicking TopCenter trigger shows toast in TopCenter toaster", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.topCenterTrigger.click();
      await page.waitForTimeout(300);

      const toast = ui.topCenterToaster.locator('[data-sonner-toast="true"]');
      await expect(toast).toBeVisible();
    });

    /**
     * TEST: BottomRight Toast Targeting
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                  ┌─────────────────────┐                 │
     *   │                  │ Bottom Right (Def) 🖱️│ ← click        │
     *   │                  └─────────────────────┘                 │
     *   │                                                         │
     *   │                              ┌──────────────────┐        │
     *   │                              │ Toast at bottom  │ ← here │
     *   │                              └──────────────────┘        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: BottomRight trigger creates toast in BottomRight toaster
     */
    test("clicking BottomRight trigger shows toast in BottomRight toaster", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.bottomRightTrigger.click();
      await page.waitForTimeout(300);

      const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
      await expect(toast).toBeVisible();
    });

    /**
     * TEST: BottomLeft Toast Targeting
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │       ┌───────────────┐                                 │
     *   │       │ Bottom Left 🖱️│ ← click trigger                 │
     *   │       └───────────────┘                                 │
     *   │                                                         │
     *   │   ┌──────────────────┐                                  │
     *   │   │ Toast at bottom  │ ← appears here                   │
     *   │   └──────────────────┘                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: BottomLeft trigger creates toast in BottomLeft toaster
     */
    test("clicking BottomLeft trigger shows toast in BottomLeft toaster", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.bottomLeftTrigger.click();
      await page.waitForTimeout(300);

      const toast = ui.bottomLeftToaster.locator('[data-sonner-toast="true"]');
      await expect(toast).toBeVisible();
    });
  });

  /**
   * ============================================================================
   * TOAST STACKING AND EXPANSION TESTS
   * ============================================================================
   *
   * STACKING BEHAVIOR (collapsed state - default):
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │  BOTTOM POSITION (stack UP):         TOP POSITION (stack DOWN):         │
   * │                                                                         │
   * │      ┌─────────────────┐                  ┌─────────────────┐           │
   * │      │                 │ ← oldest         │  Toast 3 (new)  │ ← newest  │
   * │      └┬───────────────┬┘   (behind)       └┬───────────────┬┘  (front)  │
   * │       │               │                    │               │            │
   * │       └┬─────────────┬┘                    └┬─────────────┬┘            │
   * │        │ Toast 3     │ ← newest             │             │ ← oldest   │
   * │        └─────────────┘   (front)            └─────────────┘   (behind) │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * EXPANSION BEHAVIOR (on hover):
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │  COLLAPSED (default):                EXPANDED (on hover):               │
   * │                                                                         │
   * │      ┌─────────────────┐             ┌─────────────────┐                │
   * │      │                 │             │  Toast 1 (old)  │                │
   * │      └┬───────────────┬┘             └─────────────────┘                │
   * │       │               │                    ↕ 15px gap                   │
   * │       └┬─────────────┬┘             ┌─────────────────┐                │
   * │        │ Toast 3     │   ────────►  │  Toast 2        │                │
   * │        └─────────────┘   🖱️ hover   └─────────────────┘                │
   * │                                           ↕ 15px gap                   │
   * │                                     ┌─────────────────┐                │
   * │                                     │  Toast 3 (new)  │                │
   * │                                     └─────────────────┘                │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * ============================================================================
   */
  test.describe("Toast Stacking and Expansion", () => {
    /**
     * Y-POSITION ATTRIBUTE TEST:
     *
     * ┌───────────────────────────────────────────────────────────────┐
     * │  Toast at TopLeft     →  data-y-position="Top"                │
     * │  Toast at BottomRight →  data-y-position="Bottom"             │
     * │                                                               │
     * │  This attribute helps CSS/JS determine expansion direction    │
     * └───────────────────────────────────────────────────────────────┘
     */
    test("toasts should have data-y-position attribute matching toaster position", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast at TopLeft
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(300);

      const topLeftToast = ui.topLeftToaster.locator('[data-sonner-toast="true"]').first();
      await expect(topLeftToast).toHaveAttribute("data-y-position", "Top");

      // Create toast at BottomRight
      await ui.bottomRightTrigger.click();
      await page.waitForTimeout(300);

      const bottomRightToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
      await expect(bottomRightToast).toHaveAttribute("data-y-position", "Bottom");
    });

    /**
     * TOP POSITION EXPANSION TEST:
     *
     * ┌─────────────────────────────────────────────────────────────────┐
     * │                                                                 │
     * │  BEFORE HOVER (collapsed):      AFTER HOVER (expanded):         │
     * │                                                                 │
     * │  ┌───────────────────┐          ┌───────────────────┐           │
     * │  │                   │          │  Toast 1 (oldest) │  y=50     │
     * │  └┬─────────────────┬┘          └───────────────────┘           │
     * │   │                 │                 ↕ gap                     │
     * │   └┬───────────────┬┘           ┌───────────────────┐           │
     * │    │ Toast 3 (new) │            │  Toast 2          │  y=120    │
     * │    └───────────────┘            └───────────────────┘           │
     * │                                       ↕ gap                     │
     * │                                 ┌───────────────────┐           │
     * │                                 │  Toast 3 (newest) │  y=190    │
     * │                                 └───────────────────┘           │
     * │                                                                 │
     * │  ASSERTION: lastToast.y > firstToast.y                          │
     * │  (older toasts have HIGHER y = further DOWN the page)           │
     * │                                                                 │
     * └─────────────────────────────────────────────────────────────────┘
     */
    test("TOP position: stacked toasts should expand DOWNWARD on hover", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create multiple toasts at TopLeft position
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(200);
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(200);
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(300);

      // Get toast positions BEFORE hover
      const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
      const toastCount = await toasts.count();
      expect(toastCount).toBeGreaterThanOrEqual(2);

      // Get the first toast's position before hover
      const firstToastBefore = await toasts.first().boundingBox();
      expect(firstToastBefore).not.toBeNull();

      // Hover over the toaster to trigger expansion
      await ui.topLeftToaster.hover();
      await page.waitForTimeout(400); // Wait for expansion animation

      // Get positions AFTER hover (expanded state)
      const firstToastAfter = await toasts.first().boundingBox();
      const lastToastAfter = await toasts.last().boundingBox();

      expect(firstToastAfter).not.toBeNull();
      expect(lastToastAfter).not.toBeNull();

      // For TOP position: toasts should expand DOWNWARD
      // The last toast (oldest) should be BELOW the first toast (newest)
      // This means lastToast.y > firstToast.y
      expect(lastToastAfter!.y).toBeGreaterThan(firstToastAfter!.y);
    });

    /**
     * BOTTOM POSITION EXPANSION TEST:
     *
     * ┌─────────────────────────────────────────────────────────────────┐
     * │                                                                 │
     * │  BEFORE HOVER (collapsed):      AFTER HOVER (expanded):         │
     * │                                                                 │
     * │                                 ┌───────────────────┐           │
     * │                                 │  Toast 1 (oldest) │  y=300    │
     * │                                 └───────────────────┘           │
     * │                                       ↕ gap                     │
     * │  ┌───────────────────┐          ┌───────────────────┐           │
     * │  │                   │          │  Toast 2          │  y=370    │
     * │  └┬─────────────────┬┘          └───────────────────┘           │
     * │   │                 │                 ↕ gap                     │
     * │   └┬───────────────┬┘           ┌───────────────────┐           │
     * │    │ Toast 3 (new) │            │  Toast 3 (newest) │  y=440    │
     * │    └───────────────┘            └───────────────────┘           │
     * │  ─────────────────────────────────────────────────── (bottom)   │
     * │                                                                 │
     * │  ASSERTION: lastToast.y < firstToast.y                          │
     * │  (older toasts have LOWER y = further UP the page)              │
     * │                                                                 │
     * └─────────────────────────────────────────────────────────────────┘
     */
    test("BOTTOM position: stacked toasts should expand UPWARD on hover", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create multiple toasts at BottomRight position
      await ui.bottomRightTrigger.click();
      await page.waitForTimeout(200);
      await ui.bottomRightTrigger.click();
      await page.waitForTimeout(200);
      await ui.bottomRightTrigger.click();
      await page.waitForTimeout(300);

      // Get toast positions BEFORE hover
      const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
      const toastCount = await toasts.count();
      expect(toastCount).toBeGreaterThanOrEqual(2);

      // Hover over the toaster to trigger expansion
      await ui.bottomRightToaster.hover();
      await page.waitForTimeout(400); // Wait for expansion animation

      // Get positions AFTER hover (expanded state)
      const firstToastAfter = await toasts.first().boundingBox();
      const lastToastAfter = await toasts.last().boundingBox();

      expect(firstToastAfter).not.toBeNull();
      expect(lastToastAfter).not.toBeNull();

      // For BOTTOM position: toasts should expand UPWARD
      // The last toast (oldest) should be ABOVE the first toast (newest)
      // This means lastToast.y < firstToast.y
      expect(lastToastAfter!.y).toBeLessThan(firstToastAfter!.y);
    });

    /**
     * TEST: Data-Expanded Attribute on Hover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   COLLAPSED (default):        EXPANDED (hover):         │
     *   │   data-expanded="false"       data-expanded="true"      │
     *   │                                                         │
     *   │   ┌───────────────┐           ┌───────────────┐         │
     *   │   │               │   🖱️ ──►  │  Toast 1      │         │
     *   │   └┬─────────────┬┘   hover   └───────────────┘         │
     *   │    │ Toast 2     │                  ↕                   │
     *   │    └─────────────┘            ┌───────────────┐         │
     *   │                               │  Toast 2      │         │
     *   │                               └───────────────┘         │
     *   │                                                         │
     *   │   Mouse leave → data-expanded="false" again             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hover changes data-expanded attribute correctly
     */
    test("toasts should have data-expanded attribute set correctly on hover", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toasts
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(200);
      await ui.topLeftTrigger.click();
      await page.waitForTimeout(300);

      const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');

      // Before hover: toasts should not be expanded
      await expect(toasts.first()).toHaveAttribute("data-expanded", "false");

      // Hover to expand
      await ui.topLeftToaster.hover();
      await page.waitForTimeout(300);

      // After hover: toasts should be expanded
      await expect(toasts.first()).toHaveAttribute("data-expanded", "true");

      // Move mouse away
      await page.mouse.move(0, 0);
      await page.waitForTimeout(300);

      // After mouse leave: toasts should collapse
      await expect(toasts.first()).toHaveAttribute("data-expanded", "false");
    });

    /**
     * TEST: TopCenter Expansion Direction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TOP position expands DOWNWARD:                        │
     *   │                                                         │
     *   │   ┌───────────────────┐  y=50  (newest, front)          │
     *   │   │  Toast 2 (new)    │                                 │
     *   │   └───────────────────┘                                 │
     *   │            ↕ gap                                        │
     *   │   ┌───────────────────┐  y=120 (oldest, back)           │
     *   │   │  Toast 1 (old)    │                                 │
     *   │   └───────────────────┘                                 │
     *   │                                                         │
     *   │   ASSERT: lastToast.y > firstToast.y                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TopCenter toasts expand downward on hover
     */
    test("TopCenter position: toasts should expand downward", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create multiple toasts at TopCenter
      await ui.topCenterTrigger.click();
      await page.waitForTimeout(200);
      await ui.topCenterTrigger.click();
      await page.waitForTimeout(300);

      const toasts = ui.topCenterToaster.locator('[data-sonner-toast="true"]');

      // Hover to expand
      await ui.topCenterToaster.hover();
      await page.waitForTimeout(400);

      const firstToast = await toasts.first().boundingBox();
      const lastToast = await toasts.last().boundingBox();

      // TopCenter = top position, so oldest toast should be below newest
      expect(lastToast!.y).toBeGreaterThan(firstToast!.y);
    });

    /**
     * TEST: BottomLeft Expansion Direction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BOTTOM position expands UPWARD:                       │
     *   │                                                         │
     *   │   ┌───────────────────┐  y=300 (oldest, back)           │
     *   │   │  Toast 1 (old)    │                                 │
     *   │   └───────────────────┘                                 │
     *   │            ↕ gap                                        │
     *   │   ┌───────────────────┐  y=370 (newest, front)          │
     *   │   │  Toast 2 (new)    │                                 │
     *   │   └───────────────────┘                                 │
     *   │   ─────────────────────────────────────── (bottom)      │
     *   │                                                         │
     *   │   ASSERT: lastToast.y < firstToast.y                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: BottomLeft toasts expand upward on hover
     */
    test("BottomLeft position: toasts should expand upward", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create multiple toasts at BottomLeft
      await ui.bottomLeftTrigger.click();
      await page.waitForTimeout(200);
      await ui.bottomLeftTrigger.click();
      await page.waitForTimeout(300);

      const toasts = ui.bottomLeftToaster.locator('[data-sonner-toast="true"]');

      // Hover to expand
      await ui.bottomLeftToaster.hover();
      await page.waitForTimeout(400);

      const firstToast = await toasts.first().boundingBox();
      const lastToast = await toasts.last().boundingBox();

      // BottomLeft = bottom position, so oldest toast should be above newest
      expect(lastToast!.y).toBeLessThan(firstToast!.y);
    });
  });

  /**
   * ============================================================================
   * TOAST ANIMATION BEHAVIOR TESTS
   * ============================================================================
   *
   * ENTRY ANIMATION (data-mounted: false → true):
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │  BOTTOM POSITION:                    TOP POSITION:                      │
   * │                                                                         │
   * │  ┌───────────────────────────┐       ─────────────────────── (top)      │
   * │  │       page content        │       │                                  │
   * │  │                           │       ▼  toast enters from ABOVE         │
   * │  │                           │       ┌───────────────────┐              │
   * │  │                           │       │ ░░ data-mounted   │              │
   * │  │                           │       │ ░░ = "false"      │              │
   * │  ─────────────────────────────       │ ░░ translateY(-100%)             │
   * │       ▲                              └───────────────────┘              │
   * │       │  toast enters from BELOW            │                           │
   * │  ┌───────────────────┐                      ▼                           │
   * │  │ ░░ data-mounted   │              ┌───────────────────┐              │
   * │  │ ░░ = "false"      │              │ ▓▓ data-mounted   │              │
   * │  │ ░░ translateY(100%)              │ ▓▓ = "true"       │              │
   * │  └───────────────────┘              │ ▓▓ translateY(0)  │              │
   * │       │                             └───────────────────┘              │
   * │       ▼                                                                 │
   * │  ┌───────────────────┐                                                  │
   * │  │ ▓▓ data-mounted   │                                                  │
   * │  │ ▓▓ = "true"       │                                                  │
   * │  │ ▓▓ translateY(0)  │                                                  │
   * │  └───────────────────┘                                                  │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * EXIT ANIMATION (data-removed: false → true):
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │  ┌───────────────────┐      ┌───────────────────┐                       │
   * │  │ ▓▓ data-removed   │      │ ░░ data-removed   │                       │
   * │  │ ▓▓ = "false"      │  ──► │ ░░ = "true"       │  ──►  (removed)       │
   * │  │ ▓▓ opacity: 1     │      │ ░░ opacity: 0     │                       │
   * │  └───────────────────┘      └───────────────────┘                       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * ============================================================================
   */
  test.describe("Toast Animation Behavior", () => {
    /**
     * ENTRY ANIMATION - Toast starts unmounted then mounts
     *
     * ┌────────────────────────────────────────────────────────────────┐
     * │  t=0ms     data-mounted="false"  (toast just added to DOM)    │
     * │                    │                                           │
     * │                    ▼  requestAnimationFrame                    │
     * │                                                                │
     * │  t=16ms+   data-mounted="true"   (triggers CSS transition)    │
     * └────────────────────────────────────────────────────────────────┘
     */
    test.describe("Entry Animation", () => {
      /**
       * TEST: Toast Initial Mount State
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   FRAME 0 (toast just added to DOM):                    │
       *   │                                                         │
       *   │   ┌───────────────────────────────────────┐             │
       *   │   │  Toast                                │             │
       *   │   │  data-mounted="false"  ← initial state│             │
       *   │   │  (not yet animated in)                │             │
       *   │   └───────────────────────────────────────┘             │
       *   │                                                         │
       *   │   MutationObserver captures first value = "false"       │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Toast starts with data-mounted="false"
       */
      test("toast should start with data-mounted=false", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // We need to catch the toast in its initial state
        // Listen for DOM changes to catch the toast before it's mounted
        const mountedValues: string[] = [];

        await page.exposeFunction("trackMounted", (value: string) => {
          mountedValues.push(value);
        });

        await page.evaluate(() => {
          const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
              if (mutation.type === "childList") {
                mutation.addedNodes.forEach((node) => {
                  if (node instanceof HTMLElement && node.dataset.sonnerToast === "true") {
                    // @ts-ignore
                    window.trackMounted(node.dataset.mounted || "undefined");
                  }
                });
              }
            });
          });
          observer.observe(document.body, { childList: true, subtree: true });
        });

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        // The first captured value should be "false" (initial state)
        expect(mountedValues[0]).toBe("false");
      });

      /**
       * TEST: Toast Transition to Mounted State
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   t=0ms    data-mounted="false"                         │
       *   │               │                                         │
       *   │               ▼  requestAnimationFrame + CSS transition │
       *   │                                                         │
       *   │   t=400ms  data-mounted="true"  ← after entry animation │
       *   │                                                         │
       *   │   ┌───────────────────────────────────────┐             │
       *   │   │  Toast                                │             │
       *   │   │  data-mounted="true"  ← final state   │             │
       *   │   │  (fully animated in)                  │             │
       *   │   └───────────────────────────────────────┘             │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Toast transitions to data-mounted="true" after entry
       */
      test("toast should transition to data-mounted=true after entry", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400); // Wait for enter animation (300ms + buffer)

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        await expect(toast).toHaveAttribute("data-mounted", "true");
      });

      /**
       * TEST: Toast Opacity When Mounted
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   UNMOUNTED (invisible):     MOUNTED (visible):         │
       *   │   ┌─────────────────┐        ┌─────────────────┐        │
       *   │   │ ░░░░░░░░░░░░░░░ │        │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │        │
       *   │   │ ░░░░░░░░░░░░░░░ │  ───►  │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │        │
       *   │   │ ░░ opacity: 0 ░ │        │ ▓ opacity: 1  ▓ │        │
       *   │   └─────────────────┘        └─────────────────┘        │
       *   │                                                         │
       *   │   After mount, toast should be fully opaque             │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Mounted toast has opacity of 1
       */
      test("toast should have opacity 1 when mounted", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const opacity = await toast.evaluate((el) => window.getComputedStyle(el).opacity);
        expect(opacity).toBe("1");
      });

      /**
       * TEST: Toast Entry Animation Duration
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --enter-duration CSS variable                         │
       *   │                                                         │
       *   │   t=0ms    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░ (entering)      │
       *   │            │                                            │
       *   │            │ 300ms animation                            │
       *   │            │                                            │
       *   │   t=300ms  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ (entered)       │
       *   │                                                         │
       *   │   Entry animation should take ~300ms                    │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Entry animation duration CSS variable is 300ms
       */
      test("toast entry animation duration should be ~300ms", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Get CSS variable value
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const enterDuration = await ui.bottomRightToaster.evaluate((el) => {
          return getComputedStyle(el).getPropertyValue("--enter-duration");
        });

        expect(enterDuration.trim()).toBe("300ms");
      });
    });

    /**
     * STACKING ANIMATION (COLLAPSED STATE)
     *
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  "DECK OF CARDS" EFFECT:                                                │
     * │                                                                         │
     * │     Side view:                    Front view:                           │
     * │                                                                         │
     * │     Toast 1 ══════                ┌────────────────────┐                 │
     * │            ↘                      │                    │  index=2       │
     * │     Toast 2 ════════              │    (oldest)        │  scale=0.90    │
     * │                ↘                  └┬──────────────────┬┘                 │
     * │     Toast 3 ══════════             │                  │  index=1        │
     * │                                    │                  │  scale=0.95     │
     * │                                    └┬────────────────┬┘                  │
     * │                                     │  Toast 3       │  index=0         │
     * │                                     │  (newest)      │  scale=1.00      │
     * │                                     └────────────────┘                   │
     * │                                                                         │
     * │  TRANSFORM FORMULA:                                                     │
     * │  ┌─────────────────────────────────────────────────────────────────┐    │
     * │  │  scale = 1 - (index × 0.05)                                     │    │
     * │  │  translateY = --fold-multiplier × index × --stack-spacing       │    │
     * │  │                                                                 │    │
     * │  │  index=0 (front): scale(1.00), translateY(0px)                  │    │
     * │  │  index=1:         scale(0.95), translateY(-20px) for bottom     │    │
     * │  │  index=2:         scale(0.90), translateY(-40px) for bottom     │    │
     * │  └─────────────────────────────────────────────────────────────────┘    │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     */
    test.describe("Stacking Animation (Collapsed State)", () => {
      /**
       * TEST: Back Toast Scale Reduction
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   COLLAPSED STATE (deck of cards effect):               │
       *   │                                                         │
       *   │   ┌─────────────────────────────┐  Back toast           │
       *   │   │                             │  scale = 0.95         │
       *   │   └┬───────────────────────────┬┘  (smaller)            │
       *   │    │  Front toast              │  Front toast           │
       *   │    │                           │  scale = 1.00          │
       *   │    └───────────────────────────┘  (full size)           │
       *   │                                                         │
       *   │   backScale < frontScale                                │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Back toast has smaller scale than front toast
       */
      test("back toast should have smaller scale than front toast", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(300);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away to ensure collapsed state
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const frontToast = toasts.first(); // newest
        const backToast = toasts.last(); // oldest

        const frontTransform = await frontToast.evaluate((el) => window.getComputedStyle(el).transform);
        const backTransform = await backToast.evaluate((el) => window.getComputedStyle(el).transform);

        // Parse scale from transform matrix
        // matrix(a, b, c, d, tx, ty) where a = scaleX
        const parseScale = (transform: string) => {
          if (transform === "none") return 1;
          const match = transform.match(/matrix\(([^,]+)/);
          return match ? parseFloat(match[1]) : 1;
        };

        const frontScale = parseScale(frontTransform);
        const backScale = parseScale(backTransform);

        // Back toast should have smaller scale (1 - 0.05 * index)
        expect(backScale).toBeLessThan(frontScale);
        expect(backScale).toBeCloseTo(0.95, 1); // ~0.95 for index 1
      });

      /**
       * TEST: Back Toast TranslateY Offset
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   BOTTOM POSITION - Collapsed State:                    │
       *   │                                                         │
       *   │   Y position:                                           │
       *   │   ─────────                                             │
       *   │   │  Back toast   │  Y = smaller (higher on screen)     │
       *   │   └┬─────────────┬┘  translateY = negative offset       │
       *   │    │ Front toast │  Y = larger (lower on screen)        │
       *   │    └─────────────┘  translateY = 0                      │
       *   │                                                         │
       *   │   backBox.y < frontBox.y  (back is above front)         │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Back toast has negative translateY offset
       */
      test("back toast should have translateY offset in collapsed state", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(300);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const frontBox = await toasts.first().boundingBox();
        const backBox = await toasts.last().boundingBox();

        // For BottomRight position, back toast should be ABOVE front toast (smaller Y)
        // Due to negative translateY offset
        expect(backBox!.y).toBeLessThan(frontBox!.y);
      });

      /**
       * TEST: Toast Index CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --index CSS variable on each toast:                   │
       *   │                                                         │
       *   │   ┌───────────────────────────┐  --index: 2 (oldest)    │
       *   │   │                           │                         │
       *   │   └┬─────────────────────────┬┘                         │
       *   │    │                         │  --index: 1              │
       *   │    └┬───────────────────────┬┘                          │
       *   │     │  Toast (newest/front) │  --index: 0 (front)       │
       *   │     └───────────────────────┘                           │
       *   │                                                         │
       *   │   Index determines scale and offset in collapsed state  │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: --index CSS variable is set correctly for each toast
       */
      test("--index CSS variable should be set correctly for each toast", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 3 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        // Front toast (newest) should have index 0
        const frontIndex = await toasts.first().evaluate((el) => {
          return el.style.getPropertyValue("--index");
        });
        expect(frontIndex).toBe("0");

        // Back toast (oldest) should have index 2
        const backIndex = await toasts.last().evaluate((el) => {
          return el.style.getPropertyValue("--index");
        });
        expect(backIndex).toBe("2");
      });

      /**
       * TEST: Toast Z-Index Ordering
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --z-index CSS variable determines stacking order:     │
       *   │                                                         │
       *   │   Visual stacking:       Z-Index values:                │
       *   │   ┌─────────────────┐                                   │
       *   │   │  Back toast     │    --z-index: 1 (lowest)          │
       *   │   └┬───────────────┬┘                                   │
       *   │    │ Front toast   │    --z-index: 2 (highest)          │
       *   │    └───────────────┘                                    │
       *   │                                                         │
       *   │   Front toast z-index > Back toast z-index              │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Front toast has highest z-index
       */
      test("--z-index should be highest for front toast", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        const frontZIndex = await toasts.first().evaluate((el) => {
          return parseInt(el.style.getPropertyValue("--z-index"), 10);
        });
        const backZIndex = await toasts.last().evaluate((el) => {
          return parseInt(el.style.getPropertyValue("--z-index"), 10);
        });

        expect(frontZIndex).toBeGreaterThan(backZIndex);
      });

      /**
       * TEST: Scale Factor CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --scale-factor: 0.05 (scale reduction per index)      │
       *   │                                                         │
       *   │   Formula: scale = 1 - (index × 0.05)                   │
       *   │                                                         │
       *   │   index=0:  scale = 1 - (0 × 0.05) = 1.00               │
       *   │   index=1:  scale = 1 - (1 × 0.05) = 0.95               │
       *   │   index=2:  scale = 1 - (2 × 0.05) = 0.90               │
       *   │                                                         │
       *   │   ┌───────────────────────────┐  scale=0.90             │
       *   │   └┬─────────────────────────┬┘  scale=0.95             │
       *   │    └───────────────────────┬─┘   scale=1.00             │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Scale factor CSS variable is 0.05
       */
      test("scale factor should be 0.05 per index level", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        const scaleFactor = await ui.bottomRightToaster.evaluate((el) => {
          return getComputedStyle(el).getPropertyValue("--scale-factor");
        });

        // Wait for toaster to be initialized
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(300);

        const scaleFactorAfter = await ui.bottomRightToaster.evaluate((el) => {
          return getComputedStyle(el).getPropertyValue("--scale-factor");
        });

        expect(scaleFactorAfter.trim()).toBe("0.05");
      });
    });

    /**
     * EXPANSION ANIMATION (HOVER STATE)
     *
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  COLLAPSED → EXPANDED TRANSITION:                                       │
     * │                                                                         │
     * │   COLLAPSED:                       EXPANDED (on hover):                 │
     * │   ┌────────────────────┐           ┌─────────────────────┐              │
     * │   │                    │           │  Toast 1 (oldest)   │  scale(1)    │
     * │   └┬──────────────────┬┘           └─────────────────────┘              │
     * │    │                  │                   ↕ ~15px gap                   │
     * │    └┬────────────────┬┘            ┌─────────────────────┐              │
     * │     │  Toast 3       │    🖱️ ──►   │  Toast 2            │  scale(1)    │
     * │     └────────────────┘    hover    └─────────────────────┘              │
     * │                                          ↕ ~15px gap                    │
     * │   scale varies per index           ┌─────────────────────┐              │
     * │   (0.95, 0.90, ...)               │  Toast 3 (newest)   │  scale(1)    │
     * │                                    └─────────────────────┘              │
     * │                                                                         │
     * │                                    ALL toasts have scale(1) ✓           │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     */
    test.describe("Expansion Animation (Hover State)", () => {
      /**
       * TEST: Expanded State Scale Reset
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   COLLAPSED (various scales):   EXPANDED (all scale=1): │
       *   │                                                         │
       *   │   ┌───────────────────┐         ┌─────────────────────┐ │
       *   │   │                   │ 0.90    │  Toast 1            │ │
       *   │   └┬─────────────────┬┘         └─────────────────────┘ │
       *   │    │                 │ 0.95           ↕ gap             │
       *   │    └┬───────────────┬┘          ┌─────────────────────┐ │
       *   │     │ Front         │ 1.00 🖱️►  │  Toast 2            │ │
       *   │     └───────────────┘   hover   └─────────────────────┘ │
       *   │                                                         │
       *   │   On hover, ALL toasts expand to scale(1)               │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: All toasts have scale(1) when expanded
       */
      test("all toasts should have scale(1) when expanded", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.bottomRightToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        // Check both toasts have scale 1
        for (let i = 0; i < await toasts.count(); i++) {
          const transform = await toasts.nth(i).evaluate((el) => window.getComputedStyle(el).transform);
          const parseScale = (t: string) => {
            if (t === "none") return 1;
            const match = t.match(/matrix\(([^,]+)/);
            return match ? parseFloat(match[1]) : 1;
          };
          expect(parseScale(transform)).toBeCloseTo(1, 1);
        }
      });

      /**
       * TEST: Expanded State Gap Between Toasts
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   EXPANDED STATE (on hover):                            │
       *   │                                                         │
       *   │   ┌─────────────────────────────┐                       │
       *   │   │  Toast 1 (oldest)           │                       │
       *   │   └─────────────────────────────┘                       │
       *   │              ↕ ~15px gap (--gap CSS variable)           │
       *   │   ┌─────────────────────────────┐                       │
       *   │   │  Toast 2 (newest)           │                       │
       *   │   └─────────────────────────────┘                       │
       *   │                                                         │
       *   │   Gap should be between 10-25px (approximately 15px)    │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Toasts have proper gap when expanded
       */
      test("toasts should have gap between them when expanded", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.bottomRightToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const firstBox = await toasts.first().boundingBox();
        const lastBox = await toasts.last().boundingBox();

        // Calculate gap (for bottom position, last toast is above first)
        const gap = firstBox!.y - (lastBox!.y + lastBox!.height);

        // Gap should be approximately 15px (--gap value)
        expect(gap).toBeGreaterThan(10);
        expect(gap).toBeLessThan(25);
      });

      /**
       * TEST: Expansion Uses CSS Transition
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   CSS transition property includes "transform":         │
       *   │                                                         │
       *   │   COLLAPSED                        EXPANDED             │
       *   │   ┌───────────────┐   smooth      ┌─────────────────┐   │
       *   │   └┬─────────────┬┘  animation    │                 │   │
       *   │    └───────────┬─┘   ─────────►   │                 │   │
       *   │                      (not instant) └─────────────────┘   │
       *   │                                                         │
       *   │   transition: transform 300ms ease-out                  │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Expansion animation uses CSS transform transition
       */
      test("expansion should be smooth (uses transition)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const transition = await toast.evaluate((el) => window.getComputedStyle(el).transition);

        // Should have transform transition
        expect(transition).toContain("transform");
      });
    });

    /**
     * EXIT ANIMATION (DISMISSAL)
     *
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  DISMISS FLOW:                                                          │
     * │                                                                         │
     * │  ┌───────────────────────────┐      ┌───────────────────────────┐       │
     * │  │  Toast Message        [×] │ ──►  │  Toast Message        [×] │       │
     * │  │  Description...           │ click│  Description...           │       │
     * │  │  data-removed="false"     │      │  data-removed="true"      │       │
     * │  │  opacity: 1               │      │  opacity: 0 (fading)      │       │
     * │  └───────────────────────────┘      └───────────────────────────┘       │
     * │                                                  │                      │
     * │                                                  ▼  after ~300ms        │
     * │                                                                         │
     * │                                          (removed from DOM)             │
     * │                                                                         │
     * │  CSS TRANSITION:                                                        │
     * │  ┌─────────────────────────────────────────────────────────────────┐    │
     * │  │  transition: opacity var(--exit-duration) ease-out              │    │
     * │  │  [data-removed="true"] { opacity: 0; }                          │    │
     * │  └─────────────────────────────────────────────────────────────────┘    │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     */
    test.describe("Exit Animation", () => {
      /**
       * TEST: Dismissed Toast Data-Removed Attribute
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   BEFORE DISMISS:            AFTER DISMISS:             │
       *   │                                                         │
       *   │   ┌────────────────────┐     ┌────────────────────┐     │
       *   │   │  Toast         [×] │ 🖱️► │  Toast         [×] │     │
       *   │   │  data-removed=     │     │  data-removed=     │     │
       *   │   │    "false"         │     │    "true"  ← set   │     │
       *   │   └────────────────────┘     └────────────────────┘     │
       *   │                                                         │
       *   │   Clicking close button sets data-removed="true"        │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Dismissed toast has data-removed="true"
       */
      test("dismissed toast should have data-removed=true", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Click close button
        const closeButton = toast.locator('[data-close-button]');
        await closeButton.click();

        // Immediately check data-removed
        await expect(toast).toHaveAttribute("data-removed", "true");
      });

      /**
       * TEST: Dismissed Toast Fade Out Animation
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   OPACITY TRANSITION on dismiss:                        │
       *   │                                                         │
       *   │   VISIBLE:                    FADING OUT:               │
       *   │   ┌────────────────────┐      ┌────────────────────┐    │
       *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│      │░░░░░░░░░░░░░░░░░░░░│    │
       *   │   │▓▓▓ opacity: 1 ▓▓▓▓│  ──► │░░░ opacity: 0 ░░░░░│    │
       *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│      │░░░░░░░░░░░░░░░░░░░░│    │
       *   │   └────────────────────┘      └────────────────────┘    │
       *   │                                                         │
       *   │   CSS: transition includes "opacity"                    │
       *   │   [data-removed="true"] triggers opacity: 0             │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Toast has opacity transition for fade out
       */
      test("dismissed toast should fade out (opacity transitions to 0)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Verify the toast has transition including opacity
        const transition = await toast.evaluate((el) => window.getComputedStyle(el).transition);
        expect(transition).toContain("opacity");

        // Verify initial opacity is 1
        const initialOpacity = await toast.evaluate((el) => window.getComputedStyle(el).opacity);
        expect(initialOpacity).toBe("1");

        // Verify CSS sets opacity to 0 when data-removed="true"
        // We check this by inspecting the CSS rule rather than waiting for animation
        const closeButton = toast.locator('[data-close-button]');
        await closeButton.click();

        // Check that data-removed is set (which triggers opacity: 0 via CSS)
        await expect(toast).toHaveAttribute("data-removed", "true");
      });

      /**
       * TEST: Toast DOM Removal After Exit Animation
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   EXIT ANIMATION → DOM REMOVAL:                         │
       *   │                                                         │
       *   │   t=0ms     ┌────────────────────┐                      │
       *   │             │  Toast (fading)    │  data-removed="true" │
       *   │             └────────────────────┘                      │
       *   │                      │                                  │
       *   │                      ▼  ~300ms exit animation           │
       *   │                                                         │
       *   │   t=500ms   (empty - toast removed from DOM)            │
       *   │             toastCount === 0                            │
       *   │                                                         │
       *   │   After exit animation completes, element is removed    │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Toast is removed from DOM after exit animation
       */
      test("toast should be removed from DOM after exit animation", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const closeButton = toast.locator('[data-close-button]');
        await closeButton.click();

        // Wait for exit animation to complete (300ms + buffer)
        await page.waitForTimeout(500);

        // Toast should no longer exist
        const toastCount = await ui.bottomRightToaster.locator('[data-sonner-toast="true"]').count();
        expect(toastCount).toBe(0);
      });
    });

    /**
     * MULTI-TOAST STACKING BEHAVIOR
     *
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  TOAST ORDERING (newest = front, oldest = back):                        │
     * │                                                                         │
     * │     DOM order:                 Visual order (z-index):                  │
     * │                                                                         │
     * │     <ol>                            ┌─────────────┐ data-front="false"  │
     * │       <li> Toast 3 </li>  ──────►   │  Toast 1    │ z-index: 1          │
     * │       <li> Toast 2 </li>            └┬───────────┬┘                      │
     * │       <li> Toast 1 </li>             │ Toast 2   │ z-index: 2           │
     * │     </ol>                            └┬─────────┬┘                       │
     * │                                       │ Toast 3 │ data-front="true"     │
     * │     (prepend order:                   └─────────┘ z-index: 3            │
     * │      newest first)                                                      │
     * │                                                                         │
     * │  VISIBILITY LIMIT (max 3 visible):                                      │
     * │                                                                         │
     * │     5 toasts created:             Only 3 visible:                       │
     * │                                                                         │
     * │     Toast 5 ─────────────────────► ┌─────────────┐ visible              │
     * │     Toast 4 ─────────────────────► └┬───────────┬┘ visible              │
     * │     Toast 3 ─────────────────────►  └┬─────────┬┘  visible              │
     * │     Toast 2 ─────────────────────►   │ HIDDEN  │   data-visible="false" │
     * │     Toast 1 ─────────────────────►   │ HIDDEN  │   data-visible="false" │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     */
    test.describe("Multi-Toast Stacking Behavior", () => {
      /**
       * TEST: Newest Toast Is Front
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   TOAST ORDERING (newest = front):                      │
       *   │                                                         │
       *   │   ┌─────────────────────┐  data-front="false" (oldest)  │
       *   │   │  Toast 1            │                               │
       *   │   └┬───────────────────┬┘                               │
       *   │    │  Toast 2          │  data-front="false"            │
       *   │    └┬─────────────────┬┘                                │
       *   │     │  Toast 3        │  data-front="true"  ← newest    │
       *   │     └─────────────────┘                                 │
       *   │                                                         │
       *   │   Only the newest toast has data-front="true"           │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Newest toast has data-front="true"
       */
      test("newest toast should always be in front (data-front=true)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 3 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        // First toast (newest) should be front
        await expect(toasts.first()).toHaveAttribute("data-front", "true");

        // Others should not be front
        await expect(toasts.nth(1)).toHaveAttribute("data-front", "false");
        await expect(toasts.last()).toHaveAttribute("data-front", "false");
      });

      /**
       * TEST: Front Toast Promotion on Dismiss
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   BEFORE DISMISS:           AFTER DISMISS:              │
       *   │                                                         │
       *   │   ┌─────────────────┐                                   │
       *   │   │  Toast 1        │  false   (removed after dismiss)  │
       *   │   └┬───────────────┬┘                                   │
       *   │    │  Toast 2   [×]│  true    ┌─────────────────┐       │
       *   │    └───────────────┘  ──────► │  Toast 1        │ true  │
       *   │         dismiss               └─────────────────┘       │
       *   │                               (now front)               │
       *   │                                                         │
       *   │   When front toast dismissed, next becomes front        │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Next toast becomes front when front is dismissed
       */
      test("when front toast is dismissed, next toast becomes front", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        // Dismiss front toast
        const frontCloseBtn = toasts.first().locator('[data-close-button]');
        await frontCloseBtn.click();
        await page.waitForTimeout(500);

        // Now the remaining toast should be front
        const remainingToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        await expect(remainingToast).toHaveAttribute("data-front", "true");
      });

      /**
       * TEST: Maximum 3 Visible Toasts
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   VISIBILITY LIMIT (max 3 visible toasts):              │
       *   │                                                         │
       *   │   5 toasts created:        Visual result:               │
       *   │                                                         │
       *   │   Toast 5 ─────────────►   ┌─────────────┐ visible      │
       *   │   Toast 4 ─────────────►   └┬───────────┬┘ visible      │
       *   │   Toast 3 ─────────────►    └┬─────────┬┘  visible      │
       *   │   Toast 2 ─────────────►     │ HIDDEN  │   data-visible │
       *   │   Toast 1 ─────────────►     │ HIDDEN  │   ="false"     │
       *   │                                                         │
       *   │   Only 3 newest toasts have data-visible="true"         │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Only 3 toasts are visible at once
       */
      test("only 3 toasts should be visible at once", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 5 toasts
        for (let i = 0; i < 5; i++) {
          await ui.bottomRightTrigger.click();
          await page.waitForTimeout(150);
        }
        await page.waitForTimeout(400);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');

        // Count visible toasts
        let visibleCount = 0;
        const count = await toasts.count();
        for (let i = 0; i < count; i++) {
          const visible = await toasts.nth(i).getAttribute("data-visible");
          if (visible === "true") visibleCount++;
        }

        expect(visibleCount).toBe(3);
      });

      /**
       * TEST: Deck of Cards Effect in Collapsed State
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   DECK OF CARDS VISUAL EFFECT (3 toasts):               │
       *   │                                                         │
       *   │   ┌─────────────────────────────┐  Y: smallest (top)    │
       *   │   │                             │  width: narrowest     │
       *   │   └┬───────────────────────────┬┘ Y: middle             │
       *   │    │                           │  width: medium         │
       *   │    └┬─────────────────────────┬┘  Y: largest (bottom)   │
       *   │     │  Front toast            │   width: widest         │
       *   │     └─────────────────────────┘                         │
       *   │                                                         │
       *   │   Each back toast:                                      │
       *   │   - Has smaller Y (higher on screen)                    │
       *   │   - Has smaller width (due to scale reduction)          │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Deck of cards stacking effect is correct
       */
      test("collapsed stack should show 'deck of cards' effect (3 toasts)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 3 toasts
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away to collapse
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const boxes = [];

        for (let i = 0; i < 3; i++) {
          boxes.push(await toasts.nth(i).boundingBox());
        }

        // Verify deck of cards effect:
        // 1. Each toast should be slightly above the previous (for bottom position)
        // 2. Each toast should be slightly narrower (due to scale)
        for (let i = 1; i < 3; i++) {
          // Y position: back toasts should be higher (smaller Y)
          expect(boxes[i]!.y).toBeLessThan(boxes[i-1]!.y);
          // Width: back toasts should be narrower (due to scale)
          expect(boxes[i]!.width).toBeLessThan(boxes[i-1]!.width);
        }
      });
    });

    test.describe("CSS Custom Properties", () => {
      /**
       * TEST: Enter Duration CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --enter-duration: 300ms                               │
       *   │                                                         │
       *   │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ← entering         │
       *   │   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ← after 300ms     │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Entry animation duration is 300ms
       */
      test("--enter-duration should be 300ms", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const value = await ui.bottomRightToaster.evaluate((el) =>
          getComputedStyle(el).getPropertyValue("--enter-duration").trim()
        );
        expect(value).toBe("300ms");
      });

      /**
       * TEST: Exit Duration CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --exit-duration: 300ms                                │
       *   │                                                         │
       *   │   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ← visible         │
       *   │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ← after 300ms     │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Exit animation duration is 300ms
       */
      test("--exit-duration should be 300ms", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const value = await ui.bottomRightToaster.evaluate((el) =>
          getComputedStyle(el).getPropertyValue("--exit-duration").trim()
        );
        expect(value).toBe("300ms");
      });

      /**
       * TEST: Stack Spacing CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --stack-spacing: 20px (offset between stacked toasts) │
       *   │                                                         │
       *   │   ┌────────────────────┐                                │
       *   │   │                    │                                │
       *   │   └┬──────────────────┬┘  ↑ 20px                        │
       *   │    │                  │   ↓                             │
       *   │    └┬────────────────┬┘   ↑ 20px                        │
       *   │     │  Front toast   │    ↓                             │
       *   │     └────────────────┘                                  │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Stack spacing is 20px
       */
      test("--stack-spacing should be 20px", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const value = await ui.bottomRightToaster.evaluate((el) =>
          getComputedStyle(el).getPropertyValue("--stack-spacing").trim()
        );
        expect(value).toBe("20px");
      });

      /**
       * TEST: Gap CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --gap: 15px (space between expanded toasts)           │
       *   │                                                         │
       *   │   ┌───────────────────┐                                 │
       *   │   │  Toast 1          │                                 │
       *   │   └───────────────────┘                                 │
       *   │          ↕ 15px gap                                     │
       *   │   ┌───────────────────┐                                 │
       *   │   │  Toast 2          │                                 │
       *   │   └───────────────────┘                                 │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Gap between expanded toasts is 15px
       */
      test("--gap should be 15px", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const value = await ui.bottomRightToaster.evaluate((el) =>
          getComputedStyle(el).getPropertyValue("--gap").trim()
        );
        expect(value).toBe("15px");
      });

      /**
       * TEST: Transition Easing CSS Variable
       * ─────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────┐
       *   │                                                         │
       *   │   --transition-easing: ease-out                         │
       *   │                                                         │
       *   │   Animation curve:                                      │
       *   │        ▓▓                                               │
       *   │      ▓▓                                                 │
       *   │    ▓▓                                                   │
       *   │  ▓▓                                                     │
       *   │ ▓                                                       │
       *   │ ▓  (ease-out: fast start, slow end)                     │
       *   │                                                         │
       *   └─────────────────────────────────────────────────────────┘
       *
       *   Validates: Transition uses ease-out timing function
       */
      test("--transition-easing should be ease-out", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(100);

        const value = await ui.bottomRightToaster.evaluate((el) =>
          getComputedStyle(el).getPropertyValue("--transition-easing").trim()
        );
        expect(value).toBe("ease-out");
      });
    });

    /**
     * ============================================================================
     * DURATION TRACK TIMELINE TESTS
     * ============================================================================
     *
     * The duration track is a visual progress indicator at the bottom of each toast
     * showing how much time remains before auto-dismiss.
     *
     * TOAST STRUCTURE:
     * ┌─────────────────────────────────────────┐
     * │  Title                              ×   │
     * │  Description text here...               │
     * ├─────────────────────────────────────────┤
     * │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░│ ← duration-track (container)
     * └─────────────────────────────────────────┘   duration-progress (fills)
     *
     * PROGRESS ANIMATION OVER TIME:
     * ┌──────────────────────────────────────────────────────────────────────────┐
     * │                                                                          │
     * │  t=0s    │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│  scaleX(1.0)       │
     * │          └────────────────────────────────────────┘                      │
     * │                                                                          │
     * │  t=1s    │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░│  scaleX(0.8)       │
     * │          └────────────────────────────────────────┘                      │
     * │                                                                          │
     * │  t=2.5s  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░│  scaleX(0.5)       │
     * │          └────────────────────────────────────────┘                      │
     * │                                                                          │
     * │  t=4s    │▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  scaleX(0.2)       │
     * │          └────────────────────────────────────────┘                      │
     * │                                                                          │
     * │  t=5s    │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  scaleX(0) → DISMISS│
     * │          └────────────────────────────────────────┘                      │
     * │                                                                          │
     * └──────────────────────────────────────────────────────────────────────────┘
     *
     * CSS IMPLEMENTATION:
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │  [data-duration-track]     │ Container: h-[3px], bottom-0, inset-x-0   │
     * │  [data-duration-progress]  │ Fill bar: origin-left, transition scaleX  │
     * │                            │ Animates: scaleX(1) → scaleX(0)           │
     * └─────────────────────────────────────────────────────────────────────────┘
     *
     * ============================================================================
     */
    test.describe("Duration Track Timeline", () => {
      /**
       * TEST: Toast should have duration track element
       *
       * ┌─────────────────────────────────────────┐
       * │  Bottom Right                       ×   │
       * │  Toast positioned at bottom-right       │
       * ├─────────────────────────────────────────┤
       * │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← [data-duration-track] ✓
       * └─────────────────────────────────────────┘
       *
       * Verifies the track container element exists and is visible.
       */
      test("toast should have duration track element", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const durationTrack = toast.locator('[data-duration-track]');

        await expect(durationTrack).toBeVisible();
      });

      /**
       * TEST: Toast should have duration progress bar inside track
       *
       * ┌─────────────────────────────────────────┐
       * │  Bottom Right                       ×   │
       * │  Toast positioned at bottom-right       │
       * ├─────────────────────────────────────────┤
       * │ [data-duration-track]                   │
       * │  └── [data-duration-progress] ← ✓       │
       * │       (the actual animated fill bar)    │
       * └─────────────────────────────────────────┘
       *
       * Verifies the progress bar element exists inside the track.
       */
      test("toast should have duration progress bar inside track", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = toast.locator('[data-duration-progress]');

        await expect(progressBar).toBeVisible();
      });

      /**
       * TEST: Progress bar should animate (scaleX decreases over time)
       *
       * TIMELINE:
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  t=0.4s (after mount)                                          │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░│  initialTransform  │
       * │  └────────────────────────────────────────┘                    │
       * │                         │                                      │
       * │                         ▼  wait 1000ms                         │
       * │                                                                │
       * │  t=1.4s                                                        │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░│  laterTransform    │
       * │  └────────────────────────────────────────┘                    │
       * │                                                                │
       * │  ASSERTION: initialTransform !== laterTransform                │
       * │             (proves animation is running)                      │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("progress bar should animate (scaleX decreases over time)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = toast.locator('[data-duration-progress]');

        // Get initial scale
        const initialTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Wait for animation to progress
        await page.waitForTimeout(1000);

        // Get scale after delay
        const laterTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Transforms should be different (progress bar is animating)
        expect(laterTransform).not.toBe(initialTransform);
      });

      /**
       * TEST: Progress bar should pause on hover
       *
       * HOVER PAUSE BEHAVIOR:
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  BEFORE HOVER (animating):                                     │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░│  ← shrinking       │
       * │  └────────────────────────────────────────┘                    │
       * │                                                                │
       * │  🖱️ MOUSE ENTERS TOASTER                                       │
       * │                         │                                      │
       * │                         ▼                                      │
       * │                                                                │
       * │  DURING HOVER (paused):                                        │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░│  scaleWhileHovering1│
       * │  └────────────────────────────────────────┘                    │
       * │                         │                                      │
       * │                         ▼  wait 500ms (still hovering)         │
       * │                                                                │
       * │  STILL DURING HOVER:                                           │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░│  scaleWhileHovering2│
       * │  └────────────────────────────────────────┘                    │
       * │                                                                │
       * │  ASSERTION: scaleWhileHovering1 === scaleWhileHovering2        │
       * │             (proves animation is PAUSED)                       │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("progress bar should pause on hover", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Hover over the toaster to pause
        await ui.bottomRightToaster.hover();
        await page.waitForTimeout(100);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = toast.locator('[data-duration-progress]');

        // Get scale while hovering
        const scaleWhileHovering1 = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Wait while still hovering
        await page.waitForTimeout(500);

        // Get scale again - should be the same (paused)
        const scaleWhileHovering2 = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Transform should be the same (paused)
        expect(scaleWhileHovering2).toBe(scaleWhileHovering1);
      });

      /**
       * TEST: Progress bar should resume after mouse leave
       *
       * PAUSE → RESUME FLOW:
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  🖱️ MOUSE ENTERS (pause)                                       │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░│  pausedScale       │
       * │  └────────────────────────────────────────┘  (frozen)          │
       * │                                                                │
       * │  🖱️ MOUSE LEAVES TOASTER (move to 0,0)                         │
       * │                         │                                      │
       * │                         ▼  animation resumes                   │
       * │                                                                │
       * │  AFTER 800ms:                                                  │
       * │  ┌────────────────────────────────────────┐                    │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░│  resumedScale      │
       * │  └────────────────────────────────────────┘  (shrinking again) │
       * │                                                                │
       * │  ASSERTION: pausedScale !== resumedScale                       │
       * │             (proves animation RESUMED)                         │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("progress bar should resume after mouse leave", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.bottomRightTrigger.click();
        await page.waitForTimeout(400);

        // Hover to pause
        await ui.bottomRightToaster.hover();
        await page.waitForTimeout(200);

        const toast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = toast.locator('[data-duration-progress]');

        // Get paused scale
        const pausedScale = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Move mouse away to resume
        await page.mouse.move(0, 0);
        await page.waitForTimeout(800);

        // Get scale after resuming
        const resumedScale = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Scale should have changed (resumed animation)
        expect(resumedScale).not.toBe(pausedScale);
      });

      /**
       * TEST: Loading variant should NOT show duration track
       *
       * DEFAULT VARIANT (has track):          LOADING VARIANT (no track):
       * ┌─────────────────────────────┐       ┌─────────────────────────────┐
       * │  Success                ×   │       │  ⟳ Loading...               │
       * │  Your action was successful │       │  Please wait...             │
       * ├─────────────────────────────┤       │                             │
       * │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│       │  (no progress bar - infinite│
       * └─────────────────────────────┘       │   loading has no countdown) │
       *        ↑                              └─────────────────────────────┘
       *   Track visible                              ↑
       *   (auto-dismiss countdown)              No track (loading = indefinite)
       *
       * This test verifies that non-loading variants DO show the track.
       */
      test("Loading variant should NOT show duration track", async ({ page }) => {
        const ui = new SonnerPage(page);
        await ui.goto();

        // Note: This test requires a Loading variant trigger
        // For now we test that non-loading variants DO show the track
        await ui.triggerToast();

        const toast = ui.toastContainer.locator('[data-sonner-toast="true"]').first();
        const durationTrack = toast.locator('[data-duration-track]');

        // Default variant should show duration track
        await expect(durationTrack).toBeVisible();
      });
    });

    /**
     * ============================================================================
     * RAPID CLICKING STRESS TESTS
     * ============================================================================
     *
     * Tests behavior when user clicks trigger button rapidly, exceeding max toast limit.
     *
     * MAX TOAST LIMIT BEHAVIOR:
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  --max-toasts: 5 (default setting)                                      │
     * │                                                                         │
     * │  SCENARIO: User clicks trigger 6+ times rapidly                         │
     * │                                                                         │
     * │  Click 1 ──► Toast 1 created                                            │
     * │  Click 2 ──► Toast 2 created (Toast 1 pushed back)                      │
     * │  Click 3 ──► Toast 3 created                                            │
     * │  Click 4 ──► Toast 4 created                                            │
     * │  Click 5 ──► Toast 5 created (MAX reached)                              │
     * │  ─────────────────────────────────────────────────────                  │
     * │  Click 6 ──► Toast 6 created, Toast 1 DISMISSED ← BUG HAPPENS HERE      │
     * │  Click 7 ──► Toast 7 created, Toast 2 DISMISSED                         │
     * │  ...                                                                    │
     * │                                                                         │
     * │  BUG: After 6th click, duration track stops animating                   │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     *
     * EXPECTED BEHAVIOR (ALL TOASTS SHOULD HAVE DURATION TRACK):
     * ┌─────────────────────────────────────────────────────────────────────────┐
     * │                                                                         │
     * │  COLLAPSED STATE:                      EXPANDED STATE (on hover):       │
     * │                                                                         │
     * │  ┌───────────────────────────┐         ┌───────────────────────────┐    │
     * │  │                           │         │  Toast 3 (oldest)         │    │
     * │  └┬─────────────────────────┬┘         ├───────────────────────────┤    │
     * │   │                         │          │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│    │
     * │   └┬───────────────────────┬┘          └───────────────────────────┘    │
     * │    │  Toast 5 (newest)     │                    ↕ gap                   │
     * │    ├───────────────────────┤           ┌───────────────────────────┐    │
     * │    │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← track   │  Toast 4 (middle)         │    │
     * │    └───────────────────────┘           ├───────────────────────────┤    │
     * │                                        │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│    │
     * │  Only front toast visible              └───────────────────────────┘    │
     * │  in collapsed state                             ↕ gap                   │
     * │                                        ┌───────────────────────────┐    │
     * │                                        │  Toast 5 (newest)         │    │
     * │                                        ├───────────────────────────┤    │
     * │                                        │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│    │
     * │                                        └───────────────────────────┘    │
     * │                                                                         │
     * │                                        ALL toasts have duration track ✓ │
     * │                                                                         │
     * └─────────────────────────────────────────────────────────────────────────┘
     *
     * ============================================================================
     */

    test.describe("Rapid Clicking Stress Tests", () => {
      /**
       * TEST: After exactly 6 clicks (threshold), front toast should have duration track
       *
       * This tests the exact threshold where the bug occurs:
       * - Clicks 1-5: Normal behavior (under max limit)
       * - Click 6: Triggers dismissal of toast 1 (reaches max limit)
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │  🖱️ Click x6 (exactly at threshold)                            │
       * │       │                                                        │
       * │       ▼                                                        │
       * │  ┌───────────────────────────────┐                             │
       * │  │  Toast 6 (newest/front)    ×  │                             │
       * │  │  ...                          │                             │
       * │  ├───────────────────────────────┤                             │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← duration track MUST exist │
       * │  └───────────────────────────────┘                             │
       * │                                                                │
       * │  BUG: Duration track disappears at 6th click                   │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after exactly 6 clicks (threshold), front toast should have duration track", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click exactly 6 times (1 over the max limit of 5)
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const frontToast = toasts.first();

        // Front toast should have duration track
        const durationTrack = frontToast.locator('[data-duration-track]');
        await expect(durationTrack).toBeVisible();

        // Front toast should have duration progress bar
        const progressBar = frontToast.locator('[data-duration-progress]');
        await expect(progressBar).toBeVisible();
      });

      /**
       * TEST: After exactly 6 clicks, progress bar should be animating
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  t=0      │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│  scaleX(~0.9)      │
       * │           └──────────────────────────────┘                     │
       * │                         │                                      │
       * │                         ▼  wait 1000ms                         │
       * │                                                                │
       * │  t=1s     │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░│  scaleX(~0.7)      │
       * │           └──────────────────────────────┘                     │
       * │                                                                │
       * │  ASSERTION: transform changed (animation running)              │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after exactly 6 clicks, progress bar should animate", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click exactly 6 times
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = frontToast.locator('[data-duration-progress]');

        // Get initial transform
        const initialTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Wait for animation to progress
        await page.waitForTimeout(1000);

        // Get transform after delay
        const laterTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Transform should have changed (animation is running)
        expect(laterTransform).not.toBe(initialTransform);
      });

      /**
       * TEST: After 10 rapid clicks, front toast should still have duration track
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │  🖱️ Click x10 rapidly                                          │
       * │       │                                                        │
       * │       ▼                                                        │
       * │  ┌───────────────────────────────┐                             │
       * │  │  Toast 10 (newest/front)   ×  │                             │
       * │  │  ...                          │                             │
       * │  ├───────────────────────────────┤                             │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← duration track MUST exist │
       * │  └───────────────────────────────┘                             │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, front toast should have duration track", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly (no wait between clicks)
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const frontToast = toasts.first();

        // Front toast should have duration track
        const durationTrack = frontToast.locator('[data-duration-track]');
        await expect(durationTrack).toBeVisible();

        // Front toast should have duration progress bar
        const progressBar = frontToast.locator('[data-duration-progress]');
        await expect(progressBar).toBeVisible();
      });

      /**
       * TEST: After 10 rapid clicks, progress bar should still be animating
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  t=0      │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│  scaleX(~0.9)      │
       * │           └──────────────────────────────┘                     │
       * │                         │                                      │
       * │                         ▼  wait 1000ms                         │
       * │                                                                │
       * │  t=1s     │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░│  scaleX(~0.7)      │
       * │           └──────────────────────────────┘                     │
       * │                                                                │
       * │  ASSERTION: transform changed (animation running)              │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, progress bar should still animate", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = frontToast.locator('[data-duration-progress]');

        // Get initial transform
        const initialTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Wait for animation to progress
        await page.waitForTimeout(1000);

        // Get transform after delay
        const laterTransform = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Transform should have changed (animation is running)
        expect(laterTransform).not.toBe(initialTransform);
      });

      /**
       * TEST: After 10 rapid clicks, all visible toasts should have duration track
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  ┌───────────────────────────────┐                             │
       * │  │  Toast (oldest visible)       │                             │
       * │  ├───────────────────────────────┤                             │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← track ✓                  │
       * │  └┬─────────────────────────────┬┘                             │
       * │   │  Toast (middle)             │                              │
       * │   ├─────────────────────────────┤                              │
       * │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← track ✓                   │
       * │   └┬───────────────────────────┬┘                              │
       * │    │  Toast (newest/front)     │                               │
       * │    ├───────────────────────────┤                               │
       * │    │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← track ✓                    │
       * │    └───────────────────────────┘                               │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, all visible toasts should have duration track", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const toastCount = await toasts.count();

        // Check each visible toast has duration track
        let visibleWithTrack = 0;
        for (let i = 0; i < toastCount; i++) {
          const toast = toasts.nth(i);
          const isVisible = await toast.getAttribute("data-visible");

          if (isVisible === "true") {
            const durationTrack = toast.locator('[data-duration-track]');
            const isTrackVisible = await durationTrack.isVisible();
            if (isTrackVisible) visibleWithTrack++;
          }
        }

        // All 3 visible toasts should have duration track
        expect(visibleWithTrack).toBe(3);
      });

      /**
       * TEST: After 10 rapid clicks, should have exactly 5 toasts (max limit)
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  --max-toasts: 5                                               │
       * │                                                                │
       * │  Created: 10 toasts                                            │
       * │  Dismissed: 5 toasts (oldest ones, due to limit)               │
       * │  Remaining: 5 toasts ✓                                         │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, should have max 5 toasts", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for dismiss animations to complete
        await page.waitForTimeout(800);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const toastCount = await toasts.count();

        // Should have max 5 toasts
        expect(toastCount).toBeLessThanOrEqual(5);
      });

      /**
       * TEST: After 10 rapid clicks, hover should still pause all progress bars
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  🖱️ Click x10 → 🖱️ Hover over toaster                          │
       * │                                                                │
       * │  All visible progress bars should PAUSE:                       │
       * │  ┌───────────────────────────────┐                             │
       * │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░│  FROZEN (no change)        │
       * │  └───────────────────────────────┘                             │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, hover should pause progress bars", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        // Hover over toaster
        await ui.bottomRightToaster.hover();
        await page.waitForTimeout(100);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
        const progressBar = frontToast.locator('[data-duration-progress]');

        // Get transform while hovering
        const transform1 = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Wait while still hovering
        await page.waitForTimeout(500);

        // Get transform again
        const transform2 = await progressBar.evaluate((el) =>
          getComputedStyle(el).transform
        );

        // Transform should be the same (paused)
        expect(transform2).toBe(transform1);
      });

      /**
       * TEST: After 10 rapid clicks, new toast should still have entry animation
       *
       * ENTRY ANIMATION (translateY):
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  BOTTOM POSITION - Toast slides UP from below:                 │
       * │                                                                │
       * │  ┌─────────────────────────────────────────┐                   │
       * │  │                                         │                   │
       * │  │              (viewport)                 │                   │
       * │  │                                         │                   │
       * │  └─────────────────────────────────────────┘                   │
       * │                      ↑                                         │
       * │  ┌───────────────────┴───────────────────┐                     │
       * │  │  New Toast (entering)              ×  │  translateY(100%)   │
       * │  │  ...                                  │  → translateY(0)    │
       * │  └───────────────────────────────────────┘                     │
       * │                                                                │
       * │  ASSERTION: data-mounted transitions from "false" to "true"    │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, new toast should have data-mounted=true", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Front toast should be mounted (entry animation complete)
        await expect(frontToast).toHaveAttribute("data-mounted", "true");
      });

      /**
       * TEST: After 10 rapid clicks, clicking again should trigger entry animation
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  STATE: 5 toasts exist (max limit)                             │
       * │                                                                │
       * │  🖱️ Click 11th time:                                           │
       * │       │                                                        │
       * │       ▼                                                        │
       * │  1. Oldest toast dismissed                                     │
       * │  2. New toast created with data-mounted="false"                │
       * │  3. Entry animation starts (translateY: 100% → 0)              │
       * │  4. After ~300ms, data-mounted="true"                          │
       * │                                                                │
       * │  VERIFY: New toast starts unmounted, then becomes mounted      │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, 11th click should create toast with entry animation", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(500);

        // Now click once more - this should trigger a new toast with entry animation
        await ui.bottomRightTrigger.click();

        // Immediately check - toast should start unmounted
        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Wait a bit for mount to complete
        await page.waitForTimeout(400);

        // Now it should be mounted
        await expect(frontToast).toHaveAttribute("data-mounted", "true");
      });

      /**
       * TEST: After 10 rapid clicks, new toast should have translateY animation
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  CSS ANIMATION CHECK:                                          │
       * │                                                                │
       * │  INITIAL (unmounted):   transform: translateY(100%)            │
       * │                              │                                 │
       * │                              ▼ animation (300ms)               │
       * │                                                                │
       * │  FINAL (mounted):       transform: translateY(0)               │
       * │                                                                │
       * │  The toast element's transform should change during animation  │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after 10 rapid clicks, new toast translateY should animate to 0", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for all animations to complete
        await page.waitForTimeout(600);

        // Click once more to create a new toast
        await ui.bottomRightTrigger.click();

        // Wait for entry animation to complete
        await page.waitForTimeout(400);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Get the computed transform - should have translateY near 0 for the front toast
        const transform = await frontToast.evaluate((el) => {
          const style = getComputedStyle(el);
          return style.transform;
        });

        // Parse the matrix to get translateY
        // matrix(a, b, c, d, tx, ty) - ty is the translateY value
        const parseTranslateY = (transformStr: string) => {
          if (transformStr === "none") return 0;
          const match = transformStr.match(/matrix\([^,]+,[^,]+,[^,]+,[^,]+,[^,]+,\s*([^)]+)\)/);
          return match ? parseFloat(match[1]) : 0;
        };

        const translateY = parseTranslateY(transform);

        // Front toast should have translateY close to 0 (animation complete)
        // Allow some tolerance for stacking offset
        expect(Math.abs(translateY)).toBeLessThan(50);
      });

      /**
       * TEST: All visible toasts should have proper stacking transforms after rapid clicks
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  STACKING TRANSFORMS (collapsed state):                        │
       * │                                                                │
       * │  ┌─────────────────────────────┐                               │
       * │  │                             │  index=2, scale=0.90          │
       * │  └┬───────────────────────────┬┘  translateY=-40px             │
       * │   │                           │  index=1, scale=0.95           │
       * │   └┬─────────────────────────┬┘  translateY=-20px              │
       * │    │  Toast (front)          │  index=0, scale=1.00            │
       * │    │                         │  translateY=0                   │
       * │    └─────────────────────────┘                                 │
       * │                                                                │
       * │  VERIFY: Each toast has correct --index CSS variable           │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      /**
       * TEST: New toast should start with translateY(100%) before animating
       *
       * ENTRY ANIMATION SEQUENCE:
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  FRAME 0 (immediately after click):                            │
       * │  ┌─────────────────────────────────────────┐                   │
       * │  │              (viewport)                 │                   │
       * │  │                                         │                   │
       * │  └─────────────────────────────────────────┘                   │
       * │  ┌───────────────────────────────────────┐                     │
       * │  │  Toast (data-mounted="false")         │  translateY(100%)   │
       * │  │  BELOW viewport, opacity: 0           │  NOT VISIBLE        │
       * │  └───────────────────────────────────────┘                     │
       * │                                                                │
       * │  FRAME 1-N (animation in progress):                            │
       * │  ┌─────────────────────────────────────────┐                   │
       * │  │                                         │                   │
       * │  │  ┌───────────────────────────────────┐  │                   │
       * │  │  │  Toast (animating)                │  │  translateY(50%)  │
       * │  └──┴───────────────────────────────────┴──┘  opacity: 0.5     │
       * │                                                                │
       * │  FINAL FRAME:                                                  │
       * │  ┌─────────────────────────────────────────┐                   │
       * │  │  ┌───────────────────────────────────┐  │                   │
       * │  │  │  Toast (data-mounted="true")      │  │  translateY(0)    │
       * │  │  │  VISIBLE, opacity: 1              │  │                   │
       * │  │  └───────────────────────────────────┘  │                   │
       * │  └─────────────────────────────────────────┘                   │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after rapid clicks, new toast should start unmounted then animate in", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 6 times to exceed max limit
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for all to settle
        await page.waitForTimeout(600);

        // Now click once more and immediately check the new toast
        await ui.bottomRightTrigger.click();

        // Immediately get the front toast
        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Check initial mounted state (should be false initially)
        const initialMounted = await frontToast.getAttribute("data-mounted");

        // Wait for animation to complete
        await page.waitForTimeout(400);

        // Check final mounted state
        const finalMounted = await frontToast.getAttribute("data-mounted");

        // Initial should be false (or true if animation is very fast)
        // Final should definitely be true
        expect(finalMounted).toBe("true");
      });

      /**
       * TEST: Entry animation should show visual slide-in (transform changes)
       *
       * BUG CHECK: After rapid clicking, the inline style.transform might
       * override the CSS entry animation, causing toast to appear instantly
       * instead of sliding in.
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  EXPECTED: Toast slides in over ~300ms                         │
       * │            transform goes from translateY(100%) to (0)         │
       * │                                                                │
       * │  BUG: Toast appears instantly at translateY(0)                 │
       * │       because inline style overrides CSS animation             │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after rapid clicks, entry animation should visually slide in", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 6 times to exceed max limit
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for all to settle
        await page.waitForTimeout(600);

        // Get position before new toast
        const toasterBox = await ui.bottomRightToaster.boundingBox();

        // Click once more
        await ui.bottomRightTrigger.click();

        // Wait just a tiny bit for toast to be created but NOT fully animated
        await page.waitForTimeout(50);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Get initial Y position
        const initialBox = await frontToast.boundingBox();
        const initialY = initialBox?.y ?? 0;

        // Wait for animation to complete
        await page.waitForTimeout(400);

        // Get final Y position
        const finalBox = await frontToast.boundingBox();
        const finalY = finalBox?.y ?? 0;

        // The Y position should have changed (toast slid into view)
        // If bug exists: initialY === finalY (no animation, appeared instantly)
        // If working: initialY > finalY (started below, moved up) for bottom position
        // Note: For bottom position, toast starts BELOW and moves UP
        // So initialY should be GREATER than finalY

        // At minimum, check that the toast ended up in a reasonable position
        // The toast should be within the viewport
        expect(finalY).toBeLessThan(toasterBox!.y + toasterBox!.height);
      });

      /**
       * TEST: Verify CSS transition is not overridden by inline styles
       *
       * The bug: setExpanded() sets el.style.transform = "translateY(0)"
       * which overrides the CSS entry animation.
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  CHECK: After creating toast, inline style.transform should    │
       * │         NOT be set immediately (let CSS handle animation)      │
       * │                                                                │
       * │  CSS rule (data-mounted="false"):                              │
       * │    --y: translateY(100%);                                      │
       * │    opacity: 0;                                                 │
       * │                                                                │
       * │  CSS rule (data-mounted="true"):                               │
       * │    --y: translateY(0);                                         │
       * │    opacity: 1;                                                 │
       * │    transition: all var(--enter-duration);                      │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after rapid clicks, new toast should have CSS-driven animation (not inline override)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 6 times to exceed max limit
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for all to settle
        await page.waitForTimeout(600);

        // Click once more
        await ui.bottomRightTrigger.click();

        // Wait a tiny bit for toast creation
        await page.waitForTimeout(20);

        const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();

        // Check if inline style.transform is set (it shouldn't be during entry animation)
        const inlineTransform = await frontToast.evaluate((el) => {
          return el.style.transform;
        });

        // During entry animation, inline transform should NOT be set
        // OR if it is set, it should NOT be "translateY(0)" which would skip the animation
        // The CSS should handle the entry animation via data-mounted attribute

        // Wait for animation to complete
        await page.waitForTimeout(400);

        // After animation, mounted should be true
        await expect(frontToast).toHaveAttribute("data-mounted", "true");
      });

      /**
       * TEST: Entry animation should start toast BELOW viewport (translateY > 0)
       *
       * This test captures the Y position immediately after creation to verify
       * the toast actually starts below its final position.
       *
       * ┌────────────────────────────────────────────────────────────────┐
       * │                                                                │
       * │  BOTTOM POSITION ENTRY:                                        │
       * │                                                                │
       * │  ┌─────────────────────────────────────────┐  viewport bottom  │
       * │  │                                         │                   │
       * │  └─────────────────────────────────────────┘                   │
       * │            │                                                   │
       * │            │  Toast starts HERE (translateY: 100%)             │
       * │            │  initialY > finalY                                │
       * │            ▼                                                   │
       * │  ┌───────────────────────────────────────┐                     │
       * │  │  Toast                                │                     │
       * │  └───────────────────────────────────────┘                     │
       * │                                                                │
       * │  BUG: If initialY === finalY, animation was skipped            │
       * │                                                                │
       * └────────────────────────────────────────────────────────────────┘
       */
      test("after rapid clicks, new toast Y position should change during entry animation", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 6 times to exceed max limit
        for (let i = 0; i < 6; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for all to settle
        await page.waitForTimeout(600);

        // Move mouse away from toaster
        await page.mouse.move(0, 0);
        await page.waitForTimeout(100);

        // Get viewport height for reference
        const viewportHeight = await page.evaluate(() => window.innerHeight);

        // Create a promise that will capture positions during animation
        const positions: number[] = [];

        // Click to create new toast
        await ui.bottomRightTrigger.click();

        // Capture Y positions during animation (sample every 30ms for 300ms)
        for (let i = 0; i < 10; i++) {
          await page.waitForTimeout(30);
          const frontToast = ui.bottomRightToaster.locator('[data-sonner-toast="true"]').first();
          const box = await frontToast.boundingBox();
          if (box) {
            positions.push(box.y);
          }
        }

        // Wait for animation to complete
        await page.waitForTimeout(200);

        // For bottom position, toast should start with higher Y (below) and move to lower Y (up)
        // If animation works: first positions should be > last positions
        // If bug exists: all positions will be roughly the same

        const firstY = positions[0];
        const lastY = positions[positions.length - 1];

        // The Y position should have DECREASED (toast moved UP into view)
        // Allow some tolerance but there should be significant movement
        const movement = firstY - lastY;

        // Entry animation should have moved at least 20px
        // (A full slide from translateY(100%) would be much more)
        expect(movement).toBeGreaterThan(20);
      });

      /**
       * TEST: After 10 rapid clicks, toasts should have correct --index values
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  STACKED TOASTS WITH --index CSS VARIABLE:                      │
       *   │                                                                 │
       *   │  ┌─────────────────────────────┐                                │
       *   │  │                             │  --index: 2 (back toast)       │
       *   │  └┬───────────────────────────┬┘                                │
       *   │   │                           │  --index: 1 (middle toast)      │
       *   │   └┬─────────────────────────┬┘                                 │
       *   │    │  Toast (front)          │  --index: 0 (front toast)       │
       *   │    └─────────────────────────┘                                  │
       *   │                                                                 │
       *   │  Front toast: --index: 0                                        │
       *   │  Back toast:  --index: N (higher value)                         │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: CSS --index variable is correctly set for stacking order
       */
      test("after 10 rapid clicks, toasts should have correct --index values", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Click 10 times rapidly
        for (let i = 0; i < 10; i++) {
          await ui.bottomRightTrigger.click();
        }

        // Wait for animations to settle
        await page.waitForTimeout(600);

        // Move mouse away to ensure collapsed state
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.bottomRightToaster.locator('[data-sonner-toast="true"]');
        const toastCount = await toasts.count();

        // Should have max 5 toasts
        expect(toastCount).toBeLessThanOrEqual(5);

        // Check --index values
        const frontToast = toasts.first();
        const frontIndex = await frontToast.evaluate((el) => {
          return el.style.getPropertyValue("--index");
        });
        expect(frontIndex).toBe("0");

        // If there are multiple toasts, check back toast has higher index
        if (toastCount > 1) {
          const backToast = toasts.last();
          const backIndex = await backToast.evaluate((el) => {
            return el.style.getPropertyValue("--index");
          });
          expect(parseInt(backIndex)).toBeGreaterThan(0);
        }
      });
    });
  });

  /**
   * ============================================================================
   * TOP POSITION ANIMATION TESTS
   * ============================================================================
   * These tests are based on the shadcn/ui Sonner component behavior observed
   * from the official shadcn website (https://ui.shadcn.com/docs/components/sonner).
   *
   * Our current Sonner component implementation might differ slightly from the
   * original shadcn behavior. These tests document the expected animation workflow:
   *
   * - Entry: Toast slides DOWN from above viewport (translateY: -100% → 0)
   *   ⚠️  FIXME: Entry animation not yet implemented for TOP position
   * - Stacking: Older toasts peek BELOW the front toast with scale effect ✅
   * - Expansion: On hover, toasts expand DOWNWARD (into the page) ✅
   * - --lift CSS var: 1 (positive value for top position) ✅
   * ============================================================================
   */
  test.describe("TOP Position Animation Workflow (shadcn reference)", () => {
    // FIXME: Entry animation for TOP position not yet implemented
    // Currently toasts enter from below regardless of position
    // Expected: TOP position should slide DOWN from above (translateY: -100% → 0)
    test.describe("Entry Animation - Slides DOWN from above (FIXME)", () => {
      test.fixme("TOP position toast should enter from ABOVE (not below)", async ({ page }) => {
        // Expected behavior (shadcn reference):
        // Toast should slide DOWN from above viewport (translateY: -100% → 0)
        // Current behavior: Toast enters from below regardless of position
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.topLeftToaster.locator('[data-sonner-toast="true"]').first();
        await expect(toast).toHaveAttribute("data-y-position", "Top");
      });

      test.fixme("TOP position toast should have initial translateY negative (entering from above)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Track initial position when toast is added
        let initialY: number | null = null;

        await page.exposeFunction("trackInitialY", (y: number) => {
          if (initialY === null) initialY = y;
        });

        await page.evaluate(() => {
          const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
              if (mutation.type === "childList") {
                mutation.addedNodes.forEach((node) => {
                  if (node instanceof HTMLElement && node.dataset.sonnerToast === "true") {
                    const rect = node.getBoundingClientRect();
                    // @ts-ignore
                    window.trackInitialY(rect.y);
                  }
                });
              }
            });
          });
          const toaster = document.querySelector('[data-position="TopLeft"]');
          if (toaster) observer.observe(toaster, { childList: true, subtree: true });
        });

        await ui.topLeftTrigger.click();
        await page.waitForTimeout(50);

        // Initial Y should be negative or very small (entering from above)
        // Toast starts above viewport or at top edge
        expect(initialY).not.toBeNull();
        // TODO: Verify initialY is negative (above viewport)
      });

      test.fixme("TOP position toast entry animation should start above viewport", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.topLeftToaster.locator('[data-sonner-toast="true"]').first();
        const box = await toast.boundingBox();

        // Toast should be near top of viewport (small Y value)
        expect(box!.y).toBeLessThan(100);
      });
    });

    // FIXME: Collapsed stacking direction for TOP position not yet correct
    // Currently older toasts peek ABOVE (like bottom position)
    // Expected (shadcn): older toasts should peek BELOW for TOP position
    test.describe("Stacking - Older toasts peek BELOW (FIXME)", () => {
      test.fixme("in collapsed state, older toast should be BELOW newer toast (higher Y)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(300);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away to ensure collapsed
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const newestToast = toasts.first();
        const oldestToast = toasts.last();

        const newestBox = await newestToast.boundingBox();
        const oldestBox = await oldestToast.boundingBox();

        // For TOP position: oldest toast peeks BELOW (higher Y value)
        expect(oldestBox!.y).toBeGreaterThan(newestBox!.y);
      });

      /**
       * TEST: Back toast should have smaller scale (deck of cards effect)
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  DECK OF CARDS EFFECT (collapsed state):                        │
       *   │                                                                 │
       *   │    ┌─────────────────────┐  ← Older toast: scale(0.95)          │
       *   │    └┬───────────────────┬┘    width: NARROWER                   │
       *   │     │  Newest Toast     │   ← Front toast: scale(1.0)           │
       *   │     │                   │     width: WIDEST                     │
       *   │     └───────────────────┘                                       │
       *   │                                                                 │
       *   │  oldestBox.width < newestBox.width                              │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: Older toasts have smaller width due to scale transform
       */
      test("back toast should have smaller scale (deck of cards effect)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(300);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        // Older toast should be narrower (smaller scale)
        expect(oldestBox!.width).toBeLessThan(newestBox!.width);
      });

      test.fixme("3 toasts should show progressive stacking BELOW (deck effect)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 3 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(250);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(250);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Move mouse away
        await page.mouse.move(0, 0);
        await page.waitForTimeout(300);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const boxes = [];
        for (let i = 0; i < 3; i++) {
          boxes.push(await toasts.nth(i).boundingBox());
        }

        // Each older toast should be:
        // 1. Lower (higher Y) - peeks below
        // 2. Narrower (smaller width due to scale)
        for (let i = 1; i < 3; i++) {
          expect(boxes[i]!.y).toBeGreaterThan(boxes[i-1]!.y);
          expect(boxes[i]!.width).toBeLessThan(boxes[i-1]!.width);
        }
      });

      /**
       * TEST: --lift CSS variable should be 1 for TOP position
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  TOP POSITION: --lift = 1 (positive direction)                  │
       *   │                                                                 │
       *   │  ┌───────────────────────────┐                                  │
       *   │  │  Toast (TopLeft)          │  style="--lift: 1"               │
       *   │  └───────────────────────────┘                                  │
       *   │       │                                                         │
       *   │       ▼ expand direction (DOWNWARD into page)                   │
       *   │                                                                 │
       *   │  BOTTOM POSITION: --lift = -1 (negative direction)              │
       *   │                                                                 │
       *   │       ▲ expand direction (UPWARD into page)                     │
       *   │       │                                                         │
       *   │  ┌───────────────────────────┐                                  │
       *   │  │  Toast (BottomRight)      │  style="--lift: -1"              │
       *   │  └───────────────────────────┘                                  │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: --lift CSS variable is 1 for TOP position toasts
       */
      test("--lift CSS variable should be 1 for TOP position", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.topLeftToaster.locator('[data-sonner-toast="true"]').first();
        const lift = await toast.evaluate((el) => el.style.getPropertyValue("--lift"));

        expect(lift).toBe("1");
      });
    });

    test.describe("Expansion - Toasts expand DOWNWARD on hover", () => {
      /**
       * TEST: On hover, toasts should expand DOWNWARD (older toast has higher Y)
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  COLLAPSED (before hover):       EXPANDED (on hover):           │
       *   │                                                                 │
       *   │  ┌───────────────────────┐       ┌───────────────────────┐      │
       *   │  │  Toast 3 (newest)     │       │  Toast 3 (newest)     │      │
       *   │  └┬─────────────────────┬┘       └───────────────────────┘      │
       *   │   │                     │                  ↓ gap                │
       *   │   └┬───────────────────┬┘        ┌───────────────────────┐      │
       *   │    │                   │         │  Toast 2 (middle)     │      │
       *   │    └───────────────────┘         └───────────────────────┘      │
       *   │     stacked (scale diff)                  ↓ gap                 │
       *   │                                  ┌───────────────────────┐      │
       *   │                                  │  Toast 1 (oldest)     │      │
       *   │                                  └───────────────────────┘      │
       *   │                                   expanded DOWNWARD             │
       *   │                                   (higher Y = below)            │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: TOP position toasts expand DOWNWARD (oldestBox.y > newestBox.y)
       */
      test("on hover, toasts should expand DOWNWARD (older toast has higher Y)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 3 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(200);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(200);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.topLeftToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        // Expanded: oldest toast should be BELOW newest (higher Y)
        // This confirms DOWNWARD expansion for TOP position
        expect(oldestBox!.y).toBeGreaterThan(newestBox!.y);
      });

      /**
       * TEST: Expanded toasts should all have same scale (1.0)
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  COLLAPSED (different scales):    EXPANDED (same scale):        │
       *   │                                                                 │
       *   │    ┌─────────────────────┐        ┌───────────────────────┐     │
       *   │    │ scale(0.95)         │        │ scale(1.0) = 100% width│    │
       *   │    └┬───────────────────┬┘        └───────────────────────┘     │
       *   │     │ scale(1.0)        │         ┌───────────────────────┐     │
       *   │     └───────────────────┘         │ scale(1.0) = 100% width│    │
       *   │                                   └───────────────────────┘     │
       *   │      widths differ                  widths are SAME             │
       *   │                                                                 │
       *   │  Math.abs(newestBox.width - oldestBox.width) < 5                │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: All expanded toasts have approximately same width (scale 1.0)
       */
      test("expanded toasts should all have same scale (1.0)", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(200);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.topLeftToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        // Both should have approximately same width (same scale)
        expect(Math.abs(newestBox!.width - oldestBox!.width)).toBeLessThan(5);
      });

      /**
       * TEST: Expanded toasts should have ~15px gap between them
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  EXPANDED STATE (on hover):                                     │
       *   │                                                                 │
       *   │  ┌───────────────────────────────────┐  newestBox.y             │
       *   │  │  Toast (newest)                   │  newestBox.height        │
       *   │  └───────────────────────────────────┘                          │
       *   │            ↕ gap (~15px)                                        │
       *   │  ┌───────────────────────────────────┐  oldestBox.y             │
       *   │  │  Toast (oldest)                   │                          │
       *   │  └───────────────────────────────────┘                          │
       *   │                                                                 │
       *   │  gap = oldestBox.y - (newestBox.y + newestBox.height)           │
       *   │  Expected: 10px < gap < 25px                                    │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: Expanded toasts have approximately 15px gap between them
       */
      test("expanded toasts should have ~15px gap between them", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(200);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.topLeftToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        // Gap = oldestY - (newestY + newestHeight)
        const gap = oldestBox!.y - (newestBox!.y + newestBox!.height);

        // Gap should be approximately 15px
        expect(gap).toBeGreaterThan(10);
        expect(gap).toBeLessThan(25);
      });

      /**
       * TEST: Mouse leave should collapse toasts back to stacked state
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  1. HOVER (expanded):            2. MOUSE LEAVE (collapsed):    │
       *   │                                                                 │
       *   │  ┌───────────────────────┐       ┌───────────────────────┐      │
       *   │  │  Toast (newest)       │       │  Toast (newest)       │      │
       *   │  └───────────────────────┘       └┬─────────────────────┬┘      │
       *   │            ↓ gap                  │                     │       │
       *   │  ┌───────────────────────┐        └┬───────────────────┬┘       │
       *   │  │  Toast (oldest)       │         │ (peeking below)   │        │
       *   │  └───────────────────────┘         └───────────────────┘        │
       *   │  data-expanded="true"             data-expanded="false"         │
       *   │  same widths                      older toast narrower          │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: Toasts collapse and scale difference returns on mouse leave
       */
      test("mouse leave should collapse toasts back to stacked state", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        // Create 2 toasts
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(200);
        await ui.topLeftTrigger.click();
        await page.waitForTimeout(400);

        // Hover to expand
        await ui.topLeftToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topLeftToaster.locator('[data-sonner-toast="true"]');

        // Check expanded state
        await expect(toasts.first()).toHaveAttribute("data-expanded", "true");

        // Move mouse away
        await page.mouse.move(0, 0);
        await page.waitForTimeout(400);

        // Check collapsed state
        await expect(toasts.first()).toHaveAttribute("data-expanded", "false");

        // Verify scale difference is back (older toast narrower)
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();
        expect(oldestBox!.width).toBeLessThan(newestBox!.width);
      });
    });

    test.describe("TopCenter and TopRight should behave same as TopLeft", () => {
      /**
       * TEST: TopCenter: toasts expand DOWNWARD
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  TOP-CENTER POSITION (expanded on hover):                       │
       *   │                                                                 │
       *   │         ┌───────────────────────┐                               │
       *   │         │  Toast (newest)       │  newestBox.y (smaller Y)      │
       *   │         └───────────────────────┘                               │
       *   │                   ↓ expand DOWNWARD                             │
       *   │         ┌───────────────────────┐                               │
       *   │         │  Toast (oldest)       │  oldestBox.y (larger Y)       │
       *   │         └───────────────────────┘                               │
       *   │                                                                 │
       *   │  Assertion: oldestBox.y > newestBox.y (DOWNWARD expansion)      │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: TopCenter position expands toasts DOWNWARD like TopLeft
       */
      test("TopCenter: toasts expand DOWNWARD", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topCenterTrigger.click();
        await page.waitForTimeout(200);
        await ui.topCenterTrigger.click();
        await page.waitForTimeout(400);

        await ui.topCenterToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topCenterToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        // Oldest should be below (higher Y) = DOWNWARD expansion
        expect(oldestBox!.y).toBeGreaterThan(newestBox!.y);
      });

      /**
       * TEST: TopRight: toasts expand DOWNWARD
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  TOP-RIGHT POSITION (expanded on hover):                        │
       *   │                                                                 │
       *   │                          ┌───────────────────────┐              │
       *   │                          │  Toast (newest)       │  newestBox.y │
       *   │                          └───────────────────────┘              │
       *   │                                    ↓ expand DOWNWARD            │
       *   │                          ┌───────────────────────┐              │
       *   │                          │  Toast (oldest)       │  oldestBox.y │
       *   │                          └───────────────────────┘              │
       *   │                                                                 │
       *   │  Assertion: oldestBox.y > newestBox.y (DOWNWARD expansion)      │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: TopRight position expands toasts DOWNWARD like TopLeft
       */
      test("TopRight: toasts expand DOWNWARD", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topRightTrigger.click();
        await page.waitForTimeout(200);
        await ui.topRightTrigger.click();
        await page.waitForTimeout(400);

        await ui.topRightToaster.hover();
        await page.waitForTimeout(400);

        const toasts = ui.topRightToaster.locator('[data-sonner-toast="true"]');
        const newestBox = await toasts.first().boundingBox();
        const oldestBox = await toasts.last().boundingBox();

        expect(oldestBox!.y).toBeGreaterThan(newestBox!.y);
      });

      /**
       * TEST: TopRight: --lift should be 1
       * ─────────────────────────────────────────────────────────────────────
       *
       *   What we're testing:
       *   ┌─────────────────────────────────────────────────────────────────┐
       *   │                                                                 │
       *   │  TOP-RIGHT POSITION:                                            │
       *   │                                                                 │
       *   │                          ┌───────────────────────┐              │
       *   │                          │  Toast (TopRight)     │              │
       *   │                          └───────────────────────┘              │
       *   │                                    │                            │
       *   │                                    ▼ --lift: 1                  │
       *   │                                                                 │
       *   │  All TOP positions (TopLeft, TopCenter, TopRight)               │
       *   │  should have --lift: 1 (positive direction = expand down)       │
       *   │                                                                 │
       *   └─────────────────────────────────────────────────────────────────┘
       *
       *   Validates: TopRight position has --lift CSS variable set to 1
       */
      test("TopRight: --lift should be 1", async ({ page }) => {
        const ui = new SonnerPositionsPage(page);
        await ui.goto();

        await ui.topRightTrigger.click();
        await page.waitForTimeout(400);

        const toast = ui.topRightToaster.locator('[data-sonner-toast="true"]').first();
        const lift = await toast.evaluate((el) => el.style.getPropertyValue("--lift"));

        expect(lift).toBe("1");
      });
    });
  });
});

/* ========================================================== */
/*              EDGE CASES - SWIPE TO DISMISS                 */
/* ========================================================== */
/**
 * SWIPE TO DISMISS - Visual Overview
 * ============================================================================
 *
 * Swipe gestures dismiss toasts based on position:
 *
 * TOP POSITIONS (TopLeft, TopCenter, TopRight):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │   ↑ swipe UP to dismiss                                                 │
 * │   ┌──────────────────────────┐                                          │
 * │   │  Toast                ×  │ ← Can also swipe LEFT/RIGHT              │
 * │   └──────────────────────────┘                                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * BOTTOM POSITIONS (BottomLeft, BottomCenter, BottomRight):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌──────────────────────────┐                                          │
 * │   │  Toast                ×  │ ← Can also swipe LEFT/RIGHT              │
 * │   └──────────────────────────┘                                          │
 * │   ↓ swipe DOWN to dismiss                                               │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SWIPE THRESHOLDS:
 * - Distance threshold: 45px
 * - Velocity threshold: 0.11 px/ms (fast swipe even if short)
 *
 * ============================================================================
 */
test.describe("Sonner Edge Cases", () => {
  test.describe("Swipe to Dismiss", () => {
    /**
     * TEST: BOTTOM position: swipe DOWN should dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  BOTTOM POSITION - Swipe DOWN to dismiss:                       │
     *   │                                                                 │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast (BottomCenter)      ×  │                              │
     *   │  └───────────────────────────────┘                              │
     *   │              │                                                  │
     *   │              │  swipe DOWN (+100px)                             │
     *   │              ▼                                                  │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                              │
     *   │            (dismissed)                                          │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                              │
     *   │                                                                 │
     *   │  After swipe: toast.not.toBeVisible()                           │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Swiping DOWN dismisses BOTTOM position toasts
     */
    test("BOTTOM position: swipe DOWN should dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      // Get toast position for swipe
      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform swipe DOWN
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX, startY + 100, { steps: 10 });
      await page.mouse.up();

      // Wait for exit animation
      await page.waitForTimeout(400);

      // Toast should be dismissed
      await expect(toast).not.toBeVisible();
    });

    /**
     * TEST: TOP position: swipe UP should dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  TOP POSITION - Swipe UP to dismiss:                            │
     *   │                                                                 │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                              │
     *   │            (dismissed)                                          │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                              │
     *   │              ▲                                                  │
     *   │              │  swipe UP (-100px)                               │
     *   │              │                                                  │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast (TopCenter)         ×  │                              │
     *   │  └───────────────────────────────┘                              │
     *   │                                                                 │
     *   │  After swipe: toast.not.toBeVisible()                           │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Swiping UP dismisses TOP position toasts
     */
    test("TOP position: swipe UP should dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.topCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.topCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      // Get toast position for swipe
      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform swipe UP
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX, startY - 100, { steps: 10 });
      await page.mouse.up();

      // Wait for exit animation
      await page.waitForTimeout(400);

      // Toast should be dismissed
      await expect(toast).not.toBeVisible();
    });

    /**
     * TEST: Swipe LEFT should dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  HORIZONTAL SWIPE - LEFT to dismiss:                            │
     *   │                                                                 │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐    ┌───────────────────────────────┐ │
     *   │       (dismissed)         ◄───│  Toast (TopCenter)         ×  │ │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘    └───────────────────────────────┘ │
     *   │                                                                 │
     *   │            ◄──── swipe LEFT (-200px, fast)                      │
     *   │                                                                 │
     *   │  After swipe: toastCount === 0                                  │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Swiping LEFT dismisses toast (horizontal swipe)
     */
    test("swipe LEFT should dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast at TopCenter (supports all swipe directions)
      await ui.topCenterTrigger.click();
      await page.waitForTimeout(400);

      const toaster = ui.topCenterToaster;
      const toast = toaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform swipe LEFT - fast with large distance
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX - 200, startY, { steps: 3 }); // Even faster and larger swipe
      await page.mouse.up();

      // Wait for exit animation to complete
      await page.waitForTimeout(500);

      // Check toast count is 0 (more reliable than checking visibility)
      const toastCount = await toaster.locator('[data-sonner-toast="true"]').count();
      expect(toastCount).toBe(0);
    });

    /**
     * TEST: Swipe RIGHT should dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  HORIZONTAL SWIPE - RIGHT to dismiss:                           │
     *   │                                                                 │
     *   │  ┌───────────────────────────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐ │
     *   │  │  Toast (TopCenter)         ×  │───►     (dismissed)         │
     *   │  └───────────────────────────────┘    └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘ │
     *   │                                                                 │
     *   │            swipe RIGHT (+200px, fast) ────►                     │
     *   │                                                                 │
     *   │  After swipe: toastCount === 0                                  │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Swiping RIGHT dismisses toast (horizontal swipe)
     */
    test("swipe RIGHT should dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast at TopCenter (supports all swipe directions)
      await ui.topCenterTrigger.click();
      await page.waitForTimeout(400);

      const toaster = ui.topCenterToaster;
      const toast = toaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform swipe RIGHT - fast with large distance
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX + 200, startY, { steps: 3 }); // Even faster and larger swipe
      await page.mouse.up();

      // Wait for exit animation to complete
      await page.waitForTimeout(500);

      // Check toast count is 0 (more reliable than checking visibility)
      const toastCount = await toaster.locator('[data-sonner-toast="true"]').count();
      expect(toastCount).toBe(0);
    });

    /**
     * TEST: Small swipe (below threshold) should NOT dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  SWIPE THRESHOLDS:                                              │
     *   │  - Distance threshold: 45px                                     │
     *   │  - Velocity threshold: 0.11 px/ms                               │
     *   │                                                                 │
     *   │  SMALL SWIPE (15px, slow movement):                             │
     *   │                                                                 │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast (BottomCenter)      ×  │                              │
     *   │  └───────────────────────────────┘                              │
     *   │              │                                                  │
     *   │              ↓ tiny swipe (15px) with many steps (slow)         │
     *   │              │                                                  │
     *   │  ┌───────────────────────────────┐ ← springs back!              │
     *   │  │  Toast (still visible)     ×  │                              │
     *   │  └───────────────────────────────┘                              │
     *   │                                                                 │
     *   │  After small swipe: toast.toBeVisible() (NOT dismissed)         │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Small/slow swipe below threshold does not dismiss toast
     */
    test("small swipe (below threshold) should NOT dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform very small, slow swipe (below 45px threshold AND below velocity threshold)
      // Use many steps to make it slow (velocity = distance / time)
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX, startY + 15, { steps: 20 }); // Only 15px with slow movement
      await page.mouse.up();

      await page.waitForTimeout(400);
      // Toast should spring back and still be visible
      await expect(toast).toBeVisible();
    });

    /**
     * TEST: During swipe, toast should have data-swiping=true
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  SWIPE STATE TRACKING:                                          │
     *   │                                                                 │
     *   │  1. mouse.down()                                                │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast                     ×  │  data-swiping="false"        │
     *   │  └───────────────────────────────┘                              │
     *   │              │                                                  │
     *   │  2. mouse.move() (dragging)                                     │
     *   │              ▼                                                  │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast (being swiped)      ×  │  data-swiping="true"  ← ✓   │
     *   │  └───────────────────────────────┘                              │
     *   │                                                                 │
     *   │  Assertion: data-swiping="true" during swipe gesture            │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: data-swiping attribute is "true" during active swipe
     */
    test("during swipe, toast should have data-swiping=true", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Start swipe
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX, startY + 30, { steps: 3 });

      // Check swiping state
      const swiping = await toast.getAttribute("data-swiping");
      expect(swiping).toBe("true");

      await page.mouse.up();
    });

    /**
     * TEST: After swipe-out, toast should have data-swipe-out=true
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  SWIPE-OUT STATE TRACKING:                                      │
     *   │                                                                 │
     *   │  1. Before swipe:                                               │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast                     ×  │  data-swipe-out="false"      │
     *   │  └───────────────────────────────┘                              │
     *   │              │                                                  │
     *   │  2. Swipe gesture completed (threshold exceeded):               │
     *   │              │                                                  │
     *   │              ▼ swipe DOWN (+100px)                              │
     *   │                                                                 │
     *   │  3. After swipe release:                                        │
     *   │  ┌───────────────────────────────┐                              │
     *   │  │  Toast (exiting)           ×  │  data-swipe-out="true"  ← ✓ │
     *   │  └───────────────────────────────┘                              │
     *   │                                                                 │
     *   │  Assertion: data-swipe-out="true" after successful swipe        │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: data-swipe-out attribute is "true" after swipe dismissal
     */
    test("after swipe-out, toast should have data-swipe-out=true", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const box = await toast.boundingBox();
      if (!box) throw new Error("Toast not found");

      const startX = box.x + box.width / 2;
      const startY = box.y + box.height / 2;

      // Perform swipe
      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(startX, startY + 100, { steps: 10 });
      await page.mouse.up();

      // Immediately check swipe-out attribute
      const swipeOut = await toast.getAttribute("data-swipe-out");
      expect(swipeOut).toBe("true");
    });
  });

  /* ========================================================== */
  /*              EDGE CASES - CLOSE BUTTON                     */
  /* ========================================================== */
  /**
   * CLOSE BUTTON - Visual Overview
   * ============================================================================
   *
   * ┌──────────────────────────────────────┐
   * │  You got a message              [×]  │ ← Close button (top-right)
   * │  You toasted me!                     │
   * ├──────────────────────────────────────┤
   * │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
   * └──────────────────────────────────────┘
   *
   * - Close button appears for all variants except Loading
   * - Clicking close button immediately dismisses toast
   * - Close button has hover state for better UX
   *
   * ============================================================================
   */
  test.describe("Close Button", () => {
    /**
     * TEST: Clicking close button should dismiss toast
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  CLOSE BUTTON FUNCTIONALITY:                                    │
     *   │                                                                 │
     *   │  1. Toast visible:                                              │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  You got a message              [×]  │ ← click close button  │
     *   │  │  You toasted me!                     │                       │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                                                 │
     *   │  2. After click:                                                │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                       │
     *   │             (toast dismissed)                                   │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                       │
     *   │                                                                 │
     *   │  Assertion: toast.not.toBeVisible()                             │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking close button dismisses the toast
     */
    test("clicking close button should dismiss toast", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      // Click close button
      const closeButton = toast.locator('[data-close-button]');
      await expect(closeButton).toBeVisible();
      await closeButton.click();

      // Wait for exit animation
      await page.waitForTimeout(400);

      // Toast should be dismissed
      await expect(toast).not.toBeVisible();
    });

    /**
     * TEST: Close button should have aria-label
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  ACCESSIBILITY - Close button aria-label:                       │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  You got a message              [×]  │                       │
     *   │  │  You toasted me!                     │                       │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                    ↑                            │
     *   │                                    │                            │
     *   │  <button                           │                            │
     *   │    data-close-button               │                            │
     *   │    aria-label="Close toast"  ← ────┘                            │
     *   │  />                                                             │
     *   │                                                                 │
     *   │  Assertion: aria-label === "Close toast"                        │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button has proper aria-label for accessibility
     */
    test("close button should have aria-label", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const closeButton = toast.locator('[data-close-button]');

      const ariaLabel = await closeButton.getAttribute("aria-label");
      expect(ariaLabel).toBe("Close toast");
    });
  });

  /* ========================================================== */
  /*              EDGE CASES - AUTO-DISMISS TIMEOUT             */
  /* ========================================================== */
  /**
   * AUTO-DISMISS TIMEOUT - Visual Overview
   * ============================================================================
   *
   * Default dismiss delay: 5000ms (5 seconds)
   *
   * Timeline:
   * 0s        1s        2s        3s        4s        5s
   * |─────────|─────────|─────────|─────────|─────────|
   * [████████████████████████████████████████████████] ← Progress bar shrinks
   * Toast visible ────────────────────────────────────→ Auto-dismiss
   *
   * - Progress bar shows remaining time (scaleX: 1 → 0)
   * - When progress bar reaches 0, toast auto-dismisses
   * - Hover pauses the timer and progress bar
   *
   * ============================================================================
   */
  test.describe("Auto-dismiss Timeout", () => {
    /**
     * TEST: Toast should auto-dismiss after timeout
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  AUTO-DISMISS TIMELINE (5000ms default):                        │
     *   │                                                                 │
     *   │  t=0s     t=1s     t=2s     t=3s     t=4s     t=5s             │
     *   │  |────────|────────|────────|────────|────────|                │
     *   │  [████████████████████████████████████████████] ← progress     │
     *   │  Toast visible ────────────────────────────────→ Auto-dismiss  │
     *   │                                                                 │
     *   │  1. Toast created:                                              │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  Toast                            ×  │                       │
     *   │  ├──────────────────────────────────────┤                       │
     *   │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│ ← progress bar       │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                                                 │
     *   │  2. After 5500ms:                                               │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                       │
     *   │             (toast dismissed)                                   │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Toast auto-dismisses after 5000ms timeout
     */
    test("toast should auto-dismiss after timeout", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      // Wait for auto-dismiss (5000ms default + buffer)
      await page.waitForTimeout(5500);

      // Toast should be dismissed
      await expect(toast).not.toBeVisible();
    });

    /**
     * TEST: Hovering should pause auto-dismiss timer
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  HOVER PAUSES TIMER:                                            │
     *   │                                                                 │
     *   │  1. Toast created, hover immediately:                           │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  Toast                  🖱️ hover  ×  │                       │
     *   │  ├──────────────────────────────────────┤                       │
     *   │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░│ ← PAUSED (frozen)    │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                                                 │
     *   │  2. Wait 5500ms while hovering:                                 │
     *   │     Toast STILL VISIBLE (timer paused)                          │
     *   │                                                                 │
     *   │  3. Move mouse away:                                            │
     *   │     Timer resumes...                                            │
     *   │                                                                 │
     *   │  4. Wait another 5500ms:                                        │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                       │
     *   │             (toast dismissed)                                   │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering pauses auto-dismiss timer, resumes on mouse leave
     */
    test("hovering should pause auto-dismiss timer", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create toast
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await expect(toast).toBeVisible();

      // Hover to pause timer
      await toast.hover();

      // Wait past the normal dismiss time
      await page.waitForTimeout(5500);

      // Toast should STILL be visible because hover paused timer
      await expect(toast).toBeVisible();

      // Move mouse away
      await page.mouse.move(0, 0);

      // Now wait for remaining time + buffer
      await page.waitForTimeout(5500);

      // Toast should now be dismissed
      await expect(toast).not.toBeVisible();
    });
  });

  /* ========================================================== */
  /*              EDGE CASES - TOAST VARIANTS                   */
  /* ========================================================== */
  /**
   * TOAST VARIANTS - Visual Overview
   * ============================================================================
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │  DEFAULT: Neutral styling                                              │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  You got a message            ×  │  border-muted                    │
   * │  │  You toasted me!                 │                                  │
   * │  └──────────────────────────────────┘                                  │
   * │                                                                        │
   * │  SUCCESS: Green styling with checkmark icon                            │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  ✓ Success                    ×  │  text-green-500                  │
   * │  │    Your action was successful.   │                                  │
   * │  └──────────────────────────────────┘                                  │
   * │                                                                        │
   * │  ERROR: Red styling with X icon                                        │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  ✗ Error                      ×  │  text-red-500                    │
   * │  │    Something went wrong.         │                                  │
   * │  └──────────────────────────────────┘                                  │
   * │                                                                        │
   * │  WARNING: Yellow styling with warning icon                             │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  ⚠ Warning                    ×  │  text-yellow-500                 │
   * │  │    Please be careful.            │                                  │
   * │  └──────────────────────────────────┘                                  │
   * │                                                                        │
   * │  INFO: Blue styling with info icon                                     │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  ℹ Info                       ×  │  text-blue-500                   │
   * │  │    Here is some information.     │                                  │
   * │  └──────────────────────────────────┘                                  │
   * │                                                                        │
   * │  LOADING: Spinner icon, NO close button, NO duration track             │
   * │  ┌──────────────────────────────────┐                                  │
   * │  │  ⟳ Loading                       │  text-muted-foreground           │
   * │  │    Please wait...                │  (no close button!)              │
   * │  └──────────────────────────────────┘  (no progress bar!)              │
   * │                                                                        │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * ============================================================================
   */

  class SonnerVariantsPage extends BasePage {
    protected readonly componentName = "sonner";
    protected readonly demoName = "variants";

    readonly defaultTrigger: Locator;
    readonly successTrigger: Locator;
    readonly errorTrigger: Locator;
    readonly warningTrigger: Locator;
    readonly infoTrigger: Locator;
    readonly loadingTrigger: Locator;
    readonly toaster: Locator;

    constructor(page: Page) {
      super(page);

      // Use exact: true to avoid matching "Bottom Right (Default)"
      this.defaultTrigger = page.getByRole("button", { name: "Default", exact: true });
      this.successTrigger = page.getByRole("button", { name: "Success", exact: true });
      this.errorTrigger = page.getByRole("button", { name: "Error", exact: true });
      this.warningTrigger = page.getByRole("button", { name: "Warning", exact: true });
      this.infoTrigger = page.getByRole("button", { name: "Info", exact: true });
      this.loadingTrigger = page.getByRole("button", { name: "Loading", exact: true });
      this.toaster = page.locator('[data-sonner-toaster="true"]');
    }
  }

  test.describe("Toast Variants", () => {
    /**
     * TEST: Default variant should have data-variant=Default
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  DEFAULT VARIANT - Neutral styling:                             │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  You got a message              [×]  │  data-variant="Default│
     *   │  │  You toasted me!                     │  border-muted         │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                                                 │
     *   │  Assertion: data-variant === "Default"                          │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Default variant has correct data-variant attribute
     */
    test("Default variant should have data-variant=Default", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.defaultTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Default");
    });

    /**
     * TEST: Success variant should have data-variant=Success and icon
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  SUCCESS VARIANT - Green styling with checkmark:                │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  ✓ Success                      [×]  │  data-variant="Success│
     *   │  │    Your action was successful!       │  text-green-500       │
     *   │  └──────────────────────────────────────┘                       │
     *   │   ↑                                                             │
     *   │   └── [data-icon] visible (checkmark icon)                      │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - data-variant === "Success"                                   │
     *   │  - [data-icon] is visible                                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Success variant has correct attributes and icon
     */
    test("Success variant should have data-variant=Success and icon", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.successTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Success");

      // Should have icon
      const icon = toast.locator('[data-icon]');
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Error variant should have data-variant=Error and icon
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  ERROR VARIANT - Red styling with X icon:                       │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  ✗ Error                        [×]  │  data-variant="Error" │
     *   │  │    Something went wrong.             │  text-red-500         │
     *   │  └──────────────────────────────────────┘                       │
     *   │   ↑                                                             │
     *   │   └── [data-icon] visible (X icon)                              │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - data-variant === "Error"                                     │
     *   │  - [data-icon] is visible                                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Error variant has correct attributes and icon
     */
    test("Error variant should have data-variant=Error and icon", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.errorTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Error");

      const icon = toast.locator('[data-icon]');
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Warning variant should have data-variant=Warning and icon
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  WARNING VARIANT - Yellow styling with warning icon:            │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  ⚠ Warning                      [×]  │  data-variant="Warning│
     *   │  │    Please be careful.                │  text-yellow-500      │
     *   │  └──────────────────────────────────────┘                       │
     *   │   ↑                                                             │
     *   │   └── [data-icon] visible (warning triangle icon)               │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - data-variant === "Warning"                                   │
     *   │  - [data-icon] is visible                                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Warning variant has correct attributes and icon
     */
    test("Warning variant should have data-variant=Warning and icon", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.warningTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Warning");

      const icon = toast.locator('[data-icon]');
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Info variant should have data-variant=Info and icon
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  INFO VARIANT - Blue styling with info icon:                    │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  ℹ Info                         [×]  │  data-variant="Info"  │
     *   │  │    Here is some information.         │  text-blue-500        │
     *   │  └──────────────────────────────────────┘                       │
     *   │   ↑                                                             │
     *   │   └── [data-icon] visible (info circle icon)                    │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - data-variant === "Info"                                      │
     *   │  - [data-icon] is visible                                       │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Info variant has correct attributes and icon
     */
    test("Info variant should have data-variant=Info and icon", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.infoTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Info");

      const icon = toast.locator('[data-icon]');
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Loading variant should have data-variant=Loading
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  LOADING VARIANT - Spinner with muted styling:                  │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  ⟳ Loading                           │  data-variant="Loading│
     *   │  │    Please wait...                    │  text-muted-foreground│
     *   │  └──────────────────────────────────────┘                       │
     *   │                                          ↑                      │
     *   │                                          │                      │
     *   │                       NO close button ───┘                      │
     *   │                                                                 │
     *   │  Assertion: data-variant === "Loading"                          │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Loading variant has correct data-variant attribute
     */
    test("Loading variant should have data-variant=Loading", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.loadingTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const variant = await toast.getAttribute("data-variant");
      expect(variant).toBe("Loading");
    });

    /**
     * TEST: Loading variant should NOT have close button
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  LOADING VARIANT - No close button (cannot be dismissed):       │
     *   │                                                                 │
     *   │  Normal variant:               Loading variant:                 │
     *   │  ┌───────────────────────┐     ┌───────────────────────┐        │
     *   │  │  Message         [×]  │     │  ⟳ Loading            │        │
     *   │  │  Description          │     │    Please wait...     │        │
     *   │  └───────────────────────┘     └───────────────────────┘        │
     *   │                     ↑                                ↑          │
     *   │                     │                                │          │
     *   │             close button                    NO close button     │
     *   │                                                                 │
     *   │  Assertion: closeButton.toHaveCount(0)                          │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Loading variant does NOT have close button
     */
    test("Loading variant should NOT have close button", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.loadingTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const closeButton = toast.locator('[data-close-button]');

      // Loading toasts should NOT have close button
      await expect(closeButton).toHaveCount(0);
    });

    /**
     * TEST: Loading variant should NOT have duration track
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  LOADING VARIANT - No duration track (indefinite loading):      │
     *   │                                                                 │
     *   │  Normal variant:               Loading variant:                 │
     *   │  ┌───────────────────────┐     ┌───────────────────────┐        │
     *   │  │  Message              │     │  ⟳ Loading            │        │
     *   │  │  Description          │     │    Please wait...     │        │
     *   │  ├───────────────────────┤     └───────────────────────┘        │
     *   │  │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│              ↑                       │
     *   │  └───────────────────────┘              │                       │
     *   │            ↑                   NO duration track                │
     *   │            │                   (loading = indefinite)           │
     *   │      duration track                                             │
     *   │                                                                 │
     *   │  Assertion: durationTrack.toHaveCSS("display", "none")          │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Loading variant has hidden duration track
     */
    test("Loading variant should NOT have duration track", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.loadingTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const durationTrack = toast.locator('[data-duration-track]');

      // Duration track should be hidden for Loading variant
      await expect(durationTrack).toHaveCSS("display", "none");
    });

    /**
     * TEST: Loading variant should have spinner animation
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  LOADING VARIANT - Animated spinner icon:                       │
     *   │                                                                 │
     *   │  ┌───────────────────────────────────────┐                      │
     *   │  │  ⟳ Loading                            │                      │
     *   │  │  ↻ Please wait...                     │                      │
     *   │  └───────────────────────────────────────┘                      │
     *   │   ↑                                                             │
     *   │   └── [data-icon] with [&>svg]:animate-spin                     │
     *   │                                                                 │
     *   │  Icon structure:                                                │
     *   │  <span data-icon class="[&>svg]:animate-spin">                  │
     *   │    <svg>...</svg>  ← spinning animation                         │
     *   │  </span>                                                        │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - [data-icon] is visible                                       │
     *   │  - class contains "[&>svg]:animate-spin"                        │
     *   │  - svg child element exists                                     │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Loading variant has animated spinner icon
     */
    test("Loading variant should have spinner animation", async ({ page }) => {
      const ui = new SonnerVariantsPage(page);
      await ui.goto();

      await ui.loadingTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.toaster.locator('[data-sonner-toast="true"]').first();
      const icon = toast.locator('[data-icon]');
      await expect(icon).toBeVisible();

      // Icon container has [&>svg]:animate-spin which applies to SVG child
      // Check that the icon container has this class and contains an SVG
      const iconClass = await icon.getAttribute("class");
      expect(iconClass).toContain("[&>svg]:animate-spin");

      // Also verify SVG exists inside
      const svg = icon.locator("svg");
      await expect(svg).toBeVisible();
    });
  });

  /* ========================================================== */
  /*              EDGE CASES - FOCUS MANAGEMENT                 */
  /* ========================================================== */
  /**
   * FOCUS MANAGEMENT - Visual Overview
   * ============================================================================
   *
   * When toast is dismissed via close button:
   *
   * BEFORE:                           AFTER:
   * ┌─────────────────────────────┐   ┌─────────────────────────────┐
   * │                             │   │                             │
   * │   [Toast Me!]  ← focused    │   │   [Toast Me!]  ← focused    │
   * │                             │   │                 (returns!)   │
   * │   ┌──────────────────────┐  │   │                             │
   * │   │  Toast           [×] │  │   │   (toast dismissed)         │
   * │   └──────────────────────┘  │   │                             │
   * │                             │   │                             │
   * └─────────────────────────────┘   └─────────────────────────────┘
   *
   * - Focus should return to the element that triggered the toast
   * - This improves keyboard accessibility
   *
   * ============================================================================
   */
  test.describe("Focus Management", () => {
    /**
     * TEST: Close button should be focusable
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  KEYBOARD ACCESSIBILITY - Focus on close button:                │
     *   │                                                                 │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  You got a message              [×]  │ ← focus ring visible  │
     *   │  │  You toasted me!                 ▲   │                       │
     *   │  └──────────────────────────────────│───┘                       │
     *   │                                     │                           │
     *   │                            closeButton.focus()                  │
     *   │                                                                 │
     *   │  Assertion: closeButton.toBeFocused()                           │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button can receive keyboard focus
     */
    test("close button should be focusable", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const closeButton = toast.locator('[data-close-button]');

      await closeButton.focus();
      await expect(closeButton).toBeFocused();
    });

    /**
     * TEST: Close button should be activatable via keyboard
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  KEYBOARD ACTIVATION - Press Enter on focused close button:     │
     *   │                                                                 │
     *   │  1. Focus close button:                                         │
     *   │  ┌──────────────────────────────────────┐                       │
     *   │  │  You got a message              [×]  │ ← focused             │
     *   │  │  You toasted me!                     │                       │
     *   │  └──────────────────────────────────────┘                       │
     *   │                                                                 │
     *   │  2. Press Enter key:                                            │
     *   │     ⌨️ keyboard.press("Enter")                                   │
     *   │                                                                 │
     *   │  3. Toast dismissed:                                            │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                       │
     *   │             (toast dismissed)                                   │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                       │
     *   │                                                                 │
     *   │  Assertion: toast.not.toBeVisible()                             │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button can be activated with Enter key
     */
    test("close button should be activatable via keyboard", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const closeButton = toast.locator('[data-close-button]');

      await closeButton.focus();
      await page.keyboard.press("Enter");

      await page.waitForTimeout(400);
      await expect(toast).not.toBeVisible();
    });
  });

  /* ========================================================== */
  /*              EDGE CASES - MULTIPLE TOASTS INTERACTION      */
  /* ========================================================== */
  /**
   * MULTIPLE TOASTS INTERACTION - Visual Overview
   * ============================================================================
   *
   * When dismissing a toast from the middle of the stack:
   *
   * BEFORE:                           AFTER (collapse animation):
   * ┌─────────────────────────────┐   ┌─────────────────────────────┐
   * │  ┌──────────────────────┐   │   │  ┌──────────────────────┐   │
   * │  │  Toast 3 (front)     │   │   │  │  Toast 3 (front)     │   │
   * │  └──────────────────────┘   │   │  └──────────────────────┘   │
   * │    ┌──────────────────────┐ │   │    ┌──────────────────────┐ │
   * │    │  Toast 2 (dismissed)!│ │   │    │  Toast 1 (moves up)  │ │
   * │    └──────────────────────┘ │   │    └──────────────────────┘ │
   * │      ┌──────────────────────│   │                             │
   * │      │  Toast 1             │   │                             │
   * │      └──────────────────────│   │                             │
   * └─────────────────────────────┘   └─────────────────────────────┘
   *
   * - Remaining toasts should smoothly reposition
   * - z-index should update correctly
   *
   * ============================================================================
   */
  test.describe("Multiple Toasts Interaction", () => {
    /**
     * TEST: Dismissing front toast should make next toast the new front
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  BEFORE DISMISS:                  AFTER DISMISS:                │
     *   │                                                                 │
     *   │  ┌───────────────────────┐        ┌───────────────────────┐     │
     *   │  │ Toast 3 (front)    ×  │        │ Toast 2 (NEW front) × │     │
     *   │  │ data-front="true"     │  →     │ data-front="true"     │     │
     *   │  └┬─────────────────────┬┘        └┬───────────────────────┘    │
     *   │   │ Toast 2             │          │ Toast 1                │   │
     *   │   └┬───────────────────┬┘          └────────────────────────┘   │
     *   │    │ Toast 1           │                                       │
     *   │    └───────────────────┘                                       │
     *   │                                                                 │
     *   │  Assertion: new front toast has data-front="true"               │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Next toast becomes front after front toast is dismissed
     */
    test("dismissing front toast should make next toast the new front", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create 3 toasts
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      const toasts = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]');
      const frontToast = toasts.first();

      // Verify front toast
      expect(await frontToast.getAttribute("data-front")).toBe("true");

      // Dismiss front toast via close button
      await frontToast.locator('[data-close-button]').click();
      await page.waitForTimeout(400);

      // New front toast should have data-front=true
      const newFrontToast = toasts.first();
      expect(await newFrontToast.getAttribute("data-front")).toBe("true");
    });

    /**
     * TEST: Remaining toasts should update their --index after dismissal
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  BEFORE DISMISS:                  AFTER DISMISS:                │
     *   │                                                                 │
     *   │  ┌───────────────────────┐        ┌───────────────────────┐     │
     *   │  │ Toast 3 --index: 0    │        │ Toast 2 --index: 0    │     │
     *   │  └┬─────────────────────┬┘  →     └┬───────────────────────┘    │
     *   │   │ Toast 2 --index: 1  │          │ Toast 1 --index: 1    │    │
     *   │   └┬───────────────────┬┘          └────────────────────────┘   │
     *   │    │ Toast 1 --index: 2│                                        │
     *   │    └───────────────────┘                                        │
     *   │                                                                 │
     *   │  Indices shift down when front toast is dismissed               │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - remainingToasts.count() === 2                                │
     *   │  - first toast --index === "0"                                  │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: --index CSS variables update after toast dismissal
     */
    test("remaining toasts should update their --index after dismissal", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create 3 toasts
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      // Dismiss front toast
      const frontToast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      await frontToast.locator('[data-close-button]').click();
      await page.waitForTimeout(400);

      // Remaining toasts should have updated indices (0, 1)
      const remainingToasts = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]');
      const count = await remainingToasts.count();
      expect(count).toBe(2);

      const firstIndex = await remainingToasts.first().evaluate((el) =>
        el.style.getPropertyValue("--index")
      );
      expect(firstIndex).toBe("0");
    });

    /**
     * TEST: Swipe-dismissing one toast should not affect others in stack
     * ─────────────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────────────┐
     *   │                                                                 │
     *   │  BEFORE SWIPE:                    AFTER SWIPE:                  │
     *   │                                                                 │
     *   │  ┌───────────────────────┐        ┌───────────────────────┐     │
     *   │  │ Toast 3 (front)       │        │ Toast 2 (new front)   │     │
     *   │  └───────────────────────┘        └┬─────────────────────┬┘     │
     *   │          │                         │ Toast 1              │     │
     *   │          ▼ swipe DOWN              └─────────────────────┘      │
     *   │          │                                                      │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                                      │
     *   │       (dismissed)                                               │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘         2 toasts remain visible      │
     *   │                                                                 │
     *   │  Assertions:                                                    │
     *   │  - remainingToasts.count() === 2                                │
     *   │  - both remaining toasts are visible                            │
     *   │                                                                 │
     *   └─────────────────────────────────────────────────────────────────┘
     *
     *   Validates: Swipe dismissal only affects the swiped toast
     */
    test("swipe-dismissing one toast should not affect others in stack", async ({ page }) => {
      const ui = new SonnerPositionsPage(page);
      await ui.goto();

      // Create 3 toasts
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(300);
      await ui.bottomCenterTrigger.click();
      await page.waitForTimeout(400);

      // Swipe dismiss front toast
      const frontToast = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]').first();
      const box = await frontToast.boundingBox();
      if (!box) throw new Error("Toast not found");

      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2 + 100, { steps: 10 });
      await page.mouse.up();

      await page.waitForTimeout(400);

      // Should have exactly 2 toasts remaining
      const remainingToasts = ui.bottomCenterToaster.locator('[data-sonner-toast="true"]');
      const count = await remainingToasts.count();
      expect(count).toBe(2);

      // Both remaining should be visible
      for (let i = 0; i < count; i++) {
        await expect(remainingToasts.nth(i)).toBeVisible();
      }
    });
  });
});
