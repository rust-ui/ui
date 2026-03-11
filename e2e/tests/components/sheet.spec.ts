import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SHEET COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [Open Sheet]  ← Trigger button                                        │
 * │         │                                                               │
 * │         ▼                                                               │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  SheetBackdrop (bg-black/50)                                    │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │                                                           │  │   │
 * │   │  │                   SheetContent (right side)               │  │   │
 * │   │  │                   ┌─────────────────────────────┐         │  │   │
 * │   │  │                   │  SheetTitle                 │         │  │   │
 * │   │  │                   │  SheetDescription           │         │  │   │
 * │   │  │                   │                             │         │  │   │
 * │   │  │                   │  [Close]                    │         │  │   │
 * │   │  │                   └─────────────────────────────┘         │  │   │
 * │   │  │                                                           │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-state="closed"                data-state="open"                  │
 * │   ┌───────────────────┐              ┌───────────────────┐              │
 * │   │                   │   trigger    │░░░░░░░░░░░░░░░░░░░│              │
 * │   │                   │  ────────>   │░░░┌────────────┐░░│              │
 * │   │   [Open Sheet]    │              │░░░│   Sheet    │░░│              │
 * │   │                   │   <────────  │░░░│   Content  │░░│              │
 * │   │                   │  ESC/close/  │░░░│            │░░│              │
 * │   │                   │   backdrop   │░░░└────────────┘░░│              │
 * │   └───────────────────┘              └───────────────────┘              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * POSITIONING (slides from edge):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Right (default):       Left:           Top:             Bottom:       │
 * │   ┌────────────┐         ┌────────────┐  ┌────────────┐   ┌────────────┐│
 * │   │         ▓▓▓│         │▓▓▓         │  │▓▓▓▓▓▓▓▓▓▓▓▓│   │            ││
 * │   │         ▓▓▓│         │▓▓▓         │  │            │   │            ││
 * │   │         ▓▓▓│         │▓▓▓         │  │            │   │▓▓▓▓▓▓▓▓▓▓▓▓││
 * │   └────────────┘         └────────────┘  └────────────┘   └────────────┘│
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SheetPage extends BasePage {
  protected readonly componentName = "sheet";

  // Sheet elements
  readonly triggerButton: Locator;
  readonly sheetContent: Locator;
  readonly sheetBackdrop: Locator;
  readonly sheetTitle: Locator;
  readonly sheetDescription: Locator;
  readonly closeButton: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger - scoped within preview
    this.triggerButton = this.preview.getByRole("button", { name: "Open Sheet" });

    // Content - scoped within preview
    this.sheetContent = this.preview.locator('[data-name="SheetContent"]').first();
    this.sheetBackdrop = this.preview.locator('[data-name="SheetBackdrop"]').first();
    this.sheetTitle = this.preview.locator('[data-name="SheetTitle"]').first();
    this.sheetDescription = this.preview.locator('[data-name="SheetDescription"]').first();
    this.closeButton = this.preview.getByRole("button", { name: "Close" });
  }

  async openSheet() {
    await this.waitForInitialized(this.sheetContent);
    await this.triggerButton.click();
    await this.waitForDataState(this.sheetContent, "open");
  }

  async closeSheet() {
    await this.closeButton.click();
    await this.waitForDataState(this.sheetContent, "closed");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Sheet Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Sheet    │  <-- MUST BE VISIBLE              │
     *   │   └─────────────────┘      with text "Open Sheet"       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button renders and has correct text
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toBeVisible();
      await expect(ui.triggerButton).toHaveText("Open Sheet");
    });

    /**
     * TEST: Sheet Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SheetContent                                          │
     *   │   ┌──────────────────────────────────────────────────┐  │
     *   │   │ data-state="closed"  <-- MUST BE CLOSED          │  │
     *   │   └──────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet starts in closed state on page load
     */
    test("sheet should be closed by default", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: SheetContent Exists in DOM
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM:                                                  │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ <div data-name="SheetContent">                    │ │
     *   │   │   ...                                             │ │
     *   │   │ </div>                                            │ │
     *   │   │         ↑                                         │ │
     *   │   │   MUST BE ATTACHED to DOM                         │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: SheetContent element is attached to the DOM
     */
    test("should have SheetContent", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetContent).toBeAttached();
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Trigger Opens Sheet
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌──────────────┐           ┌──────────────────┬────┐  │
     *   │   │              │  CLICK    │░░░░░░░░░░░░░░░░░░│    │  │
     *   │   │ Open Sheet   │  ────→    │░░░░░░░░░░░░░░░░░░│████│  │
     *   │   │              │           │░░░░░░░░░░░░░░░░░░│████│  │
     *   │   └──────────────┘           │░░░░░░░░░░░░░░░░░░│    │  │
     *   │   data-state=                └──────────────────┴────┘  │
     *   │     "closed"                 data-state="open"          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger transitions sheet to open state
     */
    test("clicking trigger should open sheet", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.sheetContent);

      await ui.triggerButton.click();
      await expect(ui.sheetContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Close Button Closes Sheet
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Sheet  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│[Close] │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                                                  │      │
     *   │                                              CLICK      │
     *   │                                                  ▼      │
     *   │                                       data-state="closed"│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button properly closes the sheet
     */
    test("clicking close button should close sheet", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await ui.closeButton.click();
      await expect(ui.sheetContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: ESC Key Closes Sheet
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Sheet  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                            │                            │
     *   │              ┌─────────────┴─────────────┐               │
     *   │              │     Press [ESC] key      │               │
     *   │              └─────────────┬─────────────┘               │
     *   │                            ▼                            │
     *   │                     data-state="closed"                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ESC keyboard shortcut closes the sheet
     */
    test("pressing ESC should close sheet", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await page.keyboard.press("Escape");
      await expect(ui.sheetContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Backdrop Click Closes Sheet
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░CLICK░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Sheet  │ │
     *   │   │░░░↓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░ SheetBackdrop ░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                            │                            │
     *   │                            ▼                            │
     *   │                     data-state="closed"                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking backdrop (outside sheet) closes it
     */
    test("clicking backdrop should close sheet", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await ui.sheetBackdrop.click({ force: true });
      await expect(ui.sheetContent).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Sheet Title Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Sheet  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Title  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                                              MUST EXIST │
     *   │                                         text="Sheet Title"│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet title is visible with correct text
     */
    test("should have sheet title", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await expect(ui.sheetTitle).toBeVisible();
      await expect(ui.sheetTitle).toHaveText("Sheet Title");
    });

    /**
     * TEST: Sheet Description Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Title  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Desc.  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                                              MUST EXIST │
     *   │                                 "This is the content..."│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet description is visible with correct text
     */
    test("should have sheet description", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await expect(ui.sheetDescription).toBeVisible();
      await expect(ui.sheetDescription).toContainText(
        "This is the content inside the sheet"
      );
    });

    /**
     * TEST: Close Button Inside Sheet
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Title  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Desc.  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│[Close] │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                                              MUST EXIST │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button is visible inside the sheet
     */
    test("should have close button inside sheet", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await expect(ui.closeButton).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Sheet Fixed Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SheetContent                                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .fixed  <-- position: fixed                      │ │
     *   │   │          (stays in place regardless of scroll)    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet uses fixed positioning
     */
    test("sheet content should have fixed positioning", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetContent).toHaveClass(/fixed/);
    });

    /**
     * TEST: Sheet Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   z-index layers:                                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  z-50  <-- SheetContent (MUST BE ON TOP)          │ │
     *   │   │  z-40  <-- backdrop                               │ │
     *   │   │  z-10  <-- other content                          │ │
     *   │   │  z-0   <-- base page                              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet has z-50 for proper stacking order
     */
    test("sheet content should have z-50", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetContent).toHaveClass(/z-50/);
    });

    /**
     * TEST: Sheet Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SheetContent                                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .bg-background  <-- theme background color       │ │
     *   │   │                  (solid, not transparent)         │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sheet has theme background color
     */
    test("sheet content should have bg-background", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetContent).toHaveClass(/bg-background/);
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
      const ui = new SheetPage(page);
      await ui.goto();

      await expect(ui.sheetBackdrop).toHaveClass(/bg-black\/50/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Trigger Button Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Sheet    │  <-- Can receive keyboard focus   │
     *   │   └─────────────────┘      via .focus() method          │
     *   │          ↑                                              │
     *   │     MUST BE FOCUSED                                     │
     *   │     after focus() call                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can receive keyboard focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new SheetPage(page);
      await ui.goto();

      await ui.triggerButton.focus();
      await expect(ui.triggerButton).toBeFocused();
    });

    /**
     * TEST: Close Button Focusable When Open
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────┬────────┐ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Sheet  │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│[Close] │ │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │ │
     *   │   └──────────────────────────────────────────┴────────┘ │
     *   │                                          MUST BE FOCUSED│
     *   │                                          when sheet open│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button can receive focus when sheet is open
     */
    test("close button should be focusable when sheet is open", async ({
      page,
    }) => {
      const ui = new SheetPage(page);
      await ui.goto();
      await ui.openSheet();

      await ui.closeButton.focus();
      await expect(ui.closeButton).toBeFocused();
    });
  });
});
