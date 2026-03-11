import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * LABEL COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <label for="terms">                                                   │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  "Accept terms and conditions"                                  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │      │                                                                  │
 * │      └───── Links to input with id="terms"                              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .text-sm        ← 14px font size                           │       │
 * │   │  .font-medium    ← 500 font weight                          │       │
 * │   │  .flex           ← Flex display                             │       │
 * │   │  .items-center   ← Vertical centering                       │       │
 * │   │  .select-none    ← Non-selectable text                      │       │
 * │   │  .leading-none   ← Line height: 1                           │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * USAGE WITH FORM CONTROLS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────┐  "Accept terms and conditions"                                │
 * │   │  ✓  │   ← Checkbox (id="terms")                                     │
 * │   └─────┘                                                               │
 * │      ↑                                                                  │
 * │   <label for="terms"> points to this checkbox                           │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class LabelPage extends BasePage {
  protected readonly componentName = "label";

  // Label element
  readonly label: Locator;

  constructor(page: Page) {
    super(page);

    // Main label - scoped within preview
    this.label = this.preview.locator("label").first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Label Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Label Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label>  ◄─── Is this element visible on the page?   │
     *   │     "Accept terms and conditions"                       │
     *   │   </label>                                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label component renders and is visible in DOM
     */
    test("should have label element", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toBeVisible();
    });

    /**
     * TEST: Label HTML Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Element Type Check:                                   │
     *   │                                                         │
     *   │   ✓ <label>...</label>    (correct semantic element)    │
     *   │   ✗ <div>...</div>        (wrong - not semantic)        │
     *   │   ✗ <span>...</span>      (wrong - not semantic)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Uses semantic <label> HTML element for accessibility
     */
    test("should be a label HTML element", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      const tagName = await ui.label.evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("label");
    });

    /**
     * TEST: Label Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  "Accept terms and conditions"                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │              ▲                                          │
     *   │              └── Expected text content                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label displays the expected text content
     */
    test("should have text content", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toContainText("Accept terms and conditions");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Text Size Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Font Size Comparison:                                 │
     *   │                                                         │
     *   │   text-xs:  Accept terms...     (12px - too small)      │
     *   │   text-sm:  Accept terms...     (14px - expected) ✓     │
     *   │   text-base: Accept terms...    (16px - too large)      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label uses text-sm class for 14px font size
     */
    test("should have text-sm class", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Font Weight Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Font Weight Comparison:                               │
     *   │                                                         │
     *   │   font-normal:  Accept terms...   (400 - too light)     │
     *   │   font-medium:  Accept terms...   (500 - expected) ✓    │
     *   │   font-bold:    Accept terms...   (700 - too heavy)     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label uses font-medium class for 500 weight
     */
    test("should have font-medium class", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/font-medium/);
    });

    /**
     * TEST: Flex Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   display: flex                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │ [child 1] [child 2] [child 3] → flex row      │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   │   Enables flexible layout for label content             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label uses flex display for layout control
     */
    test("should have flex display", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/flex/);
    });

    /**
     * TEST: Items Center Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   items-center (vertical alignment):                    │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────┐               │
     *   │   │          ┌─────┐                    │               │
     *   │   │  ──────  │ ✓  │  "Label text"  ──────  ◄─ centered │
     *   │   │          └─────┘                    │               │
     *   │   └─────────────────────────────────────┘               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label content is vertically centered
     */
    test("should have items-center", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/items-center/);
    });

    /**
     * TEST: Non-Selectable Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User Selection Behavior:                              │
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  "Accept terms and conditions"                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │         ▲                                               │
     *   │         └── Cannot be selected with cursor              │
     *   │             (select-none prevents text highlighting)    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label text cannot be selected by user
     */
    test("should have select-none for non-selectable text", async ({
      page,
    }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/select-none/);
    });

    /**
     * TEST: Line Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Line Height Comparison:                               │
     *   │                                                         │
     *   │   leading-normal:  ┌──────────────────┐                 │
     *   │                    │                  │ (extra space)   │
     *   │                    │  Label text      │                 │
     *   │                    │                  │                 │
     *   │                    └──────────────────┘                 │
     *   │                                                         │
     *   │   leading-none:    ┌──────────────────┐                 │
     *   │                    │  Label text      │ (tight fit) ✓   │
     *   │                    └──────────────────┘                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label uses leading-none for line-height: 1
     */
    test("should have leading-none for line height", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      await expect(ui.label).toHaveClass(/leading-none/);
    });
  });

  test.describe("For Attribute", () => {
    /**
     * TEST: For Attribute Linking
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label for="terms">           <input id="terms">      │
     *   │   ┌─────────────────────┐       ┌─────┐                 │
     *   │   │  Accept terms...    │──────▶│  ✓  │                 │
     *   │   └─────────────────────┘       └─────┘                 │
     *   │           │                         ▲                   │
     *   │           └────── for="terms" ──────┘                   │
     *   │                                                         │
     *   │   Clicking label activates the linked input             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label has for="terms" attribute for accessibility
     */
    test("should have for attribute linking to input", async ({ page }) => {
      const ui = new LabelPage(page);
      await ui.goto();

      const forAttr = await ui.label.getAttribute("for");
      expect(forAttr).toBe("terms");
    });
  });
});
