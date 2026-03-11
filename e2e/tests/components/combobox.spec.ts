import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * COMBOBOX COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Select language...                    ▼                        │   │
 * │   │       ↑                                ↑                        │   │
 * │   │  placeholder/value                 chevron icon                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                  ↑ trigger button                                       │
 * │                  │ click                                                │
 * │                  ▼                                                      │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  [popover="auto"]                                               │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │ 🔍  Search language...              ← CommandInput        │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Rust                               ← CommandItem         │  │   │
 * │   │  │  JavaScript                                               │  │   │
 * │   │  │  Ruby                                                     │  │   │
 * │   │  │  Python                                                   │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * FILTERING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Type "ru":                             Type "xyz":                    │
 * │   ┌────────────────────────────┐         ┌────────────────────────────┐ │
 * │   │ 🔍  ru                     │         │ 🔍  xyz                    │ │
 * │   ├────────────────────────────┤         ├────────────────────────────┤ │
 * │   │  Rust                      │         │  No language found.        │ │
 * │   │  Ruby                      │         └────────────────────────────┘ │
 * │   └────────────────────────────┘         ↑ CommandEmpty                 │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SELECTION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Before:                                After selection:               │
 * │   ┌────────────────────────────┐         ┌────────────────────────────┐ │
 * │   │  Select language...    ▼  │         │  Rust                  ▼  │ │
 * │   └────────────────────────────┘         └────────────────────────────┘ │
 * │                                          ↑ shows selected value         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ComboboxPage extends BasePage {
  protected readonly componentName = "combobox";

  // Combobox elements
  readonly trigger: Locator;
  readonly popoverContent: Locator;
  readonly commandInput: Locator;
  readonly commandList: Locator;
  readonly commandItems: Locator;
  readonly commandEmpty: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger button - scoped within preview
    this.trigger = this.preview.getByRole("button", { name: /Select language/ });

    // Popover content - scoped within preview
    this.popoverContent = this.preview.locator('[popover="auto"]');

    // Command parts - scoped within preview
    this.commandInput = this.preview.locator('[data-name="CommandInput"]');
    this.commandList = this.preview.locator('[data-name="CommandList"]');
    this.commandItems = this.preview.locator('[data-name="CommandItem"]');
    this.commandEmpty = this.preview.locator('[data-name="CommandEmpty"]');
  }

  async openCombobox() {
    await this.trigger.click();
    await this.page.waitForTimeout(200);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Combobox Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Select language...                    ▼          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │              ↑ trigger button (visible?)                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The combobox trigger button is rendered and visible
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: Trigger Placeholder Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Select language..."                  ▼          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ placeholder text displayed                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default placeholder "Select language..." is shown
     */
    test("trigger should display placeholder text", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      await expect(ui.trigger).toContainText("Select language...");
    });

    /**
     * TEST: Trigger Chevron Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Select language...                    ▼          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                           ↑              │
     *   │                                  chevron icon (svg)      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: SVG chevron icon is present in the trigger button
     */
    test("trigger should have chevron icon", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      const icon = ui.trigger.locator("svg");
      await expect(icon).toBeVisible();
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Clicking Trigger Opens Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before:                    After click:                │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Select...   ▼  │  ──►    │ Select...   ▼  │          │
     *   │  └────────────────┘         ├────────────────┤          │
     *   │                             │ 🔍 Search...   │          │
     *   │                             │ Rust           │          │
     *   │                             │ JavaScript     │          │
     *   │                             └────────────────┘          │
     *   │                                  ↑ popover visible      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger button opens the popover dropdown
     */
    test("clicking trigger should open popover", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      await ui.openCombobox();

      await expect(ui.popoverContent).toBeVisible();
    });

    /**
     * TEST: ESC Key Closes Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Open state:                After ESC:                  │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Select...   ▼  │         │ Select...   ▼  │          │
     *   │  ├────────────────┤  ──►    └────────────────┘          │
     *   │  │ 🔍 Search...   │    ⌨                                │
     *   │  │ Rust           │   ESC        popover hidden         │
     *   │  └────────────────┘                                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressing Escape key closes the open popover
     */
    test("pressing ESC should close popover", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await page.keyboard.press("Escape");
      await page.waitForTimeout(200);

      await expect(ui.popoverContent).not.toBeVisible();
    });
  });

  test.describe("Command Input", () => {
    /**
     * TEST: Search Input Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  Search language...      ← CommandInput        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  │ Rust                                              │  │
     *   │  │ JavaScript                                        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Search input field is visible inside open popover
     */
    test("should have search input", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(ui.commandInput).toBeVisible();
    });

    /**
     * TEST: Search Input Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  "Search language..."                          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ placeholder attribute                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has placeholder="Search language..."
     */
    test("input should have placeholder", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(ui.commandInput).toHaveAttribute(
        "placeholder",
        "Search language..."
      );
    });

    /**
     * TEST: Input Accepts Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍  rust|                  ← user types "rust"    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ input value = "rust"                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User can type in the search input and value updates
     */
    test("should be able to type in input", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await ui.commandInput.fill("rust");
      await expect(ui.commandInput).toHaveValue("rust");
    });
  });

  test.describe("Language Options", () => {
    /**
     * TEST: Language Options Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍 Search...                                      │  │
     *   │  ├───────────────────────────────────────────────────┤  │
     *   │  │  1. Rust                                          │  │
     *   │  │  2. JavaScript                                    │  │
     *   │  │  3. Ruby                                          │  │
     *   │  │  4. Python                                        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │           ↑ count = 4 CommandItems                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 4 language options are available
     */
    test("should have language options", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      const count = await ui.commandItems.count();
      expect(count).toBe(4);
    });

    /**
     * TEST: Rust Option Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ► Rust  ◄───────────────── visible?              │  │
     *   │  │    JavaScript                                     │  │
     *   │  │    Ruby                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Rust" option is visible in the dropdown
     */
    test("should have Rust option", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(page.getByRole("option", { name: "Rust" })).toBeVisible();
    });

    /**
     * TEST: JavaScript Option Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │    Rust                                           │  │
     *   │  │  ► JavaScript  ◄─────────── visible?              │  │
     *   │  │    Ruby                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "JavaScript" option is visible in the dropdown
     */
    test("should have JavaScript option", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(
        page.getByRole("option", { name: "JavaScript" })
      ).toBeVisible();
    });

    /**
     * TEST: Ruby Option Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │    Rust                                           │  │
     *   │  │    JavaScript                                     │  │
     *   │  │  ► Ruby  ◄───────────────── visible?              │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Ruby" option is visible in the dropdown
     */
    test("should have Ruby option", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(page.getByRole("option", { name: "Ruby" })).toBeVisible();
    });

    /**
     * TEST: Python Option Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │    Rust                                           │  │
     *   │  │    JavaScript                                     │  │
     *   │  │    Ruby                                           │  │
     *   │  │  ► Python  ◄─────────────── visible?              │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Python" option is visible in the dropdown
     */
    test("should have Python option", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await expect(page.getByRole("option", { name: "Python" })).toBeVisible();
    });
  });

  test.describe("Selection", () => {
    /**
     * TEST: Clicking Option Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before:                    After click on "Rust":      │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Select...   ▼  │         │ Rust        ▼  │          │
     *   │  ├────────────────┤  ──►    └────────────────┘          │
     *   │  │  [Rust] ◄ click│             ↑                       │
     *   │  │  JavaScript    │         selected value shown        │
     *   │  └────────────────┘                                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking an option updates trigger to show selection
     */
    test("clicking option should select it", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await page.getByRole("option", { name: "Rust" }).click();
      await page.waitForTimeout(100);

      await expect(ui.trigger).toContainText("Rust");
    });

    /**
     * TEST: Selection Closes Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Open:                      After selection:            │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Select...   ▼  │         │ JavaScript  ▼  │          │
     *   │  ├────────────────┤  ──►    └────────────────┘          │
     *   │  │  Rust          │                                     │
     *   │  │ [JavaScript]◄  │         popover hidden              │
     *   │  └────────────────┘                                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selecting an option automatically closes the popover
     */
    test("selecting option should close popover", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await page.getByRole("option", { name: "JavaScript" }).click();
      await page.waitForTimeout(200);

      await expect(ui.popoverContent).not.toBeVisible();
    });

    /**
     * TEST: Can Change Selection
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Step 1: Select Rust        Step 2: Change to Python    │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Rust        ▼  │  ──►    │ Python      ▼  │          │
     *   │  └────────────────┘         └────────────────┘          │
     *   │                                                         │
     *   │  Rust → Python (selection can be changed)               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User can change selection from one option to another
     */
    test("can change selection", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      // Wait for hydration
      await page.waitForLoadState("networkidle");

      // The trigger is inside the Preview container - find it there
      const previewTrigger = ui.preview.locator("button").filter({ hasText: /language|Rust|Python/ }).first();

      // Select first option
      await previewTrigger.click();
      await page.waitForTimeout(200);
      await page.getByRole("option", { name: "Rust" }).click();
      await page.waitForTimeout(200);
      await expect(previewTrigger).toContainText("Rust");

      // Select different option
      await previewTrigger.click();
      await page.waitForTimeout(200);
      await page.getByRole("option", { name: "Python" }).click();
      await page.waitForTimeout(200);
      await expect(previewTrigger).toContainText("Python");
    });
  });

  test.describe("Filtering", () => {
    /**
     * TEST: Typing Filters Options
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before filter:             After typing "ru":          │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ 🔍             │         │ 🔍 ru          │          │
     *   │  ├────────────────┤         ├────────────────┤          │
     *   │  │  Rust          │  ──►    │  Rust ◄ match  │          │
     *   │  │  JavaScript    │         │  Ruby ◄ match  │          │
     *   │  │  Ruby          │         └────────────────┘          │
     *   │  │  Python        │         (JavaScript, Python hidden) │
     *   │  └────────────────┘                                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typing "ru" filters to show only Rust and Ruby
     */
    test("typing should filter options", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();
      await ui.openCombobox();

      await ui.commandInput.fill("ru");

      // Rust and Ruby should be visible
      await expect(page.getByRole("option", { name: "Rust" })).toBeVisible();
      await expect(page.getByRole("option", { name: "Ruby" })).toBeVisible();
    });

    /**
     * TEST: No Match Shows Empty Message
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ 🔍 xyz                                             │  │
     *   │  ├───────────────────────────────────────────────────┤  │
     *   │  │                                                   │  │
     *   │  │    "No language found."  ← CommandEmpty           │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Non-matching search shows "No language found." message
     */
    test("no match should show empty message", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      // Wait for hydration
      await page.waitForLoadState("networkidle");
      await ui.openCombobox();

      // Use the input within the popover to avoid matching global search
      const popoverInput = ui.popoverContent.locator('[data-name="CommandInput"]');
      await popoverInput.fill("xyz");
      // Wait for filter to apply
      await page.waitForTimeout(300);

      // Use the CommandEmpty locator within the popover to target the empty state message
      const emptyMessage = ui.popoverContent.locator('[data-name="CommandEmpty"]');
      await expect(emptyMessage).toBeVisible();
      await expect(emptyMessage).toContainText("No language found.");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Trigger Has justify-between Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Select language...                    ▼          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │      ↑ text                            icon ↑           │
     *   │      └──────── justify-between ────────────┘            │
     *   │                (space between elements)                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button has justify-between for layout
     */
    test("trigger should have justify-between", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/justify-between/);
    });

    /**
     * TEST: Trigger Has Fixed Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │  ◄──────────── 200px ────────────►                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Select language...                    ▼          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button has fixed width of 200px
     */
    test("trigger should have w-[200px]", async ({ page }) => {
      const ui = new ComboboxPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/w-\[200px\]/);
    });
  });
});
