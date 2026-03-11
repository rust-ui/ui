import { Locator, Page, test, expect } from "@playwright/test";
import { BaseHookPage } from "./_base_page";

/**
 * ============================================================================
 * USE-PRESS-HOLD HOOK - VISUAL OVERVIEW
 * ============================================================================
 *
 * HOOK ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────┐                           │
 * │   │         🗑️  Hold to Delete              │  ← Hold button            │
 * │   │         ─────────────────────           │    with progress          │
 * │   │         ████████░░░░░░░░░░░░            │    indicator              │
 * │   │              ↑ progress bar             │                           │
 * │   └─────────────────────────────────────────┘                           │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * PRESS-AND-HOLD MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Time: 0ms          500ms         1000ms        1500ms (threshold)     │
 * │         │             │              │              │                   │
 * │   ┌─────┴─────────────┴──────────────┴──────────────┴─────┐             │
 * │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  Not held   │
 * │   └───────────────────────────────────────────────────────┘             │
 * │                                                                         │
 * │   ┌─────┴─────────────┴──────────────┴──────────────┴─────┐             │
 * │   │████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  Holding    │
 * │   └───────────────────────────────────────────────────────┘             │
 * │                                                                         │
 * │   ┌─────┴─────────────┴──────────────┴──────────────┴─────┐             │
 * │   │███████████████████████████████████████████████████████│  Complete!  │
 * │   └───────────────────────────────────────────────────────┘             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE TRANSITIONS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌──────────┐   mousedown    ┌──────────┐   threshold   ┌──────────┐   │
 * │   │  Idle    │ ─────────────→ │ Holding  │ ────────────→ │ Complete │   │
 * │   └──────────┘                └──────────┘               └──────────┘   │
 * │        ↑                           │                          │         │
 * │        │                    mouseup│                          │         │
 * │        │                    (early)│                          │         │
 * │        └───────────────────────────┘                          │         │
 * │        │                         Reset to idle                │         │
 * │        └──────────────────────────────────────────────────────┘         │
 * │                           Action triggered                              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class UsePressHoldPage extends BaseHookPage {
  protected readonly hookName = "use-press-hold";

  readonly holdButton: Locator;

  constructor(page: Page) {
    super(page);
    // ButtonAction component has "Hold to Delete" text
    this.holdButton = page.getByRole("button", { name: /Hold/i }).first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Use Press Hold Hook", () => {
  /**
   * TEST: Hold Button Visibility
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   ┌─────────────────────────────────────────────────┐   │
   *   │   │           🗑️  Hold to Delete                    │   │
   *   │   │           ░░░░░░░░░░░░░░░░░░░░░░                │   │
   *   │   └─────────────────────────────────────────────────┘   │
   *   │                          ↑                              │
   *   │                   MUST BE VISIBLE                       │
   *   │                   role="button" with /Hold/i            │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: The hold-to-action button renders correctly
   */
  test("should have hold button", async ({ page }) => {
    const ui = new UsePressHoldPage(page);
    await ui.goto();

    await expect(ui.holdButton).toBeVisible();
  });

  /**
   * TEST: Short Click Does Not Trigger Action
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Timeline:                                             │
   *   │   0ms              ~50ms (click released)               │
   *   │   │                 │                                   │
   *   │   ├─────────────────┤                                   │
   *   │   │   SHORT CLICK   │                                   │
   *   │   └─────────────────┘                                   │
   *   │                                                         │
   *   │   ┌─────────────────────────────────────────────────┐   │
   *   │   │           🗑️  Hold to Delete                    │   │
   *   │   │           ██░░░░░░░░░░░░░░░░░░░░  (partial)     │   │
   *   │   └─────────────────────────────────────────────────┘   │
   *   │                          ↑                              │
   *   │               ACTION NOT TRIGGERED                      │
   *   │               (button still visible)                    │
   *   │                                                         │
   *   │   Required hold time: ~1500ms                           │
   *   │   Actual hold time:   ~50ms                             │
   *   │   Result: ❌ Action NOT executed                        │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Quick clicks don't accidentally trigger the action
   */
  test("short click should not complete action", async ({ page }) => {
    const ui = new UsePressHoldPage(page);
    await ui.goto();

    await ui.holdButton.click();
    // Button should still be visible after short click
    await expect(ui.holdButton).toBeVisible();
  });
});
