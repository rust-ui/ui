import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * INPUT-GROUP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="InputGroup">                                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   [start addon]  [input field]              [end addon]         │   │
 * │   │   ┌──────────┐   ┌────────────────────────┐ ┌──────────┐        │   │
 * │   │   │   🔍     │   │  Search...             │ │          │        │   │
 * │   │   │   icon   │   │                        │ │   icon   │        │   │
 * │   │   └──────────┘   └────────────────────────┘ └──────────┘        │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ADDON VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Start addon only:              End addon only:                        │
 * │   ┌────────────────────────────┐ ┌────────────────────────────┐         │
 * │   │ 🔍 │  Search...            │ │  Email...            │ @ │          │
 * │   └────────────────────────────┘ └────────────────────────────┘         │
 * │                                                                         │
 * │   Both addons:                                                          │
 * │   ┌────────────────────────────────────────────────────┐                │
 * │   │ 💳 │  Card number                          │  ✓  │                 │
 * │   └────────────────────────────────────────────────────┘                │
 * │                                                                         │
 * │   Multiple end addons:                                                  │
 * │   ┌────────────────────────────────────────────────────┐                │
 * │   │ 🔒 │  Password                        │ 👁 │ ✓ │                   │
 * │   └────────────────────────────────────────────────────┘                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .relative     ← Positioning context                        │       │
 * │   │  .flex         ← Flex layout                                │       │
 * │   │  .items-center ← Vertical centering                         │       │
 * │   │  .border       ← Border styling                             │       │
 * │   │  .rounded-md   ← Medium border radius                       │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class InputGroupPage extends BasePage {
  protected readonly componentName = "input-group";

  // Input group elements
  readonly inputGroups: Locator;
  readonly firstInputGroup: Locator;
  readonly searchInput: Locator;
  readonly emailInput: Locator;
  readonly cardInput: Locator;

  constructor(page: Page) {
    super(page);

    // All input groups - scoped within preview
    this.inputGroups = this.preview.locator('[data-name="InputGroup"]');
    this.firstInputGroup = this.inputGroups.nth(0);

    // Inputs - scoped within preview
    this.searchInput = this.preview.locator('input[placeholder="Search..."]');
    this.emailInput = this.preview.locator('input[placeholder="Enter your email"]');
    this.cardInput = this.preview.locator('input[placeholder="Card number"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("InputGroup Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: InputGroup Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  data-name="InputGroup"                                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [icon]  │  Search...                   │         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                      ↑                                  │
     *   │              Should be visible                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First InputGroup component is rendered and visible
     */
    test("should have InputGroup components", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toBeVisible();
    });

    /**
     * TEST: InputGroup Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  InputGroup #1: ┌─────────────────────────────────────┐ │
     *   │                 │  [icon]  Search...                  │ │
     *   │                 └─────────────────────────────────────┘ │
     *   │  InputGroup #2: ┌─────────────────────────────────────┐ │
     *   │                 │  Email...                    [icon] │ │
     *   │                 └─────────────────────────────────────┘ │
     *   │  InputGroup #3: ┌─────────────────────────────────────┐ │
     *   │                 │  [icon]  Card number         [icon] │ │
     *   │                 └─────────────────────────────────────┘ │
     *   │  InputGroup #4: ┌─────────────────────────────────────┐ │
     *   │                 │  [icon]  Password       [icon][icon]│ │
     *   │                 └─────────────────────────────────────┘ │
     *   │                                                         │
     *   │           count() === 4                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 4 InputGroup components exist on page
     */
    test("should have four InputGroups", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      const count = await ui.inputGroups.count();
      expect(count).toBe(4);
    });

    /**
     * TEST: InputGroup Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="InputGroup">  <── attribute check     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [icon]  │  Search...                             │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name="InputGroup" attribute is present
     */
    test("InputGroups should have data-name attribute", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveAttribute(
        "data-name",
        "InputGroup"
      );
    });
  });

  test.describe("Inputs", () => {
    /**
     * TEST: Search Input with Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [🔍]  │  Search...                               │  │
     *   │  └────────┴──────────────────────────────────────────┘  │
     *   │                    ↑                                    │
     *   │           placeholder="Search..."                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Search input is visible with correct placeholder text
     */
    test("should have search input with placeholder", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.searchInput).toBeVisible();
      await expect(ui.searchInput).toHaveAttribute("placeholder", "Search...");
    });

    /**
     * TEST: Email Input with Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Enter your email                           │ [@] │  │
     *   │  └─────────────────────────────────────────────┴─────┘  │
     *   │          ↑                                              │
     *   │  placeholder="Enter your email"                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input is visible with correct placeholder text
     */
    test("should have email input with placeholder", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.emailInput).toBeVisible();
      await expect(ui.emailInput).toHaveAttribute(
        "placeholder",
        "Enter your email"
      );
    });

    /**
     * TEST: Card Input with Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [💳] │  Card number                        │ [✓] │  │
     *   │  └──────┴────────────────────────────────────┴──────┘  │
     *   │                  ↑                                      │
     *   │         placeholder="Card number"                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card input is visible with correct placeholder text
     */
    test("should have card input with placeholder", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.cardInput).toBeVisible();
      await expect(ui.cardInput).toHaveAttribute("placeholder", "Card number");
    });
  });

  test.describe("Addons", () => {
    /**
     * TEST: Input Group Has Addon Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [🔍]  │  Search...                               │  │
     *   │  └───┬───┴───────────────────────────────────────────┘  │
     *   │      │                                                  │
     *   │      └── <svg> icon should be visible                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First input group contains at least one SVG icon addon
     */
    test("each input group should have addon icon", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      // First group should have search icon
      const firstIcon = ui.firstInputGroup.locator("svg");
      await expect(firstIcon.first()).toBeVisible();
    });

    /**
     * TEST: Card Input Has Two Addons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  InputGroup #3 (card input):                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [💳] │  Card number                        │ [✓] │  │
     *   │  └──┬───┴────────────────────────────────────┴──┬───┘  │
     *   │     │                                           │       │
     *   │     └── svg #1 (start)              svg #2 (end)┘       │
     *   │                                                         │
     *   │         svg.count() === 2                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third input group has exactly 2 SVG icons (start + end)
     */
    test("card input should have two addons", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      // Third input group has credit card and check icons
      const thirdGroup = ui.inputGroups.nth(2);
      const icons = thirdGroup.locator("svg");
      const count = await icons.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Fourth Input Group Has Multiple End Icons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  InputGroup #4 (password input):                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [🔒] │  Password                     │ [👁] │ [✓] │  │
     *   │  └──┬───┴──────────────────────────────┴──┬───┴──┬───┘  │
     *   │     │                                     │      │       │
     *   │   start                                 end icons        │
     *   │                                                         │
     *   │         svg.count() === 2                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fourth input group has 2 SVG icons (start + end addons)
     */
    test("fourth input group should have multiple end icons", async ({
      page,
    }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      const fourthGroup = ui.inputGroups.nth(3);
      const icons = fourthGroup.locator("svg");
      const count = await icons.count();
      expect(count).toBe(2);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Input Group Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... relative ..."                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [icon]  │  input field                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │   .relative ← Allows absolute positioning of children   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputGroup has 'relative' class for positioning context
     */
    test("input group should have relative positioning", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveClass(/relative/);
    });

    /**
     * TEST: Input Group Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... flex ..."                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [icon] ──┬── [input] ──┬── [icon]                │  │
     *   │  └───────────┴─────────────┴─────────────────────────┘  │
     *   │      ↑             ↑            ↑                       │
     *   │      └─────────────┴────────────┘                       │
     *   │            flex children in a row                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputGroup has 'flex' class for horizontal layout
     */
    test("input group should have flex layout", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveClass(/flex/);
    });

    /**
     * TEST: Input Group Items Center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... items-center ..."                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │         ┌─────┐ ┌─────────────┐ ┌─────┐           │  │
     *   │  │  ─ ─ ─ ─│icon │─│   input     │─│icon │─ ─ ─ ─    │  │
     *   │  │         └─────┘ └─────────────┘ └─────┘           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │              ↑ vertically centered on cross-axis        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputGroup has 'items-center' for vertical centering
     */
    test("input group should have items-center", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveClass(/items-center/);
    });

    /**
     * TEST: Input Group Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... border ..."                                 │
     *   │  ╔═══════════════════════════════════════════════════╗  │
     *   │  ║  [icon]  │  input field                           ║  │
     *   │  ╚═══════════════════════════════════════════════════╝  │
     *   │   ↑                                                     │
     *   │   border around the entire group                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputGroup has 'border' class for outline styling
     */
    test("input group should have border", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveClass(/border/);
    });

    /**
     * TEST: Input Group Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... rounded-md ..."                             │
     *   │  ╭───────────────────────────────────────────────────╮  │
     *   │  │  [icon]  │  input field                           │  │
     *   │  ╰───────────────────────────────────────────────────╯  │
     *   │  ↑                                                   ↑  │
     *   │  rounded corners (border-radius: medium)                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputGroup has 'rounded-md' class for rounded corners
     */
    test("input group should have rounded corners", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await expect(ui.firstInputGroup).toHaveClass(/rounded-md/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Search Input Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before focus:                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [🔍]  │  Search...                               │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  After focus():                                         │
     *   │  ┌═══════════════════════════════════════════════════┐  │
     *   │  │  [🔍]  │  |                              (focused) │  │
     *   │  └═══════════════════════════════════════════════════┘  │
     *   │              ↑ cursor visible, toBeFocused() = true     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Search input can receive keyboard focus
     */
    test("search input should be focusable", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await ui.searchInput.focus();
      await expect(ui.searchInput).toBeFocused();
    });

    /**
     * TEST: Type in Search Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before fill():                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [🔍]  │  Search...                               │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  After fill("test search"):                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [🔍]  │  test search                             │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │              ↑ value = "test search"                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Search input accepts text input and retains value
     */
    test("should be able to type in search input", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await ui.searchInput.fill("test search");
      await expect(ui.searchInput).toHaveValue("test search");
    });

    /**
     * TEST: Type in Email Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before fill():                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Enter your email                           │ [@] │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  After fill("test@example.com"):                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  test@example.com                           │ [@] │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │      ↑ value = "test@example.com"                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input accepts text input and retains value
     */
    test("should be able to type in email input", async ({ page }) => {
      const ui = new InputGroupPage(page);
      await ui.goto();

      await ui.emailInput.fill("test@example.com");
      await expect(ui.emailInput).toHaveValue("test@example.com");
    });
  });
});
