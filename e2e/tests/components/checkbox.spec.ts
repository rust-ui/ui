import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * CHECKBOX COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <button role="checkbox" data-name="Checkbox">                         │
 * │   ┌───────────────────────────────────────────────────────────────────┐ │
 * │   │  ┌─────┐                                                          │ │
 * │   │  │  ✓  │  "Item 1"  ← Label with for="item1"                      │ │
 * │   │  └─────┘                                                          │ │
 * │   │    ↑                                                              │ │
 * │   │  Checkbox button (size-4, rounded-[4px])                          │ │
 * │   │  Contains CheckboxIndicator with check icon when checked          │ │
 * │   └───────────────────────────────────────────────────────────────────┘ │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   UNCHECKED:                   CHECKED:                                 │
 * │   ┌─────┐                      ┌─────┐                                  │
 * │   │     │  data-state=         │  ✓  │  data-state="checked"            │
 * │   │     │  "unchecked"         │     │  aria-checked="true"             │
 * │   └─────┘  aria-checked=       └─────┘  bg-primary                      │
 * │            "false"                      text-primary-foreground         │
 * │            border-input                                                 │
 * │                                                                         │
 * │   DISABLED:                                                             │
 * │   ┌─────┐                                                               │
 * │   │     │  opacity-50                                                   │
 * │   │     │  cursor-not-allowed                                           │
 * │   └─────┘  pointer-events: none                                         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CHECK INDICATOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   When data-state="checked":                                            │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  [data-name="CheckboxIndicator"]                            │       │
 * │   │     └── <svg> Check icon (visible)                          │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * │   When data-state="unchecked":                                          │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  CheckboxIndicator not visible (no icon)                    │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class CheckboxPage extends BasePage {
  protected readonly componentName = "checkbox";

  // Demo checkboxes
  readonly item1Checkbox: Locator;
  readonly item2Checkbox: Locator;
  readonly disabled1Checkbox: Locator;
  readonly disabled2Checkbox: Locator;

  // Labels
  readonly item1Label: Locator;
  readonly item2Label: Locator;
  readonly disabled1Label: Locator;
  readonly disabled2Label: Locator;

  // All checkboxes
  readonly allCheckboxes: Locator;

  constructor(page: Page) {
    super(page);

    // Checkboxes by id - scoped within preview
    this.item1Checkbox = this.preview.locator("#item1");
    this.item2Checkbox = this.preview.locator("#item2");
    this.disabled1Checkbox = this.preview.locator("#disabled1");
    this.disabled2Checkbox = this.preview.locator("#disabled2");

    // Labels - scoped within preview
    this.item1Label = this.preview.locator('label[for="item1"]');
    this.item2Label = this.preview.locator('label[for="item2"]');
    this.disabled1Label = this.preview.locator('label[for="disabled1"]');
    this.disabled2Label = this.preview.locator('label[for="disabled2"]');

    // All checkboxes - scoped within preview
    this.allCheckboxes = this.preview.locator('[data-name="Checkbox"]');
  }

  async getCheckedState(checkbox: Locator): Promise<string | null> {
    return checkbox.getAttribute("data-state");
  }

  async isAriaChecked(checkbox: Locator): Promise<boolean> {
    const ariaChecked = await checkbox.getAttribute("aria-checked");
    return ariaChecked === "true";
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Checkbox Page", () => {
  /**
   * STRUCTURE TESTS - Verify checkbox elements
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <button role="checkbox" type="button">                        │
   * │   ┌─────┐                                                       │
   * │   │     │  ← data-name="Checkbox"                               │
   * │   └─────┘    aria-label="Checkbox"                              │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Checkbox Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Demo page checkbox count:                             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   [✓] Item 1          ← checkbox #1             │   │
     *   │   │   [ ] Item 2          ← checkbox #2             │   │
     *   │   │   [✓] Disabled        ← checkbox #3 (disabled)  │   │
     *   │   │   [ ] Disabled        ← checkbox #4 (disabled)  │   │
     *   │   │                                                 │   │
     *   │   │   count([data-name="Checkbox"]) === 4           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page renders exactly 4 checkbox components
     */
    test("should have four checkboxes", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      const count = await ui.allCheckboxes.count();
      expect(count).toBe(4);
    });

    /**
     * TEST: Checkbox Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Component identification attribute:                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button data-name="Checkbox" ...>             │   │
     *   │   │              ↑                                  │   │
     *   │   │       getAttribute("data-name")                 │   │
     *   │   │              === "Checkbox"                     │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has correct data-name attribute for testing
     */
    test("should have Checkbox data-name attribute", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveAttribute("data-name", "Checkbox");
    });

    /**
     * TEST: Checkbox HTML Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checkbox element tag verification:                    │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button role="checkbox">                      │   │
     *   │   │      ↑                                          │   │
     *   │   │   tagName.toLowerCase() === "button"            │   │
     *   │   │                                                 │   │
     *   │   │   Note: Uses button (not input) for better      │   │
     *   │   │   accessibility and custom styling              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox renders as button element
     */
    test("checkbox should be a button element", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      const tagName = await ui.item1Checkbox.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });

    /**
     * TEST: Checkbox ARIA Role
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA role attribute:                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button role="checkbox">                      │   │
     *   │   │                  ↑                              │   │
     *   │   │         getAttribute("role")                    │   │
     *   │   │              === "checkbox"                     │   │
     *   │   │                                                 │   │
     *   │   │   Screen readers announce as checkbox           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has correct ARIA role for accessibility
     */
    test("checkbox should have role checkbox", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveAttribute("role", "checkbox");
    });

    /**
     * TEST: Checkbox Button Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Button type attribute:                                │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button type="button">                        │   │
     *   │   │                  ↑                              │   │
     *   │   │         getAttribute("type")                    │   │
     *   │   │              === "button"                       │   │
     *   │   │                                                 │   │
     *   │   │   Prevents form submission when in form         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has type="button" to prevent form submit
     */
    test("checkbox should have type button", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveAttribute("type", "button");
    });
  });

  test.describe("Initial States", () => {
    /**
     * TEST: Item1 Default Checked State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Item1 checkbox initial state:                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │ ✓ │ Item 1      ← Initial state             │   │
     *   │   │   └───┘                                         │   │
     *   │   │     ↓                                           │   │
     *   │   │   data-state="checked"                          │   │
     *   │   │   aria-checked="true"                           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item1 starts in checked state with correct attributes
     */
    test("item1 should be checked by default", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveAttribute("data-state", "checked");
      expect(await ui.isAriaChecked(ui.item1Checkbox)).toBe(true);
    });

    /**
     * TEST: Item2 Default Unchecked State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Item2 checkbox initial state:                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │   │ Item 2      ← Initial state             │   │
     *   │   │   └───┘                                         │   │
     *   │   │     ↓                                           │   │
     *   │   │   data-state="unchecked"                        │   │
     *   │   │   aria-checked="false"                          │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item2 starts in unchecked state with correct attributes
     */
    test("item2 should be unchecked by default", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item2Checkbox).toHaveAttribute("data-state", "unchecked");
      expect(await ui.isAriaChecked(ui.item2Checkbox)).toBe(false);
    });

    /**
     * TEST: Disabled1 Checked and Disabled State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled1 checkbox state (checked + disabled):        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │ ✓ │ Disabled    ← Checked but disabled      │   │
     *   │   │   └───┘  (grayed)                               │   │
     *   │   │     ↓                                           │   │
     *   │   │   data-state="checked"                          │   │
     *   │   │   disabled (isDisabled() === true)              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled1 is both checked and disabled
     */
    test("disabled1 should be checked and disabled", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.disabled1Checkbox).toHaveAttribute(
        "data-state",
        "checked"
      );
      await expect(ui.disabled1Checkbox).toBeDisabled();
    });

    /**
     * TEST: Disabled2 Unchecked and Disabled State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled2 checkbox state (unchecked + disabled):      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │   │ Disabled    ← Unchecked and disabled    │   │
     *   │   │   └───┘  (grayed)                               │   │
     *   │   │     ↓                                           │   │
     *   │   │   data-state="unchecked"                        │   │
     *   │   │   disabled (isDisabled() === true)              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled2 is both unchecked and disabled
     */
    test("disabled2 should be unchecked and disabled", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.disabled2Checkbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );
      await expect(ui.disabled2Checkbox).toBeDisabled();
    });
  });

  test.describe("Toggle Behavior", () => {
    /**
     * TEST: Click Toggle Behavior
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Toggle cycle (checked -> unchecked -> checked):       │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐         ┌───┐         ┌───┐             │   │
     *   │   │   │ ✓ │ ──CLICK──→│   │ ──CLICK──→│ ✓ │             │   │
     *   │   │   └───┘         └───┘         └───┘             │   │
     *   │   │   checked      unchecked      checked           │   │
     *   │   │                                                 │   │
     *   │   │   data-state toggles between values             │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking checkbox toggles between checked/unchecked
     */
    test("clicking should toggle checked state", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Wait for WASM hydration before clicking
      await page.waitForLoadState("networkidle");

      // item1 starts checked, click to uncheck
      await ui.item1Checkbox.click();
      await expect(ui.item1Checkbox).toHaveAttribute("data-state", "unchecked");

      // Click again to check
      await ui.item1Checkbox.click();
      await expect(ui.item1Checkbox).toHaveAttribute("data-state", "checked");
    });

    /**
     * TEST: Item2 Can Be Checked
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Item2 state transition:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Initial:            After click:              │   │
     *   │   │   ┌───┐               ┌───┐                     │   │
     *   │   │   │   │ Item 2  ────→ │ ✓ │ Item 2              │   │
     *   │   │   └───┘               └───┘                     │   │
     *   │   │   "unchecked"         "checked"                 │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Unchecked item2 can transition to checked state
     */
    test("item2 can be checked", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Wait for WASM hydration before clicking
      await page.waitForLoadState("networkidle");

      // item2 starts unchecked
      await expect(ui.item2Checkbox).toHaveAttribute("data-state", "unchecked");

      // Click to check
      await ui.item2Checkbox.click();
      await expect(ui.item2Checkbox).toHaveAttribute("data-state", "checked");
    });

    /**
     * TEST: ARIA-Checked Updates on Toggle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   aria-checked attribute synchronization:               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐  aria-checked="true"                    │   │
     *   │   │   │ ✓ │        │                                │   │
     *   │   │   └───┘        ▼  CLICK                         │   │
     *   │   │   ┌───┐  aria-checked="false"                   │   │
     *   │   │   │   │        │                                │   │
     *   │   │   └───┘        ▼  CLICK                         │   │
     *   │   │   ┌───┐  aria-checked="true"                    │   │
     *   │   │   │ ✓ │                                         │   │
     *   │   │   └───┘                                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: aria-checked attribute updates in sync with visual state
     */
    test("aria-checked should update on toggle", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Wait for WASM hydration before clicking
      await page.waitForLoadState("networkidle");

      expect(await ui.isAriaChecked(ui.item1Checkbox)).toBe(true);

      await ui.item1Checkbox.click();
      expect(await ui.isAriaChecked(ui.item1Checkbox)).toBe(false);

      await ui.item1Checkbox.click();
      expect(await ui.isAriaChecked(ui.item1Checkbox)).toBe(true);
    });

    /**
     * TEST: Multiple Toggle Cycles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Repeated toggle stability (5 iterations):             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   i=0: [ ] ──→ [✓]  (checked)                   │   │
     *   │   │   i=1: [✓] ──→ [ ]  (unchecked)                 │   │
     *   │   │   i=2: [ ] ──→ [✓]  (checked)                   │   │
     *   │   │   i=3: [✓] ──→ [ ]  (unchecked)                 │   │
     *   │   │   i=4: [ ] ──→ [✓]  (checked)                   │   │
     *   │   │                                                 │   │
     *   │   │   State alternates correctly on each click      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox state remains consistent through multiple toggles
     */
    test("should toggle multiple times", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Wait for WASM hydration before clicking
      await page.waitForLoadState("networkidle");

      for (let i = 0; i < 5; i++) {
        await ui.item2Checkbox.click();
        const expectedState = i % 2 === 0 ? "checked" : "unchecked";
        await expect(ui.item2Checkbox).toHaveAttribute(
          "data-state",
          expectedState
        );
      }
    });
  });

  test.describe("Disabled State", () => {
    /**
     * TEST: Disabled Checked Checkbox No Toggle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled1 click rejection:                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Before click:         After click (force):    │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │ ✓ │ Disabled  ──X──→│ ✓ │ Disabled          │   │
     *   │   │   └───┘                 └───┘                   │   │
     *   │   │   (disabled)            (still checked!)        │   │
     *   │   │                                                 │   │
     *   │   │   Click is ignored when disabled                │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled checkbox ignores click events
     */
    test("disabled checkbox should not toggle on click", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // disabled1 is checked
      await expect(ui.disabled1Checkbox).toHaveAttribute(
        "data-state",
        "checked"
      );

      // Try to click (should not change)
      await ui.disabled1Checkbox.click({ force: true });
      await expect(ui.disabled1Checkbox).toHaveAttribute(
        "data-state",
        "checked"
      );
    });

    /**
     * TEST: Disabled Unchecked Checkbox No Toggle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled2 click rejection:                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Before click:         After click (force):    │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │   │ Disabled  ──X──→│   │ Disabled          │   │
     *   │   │   └───┘                 └───┘                   │   │
     *   │   │   (disabled)            (still unchecked!)      │   │
     *   │   │                                                 │   │
     *   │   │   Click is ignored when disabled                │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled unchecked checkbox ignores click events
     */
    test("disabled unchecked should not toggle", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.disabled2Checkbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );

      await ui.disabled2Checkbox.click({ force: true });
      await expect(ui.disabled2Checkbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );
    });

    /**
     * TEST: Disabled Checkbox Opacity
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled visual styling:                              │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Normal:               Disabled:               │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │ ✓ │ 100%  vs        │ ✓ │ 50%               │   │
     *   │   │   └───┘ opacity         └───┘ opacity           │   │
     *   │   │                           ↑                     │   │
     *   │   │                    .disabled:opacity-50         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled checkbox has 50% opacity class
     */
    test("disabled checkbox should have opacity-50 class", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.disabled1Checkbox).toHaveClass(/disabled:opacity-50/);
    });

    /**
     * TEST: Disabled Checkbox Cursor
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled cursor styling:                              │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Normal:               Disabled:               │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │ ✓ │ pointer   vs    │ ✓ │ not-allowed       │   │
     *   │   │   └───┘   ↓             └───┘   ↓               │   │
     *   │   │          👆                    🚫               │   │
     *   │   │                                 ↑               │   │
     *   │   │                 .disabled:cursor-not-allowed    │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled checkbox shows not-allowed cursor
     */
    test("disabled checkbox should have cursor-not-allowed", async ({
      page,
    }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.disabled1Checkbox).toHaveClass(
        /disabled:cursor-not-allowed/
      );
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Checkbox Border Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checkbox border styling:                              │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────┐                               │   │
     *   │   │   │  .border-input                              │   │
     *   │   │   │  ┌───┐ │                                    │   │
     *   │   │   │  │   │ │ ← Theme-aware border color         │   │
     *   │   │   │  └───┘ │                                    │   │
     *   │   │   └─────────────┘                               │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has border-input class for theming
     */
    test("checkbox should have border-input class", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(/border-input/);
    });

    /**
     * TEST: Checkbox Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checkbox corner radius:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ╭───╮                                         │   │
     *   │   │   │ ✓ │  ← .rounded-[4px]                       │   │
     *   │   │   ╰───╯                                         │   │
     *   │   │                                                 │   │
     *   │   │   Slightly rounded corners (not circular)       │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has 4px rounded corners
     */
    test("checkbox should have rounded corners", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(/rounded-\[4px\]/);
    });

    /**
     * TEST: Checkbox Size Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checkbox dimensions:                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   .size-4 = width: 1rem + height: 1rem          │   │
     *   │   │                                                 │   │
     *   │   │   ←─ 16px ─→                                    │   │
     *   │   │   ┌────────┐ ↑                                  │   │
     *   │   │   │   ✓    │ 16px                               │   │
     *   │   │   └────────┘ ↓                                  │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has size-4 class (16x16 pixels)
     */
    test("checkbox should have size-4 class", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(/size-4/);
    });

    /**
     * TEST: Checked Checkbox Primary Background
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checked state background styling:                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Unchecked:            Checked:                │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │   │ transparent     │███│ bg-primary        │   │
     *   │   │   └───┘                 └───┘                   │   │
     *   │   │                           ↑                     │   │
     *   │   │           data-[state=checked]:bg-primary       │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checked state applies primary background color
     */
    test("checked checkbox should have primary background", async ({
      page,
    }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(
        /data-\[state=checked\]:bg-primary/
      );
    });

    /**
     * TEST: Checkbox Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checkbox drop shadow:                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │ ✓ │  .shadow-xs                             │   │
     *   │   │   └───┘                                         │   │
     *   │   │   ░░░░░  ← Subtle shadow for depth              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has subtle shadow-xs class
     */
    test("checkbox should have shadow", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(/shadow-xs/);
    });
  });

  test.describe("Check Indicator", () => {
    /**
     * TEST: Checked Checkbox Has Indicator
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CheckboxIndicator presence when checked:              │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button data-state="checked">                 │   │
     *   │   │     └── [data-name="CheckboxIndicator"]         │   │
     *   │   │              ↓                                  │   │
     *   │   │           VISIBLE                               │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │ ✓ │ ← Indicator visible inside button       │   │
     *   │   │   └───┘                                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: CheckboxIndicator element is visible when checked
     */
    test("checked checkbox should have CheckboxIndicator", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      const indicator = ui.item1Checkbox.locator(
        '[data-name="CheckboxIndicator"]'
      );
      await expect(indicator).toBeVisible();
    });

    /**
     * TEST: Checked Checkbox Shows Check Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SVG check icon visibility when checked:               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button data-state="checked">                 │   │
     *   │   │     └── CheckboxIndicator                       │   │
     *   │   │           └── <svg>  ← Check icon               │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │ ✓ │ ← SVG is VISIBLE                        │   │
     *   │   │   └───┘                                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Check icon SVG is visible inside checked checkbox
     */
    test("checked checkbox should show check icon", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      const checkIcon = ui.item1Checkbox.locator("svg");
      await expect(checkIcon).toBeVisible();
    });

    /**
     * TEST: Unchecked Checkbox Hides Check Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SVG check icon hidden when unchecked:                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button data-state="unchecked">               │   │
     *   │   │     └── (no indicator or hidden)                │   │
     *   │   │                                                 │   │
     *   │   │   ┌───┐                                         │   │
     *   │   │   │   │ ← SVG is NOT VISIBLE                    │   │
     *   │   │   └───┘                                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Check icon SVG is not visible in unchecked state
     */
    test("unchecked checkbox should not show check icon", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      const checkIcon = ui.item2Checkbox.locator("svg");
      await expect(checkIcon).not.toBeVisible();
    });

    /**
     * TEST: Indicator Appears After Checking
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Indicator appearance on state change:                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Before click:         After click:            │   │
     *   │   │   ┌───┐                 ┌───┐                   │   │
     *   │   │   │   │ Item 2  ──CLICK──→│ ✓ │ Item 2            │   │
     *   │   │   └───┘                 └───┘                   │   │
     *   │   │   svg: hidden           svg: VISIBLE            │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Check icon appears dynamically after clicking
     */
    test("indicator should appear after checking", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Wait for WASM hydration before clicking
      await page.waitForLoadState("networkidle");

      // item2 starts unchecked - no icon
      await expect(ui.item2Checkbox.locator("svg")).not.toBeVisible();

      // Check it
      await ui.item2Checkbox.click();

      // Now icon should be visible
      await expect(ui.item2Checkbox.locator("svg")).toBeVisible();
    });
  });

  test.describe("Labels", () => {
    /**
     * TEST: Labels Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label text content verification:                      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   [✓] Item 1      ← label text = "Item 1"       │   │
     *   │   │   [ ] Item 2      ← label text = "Item 2"       │   │
     *   │   │   [✓] Disabled    ← label text = "Disabled"     │   │
     *   │   │   [ ] Disabled    ← label text = "Disabled"     │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All labels are visible with correct text content
     */
    test("labels should be visible", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Label).toHaveText("Item 1");
      await expect(ui.item2Label).toHaveText("Item 2");
      await expect(ui.disabled1Label).toHaveText("Disabled");
      await expect(ui.disabled2Label).toHaveText("Disabled");
    });

    /**
     * TEST: Label For Attribute Matches Checkbox ID
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label-checkbox association:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <label for="item1">Item 1</label>             │   │
     *   │   │          ↑                                      │   │
     *   │   │   <button id="item1" role="checkbox">           │   │
     *   │   │              ↑                                  │   │
     *   │   │   for === id  (proper association)              │   │
     *   │   │                                                 │   │
     *   │   │   Clicking label should toggle checkbox         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Labels have for attributes matching checkbox IDs
     */
    test("labels should have for attribute matching checkbox id", async ({
      page,
    }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Label).toHaveAttribute("for", "item1");
      await expect(ui.item2Label).toHaveAttribute("for", "item2");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Checkbox ARIA Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA label for screen readers:                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <button aria-label="Checkbox" ...>            │   │
     *   │   │                  ↑                              │   │
     *   │   │   Screen reader announces:                      │   │
     *   │   │   "Checkbox, checkbox, checked"                 │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has aria-label for accessibility
     */
    test("checkbox should have aria-label", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveAttribute("aria-label", "Checkbox");
    });

    /**
     * TEST: Checkbox Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus capability:                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Before focus():       After focus():          │   │
     *   │   │   ┌───┐                 ┏━━━┓                   │   │
     *   │   │   │ ✓ │                 ┃ ✓ ┃ ← Focus ring      │   │
     *   │   │   └───┘                 ┗━━━┛                   │   │
     *   │   │                         isFocused() === true    │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox can receive keyboard focus
     */
    test("checkbox should be focusable", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await ui.item1Checkbox.focus();
      await expect(ui.item1Checkbox).toBeFocused();
    });

    /**
     * TEST: Checkbox Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Focus-visible ring styling:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   When focused via keyboard:                    │   │
     *   │   │   ┏━━━━━━━━━━━┓                                 │   │
     *   │   │   ┃  ┌───┐    ┃  ← focus-visible:ring           │   │
     *   │   │   ┃  │ ✓ │    ┃                                 │   │
     *   │   │   ┃  └───┘    ┃                                 │   │
     *   │   │   ┗━━━━━━━━━━━┛                                 │   │
     *   │   │   Visual indicator for keyboard navigation      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox has focus-visible ring class for keyboard users
     */
    test("checkbox should have focus-visible ring styles", async ({ page }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      await expect(ui.item1Checkbox).toHaveClass(/focus-visible:ring/);
    });

    /**
     * TEST: Disabled Checkbox Not Focusable via Tab
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tab navigation skips disabled checkboxes:             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   [✓] Item 1                                    │   │
     *   │   │   [○] Item 2     ← Focus here, press Tab        │   │
     *   │   │   [✓] Disabled   ← SKIPPED (disabled)           │   │
     *   │   │   [ ] Disabled   ← SKIPPED (disabled)           │   │
     *   │   │                                                 │   │
     *   │   │   Tab key skips disabled elements               │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled checkboxes are excluded from tab order
     */
    test("disabled checkbox should not be focusable via tab", async ({
      page,
    }) => {
      const ui = new CheckboxPage(page);
      await ui.goto();

      // Focus item2 and tab - should skip disabled checkboxes
      await ui.item2Checkbox.focus();
      await page.keyboard.press("Tab");

      // Disabled checkboxes should be skipped
      await expect(ui.disabled1Checkbox).not.toBeFocused();
      await expect(ui.disabled2Checkbox).not.toBeFocused();
    });
  });
});
