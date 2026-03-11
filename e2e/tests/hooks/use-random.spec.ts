import { Locator, Page, test, expect } from "@playwright/test";
import { BaseHookPage } from "./_base_page";

/**
 * ============================================================================
 * USE-RANDOM HOOK - VISUAL OVERVIEW
 * ============================================================================
 *
 * HOOK PURPOSE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Generates unique, stable IDs for components that need                 │
 * │   accessibility attributes (id, aria-labelledby, etc.)                  │
 * │                                                                         │
 * │   use_random() → "_rust_ui_a7b3c9d2"                                    │
 * │                         ↑                                               │
 * │                   unique random suffix                                  │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * DEMO ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Generated ID:                                                  │   │
 * │   │  ┌────────────────────────────────────────────────────────────┐ │   │
 * │   │  │  _rust_ui_a7b3c9d2                                         │ │   │
 * │   │  └────────────────────────────────────────────────────────────┘ │   │
 * │   │         ↑ <code> element showing the ID                         │   │
 * │   │                                                                  │   │
 * │   │  ┌───┐                                                          │   │
 * │   │  │ ☐ │  Enable feature                                          │   │
 * │   │  └───┘                                                          │   │
 * │   │    ↑                                                            │   │
 * │   │  Checkbox using id="_rust_ui_a7b3c9d2"                          │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ID PATTERN:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   _rust_ui_XXXXXXXX                                                     │
 * │   ├───────┤├───────┤                                                    │
 * │      │         │                                                        │
 * │   prefix    random hex (8 characters)                                   │
 * │                                                                         │
 * │   Examples:                                                             │
 * │   • _rust_ui_a7b3c9d2                                                   │
 * │   • _rust_ui_f1e2d3c4                                                   │
 * │   • _rust_ui_12345678                                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class UseRandomPage extends BaseHookPage {
  protected readonly hookName = "use-random";

  readonly generatedIdCode: Locator;
  readonly checkbox: Locator;

  constructor(page: Page) {
    super(page);
    // Find the code element that contains the generated ID (starts with _rust_ui_)
    this.generatedIdCode = page.locator("code").filter({ hasText: /_rust_ui_/ }).first();
    this.checkbox = page.locator('input[type="checkbox"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Use Random Hook", () => {
  /**
   * TEST: Generated ID Format
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Generated ID:                                         │
   *   │   ┌───────────────────────────────────────────────┐     │
   *   │   │  _rust_ui_a7b3c9d2                            │     │
   *   │   └───────────────────────────────────────────────┘     │
   *   │      ├────────┤├────────┤                               │
   *   │       prefix    random hex                              │
   *   │                                                         │
   *   │   Assertions:                                           │
   *   │   ✓ <code> element is visible                           │
   *   │   ✓ Text matches /^_rust_ui_/                           │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Hook generates IDs with correct prefix format
   */
  test("should display generated ID starting with _rust_ui_", async ({ page }) => {
    const ui = new UseRandomPage(page);
    await ui.goto();

    await expect(ui.generatedIdCode).toBeVisible();
    const idText = await ui.generatedIdCode.textContent();
    expect(idText).toMatch(/^_rust_ui_/);
  });

  /**
   * TEST: Checkbox Uses Generated ID
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   <code>: "_rust_ui_a7b3c9d2"                           │
   *   │                    │                                    │
   *   │                    │ should match                       │
   *   │                    ▼                                    │
   *   │   <input type="checkbox" id="_rust_ui_a7b3c9d2">        │
   *   │                              ├────────────────┤         │
   *   │                               checkbox's id             │
   *   │                                                         │
   *   │   Comparison:                                           │
   *   │   ┌──────────────────┐    ┌──────────────────┐          │
   *   │   │ generatedIdCode  │ == │ checkbox.id      │          │
   *   │   │ .textContent()   │    │ getAttribute()   │          │
   *   │   └──────────────────┘    └──────────────────┘          │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Generated ID is actually used by the component
   */
  test("checkbox should use the generated ID", async ({ page }) => {
    const ui = new UseRandomPage(page);
    await ui.goto();

    const generatedId = await ui.generatedIdCode.textContent();
    const checkboxId = await ui.checkbox.getAttribute("id");
    expect(checkboxId).toBe(generatedId);
  });

  /**
   * TEST: Checkbox Functionality
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Before:                  After:                       │
   *   │   ┌───┐                    ┌───┐                        │
   *   │   │   │  Enable feature    │ ✓ │  Enable feature        │
   *   │   └───┘                    └───┘                        │
   *   │   unchecked                checked                      │
   *   │                                                         │
   *   │   Action: ui.checkbox.check()                           │
   *   │   Assert: expect(checkbox).toBeChecked()                │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Checkbox with generated ID is fully functional
   */
  test("checkbox should be functional", async ({ page }) => {
    const ui = new UseRandomPage(page);
    await ui.goto();

    await ui.checkbox.check();
    await expect(ui.checkbox).toBeChecked();
  });
});
