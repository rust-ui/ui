import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * RADIO-BUTTON-GROUP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   RadioButtonGroup                                                      │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │   │
 * │   │   │   Women     │ │    Men      │ │  Divided    │               │   │
 * │   │   │  (selected) │ │             │ │             │               │   │
 * │   │   └─────────────┘ └─────────────┘ └─────────────┘               │   │
 * │   │         ↑               ↑               ↑                       │   │
 * │   │    RadioButton    RadioButton    RadioButton                    │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SELECTION STATE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Unselected:                    Selected:                              │
 * │   ┌─────────────────┐            ┌─────────────────┐                    │
 * │   │     Women       │            │     Women       │                    │
 * │   │   (default)     │            │   ████████████  │  ← highlighted     │
 * │   └─────────────────┘            └─────────────────┘    background      │
 * │                                                                         │
 * │   CSS when checked:                                                     │
 * │   • box-shadow: 0 0 0 0.0625em var(--primary)                           │
 * │   • background-color: var(--secondary)                                  │
 * │   • color: var(--primary)                                               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * RADIO BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Only ONE option can be selected at a time:                            │
 * │                                                                         │
 * │   Initial:    [Women ✓] [Men] [Divided]                                 │
 * │   Click Men:  [Women] [Men ✓] [Divided]                                 │
 * │   Click Div:  [Women] [Men] [Divided ✓]                                 │
 * │                                                                         │
 * │   Hidden radio input provides native form behavior                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HIDDEN INPUT STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   .radio__button[type="radio"] {                                        │
 * │     clip: rect(0 0 0 0);                                                │
 * │     clip-path: inset(100%);                                             │
 * │     height: 1px;                                                        │
 * │     width: 1px;                                                         │
 * │     position: absolute;                                                 │
 * │   }                                                                     │
 * │                                                                         │
 * │   Visually hidden but accessible to screen readers                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class RadioButtonGroupPage extends BasePage {
  protected readonly componentName = "radio-button-group";

  // RadioButtonGroup elements
  readonly radioInputs: Locator;
  readonly radioTexts: Locator;

  // Individual buttons (first() to handle multiple demos)
  readonly womenButton: Locator;
  readonly menButton: Locator;
  readonly dividedButton: Locator;

  constructor(page: Page) {
    super(page);

    // All locators scoped within preview
    this.radioInputs = this.preview.locator('input[type="radio"]');
    this.radioTexts = this.preview.locator('[data-name="RadioButtonText"]');

    this.womenButton = this.preview.getByText("Women", { exact: true }).first();
    this.menButton = this.preview.getByText("Men", { exact: true }).first();
    this.dividedButton = this.preview
      .getByText("Divided", { exact: true })
      .first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("RadioButtonGroup Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Radio Inputs Existence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   RadioButtonGroup:                                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ○ input[type="radio"]  (hidden)                │   │
     *   │   │  ○ input[type="radio"]  (hidden)                │   │
     *   │   │  ○ input[type="radio"]  (hidden)                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Count >= 3 radio inputs                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least 3 radio input elements exist
     */
    test("should have radio inputs", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const count = await ui.radioInputs.count();
      expect(count).toBeGreaterThanOrEqual(3);
    });

    /**
     * TEST: Radio Text Labels Existence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   RadioButtonGroup visible labels:                      │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │  (text)     │ │   (text)    │ │   (text)    │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │         ↑               ↑               ↑               │
     *   │    data-name="RadioButtonText"                          │
     *   │                                                         │
     *   │   Count >= 3 text labels                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least 3 radio text labels exist
     */
    test("should have radio text labels", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const count = await ui.radioTexts.count();
      expect(count).toBeGreaterThanOrEqual(3);
    });
  });

  test.describe("Options", () => {
    /**
     * TEST: Women Option Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │  ═══════    │ │             │ │             │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │         ↑                                               │
     *   │    MUST BE VISIBLE                                      │
     *   │    text="Women" (exact)                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Women option button is visible
     */
    test("should have Women option", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      await expect(ui.womenButton).toBeVisible();
    });

    /**
     * TEST: Men Option Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │             │ │  ═══════    │ │             │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │                         ↑                               │
     *   │                    MUST BE VISIBLE                      │
     *   │                    text="Men" (exact)                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Men option button is visible
     */
    test("should have Men option", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      await expect(ui.menButton).toBeVisible();
    });

    /**
     * TEST: Divided Option Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │             │ │             │ │  ═══════    │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │                                         ↑               │
     *   │                                    MUST BE VISIBLE      │
     *   │                                    text="Divided"       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Divided option button is visible
     */
    test("should have Divided option", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      await expect(ui.dividedButton).toBeVisible();
    });
  });

  test.describe("Initial State", () => {
    /**
     * TEST: Women Default Selection
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   On page load:                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │  ●══════════│ │  ○          │ │  ○          │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │         ↑                                               │
     *   │    MUST be checked                                      │
     *   │    (first input is selected by default)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First radio (Women) is checked by default
     */
    test("Women should be checked by default", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const firstInput = ui.radioInputs.first();
      await expect(firstInput).toBeChecked();
    });

    /**
     * TEST: Men Not Initially Selected
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   On page load:                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │  ●          │ │  ○          │ │  ○          │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │                         ↑                               │
     *   │                    MUST NOT be checked                  │
     *   │                    (second input)                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second radio (Men) is not checked initially
     */
    test("Men should not be checked initially", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const secondInput = ui.radioInputs.nth(1);
      await expect(secondInput).not.toBeChecked();
    });

    /**
     * TEST: Divided Not Initially Selected
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   On page load:                                         │
     *   │   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
     *   │   │   Women     │ │    Men      │ │  Divided    │       │
     *   │   │  ●          │ │  ○          │ │  ○          │       │
     *   │   └─────────────┘ └─────────────┘ └─────────────┘       │
     *   │                                         ↑               │
     *   │                                    MUST NOT be checked  │
     *   │                                    (third input)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third radio (Divided) is not checked initially
     */
    test("Divided should not be checked initially", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const thirdInput = ui.radioInputs.nth(2);
      await expect(thirdInput).not.toBeChecked();
    });
  });

  test.describe("Radio Inputs", () => {
    /**
     * TEST: Input Type Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Hidden radio input element:                           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <input type="radio" ...>                         │ │
     *   │   │              ↑                                    │ │
     *   │   │         MUST be "radio"                           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Native radio input for form behavior                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input element has type="radio" attribute
     */
    test("inputs should have type radio", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const firstInput = ui.radioInputs.first();
      await expect(firstInput).toHaveAttribute("type", "radio");
    });

    /**
     * TEST: Radio Button CSS Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Input styling class:                                  │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <input class="radio__button ...">                │ │
     *   │   │                    ↑                              │ │
     *   │   │         MUST contain "radio__button"              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   CSS class for visually hidden but accessible radio    │
     *   │   clip: rect(0 0 0 0); position: absolute;              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has radio__button class for styling
     */
    test("inputs should have radio__button class", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      const firstInput = ui.radioInputs.first();
      await expect(firstInput).toHaveClass(/radio__button/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Keyboard Navigation Focus
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus flow:                                  │
     *   │                                                         │
     *   │   [Tab] ──► ┌─────────────┐                             │
     *   │             │   Women     │                             │
     *   │             │  ●══════════│  ← Focus ring visible       │
     *   │             └─────────────┘                             │
     *   │                                                         │
     *   │   First radio input MUST be focusable                   │
     *   │   (required for screen reader accessibility)            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Radio inputs can receive keyboard focus
     */
    test("radio buttons should be keyboard navigable", async ({ page }) => {
      const ui = new RadioButtonGroupPage(page);
      await ui.goto();

      // Focus first radio and use arrow keys
      await ui.radioInputs.first().focus();
      await expect(ui.radioInputs.first()).toBeFocused();
    });
  });
});
