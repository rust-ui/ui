import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * MULTI-SELECT COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Status Display                                                        │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Selected: Apple, Banana, Mango                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * │   MultiSelect Trigger                                                   │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Select fruits...                                          ▼   │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                  │                                                      │
 * │                  │ click                                                │
 * │                  ▼                                                      │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ☑ Apple                                                        │   │
 * │   │  ☑ Banana                                                       │   │
 * │   │  ☐ Orange                                                       │   │
 * │   │  ☐ Strawberry                                                   │   │
 * │   │  ☑ Mango                                                        │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │        ↑ MultiSelectContent with MultiSelectItems                       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SELECTION BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Unlike single Select, MultiSelect allows multiple selections:         │
 * │                                                                         │
 * │   Click Apple:      ☑ Apple                                             │
 * │   Click Banana:     ☑ Apple, ☑ Banana                                   │
 * │   Click Apple:      ☐ Apple, ☑ Banana  (toggle off)                     │
 * │                                                                         │
 * │   HashSet<String> stores selected values                                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * AVAILABLE OPTIONS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   • Apple                                                               │
 * │   • Banana                                                              │
 * │   • Orange                                                              │
 * │   • Strawberry                                                          │
 * │   • Mango                                                               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class MultiSelectPage extends BasePage {
  protected readonly componentName = "multi-select";

  // MultiSelect elements
  readonly multiSelect: Locator;
  readonly trigger: Locator;
  readonly content: Locator;
  readonly options: Locator;
  readonly statusText: Locator;

  constructor(page: Page) {
    super(page);

    // Scoped within preview
    this.multiSelect = this.preview.locator('[data-name="MultiSelect"]').first();
    this.trigger = this.preview.locator('[data-name="MultiSelectTrigger"]').first();
    // Content rendered in portal
    this.content = page.locator('[data-name="MultiSelectContent"]').first();
    this.options = page.locator('[data-name="MultiSelectOption"]');
    // Status display - scoped within preview
    this.statusText = this.preview.getByText("No fruits selected").first();
  }

  async openSelect() {
    await this.trigger.click();
    await this.page.waitForTimeout(200);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("MultiSelect Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: MultiSelect Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  MultiSelect Container                         │   │
     *   │   │  data-name="MultiSelect"                       │   │
     *   │   │         ↑                                      │   │
     *   │   │    MUST BE VISIBLE                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: MultiSelect wrapper element renders on page
     */
    test("should have MultiSelect container", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.multiSelect).toBeVisible();
    });

    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Select fruits...                          ▼   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │         ↑                                               │
     *   │    MultiSelectTrigger  <-- MUST BE VISIBLE              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button is rendered and visible
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: Status Text Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "No fruits selected"                           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │         ↑                                               │
     *   │    Status display  <-- MUST BE VISIBLE                  │
     *   │    Shows current selection state                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Status text element renders on page
     */
    test("should have status text", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.statusText).toBeVisible();
    });
  });

  test.describe("Initial State", () => {
    /**
     * TEST: Initial Status Message
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   On page load (no selections):                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "No fruits selected"  <-- MUST DISPLAY         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   HashSet<String> = { }  (empty)                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default empty state message is shown
     */
    test("should show 'No fruits selected' initially", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.statusText).toBeVisible();
    });

    /**
     * TEST: Trigger Placeholder Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "Select fruits..."                        ▼   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │         ↑                                               │
     *   │    Placeholder text when nothing selected               │
     *   │    MUST contain "Select fruits"                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger shows placeholder when no selection
     */
    test("trigger should show placeholder", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toContainText("Select fruits");
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Dropdown Opens on Trigger Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                     AFTER:                    │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │ Select fruits ▼  │  ──►   │ Select fruits ▼  │      │
     *   │   └──────────────────┘ click  ├──────────────────┤      │
     *   │                               │ ☐ Apple          │      │
     *   │                               │ ☐ Banana         │      │
     *   │                               │ ☐ Orange         │      │
     *   │                               └──────────────────┘      │
     *   │                                        ↑                │
     *   │                               MUST BE VISIBLE           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content dropdown appears when trigger clicked
     */
    test("clicking trigger should open dropdown", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await ui.openSelect();

      await expect(ui.content).toBeVisible();
    });
  });

  test.describe("Options", () => {
    /**
     * TEST: Fruit Options Available
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   MultiSelectContent (opened):                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ☐ Apple        │  role="option"                │   │
     *   │   │  ☐ Banana       │  role="option"                │   │
     *   │   │  ☐ Orange       │  role="option"                │   │
     *   │   │  ☐ Strawberry   │  role="option"                │   │
     *   │   │  ☐ Mango        │  role="option"                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Count >= 5 options                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least 5 fruit options are displayed
     */
    test("should have fruit options after opening", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();
      await ui.openSelect();

      // Check that options are visible using role-based selector
      const options = page.getByRole("option");
      const count = await options.count();
      expect(count).toBeGreaterThanOrEqual(5);
    });
  });

  test.describe("Selection", () => {
    /**
     * TEST: Option Selection Updates Status
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE:                     AFTER:                    │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │ No fruits        │  ──►   │ Selected: Apple  │      │
     *   │   │ selected         │        │                  │      │
     *   │   └──────────────────┘        └──────────────────┘      │
     *   │                                        ↑                │
     *   │   Click "Apple":             Status changes to show     │
     *   │   ┌──────────────────┐       "Selected:" prefix         │
     *   │   │ ☑ Apple  ← click │                                  │
     *   │   │ ☐ Banana         │                                  │
     *   │   └──────────────────┘                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selecting option updates status display
     */
    test("clicking option should select it", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();
      await ui.openSelect();

      // Click the first Apple option
      await page.getByRole("option", { name: /Apple/ }).first().click();
      await page.waitForTimeout(100);

      // Status should change from "No fruits selected"
      await expect(page.getByText("Selected:").first()).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Trigger Width Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Select fruits...                          ▼   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   │◄──────────── w-[150px] ────────────────────►│       │
     *   │                                                         │
     *   │   Trigger MUST have class: w-[150px]                    │
     *   │   (fixed width for consistent layout)                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has correct width class applied
     */
    test("trigger should have width class", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/w-\[150px\]/);
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
     *   │   Keyboard Navigation:                                  │
     *   │                                                         │
     *   │   [Tab] ──► ┌─────────────────────────────────────┐     │
     *   │             │  Select fruits...              ▼   │     │
     *   │             │  ════════════════════════════════  │     │
     *   │             │         ↑ focus ring visible       │     │
     *   │             └─────────────────────────────────────┘     │
     *   │                                                         │
     *   │   Trigger MUST be focusable via keyboard                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger can receive keyboard focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new MultiSelectPage(page);
      await ui.goto();

      await ui.trigger.focus();
      await expect(ui.trigger).toBeFocused();
    });
  });
});
