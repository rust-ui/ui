import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * RADIO-BUTTON COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="RadioButtonGroup">                                    │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   <label data-name="RadioButton">                               │   │
 * │   │   ┌─────────────────────────────────────────────────────────┐   │   │
 * │   │   │   ◉  Women                                              │   │   │
 * │   │   │   ↑    ↑                                                │   │   │
 * │   │   │ input  RadioButtonText                                  │   │   │
 * │   │   └─────────────────────────────────────────────────────────┘   │   │
 * │   │                                                                 │   │
 * │   │   ┌───────────────┐  ┌───────────────┐  ┌───────────────┐       │   │
 * │   │   │  ◉  Women     │  │  ○  Men       │  │  ○  Divided   │       │   │
 * │   │   └───────────────┘  └───────────────┘  └───────────────┘       │   │
 * │   │        ↑                  ↑                   ↑                 │   │
 * │   │     checked          unchecked            unchecked             │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SELECTION BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Only one can be selected at a time (same name attribute):             │
 * │                                                                         │
 * │   Before click on "Men":          After click on "Men":                 │
 * │   ┌───────┐ ┌───────┐ ┌───────┐   ┌───────┐ ┌───────┐ ┌───────┐        │
 * │   │ ◉ W   │ │ ○ M   │ │ ○ D   │   │ ○ W   │ │ ◉ M   │ │ ○ D   │        │
 * │   └───────┘ └───────┘ └───────┘   └───────┘ └───────┘ └───────┘        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Unchecked:     Checked:        Focused:        Disabled:              │
 * │   ┌───────┐      ┌───────┐       ┌═══════┐       ┌───────┐              │
 * │   │  ○    │      │  ◉    │       ║  ○    ║       │  ○    │              │
 * │   └───────┘      └───────┘       └═══════┘       └───────┘              │
 * │                                   focus ring     opacity: 0.5           │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class RadioButtonPage extends BasePage {
  protected readonly componentName = "radio-button";

  // Radio button group elements
  readonly radioButtonGroup: Locator;
  readonly radioButtons: Locator;
  readonly womenRadio: Locator;
  readonly menRadio: Locator;
  readonly dividedRadio: Locator;

  constructor(page: Page) {
    super(page);

    // Main radio button group (scoped within preview)
    this.radioButtonGroup = this.preview
      .locator('[data-name="RadioButtonGroup"]')
      .first();

    // Radio buttons
    this.radioButtons = this.radioButtonGroup.locator(
      '[data-name="RadioButton"]'
    );
    this.womenRadio = this.radioButtons.nth(0);
    this.menRadio = this.radioButtons.nth(1);
    this.dividedRadio = this.radioButtons.nth(2);
  }

  getRadioInput(radioButton: Locator): Locator {
    return radioButton.locator('input[type="radio"]');
  }

  getRadioText(radioButton: Locator): Locator {
    return radioButton.locator('[data-name="RadioButtonText"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("RadioButton Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: RadioButtonGroup Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="RadioButtonGroup"]                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The RadioButtonGroup container is visible on the page
     */
    test("should have RadioButtonGroup container", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await expect(ui.radioButtonGroup).toBeVisible();
    });

    /**
     * TEST: RadioButtonGroup Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="RadioButtonGroup">  <-- checking this │
     *   │       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name attribute equals "RadioButtonGroup"
     */
    test("should have RadioButtonGroup data-name attribute", async ({
      page,
    }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await expect(ui.radioButtonGroup).toHaveAttribute(
        "data-name",
        "RadioButtonGroup"
      );
    });

    /**
     * TEST: Three Radio Buttons Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButtonGroup                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │        1              2              3            │  │
     *   │  │   count = 3  <-- checking this                    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 3 RadioButton elements exist in the group
     */
    test("should have three radio buttons", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const count = await ui.radioButtons.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: Radio Button Input Elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButton anatomy:                                   │
     *   │                                                         │
     *   │   <label data-name="RadioButton">                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  ┌───┐                                            │ │
     *   │   │  │ ◉ │  Women                                     │ │
     *   │   │  └───┘                                            │ │
     *   │   │    ^                                              │ │
     *   │   │  <input type="radio">  <-- checking this          │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each RadioButton contains an input element
     */
    test("radio buttons should have input elements", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const input = ui.getRadioInput(ui.womenRadio);
      await expect(input).toBeAttached();
    });
  });

  test.describe("Labels", () => {
    /**
     * TEST: First Radio "Women" Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButtonGroup                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │        ^                                          │  │
     *   │  │   checking text = "Women"                         │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First radio button displays "Women" text
     */
    test("first radio should have 'Women' label", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const text = ui.getRadioText(ui.womenRadio);
      await expect(text).toHaveText("Women");
    });

    /**
     * TEST: Second Radio "Men" Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButtonGroup                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │                       ^                           │  │
     *   │  │               checking text = "Men"               │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second radio button displays "Men" text
     */
    test("second radio should have 'Men' label", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const text = ui.getRadioText(ui.menRadio);
      await expect(text).toHaveText("Men");
    });

    /**
     * TEST: Third Radio "Divided" Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButtonGroup                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │                                      ^            │  │
     *   │  │                         checking text = "Divided" │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third radio button displays "Divided" text
     */
    test("third radio should have 'Divided' label", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const text = ui.getRadioText(ui.dividedRadio);
      await expect(text).toHaveText("Divided");
    });
  });

  test.describe("Initial State", () => {
    /**
     * TEST: First Radio Default Checked
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Initial state (page load):                             │
     *   │                                                         │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │     ^                                                   │
     *   │   checked = true  <-- default selection                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First radio is checked by default on page load
     */
    test("first radio should be checked by default", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const input = ui.getRadioInput(ui.womenRadio);
      await expect(input).toBeChecked();
    });

    /**
     * TEST: Second Radio Not Checked Initially
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Initial state (page load):                             │
     *   │                                                         │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                       ^                                 │
     *   │               checked = false                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second radio is NOT checked by default
     */
    test("second radio should not be checked", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const input = ui.getRadioInput(ui.menRadio);
      await expect(input).not.toBeChecked();
    });

    /**
     * TEST: Third Radio Not Checked Initially
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Initial state (page load):                             │
     *   │                                                         │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                                      ^                  │
     *   │                              checked = false            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third radio is NOT checked by default
     */
    test("third radio should not be checked", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const input = ui.getRadioInput(ui.dividedRadio);
      await expect(input).not.toBeChecked();
    });
  });

  test.describe("Selection Behavior", () => {
    /**
     * TEST: Click Second Radio Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before click:                                          │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                       │                                 │
     *   │                       │ click()                         │
     *   │                       ▼                                 │
     *   │  After click:                                           │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ? Women   │  │ ◉ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                       ^                                 │
     *   │                 now checked = true                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking "Men" radio selects it (checked = true)
     */
    test("clicking second radio should select it", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await ui.menRadio.click();

      const input = ui.getRadioInput(ui.menRadio);
      await expect(input).toBeChecked();
    });

    /**
     * TEST: Select Second Deselects First
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before click:                                          │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                       │                                 │
     *   │                       │ click()                         │
     *   │                       ▼                                 │
     *   │  After click:                                           │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ○ Women   │  │ ◉ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │        ^                                                │
     *   │   now checked = false (auto-deselected)                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selecting "Men" automatically deselects "Women"
     */
    test("selecting second radio should deselect first", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await ui.menRadio.click();

      const firstInput = ui.getRadioInput(ui.womenRadio);
      await expect(firstInput).not.toBeChecked();
    });

    /**
     * TEST: Click Third Radio Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before click:                                          │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                                      │                  │
     *   │                                      │ click()          │
     *   │                                      ▼                  │
     *   │  After click:                                           │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ? Women   │  │ ? Men     │  │ ◉ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                                      ^                  │
     *   │                               now checked = true        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking "Divided" radio selects it
     */
    test("clicking third radio should select it", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await ui.dividedRadio.click();

      const input = ui.getRadioInput(ui.dividedRadio);
      await expect(input).toBeChecked();
    });

    /**
     * TEST: Only One Radio Selected at a Time
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Radio button mutual exclusivity:                       │
     *   │                                                         │
     *   │  Before click on "Divided":                             │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ◉ Women   │  │ ○ Men     │  │ ○ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │                                                         │
     *   │  After click on "Divided":                              │
     *   │   ┌───────────┐  ┌───────────┐  ┌───────────┐           │
     *   │   │ ○ Women   │  │ ○ Men     │  │ ◉ Divided │           │
     *   │   └───────────┘  └───────────┘  └───────────┘           │
     *   │     false          false          true                  │
     *   │                                    ^                    │
     *   │                            only ONE checked             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Only one radio can be selected at any time
     */
    test("only one radio can be selected at a time", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await ui.dividedRadio.click();

      const firstInput = ui.getRadioInput(ui.womenRadio);
      const secondInput = ui.getRadioInput(ui.menRadio);
      const thirdInput = ui.getRadioInput(ui.dividedRadio);

      await expect(firstInput).not.toBeChecked();
      await expect(secondInput).not.toBeChecked();
      await expect(thirdInput).toBeChecked();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: RadioButtonGroup Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButtonGroup: display: flex                        │
     *   │  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───────────┐  ┌───────────┐  ┌───────────┐     │  │
     *   │  │   │ ◉ Women   │→ │ ○ Men     │→ │ ○ Divided │     │  │
     *   │  │   └───────────┘  └───────────┘  └───────────┘     │  │
     *   │  │        └──────────────┴──────────────┘            │  │
     *   │  │              flex row layout                      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: RadioButtonGroup has "flex" class for layout
     */
    test("radio button group should have flex layout", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await expect(ui.radioButtonGroup).toHaveClass(/flex/);
    });

    /**
     * TEST: RadioButton is Label Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButton element type:                              │
     *   │                                                         │
     *   │   <label data-name="RadioButton">  <-- checking tag     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  ┌───┐                                            │ │
     *   │   │  │ ◉ │  Women                                     │ │
     *   │   │  └───┘                                            │ │
     *   │   │    ^                                              │ │
     *   │   │   input is clickable via label                    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   </label>                                              │
     *   │                                                         │
     *   │   tagName === "label" (for accessibility)               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: RadioButton uses <label> element for clickability
     */
    test("radio button should be a label element", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const tagName = await ui.womenRadio.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("label");
    });

    /**
     * TEST: RadioButton Cursor Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadioButton cursor style:                              │
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   ┌───────────┐                                   │ │
     *   │   │   │ ◉ Women   │  ← cursor: pointer                │ │
     *   │   │   └───────────┘      (hand icon on hover)         │ │
     *   │   │                       👆                          │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   cursor-pointer indicates clickable element            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: RadioButton has cursor-pointer for UX affordance
     */
    test("radio button should have cursor-pointer", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      await expect(ui.womenRadio).toHaveClass(/cursor-pointer/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Radio Input Type Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Input element type:                                    │
     *   │                                                         │
     *   │   <label>                                               │
     *   │     <input type="radio" />  <-- checking this           │
     *   │            ^^^^^^^^^^^^                                 │
     *   │     <span>Women</span>                                  │
     *   │   </label>                                              │
     *   │                                                         │
     *   │   type="radio" required for:                            │
     *   │   - Screen readers to announce as radio button          │
     *   │   - Browser native radio behavior                       │
     *   │   - Form submission handling                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input elements have type="radio" for semantics
     */
    test("radio inputs should have type=radio", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const input = ui.getRadioInput(ui.womenRadio);
      await expect(input).toHaveAttribute("type", "radio");
    });

    /**
     * TEST: Radio Inputs Share Same Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Name attribute grouping:                               │
     *   │                                                         │
     *   │   <input type="radio" name="gender" /> Women            │
     *   │   <input type="radio" name="gender" /> Men              │
     *   │   <input type="radio" name="gender" /> Divided          │
     *   │                        ^^^^^^^                          │
     *   │                    same name                            │
     *   │                                                         │
     *   │   Same name attribute:                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ - Groups radios together                        │   │
     *   │   │ - Enables mutual exclusivity                    │   │
     *   │   │ - Browser handles single-selection              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All radio inputs share the same name for grouping
     */
    test("radio inputs should share same name attribute", async ({ page }) => {
      const ui = new RadioButtonPage(page);
      await ui.goto();

      const firstName = await ui
        .getRadioInput(ui.womenRadio)
        .getAttribute("name");
      const secondName = await ui
        .getRadioInput(ui.menRadio)
        .getAttribute("name");
      const thirdName = await ui
        .getRadioInput(ui.dividedRadio)
        .getAttribute("name");

      expect(firstName).toBe(secondName);
      expect(secondName).toBe(thirdName);
    });
  });
});
