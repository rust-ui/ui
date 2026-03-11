import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SEPARATOR COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div role="separator">                                                │
 * │                                                                         │
 * │   HORIZONTAL:                   VERTICAL:                               │
 * │   ════════════════════════      │                                       │
 * │   w-full, h-[1px]               │  w-[1px], h-full                      │
 * │                                 │                                       │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * USAGE EXAMPLE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  Rust/UI                                                    │       │
 * │   │  An open-source UI component library                        │       │
 * │   ├─────────────────────────────────────────────────────────────┤       │
 * │   │           ↑ Horizontal separator                            │       │
 * │   │                                                             │       │
 * │   │  Blog  │  Docs  │  Source                                   │       │
 * │   │        ↑        ↑                                           │       │
 * │   │   Vertical separators                                       │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .shrink-0    ← Prevent shrinking in flex containers        │       │
 * │   │  .bg-border   ← Uses border color from theme                │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SeparatorPage extends BasePage {
  protected readonly componentName = "separator";

  // Separator elements
  readonly horizontalSeparator: Locator;
  readonly verticalSeparators: Locator;

  constructor(page: Page) {
    super(page);

    // Horizontal separator (first one) - scoped within preview
    this.horizontalSeparator = this.preview
      .locator('[role="separator"]')
      .filter({ hasNotText: /.+/ })
      .first();

    // Vertical separators - scoped within preview
    this.verticalSeparators = this.preview.locator(
      '[role="separator"].h-full, [role="separator"].w-\\[1px\\]'
    );
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Separator Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Separator Elements Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Page should contain separator elements:               │
     *   │                                                         │
     *   │   ════════════════════════  ◄─ [role="separator"]       │
     *   │                                                         │
     *   │   count > 0 ?                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least one separator element exists on page
     */
    test("should have separator elements", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      const separators = page.locator('[role="separator"]');
      const count = await separators.count();
      expect(count).toBeGreaterThan(0);
    });

    /**
     * TEST: Separator ARIA Role
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Accessibility Role Check:                             │
     *   │                                                         │
     *   │   <div role="separator">                                │
     *   │   ════════════════════════                              │
     *   │              ▲                                          │
     *   │              └── role="separator" for screen readers    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separator has proper ARIA role for accessibility
     */
    test("separators should have role=separator", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(ui.horizontalSeparator).toHaveAttribute(
        "role",
        "separator"
      );
    });

    /**
     * TEST: Separator HTML Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Element Type Check:                                   │
     *   │                                                         │
     *   │   ✓ <div role="separator">    (correct)                 │
     *   │   ✗ <hr>                      (native but less control) │
     *   │   ✗ <span>                    (wrong semantic)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separator uses <div> element with role attribute
     */
    test("separator should be a div element", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      const tagName = await ui.horizontalSeparator.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("div");
    });
  });

  test.describe("Horizontal Separator", () => {
    /**
     * TEST: Horizontal Separator Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   w-full (100% width):                                  │
     *   │                                                         │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │◄────────────── w-full ────────────────────────▶│   │
     *   │   ════════════════════════════════════════════════════  │
     *   │                                                         │
     *   │   Horizontal separator spans full container width       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Horizontal separator has w-full class
     */
    test("horizontal separator should have w-full", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(ui.horizontalSeparator).toHaveClass(/w-full/);
    });

    /**
     * TEST: Horizontal Separator Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   h-[1px] (1 pixel height):                             │
     *   │                                                         │
     *   │   Content above                                         │
     *   │   ════════════════════════  ◄─ h-[1px] (thin line)      │
     *   │   Content below                                         │
     *   │                                                         │
     *   │   Creates a subtle visual divider                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Horizontal separator has 1px height
     */
    test("horizontal separator should have h-[1px]", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(ui.horizontalSeparator).toHaveClass(/h-\[1px\]/);
    });
  });

  test.describe("Vertical Separator", () => {
    /**
     * TEST: Vertical Separators Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Vertical separators in demo:                          │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Blog  │  Docs  │  Source                       │   │
     *   │   │        │        │                               │   │
     *   │   │        ▲        ▲                               │   │
     *   │   │   w-[1px]  w-[1px]                              │   │
     *   │   │   h-full   h-full                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Vertical separators exist with h-full or w-[1px]
     */
    test("should have vertical separators in demo", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      // Look for vertical separators
      const verticalSeps = page.locator(
        '[role="separator"][class*="h-full"], [role="separator"][class*="w-[1px]"]'
      );
      const count = await verticalSeps.count();
      expect(count).toBeGreaterThan(0);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Shrink Prevention
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   shrink-0 behavior in flex container:                  │
     *   │                                                         │
     *   │   Without shrink-0:                                     │
     *   │   ┌─────────┐ ═══ ┌─────────────────────────────────┐   │
     *   │   │ Content │     │ Long content that pushes...     │   │
     *   │   └─────────┘  ▲  └─────────────────────────────────┘   │
     *   │              shrinks!                                   │
     *   │                                                         │
     *   │   With shrink-0:                                        │
     *   │   ┌─────────┐ ════ ┌────────────────────────────────┐   │
     *   │   │ Content │      │ Long content...                │   │
     *   │   └─────────┘  ▲   └────────────────────────────────┘   │
     *   │            stays fixed                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separator won't shrink in flex containers
     */
    test("separator should have shrink-0", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(ui.horizontalSeparator).toHaveClass(/shrink-0/);
    });

    /**
     * TEST: Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   bg-border class:                                      │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                    Content                      │   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │████████████████████████████████████████████████│   │
     *   │   │        ▲ bg-border (theme border color)        │   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │                    Content                      │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separator uses theme border color
     */
    test("separator should have bg-border", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(ui.horizontalSeparator).toHaveClass(/bg-border/);
    });
  });

  test.describe("Demo Content", () => {
    /**
     * TEST: Heading Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Demo Layout:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <h3> Rust/UI </h3>  ◄─── Expected heading    │   │
     *   │   │                                                 │   │
     *   │   │   Description text...                          │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Demo has "Rust/UI" heading text
     */
    test("should have heading text", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      const heading = page.locator("h3");
      await expect(heading.first()).toContainText("Rust/UI");
    });

    /**
     * TEST: Description Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Demo Layout:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Rust/UI                                       │   │
     *   │   │   "An open-source UI component library"         │   │
     *   │   │              ▲                                  │   │
     *   │   │              └── Expected description text      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Demo has description text visible
     */
    test("should have description text", async ({ page }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      const desc = page.getByText("open-source UI component library");
      await expect(desc).toBeVisible();
    });

    /**
     * TEST: Navigation Links with Vertical Separators
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Navigation row with separators:                       │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │    Blog   │   Docs   │   Source                 │   │
     *   │   │      ▲    │    ▲     │     ▲                    │   │
     *   │   │    link   │  link    │   link                   │   │
     *   │   │           ▲          ▲                          │   │
     *   │   │      vertical separators                        │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Blog, Docs, Source links are visible
     */
    test("should have navigation links separated by vertical separators", async ({
      page,
    }) => {
      const ui = new SeparatorPage(page);
      await ui.goto();

      await expect(page.getByText("Blog")).toBeVisible();
      await expect(page.getByText("Docs")).toBeVisible();
      await expect(page.getByText("Source")).toBeVisible();
    });
  });
});
