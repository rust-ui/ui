import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * TOAST COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Demo Button                                                           │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                    Toast me                                     │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                           │                                             │
 * │                           │ click                                       │
 * │                           ▼                                             │
 * │                                                                         │
 * │   Toast Notification (appears at bottom-right)                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ℹ️  This is a toast!                                      ✕   │   │
 * │   │      ↑                                                     ↑   │   │
 * │   │   icon (info)                                         dismiss  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TOAST VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Info:                              Success:                           │
 * │   ┌─────────────────────────┐        ┌─────────────────────────┐        │
 * │   │  ℹ️  Info message       │        │  ✓  Success message    │        │
 * │   └─────────────────────────┘        └─────────────────────────┘        │
 * │                                                                         │
 * │   Warning:                           Error:                             │
 * │   ┌─────────────────────────┐        ┌─────────────────────────┐        │
 * │   │  ⚠️  Warning message    │        │  ✕  Error message      │        │
 * │   └─────────────────────────┘        └─────────────────────────┘        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TOAST LIFECYCLE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   1. Trigger        2. Appear           3. Auto-dismiss                 │
 * │   ┌─────────┐      ┌─────────────┐      ┌─────────────┐                 │
 * │   │ Toast   │ ──→  │ ┌─────────┐ │ ──→  │             │                 │
 * │   │  me     │      │ │ Toast!  │ │      │  (hidden)   │                 │
 * │   └─────────┘      │ └─────────┘ │      │             │                 │
 * │                    └─────────────┘      └─────────────┘                 │
 * │                    animate in           after timeout                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TOAST POSITION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │                         PAGE                                    │   │
 * │   │                                                                 │   │
 * │   │                                                                 │   │
 * │   │                                          ┌───────────────────┐  │   │
 * │   │                                          │  Toast message    │  │   │
 * │   │                                          └───────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                     ↑                   │
 * │                                              bottom-right               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ToastPage extends BasePage {
  protected readonly componentName = "toast";

  // Toast elements
  readonly toastButton: Locator;
  readonly toastContainer: Locator;
  readonly toastMessage: Locator;

  constructor(page: Page) {
    super(page);

    // Toast elements - scoped within preview
    this.toastButton = this.preview.getByRole("button", { name: "Toast me" });
    this.toastContainer = this.preview.locator("[data-sonner-toaster]");
    this.toastMessage = this.preview.locator("[data-sonner-toast]");
  }

  async triggerToast() {
    await this.toastButton.click();
    await this.page.waitForTimeout(300);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Toast Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Toast Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Demo page structure:                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    Toast me                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                        ↑                                │
     *   │                   MUST BE VISIBLE                       │
     *   │                   role="button"                         │
     *   │                   name="Toast me"                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Toast trigger button renders on page
     */
    test("should have Toast me button", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await expect(ui.toastButton).toBeVisible();
    });

    /**
     * TEST: Button Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    Toast me                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                           ↑                             │
     *   │                   Text MUST equal "Toast me"            │
     *   │                   (exact text match)                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button displays correct text label
     */
    test("button should display 'Toast me'", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await expect(ui.toastButton).toHaveText("Toast me");
    });
  });

  test.describe("Toast Container", () => {
    /**
     * TEST: Sonner Toast Container Appears
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                     AFTER:                    │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │    Toast me      │  ──►   │    Toast me      │      │
     *   │   └──────────────────┘ click  └──────────────────┘      │
     *   │                                                         │
     *   │                               ┌──────────────────┐      │
     *   │                               │ [data-sonner-    │      │
     *   │                               │   toaster]       │      │
     *   │                               │  ┌────────────┐  │      │
     *   │                               │  │Toast msg   │  │      │
     *   │                               │  └────────────┘  │      │
     *   │                               └──────────────────┘      │
     *   │                                        ↑                │
     *   │                               MUST BE VISIBLE           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sonner toast container appears when triggered
     */
    test("should have toast container (sonner)", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await ui.triggerToast();

      await expect(ui.toastContainer).toBeVisible();
    });

    /**
     * TEST: Sonner Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Toast container element:                              │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div data-sonner-toaster ...>                    │ │
     *   │   │            ↑                                      │ │
     *   │   │       MUST have this attribute                    │ │
     *   │   │       (identifies Sonner toast system)            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Sonner library marker attribute                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has data-sonner-toaster attribute
     */
    test("toast container should have sonner data attribute", async ({
      page,
    }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await ui.triggerToast();

      await expect(ui.toastContainer).toHaveAttribute("data-sonner-toaster");
    });
  });

  test.describe("Button Styling", () => {
    /**
     * TEST: Button Component Data Name
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Button element:                                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <button data-name="Button" ...>                  │ │
     *   │   │                      ↑                            │ │
     *   │   │              MUST equal "Button"                  │ │
     *   │   │              (component identifier)               │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Identifies as reusable Button component               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button uses the Button component (data-name)
     */
    test("button should have Button data-name", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await expect(ui.toastButton).toHaveAttribute("data-name", "Button");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Button Keyboard Focus
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard navigation:                                  │
     *   │                                                         │
     *   │   [Tab] ──► ┌─────────────────────────────────────┐     │
     *   │             │              Toast me               │     │
     *   │             │  ════════════════════════════════   │     │
     *   │             │         ↑ focus ring visible        │     │
     *   │             └─────────────────────────────────────┘     │
     *   │                                                         │
     *   │   Button MUST be focusable via keyboard                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button can receive keyboard focus
     */
    test("button should be focusable", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await ui.toastButton.focus();
      await expect(ui.toastButton).toBeFocused();
    });

    /**
     * TEST: Button Click Interaction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User interaction:                                     │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │              Toast me                           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                     │                                   │
     *   │                     │ click                             │
     *   │                     ▼                                   │
     *   │              (no error thrown)                          │
     *   │                                                         │
     *   │   Button MUST be interactive (clickable)                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button responds to click without errors
     */
    test("button should be clickable", async ({ page }) => {
      const ui = new ToastPage(page);
      await ui.goto();

      await ui.toastButton.click();
      // Should not throw
    });
  });
});
