import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SCROLL-AREA COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="ScrollArea">                                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Tags                          ← heading                        │   │
 * │   │  ──────────────────────                                         │   │
 * │   │  v1.2.0-beta                   ┌──┐                             │   │
 * │   │  ────────────                  │░░│ ← scrollbar (optional)      │   │
 * │   │  v1.1.9-beta                   │░░│                             │   │
 * │   │  ────────────                  │▓▓│ ← thumb                     │   │
 * │   │  v1.1.8-beta                   │░░│                             │   │
 * │   │  ────────────                  │░░│                             │   │
 * │   │  v1.1.7-beta                   └──┘                             │   │
 * │   │  ... (scrollable)                                               │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SCROLL BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Visible viewport (clientHeight)                                       │
 * │   ┌──────────────────────────┐                                          │
 * │   │  v1.2.0-beta             │ ←─┐                                      │
 * │   │  v1.1.9-beta             │   │                                      │
 * │   │  v1.1.8-beta             │   │ visible                              │
 * │   │  v1.1.7-beta             │   │                                      │
 * │   │  v1.1.6-beta             │ ←─┘                                      │
 * │   └──────────────────────────┘                                          │
 * │   ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐                                          │
 * │   ╎  v1.1.5-beta             ╎                                          │
 * │   ╎  v1.1.4-beta             ╎ ← hidden (scrollHeight > clientHeight)   │
 * │   ╎  v1.1.3-beta             ╎                                          │
 * │   └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘                                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Vertical:                      Horizontal:                            │
 * │   ┌────────────────┬──┐          ┌───────────────────────────────────┐  │
 * │   │  Item 1        │▓▓│          │  Img1  │  Img2  │  Img3  │  ...   │  │
 * │   │  Item 2        │░░│          └───────────────────────────────────┘  │
 * │   │  Item 3        │░░│          ═══════▓▓▓▓═════════════════════════   │
 * │   │  Item 4        │░░│                  ↑ horizontal scrollbar        │
 * │   └────────────────┴──┘                                                 │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ScrollAreaPage extends BasePage {
  protected readonly componentName = "scroll-area";

  // Scroll area elements
  readonly scrollArea: Locator;
  readonly content: Locator;
  readonly heading: Locator;
  readonly items: Locator;

  constructor(page: Page) {
    super(page);

    // Main scroll area (scoped within preview)
    this.scrollArea = this.preview.locator('[data-name="ScrollArea"]').first();

    // Content
    this.content = this.scrollArea.locator("div").first();
    this.heading = this.scrollArea.locator("h4");
    this.items = this.scrollArea.locator(".text-sm");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("ScrollArea Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: should have ScrollArea component
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="ScrollArea">                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Tags                                             │ │
     *   │   │  ──────────                                       │ │
     *   │   │  v1.2.0-beta                                      │ │
     *   │   │  v1.1.9-beta                                      │ │
     *   │   │  ...                                              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │       ↑                                                 │
     *   │   ScrollArea container (visible)                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea component is visible on page
     */
    test("should have ScrollArea component", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toBeVisible();
    });

    /**
     * TEST: should have ScrollArea data-name attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="ScrollArea">                          │
     *   │        └────────────────────┘                           │
     *   │                 ↑                                       │
     *   │        Must equal "ScrollArea"                          │
     *   │                                                         │
     *   │   Used for component identification and testing         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has correct data-name attribute
     */
    test("should have ScrollArea data-name attribute", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveAttribute("data-name", "ScrollArea");
    });

    /**
     * TEST: should have Tags heading
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │  Tags                         │  ← <h4> heading     │
     *   │   │  ═════                        │     text = "Tags"   │
     *   │   │  v1.2.0-beta                  │                     │
     *   │   │  v1.1.9-beta                  │                     │
     *   │   │  ...                          │                     │
     *   │   └───────────────────────────────┘                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Heading displays "Tags" text
     */
    test("should have Tags heading", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.heading).toHaveText("Tags");
    });

    /**
     * TEST: should have multiple version items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Version list count > 10:                              │
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │  Tags                         │                     │
     *   │   │  ──────────────────────       │                     │
     *   │   │  v1.2.0-beta            [1]   │                     │
     *   │   │  ──────────────────────       │                     │
     *   │   │  v1.1.9-beta            [2]   │                     │
     *   │   │  ──────────────────────       │                     │
     *   │   │  v1.1.8-beta            [3]   │                     │
     *   │   │  ...                          │                     │
     *   │   │  v1.0.0-beta            [n]   │  ← count > 10       │
     *   │   └───────────────────────────────┘                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 10 version items exist
     */
    test("should have multiple version items", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      const count = await ui.items.count();
      expect(count).toBeGreaterThan(10);
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: should display version tags
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Version tag format verification:                      │
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │  Tags                         │                     │
     *   │   │  ──────────────────────       │                     │
     *   │   │  v1.2.0-beta             ←─── │ ← Must contain      │
     *   │   │  ──────────────────────       │   "v1.2.0-beta"     │
     *   │   │  v1.1.9-beta                  │                     │
     *   │   │  ...                          │                     │
     *   │   └───────────────────────────────┘                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea contains version string "v1.2.0-beta"
     */
    test("should display version tags", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      // Should have version strings
      await expect(ui.scrollArea).toContainText("v1.2.0-beta");
    });

    /**
     * TEST: should have separators between items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Separator elements between items:                     │
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │  Tags                         │                     │
     *   │   │  ════════════════════════     │ ← [role="separator"]│
     *   │   │  v1.2.0-beta                  │                     │
     *   │   │  ════════════════════════     │ ← [role="separator"]│
     *   │   │  v1.1.9-beta                  │                     │
     *   │   │  ════════════════════════     │ ← [role="separator"]│
     *   │   │  ...                          │                     │
     *   │   └───────────────────────────────┘                     │
     *   │                                                         │
     *   │   count > 10 separator elements                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 10 separator elements exist
     */
    test("should have separators between items", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      const separators = ui.scrollArea.locator('[role="separator"]');
      const count = await separators.count();
      expect(count).toBeGreaterThan(10);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: scroll area should have w-48
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Fixed width styling (w-48 = 12rem = 192px):           │
     *   │                                                         │
     *   │   ┌─────────────────────┐                               │
     *   │   │                     │                               │
     *   │   │  Tags               │                               │
     *   │   │  v1.2.0-beta        │                               │
     *   │   │  ...                │                               │
     *   │   │                     │                               │
     *   │   └─────────────────────┘                               │
     *   │   ←────── 192px ───────→                                │
     *   │          (w-48)                                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has w-48 width class
     */
    test("scroll area should have w-48", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveClass(/w-48/);
    });

    /**
     * TEST: scroll area should have h-72
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Fixed height styling (h-72 = 18rem = 288px):          │
     *   │                                                         │
     *   │   ┌─────────────────────┐ ↑                             │
     *   │   │  Tags               │ │                             │
     *   │   │  ──────────────     │ │                             │
     *   │   │  v1.2.0-beta        │ │                             │
     *   │   │  v1.1.9-beta        │ │ 288px                       │
     *   │   │  v1.1.8-beta        │ │ (h-72)                      │
     *   │   │  v1.1.7-beta        │ │                             │
     *   │   │  ...                │ │                             │
     *   │   └─────────────────────┘ ↓                             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has h-72 height class
     */
    test("scroll area should have h-72", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveClass(/h-72/);
    });

    /**
     * TEST: scroll area should have rounded-md
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Border radius styling (rounded-md = 0.375rem):        │
     *   │                                                         │
     *   │   ╭─────────────────────╮  ← rounded corners            │
     *   │   │  Tags               │                               │
     *   │   │  ──────────────     │                               │
     *   │   │  v1.2.0-beta        │                               │
     *   │   │  ...                │                               │
     *   │   ╰─────────────────────╯  ← rounded corners            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has rounded-md border radius
     */
    test("scroll area should have rounded-md", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: scroll area should have border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Border styling for visual containment:                │
     *   │                                                         │
     *   │   ┌─────────────────────┐  ← border (1px)               │
     *   │   │  Tags               │  │                            │
     *   │   │  ──────────────     │  │                            │
     *   │   │  v1.2.0-beta        │  │  border on all sides       │
     *   │   │  ...                │  │                            │
     *   │   └─────────────────────┘  ← border (1px)               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has border class
     */
    test("scroll area should have border", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveClass(/border/);
    });

    /**
     * TEST: scroll area should have overflow-auto
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Overflow behavior (overflow-auto):                    │
     *   │                                                         │
     *   │   ┌───────────────────────────┬──┐                      │
     *   │   │  Tags                     │▓▓│  ← scrollbar appears │
     *   │   │  ────────────────         │░░│    when content      │
     *   │   │  v1.2.0-beta              │░░│    overflows         │
     *   │   │  v1.1.9-beta              │░░│                      │
     *   │   │  v1.1.8-beta              │░░│                      │
     *   │   └───────────────────────────┴──┘                      │
     *   │   ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐                      │
     *   │   ╎  v1.1.7-beta (hidden)       ╎                       │
     *   │   ╎  ... (scrollable content)   ╎                       │
     *   │   └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea has overflow-auto for scrolling
     */
    test("scroll area should have overflow-auto", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      await expect(ui.scrollArea).toHaveClass(/overflow-auto/);
    });
  });

  test.describe("Scrolling", () => {
    /**
     * TEST: content should be scrollable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Content exceeds visible viewport:                     │
     *   │                                                         │
     *   │   ┌───────────────────────────┐ ← clientHeight          │
     *   │   │  Tags                     │   (visible area)        │
     *   │   │  v1.2.0-beta              │                         │
     *   │   │  v1.1.9-beta              │                         │
     *   │   │  v1.1.8-beta              │                         │
     *   │   │  v1.1.7-beta              │                         │
     *   │   └───────────────────────────┘                         │
     *   │   ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐                         │
     *   │   ╎  v1.1.6-beta              ╎ ← scrollHeight          │
     *   │   ╎  v1.1.5-beta              ╎   (total content)       │
     *   │   ╎  ... more items           ╎                         │
     *   │   └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘                         │
     *   │                                                         │
     *   │   scrollHeight > clientHeight = scrollable              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content height exceeds visible viewport height
     */
    test("content should be scrollable", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      // Get the scroll area element
      const scrollHeight = await ui.scrollArea.evaluate(
        (el) => el.scrollHeight
      );
      const clientHeight = await ui.scrollArea.evaluate(
        (el) => el.clientHeight
      );

      // Content should exceed visible height
      expect(scrollHeight).toBeGreaterThan(clientHeight);
    });

    /**
     * TEST: should be able to scroll down
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Scroll position change:                               │
     *   │                                                         │
     *   │   Before (scrollTop = 0):      After (scrollTop = 100): │
     *   │   ┌─────────────────────┐      ┌─────────────────────┐  │
     *   │   │  Tags         ▲    │      │  v1.1.6-beta   │▓▓│  │  │
     *   │   │  v1.2.0-beta  │░░│ │  →   │  v1.1.5-beta   │▓▓│  │  │
     *   │   │  v1.1.9-beta  │░░│ │      │  v1.1.4-beta   │░░│  │  │
     *   │   │  v1.1.8-beta  │░░│ │      │  v1.1.3-beta   │░░│  │  │
     *   │   └─────────────────────┘      └─────────────────────┘  │
     *   │                                                         │
     *   │   scrollTop = 100 (scrolled down 100px)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ScrollArea can be scrolled programmatically
     */
    test("should be able to scroll down", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto();

      // Scroll down
      await ui.scrollArea.evaluate((el) => {
        el.scrollTop = 100;
      });

      const scrollTop = await ui.scrollArea.evaluate((el) => el.scrollTop);
      expect(scrollTop).toBeGreaterThan(0);
    });
  });

  test.describe("Horizontal ScrollArea", () => {
    /**
     * TEST: should have horizontal scroll area demo
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Horizontal scroll variant page loads:                 │
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │  │ Img1 │ Img2 │ Img3 │ Img4 │ Img5 │ ...        │ │
     *   │   │                                                   │ │
     *   │   │  ═══════▓▓▓▓═══════════════════════════════════   │ │
     *   │   │         ↑ horizontal scrollbar                    │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   URL: /docs/components/horizontal-scroll-area          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Horizontal scroll area demo page loads
     */
    test("should have horizontal scroll area demo", async ({ page }) => {
      const ui = new ScrollAreaPage(page);
      await ui.goto("horizontal-scroll-area");

      // The page should load successfully
      await expect(ui.page.locator("body")).toBeVisible();
    });
  });
});
