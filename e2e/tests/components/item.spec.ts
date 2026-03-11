import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * ITEM COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="Item">                                                │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   [ItemMedia]  [ItemContent]           [ItemActions]            │   │
 * │   │   ┌────────┐   ┌─────────────────────┐ ┌────────────┐           │   │
 * │   │   │        │   │ ItemTitle           │ │            │           │   │
 * │   │   │  icon  │   │ ItemDescription     │ │  [Action]  │           │   │
 * │   │   │        │   │                     │ │            │           │   │
 * │   │   └────────┘   └─────────────────────┘ └────────────┘           │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ITEM VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Basic Item (outline variant):                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Basic Item                                    [Action]         │   │
 * │   │  This is a simple item with title and description              │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * │   Link Item (with media):                                               │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ✓  Email is verified                              [>]          │   │
 * │   │     ↑                                               ↑           │   │
 * │   │  ItemMedia                                    chevron icon      │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .flex          ← Flex layout                               │       │
 * │   │  .items-center  ← Vertical centering                        │       │
 * │   │  .gap-3         ← Spacing between parts                     │       │
 * │   │  .border        ← Border (outline variant)                  │       │
 * │   │  .rounded-lg    ← Large border radius                       │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ItemPage extends BasePage {
  protected readonly componentName = "item";

  // Item elements
  readonly items: Locator;
  readonly firstItem: Locator;
  readonly secondItem: Locator;

  // First item parts
  readonly firstItemTitle: Locator;
  readonly firstItemDescription: Locator;
  readonly firstItemAction: Locator;

  // Second item parts
  readonly secondItemMedia: Locator;
  readonly secondItemTitle: Locator;
  readonly secondItemAction: Locator;

  constructor(page: Page) {
    super(page);

    // Items - scoped within preview
    this.items = this.preview.locator('[data-name="Item"]');
    this.firstItem = this.items.nth(0);
    this.secondItem = this.items.nth(1);

    // First item parts
    this.firstItemTitle = this.firstItem.locator('[data-name="ItemTitle"]');
    this.firstItemDescription = this.firstItem.locator('[data-name="ItemDescription"]');
    this.firstItemAction = this.firstItem.locator('[data-name="ItemActions"]');

    // Second item parts
    this.secondItemMedia = this.secondItem.locator('[data-name="ItemMedia"]');
    this.secondItemTitle = this.secondItem.locator('[data-name="ItemTitle"]');
    this.secondItemAction = this.secondItem.locator('[data-name="ItemActions"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Item Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have Item components
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Page                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [Item] <── visible? ✓                             │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least one Item component is visible on the page
     */
    test("should have Item components", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toBeVisible();
    });

    /**
     * TEST: Should have two items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Page                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [Item #1]                                         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [Item #2]                                         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  count() === 2 ✓                                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 2 Item components exist on the page
     */
    test("should have two items", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      const count = await ui.items.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Items should have data-name attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="Item"> <── attribute check             │
     *   │       ↓                                                 │
     *   │  data-name === "Item" ✓                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item components have correct data-name attribute
     */
    test("items should have data-name attribute", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveAttribute("data-name", "Item");
    });
  });

  test.describe("First Item", () => {
    /**
     * TEST: Should have title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #1]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────┐                                  │  │
     *   │  │  │ ItemTitle   │ <── visible? text === "Basic Item"│  │
     *   │  │  │ "Basic Item"│                                  │  │
     *   │  │  └─────────────┘                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First item has visible title with correct text
     */
    test("should have title", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItemTitle).toBeVisible();
      await expect(ui.firstItemTitle).toHaveText("Basic Item");
    });

    /**
     * TEST: Should have description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #1]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ItemTitle                                        │  │
     *   │  │  ┌──────────────────────────────────────────────┐ │  │
     *   │  │  │ ItemDescription                              │ │  │
     *   │  │  │ "This is a simple item..."                   │ │  │
     *   │  │  └──────────────────────────────────────────────┘ │  │
     *   │  │       ↑                                           │  │
     *   │  │  visible? contains "simple item" ✓                │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First item has visible description with expected text
     */
    test("should have description", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItemDescription).toBeVisible();
      await expect(ui.firstItemDescription).toContainText("simple item");
    });

    /**
     * TEST: Should have action button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #1]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ItemTitle  ItemDescription       ┌──────────┐    │  │
     *   │  │                                   │ [Action] │    │  │
     *   │  │                                   │  button  │    │  │
     *   │  │                                   └──────────┘    │  │
     *   │  │                                        ↑          │  │
     *   │  │                              visible? ✓           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First item has a visible action button
     */
    test("should have action button", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      const button = ui.firstItemAction.getByRole("button", { name: "Action" });
      await expect(button).toBeVisible();
    });
  });

  test.describe("Second Item", () => {
    /**
     * TEST: Should have media (icon)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #2]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌────────┐                                       │  │
     *   │  │  │ [svg]  │  ItemTitle  ItemActions               │  │
     *   │  │  │  ✓     │                                       │  │
     *   │  │  └────────┘                                       │  │
     *   │  │  ItemMedia                                        │  │
     *   │  │      ↑                                            │  │
     *   │  │  visible? has svg? ✓                              │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item has visible media section with an icon
     */
    test("should have media (icon)", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.secondItemMedia).toBeVisible();
      const icon = ui.secondItemMedia.locator("svg");
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Should have title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #2]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ItemMedia  ┌─────────────────────┐  ItemActions  │  │
     *   │  │             │ ItemTitle           │               │  │
     *   │  │             │ "Email is verified" │               │  │
     *   │  │             └─────────────────────┘               │  │
     *   │  │                      ↑                            │  │
     *   │  │            visible? contains "verified" ✓         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item has visible title containing "verified"
     */
    test("should have title", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.secondItemTitle).toBeVisible();
      await expect(ui.secondItemTitle).toContainText("verified");
    });

    /**
     * TEST: Should have chevron action icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #2]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ItemMedia  ItemTitle       ┌──────────────────┐  │  │
     *   │  │                             │ ItemActions      │  │  │
     *   │  │                             │    [svg >]       │  │  │
     *   │  │                             │    chevron       │  │  │
     *   │  │                             └──────────────────┘  │  │
     *   │  │                                      ↑            │  │
     *   │  │                            visible? svg? ✓        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item has a chevron icon in the action area
     */
    test("should have chevron action icon", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      const icon = ui.secondItemAction.locator("svg");
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: Should be a link (has href)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #2]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <a href="#">  <── attribute check                │  │
     *   │  │       ↓                                           │  │
     *   │  │  href === "#" ✓                                   │  │
     *   │  │                                                   │  │
     *   │  │  (entire item is clickable as a link)             │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item is rendered as a link with href attribute
     */
    test("should be a link (has href)", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      const href = await ui.secondItem.getAttribute("href");
      expect(href).toBe("#");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Item should have flex layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item]  class="... flex ..."                           │
     *   │  ┌──────────┬──────────────────┬─────────────┐          │
     *   │  │ Media    │ Content          │ Actions     │          │
     *   │  │ ←──────────── flex ────────────────────→  │          │
     *   │  └──────────┴──────────────────┴─────────────┘          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item uses flexbox for horizontal layout
     */
    test("item should have flex layout", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveClass(/flex/);
    });

    /**
     * TEST: Item should have items-center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item]  class="... items-center ..."                   │
     *   │                                                         │
     *   │  ┌──────────┬──────────────────┬─────────────┐          │
     *   │  │          │                  │             │          │
     *   │  │  Media ──┼── Content ───────┼── Actions   │ ← center │
     *   │  │          │                  │             │          │
     *   │  └──────────┴──────────────────┴─────────────┘          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item children are vertically centered
     */
    test("item should have items-center", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveClass(/items-center/);
    });

    /**
     * TEST: Item should have gap
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item]  class="... gap-3 ..."                          │
     *   │                                                         │
     *   │  ┌────────┐   ┌────────────────┐   ┌─────────┐          │
     *   │  │ Media  │←→│ Content        │←→│ Actions │          │
     *   │  └────────┘   └────────────────┘   └─────────┘          │
     *   │           gap-3            gap-3                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item has gap-3 spacing between children
     */
    test("item should have gap", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveClass(/gap-3/);
    });

    /**
     * TEST: Item with outline variant should have border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item variant="outline"]  class="... border ..."       │
     *   │                                                         │
     *   │  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓  │
     *   │  ┃  Content inside bordered container                ┃  │
     *   │  ┃  ← border ✓                                       ┃  │
     *   │  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Outline variant items have border class
     */
    test("item with outline variant should have border", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveClass(/border/);
    });

    /**
     * TEST: Item with outline variant should have rounded
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item variant="outline"]  class="... rounded-lg ..."   │
     *   │                                                         │
     *   │  ╭───────────────────────────────────────────────────╮  │
     *   │  │  Content with rounded corners                     │  │
     *   │  │  ← rounded-lg ✓                                   │  │
     *   │  ╰───────────────────────────────────────────────────╯  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Outline variant items have large rounded corners
     */
    test("item with outline variant should have rounded", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItem).toHaveClass(/rounded-lg/);
    });

    /**
     * TEST: Title should have font-medium
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ItemTitle]  class="... font-medium ..."               │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Basic Item"  ← font-weight: 500 (medium)        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title text has medium font weight
     */
    test("title should have font-medium", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItemTitle).toHaveClass(/font-medium/);
    });

    /**
     * TEST: Description should have text-muted-foreground
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [ItemDescription]  class="... text-muted-foreground ." │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "This is a simple item..."                       │  │
     *   │  │  ← muted color (secondary text) ✓                 │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description uses muted text color for visual hierarchy
     */
    test("description should have text-muted-foreground", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await expect(ui.firstItemDescription).toHaveClass(/text-muted-foreground/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Action button should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #1]                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Content                     ┌──────────┐         │  │
     *   │  │                              │ [Action] │ ← click │  │
     *   │  │                              │  button  │         │  │
     *   │  │                              └──────────┘         │  │
     *   │  │                                   ↓               │  │
     *   │  │                          no error thrown ✓        │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Action button responds to click events
     */
    test("action button should be clickable", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      const button = ui.firstItemAction.getByRole("button", { name: "Action" });
      await button.click();
      // Should not throw error
    });

    /**
     * TEST: Link item should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Item #2 - link]                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <a href="#">                                     │  │
     *   │  │       ↓                                           │  │
     *   │  │  focus() → isFocused() === true ✓                 │  │
     *   │  │                                                   │  │
     *   │  │  (keyboard navigation support)                    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Link items can receive keyboard focus
     */
    test("link item should be focusable", async ({ page }) => {
      const ui = new ItemPage(page);
      await ui.goto();

      await ui.secondItem.focus();
      await expect(ui.secondItem).toBeFocused();
    });
  });
});
