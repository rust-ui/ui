import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DIALOG COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [Open Dialog]  ← Trigger button                                       │
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░  DialogBackdrop (bg-black/50)  ░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░┌─────────────────────────────────────────┐░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  DialogContent                      [×] │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌─────────────────────────────────┐    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  <h3> Edit profile              │    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  <p> Make changes to your...    │    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └─────────────────────────────────┘    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌─────────────────────────────────┐    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  Name: [Max Wells_________]     │    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │  Username: [@maxwells_____]     │    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └─────────────────────────────────┘    │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  ┌────────────┐    ┌────────────────┐   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  │   Cancel   │    │  Save changes  │   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░│  └────────────┘    └────────────────┘   │░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░└─────────────────────────────────────────┘░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-state="closed"  ◄───────────────────►  data-state="open"         │
 * │                                                                         │
 * │   CLOSING TRIGGERS:                                                     │
 * │   ├── [×] Close button (aria-label="Close dialog")                      │
 * │   ├── [Cancel] button                                                   │
 * │   ├── [ESC] key                                                         │
 * │   └── Backdrop click                                                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SCROLLABLE VARIANT:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  Terms of Service                                           │       │
 * │   │  ┌───────────────────────────────────────────────────────┐  │       │
 * │   │  │  ScrollArea                                       ▲   │  │       │
 * │   │  │  ┌─────────────────────────────────────────────┐  █   │  │       │
 * │   │  │  │  Long scrollable content...                 │  █   │  │       │
 * │   │  │  │  ...                                        │  ░   │  │       │
 * │   │  │  │  ...                                        │  ░   │  │       │
 * │   │  │  └─────────────────────────────────────────────┘  ▼   │  │       │
 * │   │  └───────────────────────────────────────────────────────┘  │       │
 * │   │  [Decline]                               [Accept]           │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DialogPage extends BasePage {
  protected readonly componentName = "dialog";

  // Main dialog demo
  readonly triggerButton: Locator;
  readonly dialogContent: Locator;
  readonly dialogBackdrop: Locator;
  readonly closeButton: Locator;
  readonly dialogTitle: Locator;
  readonly dialogDescription: Locator;
  readonly cancelButton: Locator;
  readonly saveButton: Locator;

  // Form inputs
  readonly nameInput: Locator;
  readonly usernameInput: Locator;

  // Scrollable dialog
  readonly scrollableTrigger: Locator;
  readonly scrollableContent: Locator;
  readonly declineButton: Locator;
  readonly acceptButton: Locator;

  constructor(page: Page) {
    super(page);

    // Main dialog - trigger scoped within preview, content in portal
    this.triggerButton = this.preview.getByRole("button", { name: "Open Dialog" });
    this.dialogContent = page.locator('[data-name="DialogContent"]').first();
    this.dialogBackdrop = page.locator('[data-name="DialogBackdrop"]').first();
    // X close button has absolute positioning
    this.closeButton = this.dialogContent.locator(
      "button.absolute.top-4.right-4"
    );
    this.dialogTitle = this.dialogContent.locator("h3");
    this.dialogDescription = this.dialogContent.locator(
      "p.text-muted-foreground"
    );
    // Cancel and Save buttons (use text content, not role name due to aria-label)
    this.cancelButton = this.dialogContent.getByText("Cancel", { exact: true });
    this.saveButton = this.dialogContent.getByText("Save changes", {
      exact: true,
    });

    // Form inputs - scoped to dialog content
    this.nameInput = this.dialogContent.locator('input[name="name"]');
    this.usernameInput = this.dialogContent.locator('input[name="username"]');

    // Scrollable dialog - trigger scoped within preview
    this.scrollableTrigger = this.preview.getByRole("button", {
      name: "Open Scrollable Dialog",
    });
    // Scrollable content - need to find the right portal
    this.scrollableContent = page.locator('[data-name="DialogContent"]').nth(1);
    this.declineButton = this.scrollableContent.getByText("Decline", {
      exact: true,
    });
    this.acceptButton = this.scrollableContent.getByText("Accept", {
      exact: true,
    });
  }

  async openDialog() {
    await this.waitForInitialized(this.dialogContent);
    await this.triggerButton.click();
    await this.waitForDataState(this.dialogContent, "open");
    await expect(this.closeButton).toBeVisible();
  }

  async closeDialog() {
    await this.closeButton.click();
    await this.waitForDataState(this.dialogContent, "closed");
  }

  async openScrollableDialog() {
    await this.waitForInitialized(this.scrollableContent);
    await this.scrollableTrigger.click();
    await this.waitForDataState(this.scrollableContent, "open");
    await expect(this.declineButton).toBeVisible();
  }

  async isDialogOpen(): Promise<boolean> {
    const state = await this.dialogContent.getAttribute("data-state");
    return state === "open";
  }

  async isDialogClosed(): Promise<boolean> {
    const state = await this.dialogContent.getAttribute("data-state");
    return state === "closed";
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Dialog Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Dialog   │  <-- MUST BE VISIBLE              │
     *   │   └─────────────────┘      with text "Open Dialog"      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button renders and has correct text
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toBeVisible();
      await expect(ui.triggerButton).toHaveText("Open Dialog");
    });

    /**
     * TEST: Dialog Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DialogContent         DialogBackdrop                  │
     *   │   ┌──────────────┐      ┌──────────────┐                │
     *   │   │ data-state=  │      │ data-state=  │                │
     *   │   │   "closed"   │      │   "closed"   │                │
     *   │   └──────────────┘      └──────────────┘                │
     *   │          ↑                     ↑                        │
     *   │    MUST BE CLOSED        MUST BE CLOSED                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog starts in closed state on page load
     */
    test("dialog should be closed by default", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
      await expect(ui.dialogBackdrop).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: DialogContent Styling Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DialogContent classes:                                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .bg-background   <-- background color             │ │
     *   │   │  .border          <-- border styling               │ │
     *   │   │  .rounded-2xl     <-- rounded corners              │ │
     *   │   │  .shadow-lg       <-- shadow effect                │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: DialogContent has proper Tailwind styling classes
     */
    test("should have DialogContent with correct classes", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/bg-background/);
      await expect(ui.dialogContent).toHaveClass(/border/);
      await expect(ui.dialogContent).toHaveClass(/rounded-2xl/);
      await expect(ui.dialogContent).toHaveClass(/shadow-lg/);
    });

    /**
     * TEST: Close Button in Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  DialogContent                              [×] │   │
     *   │   │                                              ↑   │   │
     *   │   │                              MUST BE VISIBLE     │   │
     *   │   │                              aria-label=         │   │
     *   │   │                              "Close dialog"      │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button is visible with proper aria-label
     */
    test("should have close button in dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.closeButton).toBeVisible();
      await expect(ui.closeButton).toHaveAttribute(
        "aria-label",
        "Close dialog"
      );
    });

    /**
     * TEST: Dialog Title and Description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h3> Edit profile              <-- TITLE       │   │
     *   │   │  <p> Make changes to your...    <-- DESCRIPTION │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog has visible title and description text
     */
    test("should have dialog title and description", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogTitle).toBeVisible();
      await expect(ui.dialogTitle).toHaveText("Edit profile");
      await expect(ui.dialogDescription).toContainText(
        "Make changes to your profile here"
      );
    });

    /**
     * TEST: Form Inputs Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Name:     [________________]  <-- MUST EXIST   │   │
     *   │   │  Username: [________________]  <-- MUST EXIST   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Form has name and username input fields
     */
    test("should have form inputs", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.nameInput).toBeVisible();
      await expect(ui.usernameInput).toBeVisible();
    });

    /**
     * TEST: Footer Buttons Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌────────────┐    ┌────────────────┐           │   │
     *   │   │  │   Cancel   │    │  Save changes  │           │   │
     *   │   │  └────────────┘    └────────────────┘           │   │
     *   │   │        ↑                   ↑                    │   │
     *   │   │   MUST EXIST          MUST EXIST                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel and Save buttons are visible in footer
     */
    test("should have footer buttons", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.cancelButton).toBeVisible();
      await expect(ui.saveButton).toBeVisible();
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Trigger Opens Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌──────────────┐           ┌──────────────────────┐   │
     *   │   │ Open Dialog  │  CLICK    │░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   └──────────────┘  ────→    │░░┌──────────────┐░░░░│   │
     *   │   data-state=                │░░│   Dialog     │░░░░│   │
     *   │     "closed"                 │░░└──────────────┘░░░░│   │
     *   │                              └──────────────────────┘   │
     *   │                              data-state="open"          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger transitions dialog to open state
     */
    test("clicking trigger should open dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
      await ui.triggerButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "open");
      await expect(ui.dialogBackdrop).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: X Button Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  DialogContent                              [×] │   │
     *   │   │                                              ↑   │   │
     *   │   │                                           CLICK  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                         │                               │
     *   │                         ▼                               │
     *   │                  data-state="closed"                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: X close button properly closes the dialog
     */
    test("clicking X button should close dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.closeButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Cancel Button Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌────────────┐    ┌────────────────┐           │   │
     *   │   │  │   Cancel   │    │  Save changes  │           │   │
     *   │   │  └─────┬──────┘    └────────────────┘           │   │
     *   │   │        │                                        │   │
     *   │   └────────│────────────────────────────────────────┘   │
     *   │            │ CLICK                                      │
     *   │            ▼                                            │
     *   │     data-state="closed"                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel button closes the dialog
     */
    test("clicking Cancel button should close dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.cancelButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Backdrop Click Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░ CLICK HERE ░░┌──────────────┐░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░↓░░░░░░░░░░│   Dialog     │░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░░░└──────────────┘░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                         │                               │
     *   │                         ▼                               │
     *   │                  data-state="closed"                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking backdrop (outside dialog) closes it
     */
    test("clicking backdrop should close dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      // Click the backdrop (not the dialog content)
      await ui.dialogBackdrop.click({ force: true });
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: ESC Key Closes Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░┌──────────────┐░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░│   Dialog     │░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░└──────────────┘░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                         │                               │
     *   │              ┌──────────┴──────────┐                    │
     *   │              │   Press [ESC] key   │                    │
     *   │              └──────────┬──────────┘                    │
     *   │                         ▼                               │
     *   │                  data-state="closed"                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ESC keyboard shortcut closes the dialog
     */
    test("pressing ESC should close dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await page.keyboard.press("Escape");
      await expect(ui.dialogContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Dialog Reopen After Close
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Open      Step 2: Close     Step 3: Reopen    │
     *   │   ┌──────────┐      ┌──────────┐      ┌──────────────┐  │
     *   │   │░░░░░░░░░░│      │          │      │░░░░░░░░░░░░░░│  │
     *   │   │░░Dialog░░│ ───→ │ [Trigger]│ ───→ │░░░Dialog░░░░│  │
     *   │   │░░░░░░░░░░│      │          │      │░░░░░░░░░░░░░░│  │
     *   │   └──────────┘      └──────────┘      └──────────────┘  │
     *   │   state="open"      state="closed"    state="open"      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog can be reopened after being closed
     */
    test("dialog can be reopened after closing", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      // Open and close
      await ui.openDialog();
      await ui.closeDialog();

      // Reopen
      await ui.triggerButton.click();
      await expect(ui.dialogContent).toHaveAttribute("data-state", "open");
    });
  });

  test.describe("Form Interaction", () => {
    /**
     * TEST: Name Input Default Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Name:     [Max Wells_____________]             │   │
     *   │   │                    ↑                            │   │
     *   │   │            DEFAULT VALUE                        │   │
     *   │   │            must be "Max Wells"                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input is pre-filled with default value
     */
    test("name input should have default value", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.nameInput).toHaveValue("Max Wells");
    });

    /**
     * TEST: Username Input Default Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Username: [@maxwells____________]              │   │
     *   │   │                    ↑                            │   │
     *   │   │            DEFAULT VALUE                        │   │
     *   │   │            must be "@maxwells"                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Username input is pre-filled with default value
     */
    test("username input should have default value", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.usernameInput).toHaveValue("@maxwells");
    });

    /**
     * TEST: Edit Name Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌─────────────────────┐    ┌─────────────────────┐    │
     *   │   │ [Max Wells_______]  │    │ [John Doe________]  │    │
     *   │   └─────────────────────┘    └─────────────────────┘    │
     *   │              │                         ↑                │
     *   │              └───── clear + fill ──────┘                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input is editable
     */
    test("should be able to edit name input", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.nameInput.clear();
      await ui.nameInput.fill("John Doe");
      await expect(ui.nameInput).toHaveValue("John Doe");
    });

    /**
     * TEST: Edit Username Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌─────────────────────┐    ┌─────────────────────┐    │
     *   │   │ [@maxwells______]   │    │ [@johndoe_______]   │    │
     *   │   └─────────────────────┘    └─────────────────────┘    │
     *   │              │                         ↑                │
     *   │              └───── clear + fill ──────┘                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Username input is editable
     */
    test("should be able to edit username input", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.usernameInput.clear();
      await ui.usernameInput.fill("@johndoe");
      await expect(ui.usernameInput).toHaveValue("@johndoe");
    });

    /**
     * TEST: Name Input Autofocus
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Dialog opens:                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Name:     [|________________]  <-- FOCUSED     │   │
     *   │   │                ↑                                │   │
     *   │   │          autofocus attribute                    │   │
     *   │   │          must be present                        │   │
     *   │   │                                                 │   │
     *   │   │  Username: [________________]                   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input has autofocus attribute
     */
    test("name input should have autofocus", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      // Check autofocus attribute exists (boolean attribute may be empty or "true")
      const hasAutofocus = await ui.nameInput.evaluate(
        (el) => el.hasAttribute("autofocus")
      );
      expect(hasAutofocus).toBe(true);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Dialog Title Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h3 class="font-semibold">Edit profile</h3>    │   │
     *   │   │       ↑              ↑                          │   │
     *   │   │   TAG: h3      CLASS: font-semibold             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title is h3 element with semibold font weight
     */
    test("dialog title should be h3 with font-semibold", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      const tagName = await ui.dialogTitle.evaluate(
        (el) => el.tagName.toLowerCase()
      );
      expect(tagName).toBe("h3");
      await expect(ui.dialogTitle).toHaveClass(/font-semibold/);
    });

    /**
     * TEST: Dialog Description Muted Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h3>Edit profile</h3>                          │   │
     *   │   │  <p class="text-muted-foreground">              │   │
     *   │   │    Make changes to your profile here...         │   │
     *   │   │  </p>        ↑                                  │   │
     *   │   │        MUTED COLOR                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description has muted text color class
     */
    test("dialog description should have muted foreground", async ({
      page,
    }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.dialogDescription).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: Backdrop Semi-Transparent Background
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░ bg-black/50 ░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░ (50% opacity) ░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Backdrop has 50% black background for dimming
     */
    test("backdrop should have semi-transparent background", async ({
      page,
    }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.dialogBackdrop).toHaveClass(/bg-black\/50/);
    });

    /**
     * TEST: Dialog Centered Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │              ┌───────────────┐                  │   │
     *   │   │              │    Dialog     │                  │   │
     *   │   │              │  fixed        │                  │   │
     *   │   │              │  top-[50%]    │                  │   │
     *   │   │              │  left-[50%]   │                  │   │
     *   │   │              └───────────────┘                  │   │
     *   │   │                     ↑                           │   │
     *   │   │                 CENTERED                        │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dialog is centered using fixed positioning
     */
    test("dialog should be centered on screen", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await expect(ui.dialogContent).toHaveClass(/fixed/);
      await expect(ui.dialogContent).toHaveClass(/top-\[50%\]/);
      await expect(ui.dialogContent).toHaveClass(/left-\[50%\]/);
    });

    /**
     * TEST: Cancel Button Outline Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────────────────┐                    │
     *   │   │          Cancel                │                    │
     *   │   │   ┌──────────────────────┐     │                    │
     *   │   │   │ .border              │     │                    │
     *   │   │   │ .bg-background       │     │                    │
     *   │   │   └──────────────────────┘     │                    │
     *   │   │            ↑                   │                    │
     *   │   │      OUTLINE STYLE             │                    │
     *   │   └────────────────────────────────┘                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cancel button has outline variant styling
     */
    test("Cancel button should have outline variant", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.cancelButton).toHaveClass(/border/);
      await expect(ui.cancelButton).toHaveClass(/bg-background/);
    });

    /**
     * TEST: Save Button Primary Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────────────────┐                    │
     *   │   │       Save changes             │                    │
     *   │   │   ┌──────────────────────┐     │                    │
     *   │   │   │ .bg-primary          │     │                    │
     *   │   │   └──────────────────────┘     │                    │
     *   │   │            ↑                   │                    │
     *   │   │      PRIMARY STYLE             │                    │
     *   │   └────────────────────────────────┘                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Save button has primary variant styling
     */
    test("Save button should have primary variant", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.saveButton).toHaveClass(/bg-primary/);
    });
  });

  test.describe("Scrollable Dialog", () => {
    /**
     * TEST: Scrollable Dialog Trigger
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────┐                         │
     *   │   │  Open Scrollable Dialog   │  <-- MUST BE VISIBLE    │
     *   │   └───────────────────────────┘      with exact text    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Scrollable dialog trigger button exists
     */
    test("should have scrollable dialog trigger", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");

      await expect(ui.scrollableTrigger).toBeVisible();
      await expect(ui.scrollableTrigger).toHaveText("Open Scrollable Dialog");
    });

    /**
     * TEST: Scrollable Trigger Opens Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────┐       ┌────────────────┐  │
     *   │   │ Open Scrollable Dialog  │ CLICK │░░░░░░░░░░░░░░░░│  │
     *   │   └─────────────────────────┘ ────→ │░░ Scrollable ░░│  │
     *   │                                     │░░  Dialog    ░░│  │
     *   │                                     │░░░░░░░░░░░░░░░░│  │
     *   │                                     └────────────────┘  │
     *   │                                     data-state="open"   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger opens scrollable dialog
     */
    test("clicking scrollable trigger should open dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");

      await ui.scrollableTrigger.click();
      await expect(ui.scrollableContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Scrollable Dialog Title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h3>Terms of Service</h3>  <-- MUST MATCH      │   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  Long scrollable content...               │  │   │
     *   │   │  │  ...                                      │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Scrollable dialog has correct title text
     */
    test("scrollable dialog should have Terms of Service title", async ({
      page,
    }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");
      await ui.openScrollableDialog();

      const title = ui.scrollableContent.locator("h3");
      await expect(title).toHaveText("Terms of Service");
    });

    /**
     * TEST: Scrollable Dialog Action Buttons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Terms of Service                               │   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  Scrollable content...                    │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   │  ┌──────────┐              ┌──────────┐         │   │
     *   │   │  │ Decline  │              │  Accept  │         │   │
     *   │   │  └──────────┘              └──────────┘         │   │
     *   │   │       ↑                         ↑               │   │
     *   │   │   MUST EXIST               MUST EXIST           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Decline and Accept buttons are visible
     */
    test("scrollable dialog should have Decline and Accept buttons", async ({
      page,
    }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");
      await ui.openScrollableDialog();

      await expect(ui.declineButton).toBeVisible();
      await expect(ui.acceptButton).toBeVisible();
    });

    /**
     * TEST: Decline Button Closes Scrollable Dialog
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Terms of Service                               │   │
     *   │   │  ┌──────────┐              ┌──────────┐         │   │
     *   │   │  │ Decline  │              │  Accept  │         │   │
     *   │   │  └────┬─────┘              └──────────┘         │   │
     *   │   │       │                                         │   │
     *   │   └───────│─────────────────────────────────────────┘   │
     *   │           │ CLICK                                       │
     *   │           ▼                                             │
     *   │    data-state="closed"                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Decline button closes the scrollable dialog
     */
    test("Decline button should close scrollable dialog", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");
      await ui.openScrollableDialog();

      await ui.declineButton.click();
      await expect(ui.scrollableContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Scrollable Dialog Has ScrollArea
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Terms of Service                               │   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  ScrollArea [data-name="ScrollArea"]  ▲   │  │   │
     *   │   │  │  ┌─────────────────────────────────┐  █   │  │   │
     *   │   │  │  │  Long content here...           │  █   │  │   │
     *   │   │  │  │  ...                            │  ░   │  │   │
     *   │   │  │  └─────────────────────────────────┘  ▼   │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   │                    ↑                            │   │
     *   │   │              MUST BE VISIBLE                    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea component is present for scrollable content
     */
    test("scrollable dialog should have ScrollArea", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto("scrollable-dialog");
      await ui.openScrollableDialog();

      const scrollArea = ui.scrollableContent.locator('[data-name="ScrollArea"]');
      await expect(scrollArea).toBeVisible();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Close Button Aria-Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  DialogContent                              [×] │   │
     *   │   │                                              ↑   │   │
     *   │   │                          aria-label="Close dialog"  │
     *   │   │                          (screen reader support)    │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button has proper aria-label for accessibility
     */
    test("close button should have aria-label", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await expect(ui.closeButton).toHaveAttribute(
        "aria-label",
        "Close dialog"
      );
    });

    /**
     * TEST: Trigger Button Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Dialog   │  <-- Can receive keyboard focus   │
     *   │   └─────────────────┘      via .focus() method          │
     *   │          ↑                                              │
     *   │     MUST BE FOCUSED                                     │
     *   │     after focus() call                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can receive keyboard focus
     */
    test("trigger button should be focusable", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();

      await ui.triggerButton.focus();
      await expect(ui.triggerButton).toBeFocused();
    });

    /**
     * TEST: Dialog Inputs Tab Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Name:     [________________]  <-- 1. FOCUSED   │   │
     *   │   │                    │                            │   │
     *   │   │               [TAB] key                         │   │
     *   │   │                    ▼                            │   │
     *   │   │  Username: [________________]  <-- 2. FOCUSED   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key navigates between form inputs
     */
    test("dialog inputs should be tabbable when open", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      // Focus the name input and tab to username
      await ui.nameInput.focus();
      await expect(ui.nameInput).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.usernameInput).toBeFocused();
    });

    /**
     * TEST: Footer Buttons Tab Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌────────────┐    ┌────────────────┐           │   │
     *   │   │  │   Cancel   │    │  Save changes  │           │   │
     *   │   │  └─────┬──────┘    └────────┬───────┘           │   │
     *   │   │        │                    │                   │   │
     *   │   │   1. FOCUSED ───[TAB]─→ 2. FOCUSED              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key navigates between footer buttons
     */
    test("footer buttons should be tabbable", async ({ page }) => {
      const ui = new DialogPage(page);
      await ui.goto();
      await ui.openDialog();

      await ui.cancelButton.focus();
      await expect(ui.cancelButton).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.saveButton).toBeFocused();
    });
  });
});
