import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * ALERT DIALOG COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [Open Alert]  ← AlertDialogTrigger (button to open dialog)            │
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░  AlertDialogBackdrop (bg-black/50)  ░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░┌─────────────────────────────────────────┐░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  AlertDialogContent                    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌─────────────────────────────────┐   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  AlertDialogTitle               │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  "Are you absolutely sure?"     │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └─────────────────────────────────┘   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌─────────────────────────────────┐   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  AlertDialogDescription         │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  "This action cannot be undone" │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └─────────────────────────────────┘   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌─────────────────────────────────┐   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  [Cancel]        [Continue]     │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └─────────────────────────────────┘   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░└─────────────────────────────────────────┘░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-state="closed"                 data-state="open"                 │
 * │   ┌─────────────────────┐             ┌─────────────────────┐           │
 * │   │  Dialog hidden      │  ──────►    │  Dialog visible     │           │
 * │   │  (not visible)      │   click     │  Backdrop shown     │           │
 * │   └─────────────────────┘  trigger    │  Content centered   │           │
 * │                                       └─────────────────────┘           │
 * │                                              │                          │
 * │                           ┌──────────────────┼──────────────────┐       │
 * │                           ▼                  ▼                  ▼       │
 * │                      [Cancel]           [Backdrop]          [ESC]       │
 * │                        click              click              key        │
 * │                           │                  │                  │       │
 * │                           └──────────────────┴──────────────────┘       │
 * │                                              │                          │
 * │                                              ▼                          │
 * │                                    data-state="closed"                  │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * POSITIONING (centered modal):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   .fixed                    ← Position fixed to viewport               │
 * │   .top-[50%] .left-[50%]    ← Start at center point                    │
 * │   .-translate-x-1/2         ← Shift left by half width                 │
 * │   .-translate-y-1/2         ← Shift up by half height                  │
 * │   .z-50                     ← Above other content                      │
 * │                                                                         │
 * │   ┌───────────────────────────────────────────────────────────┐         │
 * │   │                           ↑                               │         │
 * │   │                      -translate-y-1/2                     │         │
 * │   │                           │                               │         │
 * │   │              ←───────── 50% ────────►                     │         │
 * │   │         -translate-x-1/2  ●  center point                 │         │
 * │   │                       ┌───────┐                           │         │
 * │   │                       │Dialog │                           │         │
 * │   │                       └───────┘                           │         │
 * │   │                           │                               │         │
 * │   │                          50%                              │         │
 * │   │                           ↓                               │         │
 * │   └───────────────────────────────────────────────────────────┘         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class AlertDialogPage extends BasePage {
  protected readonly componentName = "alert-dialog";

  // Alert dialog elements
  readonly triggerButton: Locator;
  readonly dialogContent: Locator;
  readonly dialogBackdrop: Locator;
  readonly dialogTitle: Locator;
  readonly dialogDescription: Locator;
  readonly cancelButton: Locator;
  readonly continueButton: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger - scoped within preview
    this.triggerButton = this.preview.getByRole("button", { name: "Open Alert" });

    // Content - scoped within preview
    this.dialogContent = this.preview.locator('[data-name="AlertDialogContent"]').first();
    this.dialogBackdrop = this.preview.locator('[data-name="AlertDialogBackdrop"]').first();
    this.dialogTitle = this.preview.locator('[data-name="AlertDialogTitle"]').first();
    this.dialogDescription = this.preview.locator('[data-name="AlertDialogDescription"]').first();

    // Buttons - scoped within preview
    this.cancelButton = this.preview.locator('button:has-text("Cancel")');
    this.continueButton = this.preview.getByRole("button", { name: "Continue" });
  }

  async openDialog() {
    await this.waitForInitialized(this.dialogContent);
    await this.triggerButton.click();
    await this.waitForDataState(this.dialogContent, "open");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Alert Dialog Page", () => {
  /**
   * STRUCTURE TESTS - Verify component elements exist
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   [Open Alert]  ← Trigger button (always visible)               │
   * │                                                                 │
   * │   AlertDialogContent  ← Hidden by default (data-state="closed")│
   * │     ├── AlertDialogTitle                                        │
   * │     ├── AlertDialogDescription                                  │
   * │     └── Action buttons (Cancel, Continue)                       │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Alert    │  <-- MUST BE VISIBLE              │
     *   │   └─────────────────┘      text="Open Alert"            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button renders with correct text
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toBeVisible();
      await expect(ui.triggerButton).toHaveText("Open Alert");
    });

    /**
     * TEST: Dialog Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   On page load:                                         │
     *   │   ┌──────────────────────────────────────────────────┐  │
     *   │   │  AlertDialogContent                              │  │
     *   │   │  data-state="closed"  <-- MUST BE CLOSED         │  │
     *   │   │  (not visible to user)                           │  │
     *   │   └──────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog starts in closed state
     */
    test("dialog should be closed by default", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: AlertDialogContent Attached
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM structure (even when closed):                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div data-name="AlertDialogContent" ...>         │ │
     *   │   │       ↑                                           │ │
     *   │   │  MUST be attached to DOM                          │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: AlertDialogContent element exists in DOM
     */
    test("should have AlertDialogContent", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toBeAttached();
    });

    /**
     * TEST: AlertDialogBackdrop Attached
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM structure:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div data-name="AlertDialogBackdrop" ...>        │ │
     *   │   │       ↑                                           │ │
     *   │   │  MUST be attached to DOM                          │ │
     *   │   │  (bg-black/50 overlay)                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: AlertDialogBackdrop element exists in DOM
     */
    test("should have AlertDialogBackdrop", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogBackdrop).toBeAttached();
    });

    /**
     * TEST: Trigger is Button Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   HTML element type:                                    │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <button>Open Alert</button>                      │ │
     *   │   │     ↑                                             │ │
     *   │   │  tagName MUST be "button"                         │ │
     *   │   │  (not div, span, or anchor)                       │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger uses semantic button element
     */
    test("trigger should be a button element", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      const tagName = await ui.triggerButton.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });
  });

  /**
   * OPEN/CLOSE BEHAVIOR TESTS - Dialog state transitions
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   OPENING:                                                              │
   * │   ┌──────────────┐                  ┌────────────────────────────┐      │
   * │   │ Open Alert 🖱️ │  ───────────►   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░│      │
   * │   └──────────────┘     click        │░░░┌────────────────────┐░░░│      │
   * │                                     │░░░│  Are you sure?     │░░░│      │
   * │                                     │░░░│  [Cancel][Continue]│░░░│      │
   * │                                     │░░░└────────────────────┘░░░│      │
   * │                                     │░░░░░░░░░░░░░░░░░░░░░░░░░░░░│      │
   * │                                     └────────────────────────────┘      │
   * │                                                                         │
   * │   CLOSING (3 ways):                                                     │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   1. [Cancel] button  ──►  data-state="closed"              │       │
   * │   │   2. [ESC] key        ──►  data-state="closed"              │       │
   * │   │   3. Backdrop click   ──►  data-state="closed"              │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Trigger Opens Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                     AFTER:                    │
     *   │   ┌──────────────┐            ┌──────────────┐          │
     *   │   │ Open Alert   │  ──────►   │░░░░░░░░░░░░░░│          │
     *   │   └──────────────┘   click    │░┌──────────┐░│          │
     *   │                               │░│ Dialog   │░│          │
     *   │   data-state=                 │░│ Content  │░│          │
     *   │     "closed"                  │░└──────────┘░│          │
     *   │                               │░░░░░░░░░░░░░░│          │
     *   │                               └──────────────┘          │
     *   │                               data-state="open"         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger changes state to "open"
     */
    test("clicking trigger should open dialog", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.dialogContent);

      await ui.triggerButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Cancel Button Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog open:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Are you absolutely sure?                       │   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │  ──►    │  Continue  │          │   │
     *   │   │  └────────────┘ click   └────────────┘          │   │
     *   │   │        │                                        │   │
     *   │   └────────┼────────────────────────────────────────┘   │
     *   │            │                                            │
     *   │            ▼                                            │
     *   │   data-state="closed"                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel button closes the dialog
     */
    test("clicking Cancel should close dialog", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.cancelButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: ESC Key Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog open:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Are you absolutely sure?                       │   │
     *   │   │                                                 │   │
     *   │   │  [Cancel]              [Continue]               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                    │                                    │
     *   │              [ESC] key                                  │
     *   │                    │                                    │
     *   │                    ▼                                    │
     *   │           data-state="closed"                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ESC keyboard shortcut closes dialog
     */
    test("pressing ESC should close dialog", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await page.keyboard.press("Escape");
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Backdrop Click Does NOT Close Alert Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   AlertDialog vs Dialog difference:                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░ ← click backdrop ░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░┌───────────────────────────────┐░░░░░░░░│   │
     *   │   │░░░░░░░░│  Are you sure?                │░░░░░░░░│   │
     *   │   │░░░░░░░░│  [Cancel] [Continue]          │░░░░░░░░│   │
     *   │   │░░░░░░░░└───────────────────────────────┘░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Dialog MUST stay open (requires explicit action)      │
     *   │   data-state="open" (unchanged)                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: AlertDialog requires button click to close
     */
    test("clicking backdrop should NOT close alert dialog", async ({ page }) => {
      // AlertDialog requires explicit user action (Cancel/Continue button)
      // and does not close on backdrop click
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.dialogBackdrop.click({ force: true });
      await expect(ui.dialogContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Continue Button Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog open:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Are you absolutely sure?                       │   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │         │  Continue  │  ──►     │   │
     *   │   │  └────────────┘         └────────────┘ click    │   │
     *   │   │                               │                 │   │
     *   │   └───────────────────────────────┼─────────────────┘   │
     *   │                                   │                     │
     *   │                                   ▼                     │
     *   │                          data-state="closed"            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Continue button closes the dialog
     */
    test("clicking Continue should close dialog", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.continueButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Dialog Reopens After Closing
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   1. Open        2. Close         3. Reopen             │
     *   │   ┌─────────┐    ┌─────────┐      ┌─────────┐           │
     *   │   │░░░░░░░░░│    │         │      │░░░░░░░░░│           │
     *   │   │░┌─────┐░│──► │         │ ──►  │░┌─────┐░│           │
     *   │   │░│ Dlg │░│    │Open Btn │      │░│ Dlg │░│           │
     *   │   │░└─────┘░│    │         │      │░└─────┘░│           │
     *   │   │░░░░░░░░░│    │         │      │░░░░░░░░░│           │
     *   │   └─────────┘    └─────────┘      └─────────┘           │
     *   │   state=open    state=closed     state=open             │
     *   │                                                         │
     *   │   Dialog MUST be reusable after closing                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog can be reopened multiple times
     */
    test("dialog can be reopened after closing", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      // Close it
      await ui.cancelButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");

      // Reopen it
      await ui.triggerButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "open");
    });
  });

  /**
   * CONTENT TESTS - Title, description, and action buttons
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   AlertDialogContent                                                    │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   ┌─────────────────────────────────────────────────────┐   │       │
   * │   │   │  AlertDialogTitle                                   │   │       │
   * │   │   │  "Are you absolutely sure?"                         │   │       │
   * │   │   │  (text-lg, font-semibold)                           │   │       │
   * │   │   └─────────────────────────────────────────────────────┘   │       │
   * │   │                                                             │       │
   * │   │   ┌─────────────────────────────────────────────────────┐   │       │
   * │   │   │  AlertDialogDescription                             │   │       │
   * │   │   │  "This action cannot be undone. This will           │   │       │
   * │   │   │   permanently delete your account..."               │   │       │
   * │   │   │  (text-sm, text-muted-foreground)                   │   │       │
   * │   │   └─────────────────────────────────────────────────────┘   │       │
   * │   │                                                             │       │
   * │   │   ┌─────────────────────────────────────────────────────┐   │       │
   * │   │   │  ┌──────────┐                    ┌──────────────┐   │   │       │
   * │   │   │  │ Cancel   │                    │  Continue    │   │   │       │
   * │   │   │  │ (outline)│                    │  (default)   │   │   │       │
   * │   │   │  └──────────┘                    └──────────────┘   │   │       │
   * │   │   └─────────────────────────────────────────────────────┘   │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Content", () => {
    /**
     * TEST: Dialog Title Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AlertDialogTitle                               │   │
     *   │   │  ┌─────────────────────────────────────────┐    │   │
     *   │   │  │  "Are you absolutely sure?"             │    │   │
     *   │   │  └─────────────────────────────────────────┘    │   │
     *   │   │       ↑                                         │   │
     *   │   │  MUST BE VISIBLE                                │   │
     *   │   │  text must match exactly                        │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title is visible with correct text
     */
    test("should have dialog title", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogTitle).toBeVisible();
      await expect(ui.dialogTitle).toHaveText("Are you absolutely sure?");
    });

    /**
     * TEST: Dialog Description Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AlertDialogDescription                         │   │
     *   │   │  ┌─────────────────────────────────────────┐    │   │
     *   │   │  │  "This action cannot be undone. This    │    │   │
     *   │   │  │   will permanently delete your..."      │    │   │
     *   │   │  └─────────────────────────────────────────┘    │   │
     *   │   │       ↑                                         │   │
     *   │   │  MUST BE VISIBLE                                │   │
     *   │   │  MUST contain "This action cannot be undone"    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description is visible with warning text
     */
    test("should have dialog description", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogDescription).toBeVisible();
      await expect(ui.dialogDescription).toContainText(
        "This action cannot be undone"
      );
    });

    /**
     * TEST: Cancel Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog footer:                                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │         │  Continue  │          │   │
     *   │   │  └────────────┘         └────────────┘          │   │
     *   │   │        ↑                                        │   │
     *   │   │   MUST BE VISIBLE                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel button is visible in dialog
     */
    test("should have Cancel button", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.cancelButton).toBeVisible();
    });

    /**
     * TEST: Continue Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog footer:                                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │         │  Continue  │          │   │
     *   │   │  └────────────┘         └────────────┘          │   │
     *   │   │                               ↑                 │   │
     *   │   │                          MUST BE VISIBLE        │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Continue button is visible in dialog
     */
    test("should have Continue button", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.continueButton).toBeVisible();
    });

    /**
     * TEST: Title Text Styling Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   AlertDialogTitle classes:                             │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .text-lg        <-- larger font size (1.125rem)  │ │
     *   │   │  .font-semibold  <-- medium-bold weight (600)     │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Ensures title stands out visually                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has proper typography classes
     */
    test("title should have correct text styling", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogTitle).toHaveClass(/text-lg/);
      await expect(ui.dialogTitle).toHaveClass(/font-semibold/);
    });

    /**
     * TEST: Description Muted Text Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   AlertDialogDescription styling:                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .text-muted-foreground                           │ │
     *   │   │       ↑                                           │ │
     *   │   │  Subdued color (secondary text)                   │ │
     *   │   │  Creates visual hierarchy:                        │ │
     *   │   │    Title (prominent) > Description (muted)        │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description uses muted foreground color
     */
    test("description should have muted text color", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogDescription).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: Description Small Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   AlertDialogDescription styling:                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .text-sm                                         │ │
     *   │   │       ↑                                           │ │
     *   │   │  Smaller font size (0.875rem)                     │ │
     *   │   │  Title: text-lg (1.125rem)                        │ │
     *   │   │  Description: text-sm (0.875rem)                  │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description uses small text size
     */
    test("description should have text-sm", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogDescription).toHaveClass(/text-sm/);
    });
  });

  /**
   * STYLING TESTS - Visual appearance and positioning
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   DIALOG CONTENT STYLING:                                               │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .fixed              ← Fixed position                       │       │
   * │   │  .z-50               ← Stack above other content            │       │
   * │   │  .top-[50%]          ← Vertical centering                   │       │
   * │   │  .left-[50%]         ← Horizontal centering                 │       │
   * │   │  .-translate-x-1/2   ← Adjust for element width             │       │
   * │   │  .-translate-y-1/2   ← Adjust for element height            │       │
   * │   │  .bg-background      ← Theme background color               │       │
   * │   │  .rounded-2xl        ← Rounded corners                      │       │
   * │   │  .shadow-lg          ← Drop shadow                          │       │
   * │   │  .p-6                ← Padding                              │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   BACKDROP STYLING:                                                     │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .fixed .inset-0     ← Cover entire viewport                │       │
   * │   │  .bg-black/50        ← Semi-transparent black               │       │
   * │   │  .z-50               ← Below dialog content                 │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Styling", () => {
    /**
     * TEST: Fixed Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Viewport-relative positioning:                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .fixed                                           │ │
     *   │   │       ↑                                           │ │
     *   │   │  position: fixed;                                 │ │
     *   │   │  Dialog stays in place during scroll              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog uses fixed positioning
     */
    test("dialog content should have fixed positioning", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/fixed/);
    });

    /**
     * TEST: Z-Index Stacking
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Layer stacking:                                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  z-100: Dialog Content  ← top layer               │ │
     *   │   │  z-50:  Backdrop                                  │ │
     *   │   │  z-0:   Page content                              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   MUST have class: z-100                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog appears above other content
     */
    test("dialog content should have z-100", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/z-100/);
    });

    /**
     * TEST: Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog content styling:                               │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .bg-background                                   │ │
     *   │   │       ↑                                           │ │
     *   │   │  Uses theme background color                      │ │
     *   │   │  (respects dark/light mode)                       │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog uses theme background color
     */
    test("dialog content should have bg-background", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/bg-background/);
    });

    /**
     * TEST: Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog shape:                                         │
     *   │   ╭─────────────────────────────────────────────────╮   │
     *   │   │                                                 │   │
     *   │   │  .rounded-2xl  (border-radius: 1rem)            │   │
     *   │   │                                                 │   │
     *   │   ╰─────────────────────────────────────────────────╯   │
     *   │        ↑ rounded corners                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog has rounded-2xl corners
     */
    test("dialog content should have rounded corners", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/rounded-2xl/);
    });

    /**
     * TEST: Center Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Centering technique:                                  │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                     50%                           │ │
     *   │   │              ←─────────────────→                  │ │
     *   │   │                    ┌─────┐                        │ │
     *   │   │     50%  ──────────│ Dlg │                        │ │
     *   │   │                    └─────┘                        │ │
     *   │   │                                                   │ │
     *   │   │  .top-[50%] .left-[50%]                           │ │
     *   │   │  + translate for perfect centering                │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog is centered in viewport
     */
    test("dialog content should be centered", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/top-\[50%\]/);
      await expect(ui.dialogContent).toHaveClass(/left-\[50%\]/);
    });

    /**
     * TEST: Backdrop Semi-Transparency
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Backdrop overlay:                                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ │
     *   │   │░░░  .bg-black/50  ←  rgba(0,0,0,0.5)  ░░░░░░░░░░░░│ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Backdrop is 50% transparent black
     */
    test("backdrop should have semi-transparent background", async ({
      page,
    }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogBackdrop).toHaveClass(/bg-black\/50/);
    });

    /**
     * TEST: Backdrop Full Coverage
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Backdrop sizing:                                      │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .fixed  .inset-0                                 │ │
     *   │   │      ↑        ↑                                   │ │
     *   │   │  position  top/right/bottom/left: 0               │ │
     *   │   │  fixed     Covers entire viewport                 │ │
     *   │   │                                                   │ │
     *   │   │  ┌─────────────────────────────────────────────┐  │ │
     *   │   │  │                  VIEWPORT                   │  │ │
     *   │   │  │                                             │  │ │
     *   │   │  │    Backdrop covers 100% of viewport         │  │ │
     *   │   │  │                                             │  │ │
     *   │   │  └─────────────────────────────────────────────┘  │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Backdrop covers full viewport
     */
    test("backdrop should cover full viewport", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogBackdrop).toHaveClass(/fixed/);
      await expect(ui.dialogBackdrop).toHaveClass(/inset-0/);
    });

    /**
     * TEST: Dialog Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Elevation effect:                                     │
     *   │            ╭───────────────────────────╮                 │
     *   │           ╱│                           │                 │
     *   │          ╱ │      Dialog Content       │                 │
     *   │         ╱  │                           │                 │
     *   │        ╱   ╰───────────────────────────╯                 │
     *   │       ╱    shadow-lg (drop shadow)                       │
     *   │      ╱                                                   │
     *   │     ↓                                                    │
     *   │   .shadow-lg creates depth perception                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog has large shadow for depth
     */
    test("dialog should have shadow", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/shadow-lg/);
    });

    /**
     * TEST: Dialog Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Content spacing:                                      │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  ╭─────────────────────────────────────────────╮  │ │
     *   │   │  │ ╭─────────────────────────────────────────╮ │  │ │
     *   │   │  │ │                                         │ │  │ │
     *   │   │  │ │         Dialog inner content            │ │  │ │
     *   │   │  │ │                                         │ │  │ │
     *   │   │  │ ╰─────────────────────────────────────────╯ │  │ │
     *   │   │  ╰─────────────────────────────────────────────╯  │ │
     *   │   │    ↑                                              │ │
     *   │   │  .p-6 (padding: 1.5rem)                           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog has p-6 padding
     */
    test("dialog should have padding", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/p-6/);
    });

    /**
     * TEST: Max-Width Constraint
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Responsive width:                                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  viewport                                         │ │
     *   │   │  │◄─────────────── 100% ───────────────────►│     │ │
     *   │   │                                                   │ │
     *   │   │     │◄──── max-w-[calc(100%-2rem)] ────►│         │ │
     *   │   │     ┌─────────────────────────────────┐           │ │
     *   │   │     │        Dialog Content           │           │ │
     *   │   │     └─────────────────────────────────┘           │ │
     *   │   │     │◄─ 1rem margin ─►│◄─ 1rem margin ─►│         │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog has max-width with side margins
     */
    test("dialog should have max-width constraint", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/max-w-\[calc\(100%-2rem\)\]/);
    });
  });

  /**
   * ACCESSIBILITY TESTS - Focus management and keyboard navigation
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   FOCUS FLOW:                                                           │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   [Open Alert]  ──► click ──►  Dialog opens                 │       │
   * │   │                                    │                        │       │
   * │   │                                    ▼                        │       │
   * │   │                              Focus trapped inside           │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   TAB NAVIGATION (within dialog):                                       │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   [Cancel] ◄──────────────────────────► [Continue]          │       │
   * │   │      │              Tab / Shift+Tab           │             │       │
   * │   │      └────────────────────────────────────────┘             │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   KEYBOARD SHORTCUTS:                                                   │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   [ESC]    ──►  Close dialog                                │       │
   * │   │   [Tab]    ──►  Move focus forward                          │       │
   * │   │   [Shift+Tab] ──►  Move focus backward                      │       │
   * │   │   [Enter/Space] ──►  Activate focused button                │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Accessibility", () => {
    /**
     * TEST: Trigger Button Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard navigation:                                  │
     *   │                                                         │
     *   │   [Tab] ──► ┌─────────────────┐                         │
     *   │             │   Open Alert    │                         │
     *   │             │  ══════════════ │  ← focus ring visible   │
     *   │             └─────────────────┘                         │
     *   │                                                         │
     *   │   Trigger MUST receive keyboard focus                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can be focused
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await ui.triggerButton.focus();
      await expect(ui.triggerButton).toBeFocused();
    });

    /**
     * TEST: Cancel Button Focus When Open
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog open with focus on Cancel:                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Are you absolutely sure?                       │   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │         │  Continue  │          │   │
     *   │   │  │ ══════════ │         └────────────┘          │   │
     *   │   │  └────────────┘                                 │   │
     *   │   │        ↑                                        │   │
     *   │   │   MUST be focusable                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel button can receive focus
     */
    test("Cancel button should be focusable when open", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.cancelButton.focus();
      await expect(ui.cancelButton).toBeFocused();
    });

    /**
     * TEST: Tab Navigation Between Buttons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tab flow inside dialog:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │  ──►    │  Continue  │          │   │
     *   │   │  │  (focused) │  [Tab]  │  (focused) │          │   │
     *   │   │  └────────────┘         └────────────┘          │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Tab moves focus from Cancel to Continue               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key navigates between buttons
     */
    test("buttons should be tabbable", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.cancelButton.focus();
      await page.keyboard.press("Tab");
      await expect(ui.continueButton).toBeFocused();
    });

    /**
     * TEST: Shift+Tab Reverse Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Reverse tab flow:                                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │  ◄──    │  Continue  │          │   │
     *   │   │  │  (focused) │ [Sh+Tab]│  (focused) │          │   │
     *   │   │  └────────────┘         └────────────┘          │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Shift+Tab moves focus backwards                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Shift+Tab navigates backwards
     */
    test("should navigate backwards with Shift+Tab", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.continueButton.focus();
      await page.keyboard.press("Shift+Tab");
      await expect(ui.cancelButton).toBeFocused();
    });

    /**
     * TEST: Continue Button Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog open with focus on Continue:                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Are you absolutely sure?                       │   │
     *   │   │                                                 │   │
     *   │   │  ┌────────────┐         ┌────────────┐          │   │
     *   │   │  │   Cancel   │         │  Continue  │          │   │
     *   │   │  └────────────┘         │ ══════════ │          │   │
     *   │   │                         └────────────┘          │   │
     *   │   │                               ↑                 │   │
     *   │   │                          MUST be focusable      │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Continue button can receive focus
     */
    test("Continue button should be focusable", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.continueButton.focus();
      await expect(ui.continueButton).toBeFocused();
    });

    /**
     * TEST: Focus Ring Styles on Trigger
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Visual focus indicator:                               │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   ┌═══════════════════════┐                       │ │
     *   │   │   ║     Open Alert        ║  ← focus-visible:ring │ │
     *   │   │   └═══════════════════════┘                       │ │
     *   │   │                                                   │ │
     *   │   │   Ring appears when focused via keyboard          │ │
     *   │   │   (not on mouse click)                            │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has focus-visible ring class
     */
    test("trigger should have focus ring styles", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toHaveClass(/focus-visible:ring/);
    });
  });

  /**
   * ANIMATION TESTS - Entry and exit transitions
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   ENTRY ANIMATION (data-state="open"):                                  │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   data-[state=open]:animate-in                              │       │
   * │   │   data-[state=open]:fade-in-0                               │       │
   * │   │   data-[state=open]:zoom-in-95                              │       │
   * │   │                                                             │       │
   * │   │   Frame 0%        Frame 50%        Frame 100%               │       │
   * │   │   ┌─────┐         ┌───────┐        ┌─────────┐              │       │
   * │   │   │ · · │  ──►    │  ···  │  ──►   │ Content │              │       │
   * │   │   └─────┘         └───────┘        └─────────┘              │       │
   * │   │   scale: 0.95     scale: 0.97      scale: 1                 │       │
   * │   │   opacity: 0      opacity: 0.5     opacity: 1               │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   EXIT ANIMATION (data-state="closed"):                                 │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   data-[state=closed]:animate-out                           │       │
   * │   │   data-[state=closed]:fade-out-0                            │       │
   * │   │   data-[state=closed]:zoom-out-95                           │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Animation", () => {
    /**
     * TEST: Entry Animation Opacity
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Entry animation (when opening):                       │
     *   │                                                         │
     *   │   data-state="closed"         data-state="open"         │
     *   │   opacity: 0%      ───────►   opacity: 100%             │
     *   │   ┌─ ─ ─ ─ ─ ┐               ┌───────────┐              │
     *   │   │ invisible │               │  visible  │              │
     *   │   └─ ─ ─ ─ ─ ┘               └───────────┘              │
     *   │                                                         │
     *   │   class: data-[state=open]:opacity-100                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog fades in when opened
     */
    test("dialog should have entry animation classes", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(
        /data-\[state=open\]:opacity-100/
      );
    });

    /**
     * TEST: Entry Scale Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Scale animation (when opening):                       │
     *   │                                                         │
     *   │   data-state="closed"         data-state="open"         │
     *   │   scale: 95%       ───────►   scale: 100%               │
     *   │   ┌─────────┐                 ┌───────────┐             │
     *   │   │  small  │                 │  normal   │             │
     *   │   └─────────┘                 └───────────┘             │
     *   │                                                         │
     *   │   class: data-[state=open]:scale-100                    │
     *   │   Subtle zoom-in effect                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog scales up when opened
     */
    test("dialog should have scale-in animation", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(
        /data-\[state=open\]:scale-100/
      );
    });

    /**
     * TEST: Exit Opacity Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Exit animation (when closing):                        │
     *   │                                                         │
     *   │   data-state="open"           data-state="closed"       │
     *   │   opacity: 100%    ───────►   opacity: 0%               │
     *   │   ┌───────────┐               ┌─ ─ ─ ─ ─ ┐              │
     *   │   │  visible  │               │ invisible │              │
     *   │   └───────────┘               └─ ─ ─ ─ ─ ┘              │
     *   │                                                         │
     *   │   class: data-[state=closed]:opacity-0                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog fades out when closed
     */
    test("dialog should have exit opacity class", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(
        /data-\[state=closed\]:opacity-0/
      );
    });

    /**
     * TEST: Exit Scale Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Scale animation (when closing):                       │
     *   │                                                         │
     *   │   data-state="open"           data-state="closed"       │
     *   │   scale: 100%      ───────►   scale: 95%                │
     *   │   ┌───────────┐               ┌─────────┐               │
     *   │   │  normal   │               │  small  │               │
     *   │   └───────────┘               └─────────┘               │
     *   │                                                         │
     *   │   class: data-[state=closed]:scale-95                   │
     *   │   Subtle zoom-out effect                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog scales down when closed
     */
    test("dialog should have exit scale class", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(
        /data-\[state=closed\]:scale-95/
      );
    });

    /**
     * TEST: Backdrop Fade Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Backdrop opacity transitions:                         │
     *   │                                                         │
     *   │   OPENING:                                              │
     *   │   ┌─────────────┐        ┌─────────────┐                │
     *   │   │             │  ──►   │░░░░░░░░░░░░░│                │
     *   │   │  invisible  │        │░ bg-black  ░│                │
     *   │   │             │        │░   /50     ░│                │
     *   │   └─────────────┘        └─────────────┘                │
     *   │   opacity-0              opacity-100                    │
     *   │                                                         │
     *   │   CLOSING: reverse (opacity-100 → opacity-0)            │
     *   │                                                         │
     *   │   Classes:                                              │
     *   │   - data-[state=open]:opacity-100                       │
     *   │   - data-[state=closed]:opacity-0                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Backdrop fades in/out smoothly
     */
    test("backdrop should have fade animation", async ({ page }) => {
      const ui = new AlertDialogPage(page);
      await ui.goto();

      await expect(ui.dialogBackdrop).toHaveClass(
        /data-\[state=open\]:opacity-100/
      );
      await expect(ui.dialogBackdrop).toHaveClass(
        /data-\[state=closed\]:opacity-0/
      );
    });
  });
});
