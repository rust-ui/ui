import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * CHIPS COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="ChipsContainer">                                      │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ╭────────╮  ╭────────╮  ╭────────╮  ╭────────╮                │   │
 * │   │   │ sunny  │  │ cloudy │  │ rainy  │  │ windy  │                │   │
 * │   │   ╰────────╯  ╰────────╯  ╰────────╯  ╰────────╯                │   │
 * │   │      ↑            ↑           ↑           ↑                     │   │
 * │   │   ChipItem    ChipItem    ChipItem    ChipItem                  │   │
 * │   │   (button)    (button)    (button)    (button)                  │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CHIP STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ╭───────────────────────────────╮                                     │
 * │   │         sunny                 │                                     │
 * │   ╰───────────────────────────────╯                                     │
 * │     ↑           ↑                ↑                                      │
 * │   rounded-full  text-sm         border                                  │
 * │   px-3 py-1     cursor-pointer                                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * LAYOUT (flex-wrap):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ╭──────╮ ╭────────╮ ╭───────╮ ╭───────╮ ╭────────╮ ╭───────╮          │
 * │   │sunny │ │ cloudy │ │ rainy │ │ windy │ │ stormy │ │ foggy │          │
 * │   ╰──────╯ ╰────────╯ ╰───────╯ ╰───────╯ ╰────────╯ ╰───────╯          │
 * │                                                                         │
 * │   ╭───────╮ ╭──────╮ ╭─────╮   ← wraps to next line                     │
 * │   │ snowy │ │ hazy │ │ etc │                                            │
 * │   ╰───────╯ ╰──────╯ ╰─────╯                                            │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ChipsPage extends BasePage {
  protected readonly componentName = "chips";

  // Chips elements
  readonly chipsContainer: Locator;
  readonly chipItems: Locator;
  readonly firstChip: Locator;

  constructor(page: Page) {
    super(page);

    // Container - scoped within preview
    this.chipsContainer = this.preview.locator('[data-name="ChipsContainer"]').first();

    // Items
    this.chipItems = this.chipsContainer.locator('[data-name="ChipItem"]');
    this.firstChip = this.chipItems.first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Chips Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: ChipsContainer Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="ChipsContainer">  <-- Must be visible │
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │ ╭────────╮ ╭────────╮ ╭────────╮ ╭────────╮     │    │
     *   │  │ │ sunny  │ │ cloudy │ │ rainy  │ │ windy  │     │    │
     *   │  │ ╰────────╯ ╰────────╯ ╰────────╯ ╰────────╯     │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ChipsContainer renders and is visible on page
     */
    test("should have ChipsContainer", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.chipsContainer).toBeVisible();
    });

    /**
     * TEST: ChipsContainer Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="ChipsContainer">                      │
     *   │       ↑                                                 │
     *   │  attribute: data-name === "ChipsContainer"             │
     *   │                                                         │
     *   │  Used for: Component identification and testing        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has correct data-name attribute
     */
    test("should have ChipsContainer data-name attribute", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.chipsContainer).toHaveAttribute(
        "data-name",
        "ChipsContainer"
      );
    });

    /**
     * TEST: Multiple ChipItems Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ChipItem count inside container:                      │
     *   │                                                         │
     *   │  ╭──────╮ ╭────────╮ ╭───────╮ ╭───────╮ ╭────────╮     │
     *   │  │sunny │ │ cloudy │ │ rainy │ │ windy │ │ stormy │ ... │
     *   │  ╰──────╯ ╰────────╯ ╰───────╯ ╰───────╯ ╰────────╯     │
     *   │    1         2          3         4          5    ...   │
     *   │                                                         │
     *   │  Total count > 10                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 10 ChipItem elements exist
     */
    test("should have multiple ChipItems", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      const count = await ui.chipItems.count();
      expect(count).toBeGreaterThan(10);
    });

    /**
     * TEST: ChipItem Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <button data-name="ChipItem">sunny</button>           │
     *   │          ↑                                              │
     *   │  attribute: data-name === "ChipItem"                   │
     *   │                                                         │
     *   │  Used for: Individual chip identification              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chip items have correct data-name attribute
     */
    test("chip items should have data-name attribute", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.firstChip).toHaveAttribute("data-name", "ChipItem");
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Weather Labels Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Container text content check:                         │
     *   │                                                         │
     *   │  ╭────────╮ ╭────────╮ ╭───────╮                        │
     *   │  │ sunny  │ │ cloudy │ │ rainy │                        │
     *   │  ╰────────╯ ╰────────╯ ╰───────╯                        │
     *   │      ↑          ↑          ↑                            │
     *   │  containsText("sunny")                                  │
     *   │  containsText("cloudy")                                 │
     *   │  containsText("rainy")                                  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Basic weather label chips are displayed
     */
    test("should display weather labels", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.chipsContainer).toContainText("sunny");
      await expect(ui.chipsContainer).toContainText("cloudy");
      await expect(ui.chipsContainer).toContainText("rainy");
    });

    /**
     * TEST: All Weather Chip Labels
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full set of weather labels:                           │
     *   │                                                         │
     *   │  ╭──────╮ ╭────────╮ ╭───────╮ ╭───────╮                │
     *   │  │sunny │ │ cloudy │ │ rainy │ │ windy │                │
     *   │  ╰──────╯ ╰────────╯ ╰───────╯ ╰───────╯                │
     *   │                                                         │
     *   │  ╭────────╮ ╭───────╮ ╭───────╮                         │
     *   │  │ stormy │ │ foggy │ │ snowy │                         │
     *   │  ╰────────╯ ╰───────╯ ╰───────╯                         │
     *   │                                                         │
     *   │  All 7 labels must be present                           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All expected weather labels are displayed
     */
    test("chips should have correct labels", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      const labels = [
        "sunny",
        "cloudy",
        "rainy",
        "windy",
        "stormy",
        "foggy",
        "snowy",
      ];

      for (const label of labels) {
        await expect(ui.chipsContainer).toContainText(label);
      }
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Container Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ChipsContainer Layout:                                │
     *   │                                                         │
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │ ╭──────╮ ╭────────╮ ╭───────╮  ← flex row      │    │
     *   │  │ │sunny │ │ cloudy │ │ rainy │                   │    │
     *   │  │ ╰──────╯ ╰────────╯ ╰───────╯                   │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   │                                                         │
     *   │  class includes: "flex"                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container uses flexbox layout
     */
    test("container should have flex layout", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.chipsContainer).toHaveClass(/flex/);
    });

    /**
     * TEST: Container Flex Wrap
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ChipsContainer Wrapping Behavior:                     │
     *   │                                                         │
     *   │  ╭──────╮ ╭────────╮ ╭───────╮ ╭───────╮ ╭────────╮     │
     *   │  │sunny │ │ cloudy │ │ rainy │ │ windy │ │ stormy │     │
     *   │  ╰──────╯ ╰────────╯ ╰───────╯ ╰───────╯ ╰────────╯     │
     *   │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─     │
     *   │  ╭───────╮ ╭──────╮   ← wraps to next line             │
     *   │  │ foggy │ │snowy │                                     │
     *   │  ╰───────╯ ╰──────╯                                     │
     *   │                                                         │
     *   │  class includes: "flex-wrap"                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips wrap to next line when container full
     */
    test("container should have flex-wrap", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.chipsContainer).toHaveClass(/flex-wrap/);
    });

    /**
     * TEST: Container Spacing Between Chips
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Spacing between chips (via margins on items):         │
     *   │                                                         │
     *   │  ╭──────╮   ╭────────╮   ╭───────╮                      │
     *   │  │sunny │   │ cloudy │   │ rainy │                      │
     *   │  ╰──────╯   ╰────────╯   ╰───────╯                      │
     *   │          ↑            ↑                                 │
     *   │       margin       margin                               │
     *   │                                                         │
     *   │  Chips have margin classes for spacing                  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have spacing via margin
     */
    test("container should have gap", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      // Container uses flex-wrap, chips have margins for spacing
      await expect(ui.chipsContainer).toHaveClass(/flex-wrap/);
      // Check first chip has margin for spacing
      await expect(ui.firstChip).toHaveClass(/m-\[/);
    });

    /**
     * TEST: ChipItem Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Chip pill shape:                                      │
     *   │                                                         │
     *   │  ╭───────────────────╮  <-- rounded-[8vmin] (pill)     │
     *   │  │       sunny       │                                  │
     *   │  ╰───────────────────╯                                  │
     *   │                                                         │
     *   │  class includes: "rounded-" prefix                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have rounded corners
     */
    test("chip item should have rounded corners", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      // Chips use viewport-relative rounding: rounded-[8vmin]
      await expect(ui.firstChip).toHaveClass(/rounded-\[/);
    });

    /**
     * TEST: ChipItem Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Chip border styling:                                  │
     *   │                                                         │
     *   │  ╭───────────────────╮  <-- visible border             │
     *   │  │       sunny       │                                  │
     *   │  ╰───────────────────╯                                  │
     *   │                                                         │
     *   │  class includes: "border"                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have border applied
     */
    test("chip item should have border", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.firstChip).toHaveClass(/border/);
    });

    /**
     * TEST: ChipItem Horizontal Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Chip horizontal padding:                              │
     *   │                                                         │
     *   │  ╭─────────────────────╮                                │
     *   │  │ ←   sunny       → │  <-- pl/pr-[vmin] (viewport)   │
     *   │  ╰─────────────────────╯                                │
     *   │    ↑              ↑                                     │
     *   │  padding-left   padding-right                           │
     *   │                                                         │
     *   │  class includes: "pl-" and "pr-" prefixes               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have horizontal padding
     */
    test("chip item should have horizontal padding", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      // Uses viewport-relative padding: pl-[3vmin] pr-[2vmin]
      await expect(ui.firstChip).toHaveClass(/pl-\[/);
      await expect(ui.firstChip).toHaveClass(/pr-\[/);
    });

    /**
     * TEST: ChipItem Vertical Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Chip vertical padding:                                │
     *   │         ↑                                               │
     *   │  ╭─────────────────────╮                                │
     *   │  │       sunny         │  <-- py-[vmin] (viewport)     │
     *   │  ╰─────────────────────╯                                │
     *   │         ↓                                               │
     *   │  padding-top/bottom                                     │
     *   │                                                         │
     *   │  class includes: "py-" prefix                           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have vertical padding
     */
    test("chip item should have vertical padding", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      // Uses viewport-relative padding: py-[2vmin]
      await expect(ui.firstChip).toHaveClass(/py-\[/);
    });

    /**
     * TEST: ChipItem Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Chip text sizing:                                     │
     *   │                                                         │
     *   │  ╭───────────────────╮                                  │
     *   │  │       sunny       │  <-- text-[vmin] (viewport)     │
     *   │  ╰───────────────────╯                                  │
     *   │                                                         │
     *   │  class includes: "text-" prefix                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips have text styling
     */
    test("chip item should have text styling", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      // Uses viewport-relative text: text-[2vmin]
      await expect(ui.firstChip).toHaveClass(/text-\[/);
    });
  });

  test.describe("Interactive Chips", () => {
    /**
     * TEST: ChipItems Are Clickable Labels
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  HTML Element Type:                                    │
     *   │                                                         │
     *   │  <label data-name="ChipItem">sunny</label>             │
     *   │    ↑                                                    │
     *   │  tagName === "label" (clickable element with checkbox) │
     *   │                                                         │
     *   │  Labels wrap hidden checkboxes for toggle behavior     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips use <label> elements with cursor-pointer
     */
    test("chip items should be clickable labels", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      const tagName = await ui.firstChip.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("label");
    });

    /**
     * TEST: ChipItems Cursor Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Cursor style on hover:                                │
     *   │                                                         │
     *   │  ╭───────────────────╮                                  │
     *   │  │       sunny       │                                  │
     *   │  ╰───────────────────╯                                  │
     *   │           ↑                                             │
     *   │         mouse                                           │
     *   │          👆  (pointer cursor)                           │
     *   │                                                         │
     *   │  class includes: "cursor-pointer"                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chips show pointer cursor indicating clickable
     */
    test("chip items should have cursor-pointer", async ({ page }) => {
      const ui = new ChipsPage(page);
      await ui.goto();

      await expect(ui.firstChip).toHaveClass(/cursor-pointer/);
    });
  });
});
