import { Locator, Page, test, expect } from "@playwright/test";
import { BaseHookPage } from "./_base_page";

/**
 * ============================================================================
 * USE-COPY-CLIPBOARD HOOK - VISUAL OVERVIEW
 * ============================================================================
 *
 * HOOK ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  https://example.com/share/abc123                     [Copy]    │   │
 * │   │       ↑                                                  ↑      │   │
 * │   │  Input field (readonly URL)                       Copy button   │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * COPY FLOW:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Step 1: Click Copy              Step 2: Feedback                      │
 * │   ┌────────────────────────┐      ┌────────────────────────┐            │
 * │   │  [📋 Copy]             │ ──→  │  [✓ Copied!]           │            │
 * │   └────────────────────────┘      └────────────────────────┘            │
 * │           │                                 │                           │
 * │           ▼                                 ▼                           │
 * │   navigator.clipboard.writeText()   Button shows success state          │
 * │                                     (resets after timeout)              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HOOK RETURN VALUE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   use_copy_clipboard() returns:                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  {                                                              │   │
 * │   │    copied: Signal<bool>,     // true after successful copy     │   │
 * │   │    copy: Fn(text: &str),     // function to copy text          │   │
 * │   │  }                                                              │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class UseCopyClipboardPage extends BaseHookPage {
  protected readonly hookName = "use-copy-clipboard";

  readonly urlInput: Locator;
  readonly copyButton: Locator;

  constructor(page: Page) {
    super(page);
    // Find the visible Input and Button in the demo (not hidden sheet triggers)
    this.urlInput = page.locator('[data-name="Input"]:visible').first();
    this.copyButton = page.locator('[data-name="Button"]:visible').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Use Copy Clipboard Hook", () => {
  /**
   * TEST: Input and Copy Button Visibility
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   ┌─────────────────────────────────┐ ┌────────────┐    │
   *   │   │  https://example.com/share/...  │ │   Copy     │    │
   *   │   └─────────────────────────────────┘ └────────────┘    │
   *   │              ↑                              ↑           │
   *   │        MUST BE VISIBLE              MUST BE VISIBLE     │
   *   │        [data-name="Input"]          [data-name="Button"]│
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Both input field and copy button render correctly
   */
  test("should have input and copy button", async ({ page }) => {
    const ui = new UseCopyClipboardPage(page);
    await ui.goto();

    await expect(ui.urlInput).toBeVisible();
    await expect(ui.copyButton).toBeVisible();
  });

  /**
   * TEST: Copy Button Interactivity
   * ─────────────────────────────────────────────────────────────
   *
   *   What we're testing:
   *   ┌─────────────────────────────────────────────────────────┐
   *   │                                                         │
   *   │   Step 1: Click the copy button                         │
   *   │   ┌─────────────────────────────────┐ ┌────────────┐    │
   *   │   │  https://example.com/...        │ │   Copy     │    │
   *   │   └─────────────────────────────────┘ └─────┬──────┘    │
   *   │                                             │           │
   *   │                                         CLICK           │
   *   │                                             ↓           │
   *   │                                       (no crash)        │
   *   │                                                         │
   *   │   Step 2: Button still visible after click              │
   *   │   ┌─────────────────────────────────┐ ┌────────────┐    │
   *   │   │  https://example.com/...        │ │  Copied!   │    │
   *   │   └─────────────────────────────────┘ └────────────┘    │
   *   │                                             ↑           │
   *   │                                       STILL VISIBLE     │
   *   │                                                         │
   *   └─────────────────────────────────────────────────────────┘
   *
   *   Validates: Copy button is interactive and doesn't break
   */
  test("copy button should be clickable", async ({ page }) => {
    const ui = new UseCopyClipboardPage(page);
    await ui.goto();

    await ui.copyButton.click();
    await expect(ui.copyButton).toBeVisible();
  });
});
