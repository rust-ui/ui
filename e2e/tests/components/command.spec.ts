import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * COMMAND COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="Command">                                             │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  CommandHeader                                                  │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Search documentation (title)                             │  │   │
 * │   │  │  Search for a command or page (description)               │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  CommandInput                                                   │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │ 🔍  Search documentation...                               │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  CommandList                                                    │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Pages ─────────────────────────────────────────          │  │   │
 * │   │  │  📄 Docs                                                  │  │   │
 * │   │  │  📄 Getting Started                                       │  │   │
 * │   │  │  Components ────────────────────────────────────          │  │   │
 * │   │  │  📦 Accordion                                             │  │   │
 * │   │  │  📦 Button                                                │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  CommandFooter                                                  │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  ↑↓ Navigate    ⏎ Go to Page                              │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * GROUPS AND ITEMS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   CommandGroup:                                                         │
 * │   ┌───────────────────────────────────────────────────────────────┐     │
 * │   │  [Group Label]                                                │     │
 * │   │  ┌─────────────────────────────────────────────────────────┐  │     │
 * │   │  │ 📄  Item Title                                          │  │     │
 * │   │  │     ↑     ↑                                             │  │     │
 * │   │  │   icon   text                                           │  │     │
 * │   │  └─────────────────────────────────────────────────────────┘  │     │
 * │   │  ┌─────────────────────────────────────────────────────────┐  │     │
 * │   │  │ 📄  Another Item                                        │  │     │
 * │   │  └─────────────────────────────────────────────────────────┘  │     │
 * │   └───────────────────────────────────────────────────────────────┘     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class CommandPage extends BasePage {
  protected readonly componentName = "command";

  // Command elements - all scoped within preview container
  readonly command: Locator;
  readonly commandHeader: Locator;
  readonly commandTitle: Locator;
  readonly commandDescription: Locator;
  readonly commandInput: Locator;
  readonly commandList: Locator;
  readonly commandGroups: Locator;
  readonly commandItems: Locator;
  readonly commandFooter: Locator;

  constructor(page: Page) {
    super(page);

    // All locators scoped within preview to avoid matching global search/nav elements
    this.command = this.preview.locator('[data-name="Command"]').first();
    this.commandHeader = this.preview.locator('[data-name="CommandHeader"]').first();
    this.commandTitle = this.preview.locator('[data-name="CommandTitle"]').first();
    this.commandDescription = this.preview.locator('[data-name="CommandDescription"]').first();
    this.commandInput = this.preview.locator('[data-name="CommandInput"]').first();
    this.commandList = this.preview.locator('[data-name="CommandList"]').first();
    this.commandGroups = this.preview.locator('[data-name="CommandGroup"]');
    this.commandItems = this.preview.locator('[data-name="CommandItemLink"]');
    this.commandFooter = this.preview.locator('[data-name="CommandFooter"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Command Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Command Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  data-name="Command"                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Header]                                         │  │
     *   │  │  [Input]                                          │  │
     *   │  │  [List]                                           │  │
     *   │  │  [Footer]                                         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                        ↑                                │
     *   │              Should be visible                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main Command component is rendered and visible
     */
    test("should have Command component", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.command).toBeVisible();
    });

    /**
     * TEST: Command Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="Command">  <── attribute check        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Command palette content...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name="Command" attribute is present
     */
    test("should have Command data-name attribute", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.command).toHaveAttribute("data-name", "Command");
    });

    /**
     * TEST: CommandHeader Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  CommandHeader  <── should exist in DOM           │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  Search documentation (title)               │  │  │
     *   │  │  │  Search for a command (description)         │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  Note: hidden on mobile, visible on sm+ screens   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandHeader section exists (may be hidden on mobile)
     */
    test("should have CommandHeader", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      // CommandHeader is hidden on mobile (hidden sm:text-left), check existence
      await expect(ui.commandHeader).toHaveAttribute("data-name", "CommandHeader");
    });

    /**
     * TEST: CommandInput Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Header]                                         │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ 🔍  Search documentation...                 │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑ CommandInput should be visible            │  │
     *   │  │  [List]                                          │  │
     *   │  │  [Footer]                                        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandInput search field is visible
     */
    test("should have CommandInput", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandInput).toBeVisible();
    });

    /**
     * TEST: CommandList Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Header]                                         │  │
     *   │  │  [Input]                                          │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  CommandList  <── should be visible         │  │  │
     *   │  │  │  ┌─────────────────────────────────────┐    │  │  │
     *   │  │  │  │ Pages                               │    │  │  │
     *   │  │  │  │   Docs                              │    │  │  │
     *   │  │  │  │ Components                          │    │  │  │
     *   │  │  │  │   Accordion                         │    │  │  │
     *   │  │  │  └─────────────────────────────────────┘    │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandList container is visible
     */
    test("should have CommandList", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandList).toBeVisible();
    });

    /**
     * TEST: CommandFooter Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Header]                                         │  │
     *   │  │  [Input]                                          │  │
     *   │  │  [List]                                           │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  CommandFooter  <── should be visible       │  │  │
     *   │  │  │  ↑↓ Navigate    ⏎ Go to Page                │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandFooter with keyboard shortcuts is visible
     */
    test("should have CommandFooter", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandFooter).toBeVisible();
    });
  });

  test.describe("Header", () => {
    /**
     * TEST: Header Has Title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandHeader                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  "Search documentation"  <── title text     │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  Search for a command or page (description)       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandTitle contains "Search documentation" text
     */
    test("should have title", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandTitle).toContainText("Search documentation");
    });

    /**
     * TEST: Header Has Description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandHeader                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Search documentation (title)                     │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  "Search for a command"  <── description    │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CommandDescription contains "Search for a command"
     */
    test("should have description", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandDescription).toContainText("Search for a command");
    });
  });

  test.describe("Input", () => {
    /**
     * TEST: Input Has Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  Search documentation...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │            ↑                                            │
     *   │   placeholder="Search documentation..."                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has correct placeholder attribute
     */
    test("input should have placeholder", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandInput).toHaveAttribute(
        "placeholder",
        "Search documentation..."
      );
    });

    /**
     * TEST: Input Is Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before focus():                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  Search documentation...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  After focus():                                         │
     *   │  ┌═══════════════════════════════════════════════════┐  │
     *   │  │ 🔍  |                                    (focused) │  │
     *   │  └═══════════════════════════════════════════════════┘  │
     *   │        ↑ cursor visible, toBeFocused() = true           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Command input can receive keyboard focus
     */
    test("input should be focusable", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await ui.commandInput.focus();
      await expect(ui.commandInput).toBeFocused();
    });

    /**
     * TEST: Type in Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before fill():                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  Search documentation...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  After fill("accordion"):                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  accordion                                     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │        ↑ value = "accordion"                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User can type search query in command input
     */
    test("should be able to type in input", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await ui.commandInput.fill("accordion");
      await expect(ui.commandInput).toHaveValue("accordion");
    });
  });

  test.describe("Groups", () => {
    /**
     * TEST: Command Groups Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandList                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ CommandGroup #1: Pages                      │  │  │
     *   │  │  │   - Docs                                    │  │  │
     *   │  │  │   - Getting Started                         │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ CommandGroup #2: Components                 │  │  │
     *   │  │  │   - Accordion                               │  │  │
     *   │  │  │   - Button                                  │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                  count() === 2                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 2 CommandGroup elements exist
     */
    test("should have command groups", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const count = await ui.commandGroups.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Pages Group Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Pages"  <── group label should be visible       │  │
     *   │  │  ─────────────────────────────────────────────────│  │
     *   │  │    📄 Docs                                        │  │
     *   │  │    📄 Components                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Pages" group label is visible within preview
     */
    test("should have Pages group", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.preview.getByText("Pages")).toBeVisible();
    });

    /**
     * TEST: Components Group Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  CommandGroupLabel: "Components"                  │  │
     *   │  │  ─────────────────────────────────────────────────│  │
     *   │  │    📦 Accordion                                   │  │
     *   │  │    📦 Alert                                       │  │
     *   │  │    📦 Avatar                                      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Components" CommandGroupLabel exists within preview
     */
    test("should have Components group", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      // Use CommandGroupLabel locator to avoid matching link items
      const componentsLabel = ui.preview
        .locator('[data-name="CommandGroupLabel"]')
        .filter({ hasText: "Components" });
      await expect(componentsLabel).toBeVisible();
    });
  });

  test.describe("Items", () => {
    /**
     * TEST: Command Items Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandList                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Pages:                                           │  │
     *   │  │    [item 1] Docs                                  │  │
     *   │  │    [item 2] Getting Started                       │  │
     *   │  │  Components:                                      │  │
     *   │  │    [item 3] Accordion                             │  │
     *   │  │    [item 4] Button                                │  │
     *   │  │    [item 5] Card                                  │  │
     *   │  │    [item 6] ...                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │           count() > 5                                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 5 command items exist in the list
     */
    test("should have command items", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const count = await ui.commandItems.count();
      expect(count).toBeGreaterThan(5);
    });

    /**
     * TEST: Items Have Icons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandItemLink                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [📄]  Docs                                       │  │
     *   │  └───┬───────────────────────────────────────────────┘  │
     *   │      │                                                  │
     *   │      └── <svg> icon should be visible                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First command item contains an SVG icon
     */
    test("items should have icons", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const icon = ui.commandItems.first().locator("svg");
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Docs Item Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │  Pages group:                                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📄  "Docs"  <── text should be visible           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📄  Components                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Docs" item text is visible within preview
     */
    test("should have Docs item", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.preview.getByText("Docs", { exact: true })).toBeVisible();
    });

    /**
     * TEST: Accordion Item Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │  Components group:                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📦  "Accordion"  <── text should be visible      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📦  Alert                                        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Accordion" item text is visible within preview
     */
    test("should have Accordion item", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.preview.getByText("Accordion", { exact: true })).toBeVisible();
    });

    /**
     * TEST: Items Are Links
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <a>  <── tagName check                                │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📄  Docs                                         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  tagName.toLowerCase() === "a"                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Command items are <a> anchor elements
     */
    test("items should be links", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const tagName = await ui.commandItems
        .first()
        .evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("a");
    });

    /**
     * TEST: Items Have Href Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <a href="/docs">  <── attribute check                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  📄  Docs                                         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  href !== null && href !== ""                           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Command items have href attribute for navigation
     */
    test("items should have href attribute", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const href = await ui.commandItems.first().getAttribute("href");
      expect(href).toBeTruthy();
    });
  });

  test.describe("Footer", () => {
    /**
     * TEST: Footer Has Keyboard Shortcuts
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandFooter                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [↑][↓] "Navigate"    [⏎] "Go to Page"            │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │           ↑                       ↑                     │
     *   │     text visible            text visible                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Footer contains "Navigate" and "Go to Page" text
     */
    test("should have keyboard shortcuts", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      await expect(ui.commandFooter).toContainText("Navigate");
      await expect(ui.commandFooter).toContainText("Go to Page");
    });

    /**
     * TEST: Footer Has Kbd Elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CommandFooter                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───┐ ┌───┐          ┌─────┐                     │  │
     *   │  │  │ ↑ │ │ ↓ │ Navigate │  ⏎  │ Go to Page         │  │
     *   │  │  └───┘ └───┘          └─────┘                     │  │
     *   │  │    ↑     ↑              ↑                         │  │
     *   │  │    └─────┴──────────────┘                         │  │
     *   │  │      data-name="Kbd" elements                     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │          count() > 0                                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Footer contains Kbd elements for keyboard hints
     */
    test("should have Kbd elements", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const kbds = ui.commandFooter.locator('[data-name="Kbd"]');
      const count = await kbds.count();
      expect(count).toBeGreaterThan(0);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Command Has Rounded Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  class="... rounded-md border ..."                │  │
     *   │  │  ╭─────────────────────────────────────────────╮  │  │
     *   │  │  │  Command palette content                    │  │  │
     *   │  │  ╰─────────────────────────────────────────────╯  │  │
     *   │  │  ↑                                             ↑  │  │
     *   │  │  rounded corners + border styling                 │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: __DemoCard wrapper has 'rounded-md' and 'border' classes
     */
    test("command should have rounded border", async ({ page }) => {
      const ui = new CommandPage(page);
      await ui.goto();

      const wrapper = ui.preview.locator('[data-name="__DemoCard"]').first();
      await expect(wrapper).toHaveClass(/rounded-md/);
      await expect(wrapper).toHaveClass(/border/);
    });
  });
});
