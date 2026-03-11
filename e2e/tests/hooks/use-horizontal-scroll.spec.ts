import { Locator, Page, test, expect } from "@playwright/test";
import { BaseHookPage } from "./_base_page";

/**
 * ============================================================================
 * USE-HORIZONTAL-SCROLL HOOK - VISUAL OVERVIEW
 * ============================================================================
 *
 * HOOK ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Navigation Controls:                                                  │
 * │   ┌────────────┐ ┌────────────┐                                         │
 * │   │     ◀      │ │     ▶      │  ← Scroll buttons                       │
 * │   └────────────┘ └────────────┘                                         │
 * │                                                                         │
 * │   Scroll Container (overflow-x-scroll):                                 │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐│   │
 * │   │ │Item1│ │Item2│ │Item3│ │Item4│ │Item5│ │Item6│ │Item7│ │Item8││   │
 * │   │ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘│   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │   │◀──────────── visible viewport ─────────────▶│                       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SCROLL BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Click ◀ (Left):                                                       │
 * │   ┌──────────────────────────────────────────────────────────────┐      │
 * │   │ [Item1] [Item2] [Item3] [Item4] │ [Item5] [Item6] ...        │      │
 * │   │              ◀──────────────────│                            │      │
 * │   └──────────────────────────────────────────────────────────────┘      │
 * │                      scrollLeft -= scrollAmount                         │
 * │                                                                         │
 * │   Click ▶ (Right):                                                      │
 * │   ┌──────────────────────────────────────────────────────────────┐      │
 * │   │ ... [Item3] [Item4] │ [Item5] [Item6] [Item7] [Item8]        │      │
 * │   │                     │──────────────────▶                     │      │
 * │   └──────────────────────────────────────────────────────────────┘      │
 * │                      scrollLeft += scrollAmount                         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HOOK RETURN VALUE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   use_horizontal_scroll(container_ref) returns:                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  {                                                              │   │
 * │   │    scroll_left: Fn(),   // scroll container left                │   │
 * │   │    scroll_right: Fn(),  // scroll container right               │   │
 * │   │    can_scroll_left: Signal<bool>,                               │   │
 * │   │    can_scroll_right: Signal<bool>,                              │   │
 * │   │  }                                                              │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class UseHorizontalScrollPage extends BaseHookPage {
  protected readonly hookName = "use-horizontal-scroll";

  readonly scrollContainer: Locator;
  readonly leftButton: Locator;
  readonly rightButton: Locator;

  constructor(page: Page) {
    super(page);
    this.scrollContainer = page.locator(".overflow-x-scroll").first();
    // Nav buttons are in a flex container with gap-2
    this.leftButton = page.locator(".flex.gap-2 button").first();
    this.rightButton = page.locator(".flex.gap-2 button").nth(1);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Use Horizontal Scroll Hook", () => {
  /**
   * TEST: Scroll Container Visibility
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   ┌───────────────────────────────────────────────┐     │
   *   │   │  .overflow-x-scroll  ← MUST BE VISIBLE        │     │
   *   │   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐      │     │
   *   │   │  │ 1   │ │ 2   │ │ 3   │ │ ... │ │ N   │      │     │
   *   │   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘      │     │
   *   │   └───────────────────────────────────────────────┘     │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: The horizontal scroll container exists and renders
   */
  test("should have scroll container", async ({ page }) => {
    const ui = new UseHorizontalScrollPage(page);
    await ui.goto();

    await expect(ui.scrollContainer).toBeVisible();
  });

  /**
   * TEST: Navigation Buttons Visibility
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   ┌────────┐   ┌────────┐                               │
   *   │   │   ◀    │   │   ▶    │  ← BOTH MUST BE VISIBLE       │
   *   │   │  LEFT  │   │ RIGHT  │                               │
   *   │   └────────┘   └────────┘                               │
   *   │    .first()     .nth(1)                                 │
   *   │                                                         │
   *   │   ┌───────────────────────────────────────────────┐     │
   *   │   │  [items...]                                   │     │
   *   │   └───────────────────────────────────────────────┘     │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Both left and right navigation buttons render
   */
  test("should have navigation buttons", async ({ page }) => {
    const ui = new UseHorizontalScrollPage(page);
    await ui.goto();

    await expect(ui.leftButton).toBeVisible();
    await expect(ui.rightButton).toBeVisible();
  });

  /**
   * TEST: Navigation Button Interactivity
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Step 1: Click right button                            │
   *   │   ┌────────┐   ┌────────┐                               │
   *   │   │   ◀    │   │   ▶    │ ← CLICK HERE                  │
   *   │   └────────┘   └────────┘                               │
   *   │                    ↓                                    │
   *   │              (no crash)                                 │
   *   │                                                         │
   *   │   Step 2: Verify button still visible after click       │
   *   │   ┌────────┐   ┌────────┐                               │
   *   │   │   ◀    │   │   ▶    │ ← STILL VISIBLE               │
   *   │   └────────┘   └────────┘                               │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Buttons are interactive and don't break on click
   */
  test("navigation buttons should be clickable", async ({ page }) => {
    const ui = new UseHorizontalScrollPage(page);
    await ui.goto();

    // Buttons should be interactive
    await ui.rightButton.click();
    await expect(ui.rightButton).toBeVisible();
  });
});
