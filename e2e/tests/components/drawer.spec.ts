import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DRAWER COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY (Bottom position):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [Open Drawer]  ← Trigger button                                       │
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░  DrawerOverlay (bg-black/50)  ░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
 * │   ├─────────────────────────────────────────────────────────────────┤   │
 * │   │  DrawerContent (fixed, bottom-0)                                │   │
 * │   │  ┌──────────────────────────────────────────────────────────┐   │   │
 * │   │  │                    ══════════                            │   │   │
 * │   │  │                    DrawerHandle (data-vaul-handle)       │   │   │
 * │   │  └──────────────────────────────────────────────────────────┘   │   │
 * │   │  ┌──────────────────────────────────────────────────────────┐   │   │
 * │   │  │  <h2> Drawer Title                                       │   │   │
 * │   │  │  <p> Description text                                    │   │   │
 * │   │  │  [Close]                                                 │   │   │
 * │   │  └──────────────────────────────────────────────────────────┘   │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * POSITION VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   BOTTOM (default):              RIGHT (side drawer):                   │
 * │   ┌─────────────────────┐        ┌─────────────────────────────────┐    │
 * │   │░░░░░░░░░░░░░░░░░░░░░│        │░░░░░░░░░░░░░░░░░│              │    │
 * │   │░░░░░░░░░░░░░░░░░░░░░│        │░░░░░░░░░░░░░░░░░│   Content    │    │
 * │   ├─────────────────────┤        │░░░░░░░░░░░░░░░░░│              │    │
 * │   │      Content        │        │░░░░░░░░░░░░░░░░░│              │    │
 * │   └─────────────────────┘        └─────────────────────────────────┘    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VAUL INTEGRATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-vaul-drawer            ← Drawer marker                           │
 * │   data-vaul-overlay           ← Overlay marker                          │
 * │   data-vaul-handle            ← Drag handle                             │
 * │   data-vaul-dismissible       ← true/false for dismissability           │
 * │   data-vaul-drawer-position   ← Bottom/Right/Left/Top                   │
 * │   data-vaul-animate           ← Animation enabled                       │
 * │   data-state                  ← open/closed                             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DrawerPage extends BasePage {
  protected readonly componentName = "drawer";

  // Main drawer demo (first on page)
  readonly triggerButton: Locator;
  readonly drawerContent: Locator;
  readonly drawerOverlay: Locator;
  readonly drawerHandle: Locator;
  readonly drawerTitle: Locator;
  readonly drawerDescription: Locator;
  readonly closeButton: Locator;

  // All drawers and triggers on page
  readonly allTriggers: Locator;
  readonly allDrawerContents: Locator;

  constructor(page: Page) {
    super(page);

    // Main drawer - trigger scoped within preview, content in portal
    this.triggerButton = this.preview.getByRole("button", { name: "Open Drawer" }).first();
    this.drawerContent = page.locator('[data-name="DrawerContent"]').first();
    this.drawerOverlay = page.locator('[data-name="DrawerOverlay"]').first();
    this.drawerHandle = this.drawerContent.locator("[data-vaul-handle]");
    this.drawerTitle = this.drawerContent.locator("h2").first();
    this.drawerDescription = this.drawerContent.locator("p").first();
    this.closeButton = this.drawerContent.getByRole("button", { name: "Close" });

    // All instances - triggers in preview, content in portal
    this.allTriggers = this.preview.getByRole("button", { name: "Open Drawer" });
    this.allDrawerContents = page.locator('[data-name="DrawerContent"]');
  }

  async openDrawer() {
    await this.triggerButton.click();
    await this.waitForDataState(this.drawerContent, "open");
    await expect(this.drawerContent).toBeVisible();
    // Wait for drawer animation to complete
    await this.page.waitForTimeout(200);
  }

  async closeDrawer() {
    await this.closeButton.click();
    await this.waitForDataState(this.drawerContent, "closed");
  }

  getNthDrawerContent(n: number): Locator {
    return this.allDrawerContents.nth(n);
  }

  getNthTrigger(n: number): Locator {
    return this.allTriggers.nth(n);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Drawer Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │   Open Drawer   │  <-- MUST BE VISIBLE              │
     *   │   └─────────────────┘      with text "Open Drawer"      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button renders and has correct text
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.triggerButton).toBeVisible();
      await expect(ui.triggerButton).toHaveText("Open Drawer");
    });

    /**
     * TEST: Drawer Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent                                         │
     *   │   ┌──────────────────────────────────────────────────┐  │
     *   │   │ data-state="closed"  <-- MUST BE CLOSED          │  │
     *   │   └──────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer starts in closed state on page load
     */
    test("drawer should be closed by default", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: DrawerContent Styling Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent classes:                                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .bg-background   <-- background color             │ │
     *   │   │  .fixed           <-- fixed positioning            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: DrawerContent has proper Tailwind styling classes
     */
    test("should have DrawerContent with correct classes", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveClass(/bg-background/);
      await expect(ui.drawerContent).toHaveClass(/fixed/);
    });

    /**
     * TEST: Drawer Handle Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    ══════════                   │   │
     *   │   │                         ↑                       │   │
     *   │   │               DrawerHandle (drag bar)           │   │
     *   │   │               [data-vaul-handle]                │   │
     *   │   │               MUST BE VISIBLE                   │   │
     *   │   │                                                 │   │
     *   │   │   Drawer Content...                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer handle is visible when drawer is open
     */
    test("should have drawer handle when open", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await expect(ui.drawerHandle).toBeVisible();
    });

    /**
     * TEST: Drawer Title and Description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    ══════════                   │   │
     *   │   │  <h2> Drawer Title              <-- TITLE       │   │
     *   │   │  <p> Description text           <-- DESCRIPTION │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer has visible title and description
     */
    test("should have drawer title and description when open", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await expect(ui.drawerTitle).toBeVisible();
      await expect(ui.drawerTitle).toHaveText("Drawer Title");
      await expect(ui.drawerDescription).toBeVisible();
    });

    /**
     * TEST: Close Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Drawer Title                                   │   │
     *   │   │  Description text                               │   │
     *   │   │  ┌───────────┐                                  │   │
     *   │   │  │   Close   │  <-- MUST BE VISIBLE             │   │
     *   │   │  └───────────┘                                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button is visible when drawer is open
     */
    test("should have close button when open", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await expect(ui.closeButton).toBeVisible();
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Trigger Opens Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌──────────────┐           ┌──────────────────────┐   │
     *   │   │              │  CLICK    │░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │ Open Drawer  │  ────→    ├──────────────────────┤   │
     *   │   │              │           │      Drawer          │   │
     *   │   └──────────────┘           │      Content         │   │
     *   │   data-state=                └──────────────────────┘   │
     *   │     "closed"                 data-state="open"          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger transitions drawer to open state
     */
    test("clicking trigger should open drawer", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveAttribute("data-state", "closed");
      await ui.triggerButton.click();
      await expect(ui.drawerContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Close Button Closes Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Drawer Title                                   │   │
     *   │   │  ┌───────────┐                                  │   │
     *   │   │  │   Close   │  <-- CLICK                       │   │
     *   │   │  └─────┬─────┘                                  │   │
     *   │   │        │                                        │   │
     *   │   └────────│────────────────────────────────────────┘   │
     *   │            │                                            │
     *   │            ▼                                            │
     *   │     data-state="closed"                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button properly closes the drawer
     */
    test("clicking close button should close drawer", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await ui.closeButton.click();
      await expect(ui.drawerContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Overlay Click Closes Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░CLICK░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░↓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░  DrawerOverlay  ░░░░░░░░░░░░░░░░░│   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │      Drawer Content                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                            │                            │
     *   │                            ▼                            │
     *   │                     data-state="closed"                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking overlay (outside drawer) closes it
     */
    test("clicking overlay should close drawer", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      // Wait a moment for the drawer to fully open and stabilize
      await page.waitForTimeout(300);

      // Click the overlay (position at top-left to avoid hitting drawer)
      await ui.drawerOverlay.click({ force: true, position: { x: 10, y: 10 } });
      await expect(ui.drawerContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: ESC Key Closes Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │      Drawer Content                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                            │                            │
     *   │              ┌─────────────┴─────────────┐               │
     *   │              │     Press [ESC] key      │               │
     *   │              └─────────────┬─────────────┘               │
     *   │                            ▼                            │
     *   │                     data-state="closed"                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ESC keyboard shortcut closes the drawer
     */
    test("pressing ESC should close drawer", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await page.keyboard.press("Escape");
      await expect(ui.drawerContent).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Drawer Reopen After Close
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Open      Step 2: Close     Step 3: Reopen    │
     *   │   ┌──────────┐      ┌──────────┐      ┌──────────────┐  │
     *   │   │░░░░░░░░░░│      │          │      │░░░░░░░░░░░░░░│  │
     *   │   ├──────────┤ ───→ │ [Trigger]│ ───→ ├──────────────┤  │
     *   │   │  Drawer  │      │          │      │    Drawer    │  │
     *   │   └──────────┘      └──────────┘      └──────────────┘  │
     *   │   state="open"      state="closed"    state="open"      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer can be reopened after being closed
     */
    test("drawer can be reopened after closing", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      // Open and close
      await ui.openDrawer();
      await ui.closeDrawer();

      // Reopen
      await ui.triggerButton.click();
      await expect(ui.drawerContent).toHaveAttribute("data-state", "open");
    });
  });

  test.describe("Non-Dismissable Drawer", () => {
    /**
     * TEST: Non-Dismissable Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ data-vaul-dismissible="false"                     │ │
     *   │   │           ↑                                       │ │
     *   │   │   ATTRIBUTE MUST BE "false"                       │ │
     *   │   │   (prevents ESC/overlay close)                    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Non-dismissable drawer has correct vaul attribute
     */
    test("non-dismissable drawer should have correct attribute", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto("non-dismissable");

      // Find the non-dismissable drawer content (has dismissible="false")
      const nonDismissableContent = page.locator(
        '[data-vaul-dismissible="false"]'
      );
      await expect(nonDismissableContent).toHaveAttribute(
        "data-vaul-dismissible",
        "false"
      );
    });

    /**
     * TEST: Non-Dismissable Drawer Opens
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────┐       ┌────────────────────────┐  │
     *   │   │   Open Drawer   │ CLICK │░░░░░░░░░░░░░░░░░░░░░░░░│  │
     *   │   └─────────────────┘ ────→ ├────────────────────────┤  │
     *   │                             │  Non-dismissable       │  │
     *   │                             │  Drawer Content        │  │
     *   │                             │  dismissible="false"   │  │
     *   │                             └────────────────────────┘  │
     *   │                             data-state="open"           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Non-dismissable drawer can be opened
     */
    test("non-dismissable drawer should open", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto("non-dismissable");

      // Find the trigger that is followed by the non-dismissable drawer
      // The non-dismissable section has index 2 (0=main, 1=family, 2=focus, 3=non-dismissable)
      // Actually demos: 0=main, 1=family, 2=focus, 3=non-dismissable
      const nonDismissableContent = page.locator(
        '[data-vaul-dismissible="false"]'
      );

      // Find trigger button closest to the non-dismissable content
      // Use the nth trigger that corresponds to the non-dismissable demo
      const triggerInSection = page.getByRole("button", { name: "Open Drawer" }).nth(3);
      await triggerInSection.click();

      await expect(nonDismissableContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: ESC Does Not Close Non-Dismissable Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │  Non-dismissable Drawer                         │   │
     *   │   │  dismissible="false"                            │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                            │                            │
     *   │              ┌─────────────┴─────────────┐               │
     *   │              │     Press [ESC] key      │               │
     *   │              └─────────────┬─────────────┘               │
     *   │                            ▼                            │
     *   │              data-state STILL "open"  (NOT closed!)     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ESC key does NOT close non-dismissable drawer
     */
    test("ESC should not close non-dismissable drawer", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto("non-dismissable");

      const nonDismissableContent = page.locator(
        '[data-vaul-dismissible="false"]'
      );
      const triggerInSection = page.getByRole("button", { name: "Open Drawer" }).nth(3);
      await triggerInSection.click();
      await ui.waitForDataState(nonDismissableContent, "open");

      await page.keyboard.press("Escape");
      // Should still be open
      await expect(nonDismissableContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Confirm Button Closes Non-Dismissable Drawer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Non-dismissable Drawer                         │   │
     *   │   │  (ESC/overlay won't close it)                   │   │
     *   │   │                                                 │   │
     *   │   │  ┌───────────┐                                  │   │
     *   │   │  │  Confirm  │  <-- ONLY WAY TO CLOSE           │   │
     *   │   │  └─────┬─────┘                                  │   │
     *   │   │        │                                        │   │
     *   │   └────────│────────────────────────────────────────┘   │
     *   │            │ CLICK                                      │
     *   │            ▼                                            │
     *   │     data-state="closed"                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Confirm button is the only way to close
     */
    test("confirm button should close non-dismissable drawer", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto("non-dismissable");

      const nonDismissableContent = page.locator(
        '[data-vaul-dismissible="false"]'
      );
      const triggerInSection = page.getByRole("button", { name: "Open Drawer" }).nth(3);
      await triggerInSection.click();
      await ui.waitForDataState(nonDismissableContent, "open");

      // Click confirm button to close
      const confirmButton = nonDismissableContent.getByRole("button", {
        name: "Confirm",
      });
      await confirmButton.click();
      await expect(nonDismissableContent).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Side Drawer", () => {
    /**
     * TEST: Side Drawer Position Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬────────┐  │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │  │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│ Drawer │  │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│        │  │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   ↑    │  │
     *   │   └─────────────────────────────────────────┴────────┘  │
     *   │                                                         │
     *   │   data-vaul-drawer-position="Right"                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Side drawer has Right position attribute
     */
    test("side drawer should have right position attribute", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto("side");

      // Use first() since there are multiple Right position drawers (side + side-floating)
      const rightDrawer = page.locator(
        '[data-vaul-drawer-position="Right"]'
      ).first();
      await expect(rightDrawer).toHaveAttribute(
        "data-vaul-drawer-position",
        "Right"
      );
    });

    /**
     * TEST: Side Drawer Opens from Right
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                    AFTER:                     │
     *   │   ┌──────────────────────┐   ┌──────────────────┬────┐  │
     *   │   │                      │   │░░░░░░░░░░░░░░░░░░│    │  │
     *   │   │    [Open Drawer]     │──→│░░░░░░░░░░░░░░░░░░│ ██ │  │
     *   │   │                      │   │░░░░░░░░░░░░░░░░░░│ ██ │  │
     *   │   └──────────────────────┘   │░░░░░░░░░░░░░░░░░░│    │  │
     *   │                              └──────────────────┴────┘  │
     *   │                              position="Right"           │
     *   │                              variant="Inset"            │
     *   │                              state="open"               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Side drawer opens and slides from the right
     */
    test("side drawer should open from right", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto("side");

      // Find trigger in side section (demos: 0=main, 1=family, 2=focus, 3=non-dismissable, 4=scrollable, 5=side)
      const sideTrigger = page.getByRole("button", { name: "Open Drawer" }).nth(5);
      await sideTrigger.click();

      // Use first() for the Inset variant (not Floating)
      const rightDrawer = page.locator(
        '[data-vaul-drawer-position="Right"][data-vaul-variant="Inset"]'
      );
      await expect(rightDrawer).toHaveAttribute("data-state", "open");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Drawer Fixed Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  .fixed  <-- position: fixed                      │ │
     *   │   │          (stays in place regardless of scroll)    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer uses fixed positioning
     */
    test("drawer should have fixed positioning", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveClass(/fixed/);
    });

    /**
     * TEST: Drawer Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭─────────────────────────────────────────────────╮   │
     *   │   │              DrawerContent                      │   │
     *   │   │                                                 │   │
     *   │   │   .rounded-*  <-- border-radius applied         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer has rounded corners styling
     */
    test("drawer should have rounded corners", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveClass(/rounded/);
    });

    /**
     * TEST: Overlay Semi-Transparent Background
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
     *   Validates: Overlay has 50% black background for dimming
     */
    test("overlay should have semi-transparent background", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerOverlay).toHaveClass(/bg-black\/50/);
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
     *   │   │   Open Drawer   │  <-- Can receive keyboard focus   │
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
      const ui = new DrawerPage(page);
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
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Drawer Content                                 │   │
     *   │   │  ┌───────────┐                                  │   │
     *   │   │  │   Close   │  <-- Can receive keyboard focus  │   │
     *   │   │  └───────────┘      when drawer is open         │   │
     *   │   │        ↑                                        │   │
     *   │   │   MUST BE FOCUSED                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Close button can receive focus when drawer is open
     */
    test("close button should be focusable when drawer is open", async ({
      page,
    }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      await ui.closeButton.focus();
      await expect(ui.closeButton).toBeFocused();
    });

    /**
     * TEST: Drawer Title is h2 Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h2>Drawer Title</h2>  <-- MUST BE h2 TAG      │   │
     *   │   │       ↑                                         │   │
     *   │   │  Semantic heading element                       │   │
     *   │   │  for accessibility                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer title uses h2 semantic element
     */
    test("drawer title should be h2 element", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      const tagName = await ui.drawerTitle.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("h2");
    });
  });

  test.describe("Vaul Integration", () => {
    /**
     * TEST: Vaul Drawer Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ data-vaul-drawer  <-- MUST EXIST                  │ │
     *   │   │                   (vaul library marker)           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer has vaul library marker attribute
     */
    test("should have vaul drawer attribute", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      const hasVaulAttr = await ui.drawerContent.evaluate((el) =>
        el.hasAttribute("data-vaul-drawer")
      );
      expect(hasVaulAttr).toBe(true);
    });

    /**
     * TEST: Vaul Overlay Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerOverlay                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ data-vaul-overlay  <-- MUST EXIST                 │ │
     *   │   │                    (vaul overlay marker)          │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Overlay has vaul overlay marker attribute
     */
    test("overlay should have vaul overlay attribute", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      const hasVaulAttr = await ui.drawerOverlay.evaluate((el) =>
        el.hasAttribute("data-vaul-overlay")
      );
      expect(hasVaulAttr).toBe(true);
    });

    /**
     * TEST: Vaul Handle Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    ══════════                   │   │
     *   │   │                         ↑                       │   │
     *   │   │               data-vaul-handle                  │   │
     *   │   │               MUST EXIST                        │   │
     *   │   │               (drag handle marker)              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Handle has vaul handle marker attribute
     */
    test("handle should have vaul handle attribute", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();
      await ui.openDrawer();

      const hasVaulAttr = await ui.drawerHandle.evaluate((el) =>
        el.hasAttribute("data-vaul-handle")
      );
      expect(hasVaulAttr).toBe(true);
    });

    /**
     * TEST: Animate Attribute Enabled
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ data-vaul-animate="true"                          │ │
     *   │   │           ↑                                       │ │
     *   │   │   Animation enabled for smooth transitions        │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Drawer has animation enabled
     */
    test("should have animate attribute enabled", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveAttribute(
        "data-vaul-animate",
        "true"
      );
    });

    /**
     * TEST: Default Drawer Dismissible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Default DrawerContent                                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │ data-vaul-dismissible="true"                      │ │
     *   │   │           ↑                                       │ │
     *   │   │   Can be closed via ESC/overlay/drag              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default drawer can be dismissed
     */
    test("default drawer should be dismissible", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveAttribute(
        "data-vaul-dismissible",
        "true"
      );
    });

    /**
     * TEST: Default Drawer Bottom Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │  data-vaul-drawer-position="Bottom"             │   │
     *   │   │           ↑                                     │   │
     *   │   │   Default position is bottom of screen          │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default drawer position is Bottom
     */
    test("default drawer should have bottom position", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      await expect(ui.drawerContent).toHaveAttribute(
        "data-vaul-drawer-position",
        "Bottom"
      );
    });
  });

  test.describe("Multiple Demos", () => {
    /**
     * TEST: Multiple Drawer Triggers on Page
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Demo 1:  [Open Drawer]                                │
     *   │   Demo 2:  [Open Drawer]                                │
     *   │   Demo 3:  [Open Drawer]                                │
     *   │   ...                                                   │
     *   │                                                         │
     *   │   COUNT > 1  <-- MUST have multiple triggers            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page contains multiple drawer demo triggers
     */
    test("page should have multiple drawer demos", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      const triggerCount = await ui.allTriggers.count();
      expect(triggerCount).toBeGreaterThan(1);
    });

    /**
     * TEST: Multiple DrawerContent Elements on Page
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DrawerContent #1:  [data-name="DrawerContent"]        │
     *   │   DrawerContent #2:  [data-name="DrawerContent"]        │
     *   │   DrawerContent #3:  [data-name="DrawerContent"]        │
     *   │   ...                                                   │
     *   │                                                         │
     *   │   COUNT > 1  <-- MUST have multiple contents            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page contains multiple drawer content elements
     */
    test("page should have multiple drawer contents", async ({ page }) => {
      const ui = new DrawerPage(page);
      await ui.goto();

      const contentCount = await ui.allDrawerContents.count();
      expect(contentCount).toBeGreaterThan(1);
    });
  });
});
