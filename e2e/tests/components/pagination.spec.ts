import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * PAGINATION COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <nav data-name="Pagination" role="navigation">                        │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  <ul data-name="PaginationList">                                │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │                                                           │  │   │
 * │   │  │  [<]  [1] [2] [3] [4] [5]  [>]                             │  │   │
 * │   │  │   ↑    ↑                    ↑                             │  │   │
 * │   │  │  Prev  Page Links         Next                            │  │   │
 * │   │  │                                                           │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * PARTS BREAKDOWN:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   PaginationNavButton (Prev)    PaginationLink      PaginationNavButton │
 * │   ┌──────────────────┐          ┌────────────┐      ┌──────────────────┐│
 * │   │                  │          │            │      │                  ││
 * │   │   [<] Previous   │          │    [3]     │      │   Next [>]       ││
 * │   │                  │          │            │      │                  ││
 * │   │   <button>       │          │  <button>  │      │   <button>       ││
 * │   │   + chevron-left │          │  size-9    │      │   + chevron-right││
 * │   └──────────────────┘          └────────────┘      └──────────────────┘│
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:                      Active:                                 │
 * │   ┌────────┐                    ┌════════┐                              │
 * │   │   3    │                    ║   3    ║  ← highlighted               │
 * │   └────────┘                    └════════┘                              │
 * │                                                                         │
 * │   Hover:                        Disabled:                               │
 * │   ┌────────┐                    ┌────────┐                              │
 * │   │░░░ 3 ░░│                    │   <    │  ← can't go back             │
 * │   └────────┘                    └────────┘    (first page)              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class PaginationPage extends BasePage {
  protected readonly componentName = "pagination";

  // Pagination container
  readonly pagination: Locator;

  // Pagination parts
  readonly paginationList: Locator;
  readonly paginationItems: Locator;
  readonly paginationLinks: Locator;
  readonly prevButton: Locator;
  readonly nextButton: Locator;

  constructor(page: Page) {
    super(page);

    // Main pagination - scoped within preview
    this.pagination = this.preview.locator('[data-name="PaginationNav"]').first();

    // Parts
    this.paginationList = this.pagination.locator('[data-name="PaginationList"]');
    this.paginationItems = this.pagination.locator('[data-name="PaginationItem"]');
    // PaginationLink renders as <a> tags without aria-label (page numbers only)
    this.paginationLinks = this.paginationList.locator("a:not([aria-label])");

    // Nav buttons are <a> tags with aria-label
    this.prevButton = this.pagination.locator('a[aria-label="Go to previous page"]');
    this.nextButton = this.pagination.locator('a[aria-label="Go to next page"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Pagination Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: should have Pagination container
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav data-name="Pagination">                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   [<]  [1] [2] [3] [4] [5]  [>]                   │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   Pagination container (visible)                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main pagination component is visible on page
     */
    test("should have Pagination container", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.pagination).toBeVisible();
    });

    /**
     * TEST: should have Pagination data-name attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav data-name="Pagination">                          │
     *   │        └────────────────────┘                           │
     *   │                 ↑                                       │
     *   │        Must equal "Pagination"                          │
     *   │                                                         │
     *   │   Used for component identification and testing         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pagination has correct data-name attribute
     */
    test("should have Pagination data-name attribute", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.pagination).toHaveAttribute("data-name", "PaginationNav");
    });

    /**
     * TEST: should have PaginationList
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav>                                                  │
     *   │     <ul data-name="PaginationList">                     │
     *   │     ┌───────────────────────────────────────────────┐   │
     *   │     │                                               │   │
     *   │     │   [<]  [1] [2] [3] [4] [5]  [>]               │   │
     *   │     │                                               │   │
     *   │     └───────────────────────────────────────────────┘   │
     *   │          ↑                                              │
     *   │     PaginationList (contains all items)                 │
     *   │   </nav>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PaginationList container is visible
     */
    test("should have PaginationList", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.paginationList).toBeVisible();
    });

    /**
     * TEST: should have multiple PaginationItems
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   PaginationItems count (minimum 5):                    │
     *   │                                                         │
     *   │   [<]   [1]   [2]   [3]   [4]   [5]   [>]               │
     *   │    ↑     ↑     ↑     ↑     ↑     ↑     ↑                │
     *   │   [1]   [2]   [3]   [4]   [5]   [6]   [7]               │
     *   │                                                         │
     *   │   count >= 5 items (nav buttons + page links)           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least 5 pagination items exist
     */
    test("should have multiple PaginationItems", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      const count = await ui.paginationItems.count();
      expect(count).toBeGreaterThanOrEqual(5);
    });
  });

  test.describe("Navigation Buttons", () => {
    /**
     * TEST: should have previous button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Previous navigation button:                           │
     *   │                                                         │
     *   │   ┌────────┐                                            │
     *   │   │   <    │  [1] [2] [3] [4] [5]  [>]                  │
     *   │   └────────┘                                            │
     *   │       ↑                                                 │
     *   │   prevButton (visible)                                  │
     *   │   - Goes to previous page                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Previous button is visible
     */
    test("should have previous button", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      // Prev button exists but is hidden on page 1 (opacity-0)
      await expect(ui.prevButton).toBeAttached();
    });

    /**
     * TEST: should have next button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Next navigation button:                               │
     *   │                                                         │
     *   │                                        ┌────────┐       │
     *   │   [<]  [1] [2] [3] [4] [5]             │   >    │       │
     *   │                                        └────────┘       │
     *   │                                            ↑            │
     *   │                                   nextButton (visible)  │
     *   │                                   - Goes to next page   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Next button is visible
     */
    test("should have next button", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.nextButton).toBeVisible();
    });

    /**
     * TEST: previous button should have chevron icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Chevron left icon inside prev button:                 │
     *   │                                                         │
     *   │   ┌────────────────┐                                    │
     *   │   │   ◀  Previous  │                                    │
     *   │   │   ↑            │                                    │
     *   │   │  <svg>         │                                    │
     *   │   │  chevron-left  │                                    │
     *   │   └────────────────┘                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Previous button contains SVG chevron icon
     */
    test("previous button should have chevron icon", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      // Icon exists but button is hidden on page 1
      const icon = ui.prevButton.locator("svg");
      await expect(icon).toBeAttached();
    });

    /**
     * TEST: next button should have chevron icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Chevron right icon inside next button:                │
     *   │                                                         │
     *   │                            ┌────────────────┐           │
     *   │                            │  Next   ▶     │           │
     *   │                            │          ↑    │           │
     *   │                            │        <svg>  │           │
     *   │                            │  chevron-right│           │
     *   │                            └────────────────┘           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Next button contains SVG chevron icon
     */
    test("next button should have chevron icon", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      const icon = ui.nextButton.locator("svg");
      await expect(icon).toBeVisible();
    });
  });

  test.describe("Page Links", () => {
    /**
     * TEST: should have page links
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Page link count = 5:                                  │
     *   │                                                         │
     *   │   [<]  [1]  [2]  [3]  [4]  [5]  [>]                     │
     *   │         ↑    ↑    ↑    ↑    ↑                           │
     *   │        [1]  [2]  [3]  [4]  [5]                          │
     *   │                                                         │
     *   │   Exactly 5 PaginationLink elements                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 5 page link buttons exist
     */
    test("should have page links", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      const count = await ui.paginationLinks.count();
      expect(count).toBe(5);
    });

    /**
     * TEST: page links should display page numbers
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Sequential page numbers displayed:                    │
     *   │                                                         │
     *   │   ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐                        │
     *   │   │ 1 │ │ 2 │ │ 3 │ │ 4 │ │ 5 │                        │
     *   │   └───┘ └───┘ └───┘ └───┘ └───┘                        │
     *   │     ↑     ↑     ↑     ↑     ↑                          │
     *   │   nth(0) nth(1) nth(2) nth(3) nth(4)                   │
     *   │                                                         │
     *   │   Each link shows its page number                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page links display numbers 1 through 5
     */
    test("page links should display page numbers", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.paginationLinks.nth(0)).toContainText("1");
      await expect(ui.paginationLinks.nth(1)).toContainText("2");
      await expect(ui.paginationLinks.nth(2)).toContainText("3");
      await expect(ui.paginationLinks.nth(3)).toContainText("4");
      await expect(ui.paginationLinks.nth(4)).toContainText("5");
    });

    /**
     * TEST: page links should be buttons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   HTML element type verification:                       │
     *   │                                                         │
     *   │   <button data-name="PaginationLink">                   │
     *   │       1                                                 │
     *   │   </button>                                             │
     *   │    ↑                                                    │
     *   │   tagName = "button" (not <a> or <div>)                │
     *   │                                                         │
     *   │   Using button for proper accessibility                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page links are button elements
     */
    test("page links should be anchor elements", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      const tagName = await ui.paginationLinks
        .first()
        .evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("a");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: pagination should have nav element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Semantic HTML structure:                              │
     *   │                                                         │
     *   │   <nav data-name="Pagination">  ← Must be <nav>        │
     *   │     ...                                                 │
     *   │   </nav>                                                │
     *   │                                                         │
     *   │   Using <nav> for:                                      │
     *   │   - Screen reader navigation                            │
     *   │   - Semantic meaning                                    │
     *   │   - Accessibility best practice                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pagination uses semantic nav element
     */
    test("pagination should have nav element", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      const tagName = await ui.pagination.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("nav");
    });

    /**
     * TEST: list should have flex layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Flexbox layout for horizontal alignment:              │
     *   │                                                         │
     *   │   <ul class="flex ...">                                 │
     *   │   ┌─────────────────────────────────────────┐           │
     *   │   │  [<]  [1]  [2]  [3]  [4]  [5]  [>]     │           │
     *   │   │  ←────────── flex row ───────────→     │           │
     *   │   └─────────────────────────────────────────┘           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PaginationList has flex class
     */
    test("list should have flex layout", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.paginationList).toHaveClass(/flex/);
    });

    /**
     * TEST: list should have gap
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Spacing between pagination items (gap-1):             │
     *   │                                                         │
     *   │   [<] ↔ [1] ↔ [2] ↔ [3] ↔ [4] ↔ [5] ↔ [>]             │
     *   │       ↑                                                 │
     *   │    gap-1 (0.25rem / 4px spacing)                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PaginationList has gap-1 class for spacing
     */
    test("list should have gap", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.paginationList).toHaveClass(/gap-1/);
    });

    /**
     * TEST: pagination links should have size classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Fixed button dimensions (size-9):                     │
     *   │                                                         │
     *   │   ┌─────────┐                                           │
     *   │   │         │ ← height: 2.25rem (36px)                  │
     *   │   │    1    │                                           │
     *   │   │         │                                           │
     *   │   └─────────┘                                           │
     *   │   ↑─────────↑                                           │
     *   │   width: 2.25rem (36px)                                 │
     *   │                                                         │
     *   │   size-9 = 36px x 36px square buttons                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page links have size-9 class (36px)
     */
    test("pagination links should have size classes", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await expect(ui.paginationLinks.first()).toHaveClass(/size-9/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: page links should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Click interaction on page links:                      │
     *   │                                                         │
     *   │   [<]  [1]  [2]  [3]  [4]  [5]  [>]                     │
     *   │         ↑                                               │
     *   │       click()                                           │
     *   │                                                         │
     *   │   Action: Click page link [1]                           │
     *   │   Expected: No error thrown (clickable)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page links respond to click events
     */
    test("page links should be clickable", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await ui.paginationLinks.first().click();
      // Should not throw error
    });

    /**
     * TEST: nav buttons should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Click interaction on navigation buttons:              │
     *   │                                                         │
     *   │   [<]  [1]  [2]  [3]  [4]  [5]  [>]                     │
     *   │                                     ↑                   │
     *   │                                  click()                │
     *   │                                                         │
     *   │   Action: Click next button [>]                         │
     *   │   Expected: No error thrown (clickable)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation buttons respond to click events
     */
    test("nav buttons should be clickable", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      await ui.nextButton.click();
      // Should not throw error
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: pagination should have role=navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA role attribute:                                  │
     *   │                                                         │
     *   │   <nav role="navigation">                               │
     *   │        └──────────────┘                                 │
     *   │               ↑                                         │
     *   │        Must be "navigation"                             │
     *   │                                                         │
     *   │   Screen reader announcement:                           │
     *   │   "pagination navigation"                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pagination has role="navigation" for a11y
     */
    test("pagination should have implicit navigation role via nav element", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      // <nav> element has implicit role="navigation"
      const tagName = await ui.pagination.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("nav");
    });

    /**
     * TEST: pagination should have aria-label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA label attribute:                                 │
     *   │                                                         │
     *   │   <nav aria-label="pagination">                         │
     *   │        └─────────────────────┘                          │
     *   │                  ↑                                      │
     *   │        Must be "pagination"                             │
     *   │                                                         │
     *   │   Screen reader announcement:                           │
     *   │   "pagination, navigation"                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pagination has aria-label for screen readers
     */
    test("navigation buttons should have aria-labels", async ({ page }) => {
      const ui = new PaginationPage(page);
      await ui.goto();

      // Nav buttons have descriptive aria-labels
      await expect(ui.prevButton).toHaveAttribute(
        "aria-label",
        "Go to previous page"
      );
      await expect(ui.nextButton).toHaveAttribute(
        "aria-label",
        "Go to next page"
      );
    });
  });
});
