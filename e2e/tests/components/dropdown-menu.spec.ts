import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DROPDOWN-MENU COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌────────────────┐                                                    │
 * │   │  Open Menu     │  ← trigger button                                  │
 * │   └────────────────┘                                                    │
 * │            │                                                            │
 * │            │ click                                                      │
 * │            ▼                                                            │
 * │   ┌─────────────────────────────────┐                                   │
 * │   │  Main Menu                      │  ← DropdownMenuLabel              │
 * │   │  ─────────────────────────────  │  ← DropdownMenuSeparator          │
 * │   │  Simple Item                    │  ← DropdownMenuAction             │
 * │   │  Home                           │  ← DropdownMenuItem               │
 * │   │  ─────────────────────────────  │                                   │
 * │   │  Settings                   ▶   │  ← DropdownMenuSubTrigger         │
 * │   │  Tools                      ▶   │                  │                │
 * │   │  ─────────────────────────────  │                  │ hover          │
 * │   │  Sign Out                       │                  ▼                │
 * │   └─────────────────────────────────┘   ┌─────────────────────────────┐ │
 * │                                         │  Account Settings           │ │
 * │                                         │  Privacy Settings           │ │
 * │                                         └─────────────────────────────┘ │
 * │                                            ↑ DropdownMenuSubContent     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   data-state="closed"                data-state="open"                  │
 * │   ┌───────────────────┐              ┌───────────────────┐              │
 * │   │                   │   click      │  ┌─────────────┐  │              │
 * │   │   [Open Menu]     │  ────────>   │  │ Main Menu   │  │              │
 * │   │                   │              │  │ Simple Item │  │              │
 * │   └───────────────────┘   <────────  │  │ Home        │  │              │
 * │                           ESC        │  └─────────────┘  │              │
 * │                                      └───────────────────┘              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SUBMENU BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Hover on "Settings":                                                  │
 * │   ┌────────────────┐    ┌──────────────────────┐                        │
 * │   │ Settings    ▶  │────│  Account Settings    │                        │
 * │   └────────────────┘    │  Privacy Settings    │                        │
 * │                         └──────────────────────┘                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DropdownMenuPage extends BasePage {
  protected readonly componentName = "dropdown-menu";

  // Dropdown menu elements
  readonly trigger: Locator;
  readonly content: Locator;
  readonly menuLabel: Locator;
  readonly menuItems: Locator;
  readonly menuActions: Locator;
  readonly subTrigger: Locator;
  readonly subContent: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger - scoped within preview
    this.trigger = this.preview.getByRole("button", { name: "Open Menu" });

    // Content - rendered in portal, use page-level
    this.content = page.locator('[data-name="DropdownMenuContent"]').first();
    this.menuLabel = this.content.locator('[data-name="DropdownMenuLabel"]');
    this.menuItems = this.content.locator('[data-name="DropdownMenuItem"]');
    this.menuActions = this.content.locator('[data-name="DropdownMenuAction"]');

    // Submenu - subContent is also in portal
    this.subTrigger = this.content.locator('[data-name="DropdownMenuSubTrigger"]');
    this.subContent = page.locator('[data-name="DropdownMenuSubContent"]').first();
  }

  async openMenu() {
    await this.waitForInitialized(this.content);
    await this.trigger.click();
    await this.waitForDataState(this.content, "open");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("DropdownMenu Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────┐                                │
     *   │   │     Open Menu      │  ← MUST BE VISIBLE             │
     *   │   └────────────────────┘                                │
     *   │                                                         │
     *   │   role="button" name="Open Menu"                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown menu trigger button is visible
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: Trigger Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────┐                                │
     *   │   │   "Open Menu"      │  ← Text must match exactly     │
     *   │   └────────────────────┘                                │
     *   │                                                         │
     *   │   Confirms button label for user interaction            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button displays "Open Menu" text
     */
    test("trigger should display 'Open Menu'", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveText("Open Menu");
    });

    /**
     * TEST: Content Default Closed State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Initial page load:                                    │
     *   │                                                         │
     *   │   ┌────────────────────┐                                │
     *   │   │     Open Menu      │                                │
     *   │   └────────────────────┘                                │
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                    │
     *   │       Main Menu                                         │
     *   │       Simple Item                    ← data-state=      │
     *   │       Home                             "closed"         │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has data-state="closed" initially
     */
    test("content should be closed by default", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Clicking Trigger Opens Menu
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before click:               After click:              │
     *   │   ┌────────────────┐          ┌────────────────┐        │
     *   │   │   Open Menu    │  click   │   Open Menu    │        │
     *   │   └────────────────┘  ─────►  └────────────────┘        │
     *   │                                       │                 │
     *   │   data-state="closed"         ┌───────▼────────┐        │
     *   │                               │  Main Menu     │        │
     *   │                               │  Simple Item   │        │
     *   │                               │  Home          │        │
     *   │                               │  Settings    ▶ │        │
     *   │                               └────────────────┘        │
     *   │                               data-state="open"         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Click on trigger opens the dropdown menu
     */
    test("clicking trigger should open menu", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await ui.openMenu();

      await expect(ui.content).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: ESC Key Closes Menu
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open state:                 After ESC:                │
     *   │   ┌────────────────┐          ┌────────────────┐        │
     *   │   │   Open Menu    │          │   Open Menu    │        │
     *   │   └────────────────┘          └────────────────┘        │
     *   │   ┌────────────────┐                                    │
     *   │   │  Main Menu     │  [ESC]   data-state="closed"       │
     *   │   │  Simple Item   │  ─────►                            │
     *   │   │  Home          │                                    │
     *   │   └────────────────┘                                    │
     *   │                                                         │
     *   │   Keyboard accessibility: ESC dismisses the menu        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressing Escape key closes the dropdown menu
     */
    test("pressing ESC should close menu", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await page.keyboard.press("Escape");

      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Menu Content", () => {
    /**
     * TEST: Main Menu Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  "Main Menu"  ← DropdownMenuLabel   │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  Simple Item                        │               │
     *   │   │  Home                               │               │
     *   │   │  ...                                │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   [data-name="DropdownMenuLabel"]                       │
     *   │   Text must be exactly "Main Menu"                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu has "Main Menu" label at the top
     */
    test("should have Main Menu label", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(ui.menuLabel).toHaveText("Main Menu");
    });

    /**
     * TEST: Simple Item Action
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  Main Menu                          │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  "Simple Item"  ← Must be visible   │               │
     *   │   │  Home                               │               │
     *   │   │  Settings                       ▶   │               │
     *   │   │  ...                                │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   DropdownMenuAction item                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Simple Item" action is visible in menu
     */
    test("should have Simple Item action", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(page.getByText("Simple Item")).toBeVisible();
    });

    /**
     * TEST: Home Link Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  Main Menu                          │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  Simple Item                        │               │
     *   │   │  "Home"  ← Must be visible (link)   │               │
     *   │   │  Settings                       ▶   │               │
     *   │   │  ...                                │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   DropdownMenuItem with href                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Home" link item is visible in menu
     */
    test("should have Home link", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(page.getByText("Home")).toBeVisible();
    });

    /**
     * TEST: Sign Out Action
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  Main Menu                          │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  Simple Item                        │               │
     *   │   │  Home                               │               │
     *   │   │  Settings                       ▶   │               │
     *   │   │  Tools                          ▶   │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  "Sign Out"  ← Must be visible      │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   Destructive action at bottom of menu                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Sign Out" action is visible in menu
     */
    test("should have Sign Out action", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(page.getByText("Sign Out")).toBeVisible();
    });
  });

  test.describe("Submenu", () => {
    /**
     * TEST: Settings Submenu Trigger Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  Main Menu                          │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  Simple Item                        │               │
     *   │   │  Home                               │               │
     *   │   │  "Settings"                     ▶   │ ← Must exist  │
     *   │   │  Tools                          ▶   │               │
     *   │   │  ...                                │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   DropdownMenuSubTrigger for Settings                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Settings" submenu trigger is visible
     */
    test("should have Settings submenu trigger", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(page.getByText("Settings")).toBeVisible();
    });

    /**
     * TEST: Tools Submenu Trigger Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │  Main Menu                          │               │
     *   │   │  ─────────────────────────────────  │               │
     *   │   │  Simple Item                        │               │
     *   │   │  Home                               │               │
     *   │   │  Settings                       ▶   │               │
     *   │   │  "Tools"                        ▶   │ ← Must exist  │
     *   │   │  ...                                │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   │   DropdownMenuSubTrigger for Tools                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Tools" submenu trigger is visible
     */
    test("should have Tools submenu trigger", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      await expect(page.getByText("Tools")).toBeVisible();
    });

    /**
     * TEST: Settings Submenu Opens on Hover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Hover on "Settings":                                  │
     *   │                                                         │
     *   │   ┌────────────────────┐    ┌──────────────────────┐    │
     *   │   │  Main Menu         │    │                      │    │
     *   │   │  ────────────────  │    │  Account Settings    │    │
     *   │   │  Settings      ▶───│────│  Privacy Settings    │    │
     *   │   │  Tools          ▶  │    │                      │    │
     *   │   └────────────────────┘    └──────────────────────┘    │
     *   │        ↑                           ↑                    │
     *   │     HOVER                  DropdownMenuSubContent       │
     *   │                            becomes visible              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering Settings shows Account Settings submenu
     */
    test("hovering Settings should open submenu", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      const settingsTrigger = page.getByText("Settings");
      await settingsTrigger.hover();
      await page.waitForTimeout(300);

      await expect(page.getByText("Account Settings")).toBeVisible();
    });

    /**
     * TEST: Settings Submenu Privacy Settings Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Settings submenu content:                             │
     *   │                                                         │
     *   │   ┌────────────────────┐    ┌──────────────────────┐    │
     *   │   │  Settings      ▶───│────│  Account Settings    │    │
     *   │   └────────────────────┘    │  "Privacy Settings"  │    │
     *   │                             │          ↑           │    │
     *   │                             │    Must be visible   │    │
     *   │                             └──────────────────────┘    │
     *   │                                                         │
     *   │   Verifies submenu has multiple items                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Settings submenu includes Privacy Settings item
     */
    test("Settings submenu should have Privacy Settings", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      const settingsTrigger = page.getByText("Settings");
      await settingsTrigger.hover();
      await page.waitForTimeout(300);

      await expect(page.getByText("Privacy Settings")).toBeVisible();
    });

    /**
     * TEST: Tools Submenu Opens on Hover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Hover on "Tools":                                     │
     *   │                                                         │
     *   │   ┌────────────────────┐    ┌──────────────────────┐    │
     *   │   │  Main Menu         │    │                      │    │
     *   │   │  ────────────────  │    │  "Export Data"       │    │
     *   │   │  Settings       ▶  │    │  Import Data         │    │
     *   │   │  Tools         ▶───│────│  ...                 │    │
     *   │   └────────────────────┘    └──────────────────────┘    │
     *   │        ↑                           ↑                    │
     *   │     HOVER                  Tools submenu visible        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering Tools shows Export Data submenu
     */
    test("hovering Tools should open submenu", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();
      await ui.openMenu();

      const toolsTrigger = page.getByText("Tools");
      await toolsTrigger.hover();
      await page.waitForTimeout(300);

      await expect(page.getByText("Export Data")).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Content Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   z-index stacking:                                     │
     *   │                                                         │
     *   │   z-50    ┌─────────────────────────┐ ← Menu on top     │
     *   │           │  Main Menu              │                   │
     *   │           │  Simple Item            │                   │
     *   │           │  Home                   │                   │
     *   │           └─────────────────────────┘                   │
     *   │   z-auto  ┌─────────────────────────┐                   │
     *   │           │    Page content         │                   │
     *   │           └─────────────────────────┘                   │
     *   │                                                         │
     *   │   class="z-50 ..."                                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has z-50 class
     */
    test("content should have z-50", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/z-50/);
    });

    /**
     * TEST: Content Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Square corners:            Rounded corners:           │
     *   │   ┌─────────────────┐        ╭─────────────────╮        │
     *   │   │  Main Menu      │        │  Main Menu      │        │
     *   │   │  Simple Item    │        │  Simple Item    │        │
     *   │   │  Home           │        │  Home           │        │
     *   │   └─────────────────┘        ╰─────────────────╯        │
     *   │                                      ↑                  │
     *   │                              rounded-lg ← EXPECTED      │
     *   │                                                         │
     *   │   class="rounded-lg ..."                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has rounded-lg class
     */
    test("content should have rounded corners", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/rounded-lg/);
    });

    /**
     * TEST: Content Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────┐                   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│                   │
     *   │   │░  Main Menu                    ░│                   │
     *   │   │░  ───────────────────────────  ░│                   │
     *   │   │░  Simple Item                  ░│                   │
     *   │   │░  Home                         ░│                   │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│                   │
     *   │   └─────────────────────────────────┘                   │
     *   │    ↑ border class (visible outline)                     │
     *   │                                                         │
     *   │   class="border ..."                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has border class
     */
    test("content should have border", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/border/);
    });

    /**
     * TEST: Content Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────┐                   │
     *   │   │  Main Menu                      │                   │
     *   │   │  ───────────────────────────    │                   │
     *   │   │  Simple Item                    │                   │
     *   │   │  Home                           │                   │
     *   │   └─────────────────────────────────┘                   │
     *   │    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                  │
     *   │    ↑ shadow-lg (large drop shadow)                      │
     *   │                                                         │
     *   │   class="shadow-lg ..."                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Menu content has shadow-lg class
     */
    test("content should have shadow", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/shadow-lg/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Trigger Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard navigation:                                  │
     *   │                                                         │
     *   │   [Tab] ────────────────────────────────►               │
     *   │                                                         │
     *   │   ╔════════════════════╗                                │
     *   │   ║     Open Menu      ║  ← :focus (ring visible)       │
     *   │   ╚════════════════════╝                                │
     *   │                                                         │
     *   │   Button must receive focus for keyboard users          │
     *   │   to open menu via Enter/Space                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can receive keyboard focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new DropdownMenuPage(page);
      await ui.goto();

      await ui.trigger.focus();
      await expect(ui.trigger).toBeFocused();
    });
  });
});
