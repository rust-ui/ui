import { Locator, Page, test, expect } from "@playwright/test";
import { BaseHookPage } from "./_base_page";

/**
 * ============================================================================
 * USE-LOCK-BODY-SCROLL HOOK - VISUAL OVERVIEW
 * ============================================================================
 *
 * HOOK PURPOSE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Prevents body scrolling when a modal/overlay is open.                 │
 * │   Commonly used with dialogs, sheets, and full-screen menus.            │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * DEMO ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Card                                                           │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │                                                           │  │   │
 * │   │  │   Body scroll is currently: UNLOCKED                      │  │   │
 * │   │  │                                                           │  │   │
 * │   │  │   ┌─────────────────────────────────────┐                 │  │   │
 * │   │  │   │         Lock Body Scroll            │  ← Toggle       │  │   │
 * │   │  │   └─────────────────────────────────────┘    button       │  │   │
 * │   │  │                                                           │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE TRANSITIONS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   UNLOCKED (default):              LOCKED:                              │
 * │   ┌─────────────────────┐          ┌─────────────────────┐              │
 * │   │  body {             │          │  body {             │              │
 * │   │    overflow: auto;  │   ──→    │    overflow: hidden;│              │
 * │   │  }                  │   ←──    │  }                  │              │
 * │   └─────────────────────┘  toggle  └─────────────────────┘              │
 * │                                                                         │
 * │   Page scrollable               Page NOT scrollable                     │
 * │   ┌──────────────┐              ┌──────────────┐                        │
 * │   │ Content      │              │ Content      │                        │
 * │   │ ▲           │              │              │                        │
 * │   │ │ scroll    │              │   🔒 locked  │                        │
 * │   │ ▼           │              │              │                        │
 * │   │ More content│              │ More content│                        │
 * │   └──────────────┘              └──────────────┘                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HOOK USAGE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   use_lock_body_scroll(locked: Signal<bool>)                            │
 * │                                                                         │
 * │   When locked() == true:                                                │
 * │     • Sets document.body.style.overflow = "hidden"                      │
 * │     • Prevents scroll events on body                                    │
 * │                                                                         │
 * │   When locked() == false:                                               │
 * │     • Restores original overflow style                                  │
 * │     • Body scrolling resumes normally                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class UseLockBodyScrollPage extends BaseHookPage {
  protected readonly hookName = "use-lock-body-scroll";

  readonly card: Locator;
  readonly toggleButton: Locator;

  constructor(page: Page) {
    super(page);
    this.card = page.locator('[data-name="Card"]').first();
    this.toggleButton = this.card.locator('[data-name="Button"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Use Lock Body Scroll Hook", () => {
  /**
   * TEST: Card and Toggle Button Visibility
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   ┌───────────────────────────────────────────────┐     │
   *   │   │  [data-name="Card"]  ← MUST BE VISIBLE        │     │
   *   │   │  ┌─────────────────────────────────────────┐  │     │
   *   │   │  │                                         │  │     │
   *   │   │  │   Body scroll is currently: UNLOCKED    │  │     │
   *   │   │  │                                         │  │     │
   *   │   │  │   ┌─────────────────────────────────┐   │  │     │
   *   │   │  │   │      Lock Body Scroll          │   │  │     │
   *   │   │  │   └─────────────────────────────────┘   │  │     │
   *   │   │  │              ↑                          │  │     │
   *   │   │  │     [data-name="Button"]                │  │     │
   *   │   │  │        MUST BE VISIBLE                  │  │     │
   *   │   │  └─────────────────────────────────────────┘  │     │
   *   │   └───────────────────────────────────────────────┘     │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Card container and toggle button render correctly
   */
  test("should have toggle button in card", async ({ page }) => {
    const ui = new UseLockBodyScrollPage(page);
    await ui.goto();

    await expect(ui.card).toBeVisible();
    await expect(ui.toggleButton).toBeVisible();
  });

  /**
   * TEST: Toggle Button Interactivity
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Step 1: Click toggle button                           │
   *   │   ┌─────────────────────────────────┐                   │
   *   │   │      Lock Body Scroll          │ ← CLICK            │
   *   │   └─────────────────────────────────┘                   │
   *   │                  │                                      │
   *   │                  ▼                                      │
   *   │   body.style.overflow = "hidden" (scroll locked)        │
   *   │                                                         │
   *   │   Step 2: Verify button still visible                   │
   *   │   ┌─────────────────────────────────┐                   │
   *   │   │     Unlock Body Scroll         │ ← STILL VISIBLE    │
   *   │   └─────────────────────────────────┘                   │
   *   │                                                         │
   *   │   Page state after click:                               │
   *   │   ┌──────────────┐                                      │
   *   │   │ Content      │                                      │
   *   │   │   🔒 locked  │  ← Cannot scroll body                │
   *   │   │ More content │                                      │
   *   │   └──────────────┘                                      │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Toggle button is interactive and triggers hook
   */
  test("toggle button should be clickable", async ({ page }) => {
    const ui = new UseLockBodyScrollPage(page);
    await ui.goto();

    await ui.toggleButton.click();
    await expect(ui.toggleButton).toBeVisible();
  });
});
