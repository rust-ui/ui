import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * CONTEXT-MENU COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────┐                                   │
 * │   │     Right click here            │  ← ContextMenuTrigger             │
 * │   │   (border-dashed, rounded-md)   │                                   │
 * │   └─────────────────────────────────┘                                   │
 * │                  │                                                      │
 * │                  │ right-click                                          │
 * │                  ▼                                                      │
 * │   ┌─────────────────────────────────┐                                   │
 * │   │  Actions                        │  ← ContextMenuLabel               │
 * │   │  ─────────────────────────────  │                                   │
 * │   │  Back                           │  ← ContextMenuItem                │
 * │   │  Forward                        │                                   │
 * │   │  Reload                         │                                   │
 * │   │  ─────────────────────────────  │  ← ContextMenuSeparator           │
 * │   │  More Tools              ▶      │  ← ContextMenuSubTrigger          │
 * │   └─────────────────────────────────┘     │                             │
 * │                                           │ hover                       │
 * │                                           ▼                             │
 * │                           ┌─────────────────────────────────┐           │
 * │                           │  Save Page As...                │           │
 * │                           │  Developer Tools                │           │
 * │                           └─────────────────────────────────┘           │
 * │                              ↑ ContextMenuSubContent                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-state="closed"                data-state="open"                  │
 * │   ┌───────────────────┐              ┌───────────────────┐              │
 * │   │                   │  right-click │                   │              │
 * │   │   [Right click]   │  ─────────>  │   [Menu Items]    │              │
 * │   │                   │              │                   │              │
 * │   └───────────────────┘   <────────  └───────────────────┘              │
 * │                           ESC / click outside                           │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ContextMenuPage extends BasePage {
  protected readonly componentName = "context-menu";

  // Context menu elements - scoped within preview where applicable
  readonly trigger: Locator;
  readonly content: Locator;
  readonly menuLabel: Locator;
  readonly menuItems: Locator;
  readonly subTrigger: Locator;
  readonly subContent: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger - scoped within preview
    this.trigger = this.preview.locator('[data-name="ContextMenuTrigger"]').first();

    // Content - rendered in portal, use page-level locator
    this.content = page.locator('[data-name="ContextMenuContent"]').first();
    this.menuLabel = this.content.locator('[data-name="ContextMenuLabel"]');
    this.menuItems = this.content.locator('[data-name="ContextMenuItem"]');

    // Submenu - subContent is also in a portal, use page-level locator
    this.subTrigger = this.content.locator('[data-name="ContextMenuSubTrigger"]');
    this.subContent = page.locator('[data-name="ContextMenuSubContent"]').first();
  }

  async openContextMenu() {
    await this.trigger.click({ button: "right" });
    await this.waitForDataState(this.content, "open");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("ContextMenu Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │     Right click here          │  ← Visible?         │
     *   │  │   (ContextMenuTrigger)        │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The trigger area is visible and can be interacted with
     */
    test("should have trigger element", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: Trigger Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │     Right click here          │  ← Text matches?    │
     *   │  │     ^^^^^^^^^^^^^^^^                                │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger displays instructional text to users
     */
    test("trigger should display 'Right click here'", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toContainText("Right click here");
    });

    /**
     * TEST: Default Closed State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │     Right click here          │                     │
     *   │  └───────────────────────────────┘                     │
     *   │                                                        │
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                     │
     *   │  │ Menu Content                  │  ← data-state?      │
     *   │  │ (data-state="closed")         │    Should be        │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘    "closed"         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content is hidden (closed) by default
     */
    test("content should be closed by default", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Right-Click Opens Menu
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │     Right click here          │  [Right-Click]      │
     *   │  └───────────────────────────────┘       │              │
     *   │                    │                     │              │
     *   │                    ▼                     │              │
     *   │  ┌───────────────────────────────┐      │              │
     *   │  │ Actions                       │ ◄────┘              │
     *   │  │ ─────────────────────────────│   data-state="open"  │
     *   │  │ Back                          │                     │
     *   │  │ Forward                       │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Right-clicking trigger opens the context menu
     */
    test("right-clicking trigger should open menu", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.trigger.click({ button: "right" });
      await expect(ui.content).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: ESC Key Closes Menu
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  BEFORE (menu open):            AFTER (ESC pressed):   │
     *   │  ┌───────────────────────┐      ┌───────────────────┐  │
     *   │  │ Actions               │  =>  │     [hidden]      │  │
     *   │  │ Back                  │ [ESC]│                   │  │
     *   │  │ Forward               │      │ data-state=       │  │
     *   │  │ (data-state="open")   │      │   "closed"        │  │
     *   │  └───────────────────────┘      └───────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressing Escape key closes the open context menu
     */
    test("pressing ESC should close menu", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      await page.keyboard.press("Escape");
      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Menu Items", () => {
    /**
     * TEST: Actions Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │ Actions                       │  ← Text = "Actions"?│
     *   │  │ ^^^^^^^                       │    (ContextMenuLabel│
     *   │  │ ─────────────────────────────│                      │
     *   │  │ Back                          │                     │
     *   │  │ Forward                       │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu has "Actions" section label at the top
     */
    test("should have Actions label", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      await expect(ui.menuLabel).toHaveText("Actions");
    });

    /**
     * TEST: Back Action Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ContextMenuContent (portal)                           │
     *   │  ┌───────────────────────────────┐                     │
     *   │  │ Actions                       │                     │
     *   │  │ ─────────────────────────────│                      │
     *   │  │ Back                          │  ← Visible?         │
     *   │  │ ^^^^                          │                     │
     *   │  │ Forward                       │                     │
     *   │  │ Reload                        │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Back" navigation action is present in menu
     */
    test("should have Back action", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      // Scope within content to avoid matching nav elements
      await expect(ui.content.getByText("Back")).toBeVisible();
    });

    /**
     * TEST: Forward Action Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ContextMenuContent (portal)                           │
     *   │  ┌───────────────────────────────┐                     │
     *   │  │ Actions                       │                     │
     *   │  │ ─────────────────────────────│                      │
     *   │  │ Back                          │                     │
     *   │  │ Forward                       │  ← Visible?         │
     *   │  │ ^^^^^^^                       │                     │
     *   │  │ Reload                        │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Forward" navigation action is present in menu
     */
    test("should have Forward action", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      // Scope within content to avoid matching nav elements
      await expect(ui.content.getByText("Forward")).toBeVisible();
    });

    /**
     * TEST: Reload Action Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ContextMenuContent (portal)                           │
     *   │  ┌───────────────────────────────┐                     │
     *   │  │ Actions                       │                     │
     *   │  │ ─────────────────────────────│                      │
     *   │  │ Back                          │                     │
     *   │  │ Forward                       │                     │
     *   │  │ Reload                        │  ← Visible?         │
     *   │  │ ^^^^^^                        │                     │
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Reload" action is present in menu
     */
    test("should have Reload action", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      // Scope within content to avoid matching other elements
      await expect(ui.content.getByText("Reload")).toBeVisible();
    });
  });

  test.describe("Submenu", () => {
    /**
     * TEST: More Tools Submenu Trigger
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────┐                     │
     *   │  │ Actions                       │                     │
     *   │  │ ─────────────────────────────│                      │
     *   │  │ Back                          │                     │
     *   │  │ Forward                       │                     │
     *   │  │ Reload                        │                     │
     *   │  │ ─────────────────────────────│                      │
     *   │  │ More Tools              ▶    │  ← Visible?          │
     *   │  │ ^^^^^^^^^^                   │    Text = "More Tools"?
     *   │  └───────────────────────────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Submenu trigger is visible with "More Tools" text
     */
    test("should have More Tools submenu trigger", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      await expect(ui.subTrigger).toBeVisible();
      await expect(ui.subTrigger).toContainText("More Tools");
    });

    /**
     * TEST: Hover Opens Submenu
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────┐                             │
     *   │  │ Actions               │                             │
     *   │  │ Back                  │                             │
     *   │  │ Forward               │                             │
     *   │  │ Reload                │                             │
     *   │  │ ─────────────────────│                              │
     *   │  │ More Tools       ▶   │  [Hover]                     │
     *   │  └───────────────────────┘    │                        │
     *   │                               ▼                        │
     *   │               ┌───────────────────────┐                │
     *   │               │ Save Page As...       │  ← Opens?      │
     *   │               │ Developer Tools       │ data-state=    │
     *   │               └───────────────────────┘   "open"       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering submenu trigger opens nested submenu
     */
    test("hovering submenu trigger should open submenu", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();

      await ui.subTrigger.hover();
      await page.waitForTimeout(300);

      await expect(ui.subContent).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: Save Page As Submenu Option
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Main Menu              Submenu (opened)               │
     *   │  ┌─────────────────┐    ┌───────────────────────┐      │
     *   │  │ More Tools    ▶ │───>│ Save Page As...       │ ←    │
     *   │  └─────────────────┘    │ ^^^^^^^^^^^^^^^       │ Visible?
     *   │                         │ Developer Tools       │      │
     *   │                         └───────────────────────┘      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Save Page As..." option exists in submenu
     */
    test("submenu should have Save Page As option", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();
      await ui.subTrigger.hover();
      await page.waitForTimeout(300);

      // Scope within subContent portal
      await expect(ui.subContent.getByText("Save Page As...")).toBeVisible();
    });

    /**
     * TEST: Developer Tools Submenu Option
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Main Menu              Submenu (opened)               │
     *   │  ┌─────────────────┐    ┌───────────────────────┐      │
     *   │  │ More Tools    ▶ │───>│ Save Page As...       │      │
     *   │  └─────────────────┘    │ Developer Tools       │ ←    │
     *   │                         │ ^^^^^^^^^^^^^^^       │ Visible?
     *   │                         └───────────────────────┘      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Developer Tools" option exists in submenu
     */
    test("submenu should have Developer Tools option", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);
      await ui.openContextMenu();
      await ui.subTrigger.hover();
      await page.waitForTimeout(300);

      // Scope within subContent portal
      await expect(ui.subContent.getByText("Developer Tools")).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Trigger Dashed Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                     │
     *   │  │     Right click here          │  ← border-dashed?   │
     *   │  │  (dashed border indicates     │                     │
     *   │  │   interactive zone)           │                     │
     *   │  └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has dashed border styling to indicate interactivity
     */
    test("trigger should have dashed border", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/border-dashed/);
    });

    /**
     * TEST: Trigger Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ╭───────────────────────────────╮                     │
     *   │  │     Right click here          │  ← rounded-md?      │
     *   │  │                               │                     │
     *   │  ╰───────────────────────────────╯                     │
     *   │   ^                             ^                      │
     *   │   rounded corners (border-radius)                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has medium rounded corners (rounded-md)
     */
    test("trigger should have rounded corners", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Content Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  z-index stacking:                                     │
     *   │                                                        │
     *   │  z-50 ───┐  ┌───────────────────────┐                  │
     *   │          │  │ Context Menu          │ ← Top layer      │
     *   │          │  │ (class="z-50 ...")    │                  │
     *   │          │  └───────────────────────┘                  │
     *   │          │                                             │
     *   │  z-0 ────┘  [Page Content Below]                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has z-50 to appear above page content
     */
    test("content should have z-50", async ({ page }) => {
      const ui = new ContextMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/z-50/);
    });
  });
});
