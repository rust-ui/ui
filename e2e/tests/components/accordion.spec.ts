import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * ACCORDION COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │  [data-name="Accordion"]                                                │
 * │  ┌───────────────────────────────────────────────────────────────────┐  │
 * │  │  [data-name="AccordionItem"]                                      │  │
 * │  │  ┌─────────────────────────────────────────────────────────────┐  │  │
 * │  │  │  <input type="checkbox" />  ← Hidden, controls state (peer) │  │  │
 * │  │  └─────────────────────────────────────────────────────────────┘  │  │
 * │  │  ┌─────────────────────────────────────────────────────────────┐  │  │
 * │  │  │  <label>  ← Trigger (clickable)                             │  │  │
 * │  │  │    ┌─────────────────────────────────────┐    ┌───┐         │  │  │
 * │  │  │    │ <h4> AccordionTitle                 │    │ ▼ │ ← Icon  │  │  │
 * │  │  │    └─────────────────────────────────────┘    └───┘         │  │  │
 * │  │  └─────────────────────────────────────────────────────────────┘  │  │
 * │  │  ┌─────────────────────────────────────────────────────────────┐  │  │
 * │  │  │  <article>  ← Content wrapper (grid for animation)          │  │  │
 * │  │  │    ┌─────────────────────────────────────────────────────┐  │  │  │
 * │  │  │    │  <div> AccordionDescription                         │  │  │  │
 * │  │  │    │    "This is the Accordion description..."           │  │  │  │
 * │  │  │    └─────────────────────────────────────────────────────┘  │  │  │
 * │  │  └─────────────────────────────────────────────────────────────┘  │  │
 * │  └───────────────────────────────────────────────────────────────────┘  │
 * │  ├─────────────────────────────────────────────────────────────────────┤│
 * │  │  AccordionItem 2...                                               │  │
 * │  ├─────────────────────────────────────────────────────────────────────┤│
 * │  │  AccordionItem 3...                                               │  │
 * │  └───────────────────────────────────────────────────────────────────┘  │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM (CSS-only, no JS):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <input type="checkbox" class="peer" />                                │
 * │         │                                                               │
 * │         ▼                                                               │
 * │   ┌─────────────┐     ┌─────────────────────────────────────────────┐   │
 * │   │  :checked   │────►│  peer-checked:grid-rows-[1fr]  (open)       │   │
 * │   └─────────────┘     │  Chevron rotates 180°                       │   │
 * │                       └─────────────────────────────────────────────┘   │
 * │   ┌─────────────┐     ┌─────────────────────────────────────────────┐   │
 * │   │  unchecked  │────►│  grid-rows-[0fr]  (collapsed)               │   │
 * │   └─────────────┘     │  Chevron at 0°                              │   │
 * │                       └─────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ANIMATION TECHNIQUE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   CLOSED STATE                      OPEN STATE                          │
 * │   ┌────────────────────┐            ┌────────────────────┐              │
 * │   │ grid-rows-[0fr]    │            │ grid-rows-[1fr]    │              │
 * │   │ ┌────────────────┐ │            │ ┌────────────────┐ │              │
 * │   │ │ overflow:hidden│ │  ──────►   │ │    Content     │ │              │
 * │   │ │   (height: 0)  │ │  animate   │ │   is visible   │ │              │
 * │   │ └────────────────┘ │            │ └────────────────┘ │              │
 * │   └────────────────────┘            └────────────────────┘              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class AccordionPage extends BasePage {
  protected readonly componentName = "accordion";

  // Accordion container
  readonly accordion: Locator;

  // Accordion items
  readonly item1: Locator;
  readonly item2: Locator;
  readonly item3: Locator;

  // Accordion triggers (labels)
  readonly trigger1: Locator;
  readonly trigger2: Locator;
  readonly trigger3: Locator;

  // Accordion inputs (checkboxes)
  readonly input1: Locator;
  readonly input2: Locator;
  readonly input3: Locator;

  // Accordion content areas
  readonly content1: Locator;
  readonly content2: Locator;
  readonly content3: Locator;

  // Accordion content areas
  readonly allItems: Locator;

  constructor(page: Page) {
    super(page);

    // Main accordion - scoped within preview
    this.accordion = this.preview.locator('[data-name="Accordion"]').first();

    // Items - scoped within accordion (which is within preview)
    this.allItems = this.accordion.locator('[data-name="AccordionItem"]');
    this.item1 = this.allItems.nth(0);
    this.item2 = this.allItems.nth(1);
    this.item3 = this.allItems.nth(2);

    // Triggers (the clickable labels)
    this.trigger1 = this.item1.locator("label");
    this.trigger2 = this.item2.locator("label");
    this.trigger3 = this.item3.locator("label");

    // Inputs (hidden checkboxes that control state)
    this.input1 = this.item1.locator('input[type="checkbox"]');
    this.input2 = this.item2.locator('input[type="checkbox"]');
    this.input3 = this.item3.locator('input[type="checkbox"]');

    // Content areas (articles)
    this.content1 = this.item1.locator("article");
    this.content2 = this.item2.locator("article");
    this.content3 = this.item3.locator("article");
  }

  async toggleItem(trigger: Locator) {
    await trigger.click();
  }

  async isItemOpen(input: Locator): Promise<boolean> {
    return input.isChecked();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Accordion Page", () => {
  /**
   * STRUCTURE TESTS - Verify component hierarchy
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │  Accordion                                                      │
   * │  ├── AccordionItem (×3)                                         │
   * │  │   ├── input[type="checkbox"]  ← Hidden state controller      │
   * │  │   ├── label                   ← Clickable trigger            │
   * │  │   │   ├── h4 (AccordionTitle)                                │
   * │  │   │   └── svg (chevron icon)                                 │
   * │  │   └── article                 ← Content wrapper              │
   * │  │       └── div (AccordionDescription)                         │
   * │  └── ... (more items)                                           │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Accordion Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │              Accordion Container                │   │
     *   │   │              (must be visible)                  │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main accordion container element is rendered and visible
     */
    test("should have Accordion container", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.accordion).toBeVisible();
    });

    /**
     * TEST: Accordion Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="Accordion">  ← must have this attr    │
     *   │     ...                                                 │
     *   │   </div>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has data-name="Accordion" attribute
     */
    test("should have Accordion data-name attribute", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.accordion).toHaveAttribute("data-name", "Accordion");
    });

    /**
     * TEST: Three Accordion Items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AccordionItem 1                                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AccordionItem 2                                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AccordionItem 3                                │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   count === 3                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Accordion contains exactly 3 items
     */
    test("should have three accordion items", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const count = await ui.allItems.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: AccordionItem Data-Name Attributes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="AccordionItem">  ← Item 1             │
     *   │   <div data-name="AccordionItem">  ← Item 2             │
     *   │   <div data-name="AccordionItem">  ← Item 3             │
     *   │                                                         │
     *   │   Each item must have data-name="AccordionItem"         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each item has correct data-name attribute
     */
    test("each item should have AccordionItem data-name", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.item1).toHaveAttribute("data-name", "AccordionItem");
      await expect(ui.item2).toHaveAttribute("data-name", "AccordionItem");
      await expect(ui.item3).toHaveAttribute("data-name", "AccordionItem");
    });

    /**
     * TEST: Trigger Labels Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  <label> Accordion item 01              │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  <label> Accordion item 02              │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  <label> Accordion item 03              │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                                                         │
     *   │   All trigger labels must be visible                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each accordion item has a visible trigger label
     */
    test("each item should have a trigger label", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.trigger1).toBeVisible();
      await expect(ui.trigger2).toBeVisible();
      await expect(ui.trigger3).toBeVisible();
    });

    /**
     * TEST: Hidden Checkbox Inputs
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CSS-only accordion uses hidden checkboxes:            │
     *   │                                                         │
     *   │   <input type="checkbox" class="peer sr-only" />        │
     *   │         ↑                                               │
     *   │   Visually hidden but exists in DOM                     │
     *   │   Controls open/close state via :checked                │
     *   │                                                         │
     *   │   3 items = 3 hidden checkboxes                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each item has a hidden checkbox for state control
     */
    test("each item should have a hidden checkbox", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Inputs are visually hidden but exist in DOM
      const inputs = ui.accordion.locator('input[type="checkbox"]');
      const count = await inputs.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: Content Article Elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Each item has a content wrapper:                      │
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────────────┐  │
     *   │   │  <article>  ← content container                  │  │
     *   │   │    <div> AccordionDescription </div>             │  │
     *   │   │  </article>                                      │  │
     *   │   └──────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │   Articles must be attached to DOM (visible or not)     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each item has a content article element in DOM
     */
    test("each item should have a content article", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.content1).toBeAttached();
      await expect(ui.content2).toBeAttached();
      await expect(ui.content3).toBeAttached();
    });
  });

  /**
   * INITIAL STATE TESTS - Default open/closed state
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────────────────────────┐                   │
   * │   │ ☑ Accordion item 01                  ▲  │  ← OPEN (checked) │
   * │   ├─────────────────────────────────────────┤                   │
   * │   │   This is the Accordion description...  │  ← Content shown  │
   * │   └─────────────────────────────────────────┘                   │
   * │   ┌─────────────────────────────────────────┐                   │
   * │   │ ☐ Accordion item 02                  ▼  │  ← CLOSED         │
   * │   └─────────────────────────────────────────┘                   │
   * │   ┌─────────────────────────────────────────┐                   │
   * │   │ ☐ Accordion item 03                  ▼  │  ← CLOSED         │
   * │   └─────────────────────────────────────────┘                   │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Initial State", () => {
    /**
     * TEST: First Item Open by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☑ Accordion item 01                     │  ▲  │     │
     *   │   ├─────────────────────────────────────────┴─────┤     │
     *   │   │   Content is visible...                       │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                    ↑                                    │
     *   │   checkbox :checked = true (open by default)            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First accordion item is open on initial load
     */
    test("first item should be open by default", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      expect(await ui.isItemOpen(ui.input1)).toBe(true);
    });

    /**
     * TEST: Second Item Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 02                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                    ↑                                    │
     *   │   checkbox :checked = false (closed by default)         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second accordion item is closed on initial load
     */
    test("second item should be closed by default", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      expect(await ui.isItemOpen(ui.input2)).toBe(false);
    });

    /**
     * TEST: Third Item Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 03                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                    ↑                                    │
     *   │   checkbox :checked = false (closed by default)         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third accordion item is closed on initial load
     */
    test("third item should be closed by default", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      expect(await ui.isItemOpen(ui.input3)).toBe(false);
    });

    /**
     * TEST: First Item Content Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☑ Accordion item 01                     │  ▲  │     │
     *   │   ├─────────────────────────────────────────┴─────┤     │
     *   │   │   "This is the Accordion description..."      │ ← ✓ │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   │   AccordionDescription is visible when open             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Open item shows its content description
     */
    test("first item content should be visible", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const description = ui.item1.locator('[data-name="AccordionDescription"]');
      await expect(description).toBeVisible();
    });

    /**
     * TEST: Second Item Content Hidden (CSS Grid Animation)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 02                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   <article style="grid-rows: 0fr; height: 0">           │
     *   │   └────────────────────────────────────────────┘        │
     *   │        ↑                                                │
     *   │   Content wrapper collapsed to 0 height                 │
     *   │   (uses CSS grid animation with overflow-hidden)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Closed item's content wrapper has 0 height
     */
    test("second item content should be hidden", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Content uses CSS grid animation - check article height is 0 when closed
      const contentWrapper = ui.content2;
      const height = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(height).toBe(0);
    });

    /**
     * TEST: Third Item Content Hidden (CSS Grid Animation)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 03                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   <article style="grid-rows: 0fr; height: 0">           │
     *   │   └────────────────────────────────────────────┘        │
     *   │        ↑                                                │
     *   │   Content wrapper collapsed to 0 height                 │
     *   │   (uses CSS grid animation with overflow-hidden)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Closed item's content wrapper has 0 height
     */
    test("third item content should be hidden", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Content uses CSS grid animation - check article height is 0 when closed
      const contentWrapper = ui.content3;
      const height = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(height).toBe(0);
    });
  });

  /**
   * TITLE TESTS - Verify title content and structure
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <label>                                                       │
   * │     ┌─────────────────────────────────────────────┐             │
   * │     │  <h4 data-name="AccordionTitle">            │             │
   * │     │     "Accordion item 01"                     │             │
   * │     │     "Accordion item 02"                     │             │
   * │     │     "Accordion item 03"                     │             │
   * │     └─────────────────────────────────────────────┘             │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Titles", () => {
    /**
     * TEST: Item 1 Title Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  "Accordion item 01"                    │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │           ↑                                             │
     *   │   AccordionTitle text must match exactly                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First item has correct title text
     */
    test("should have correct title for item 1", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const title = ui.item1.locator('[data-name="AccordionTitle"]');
      await expect(title).toHaveText("Accordion item 01");
    });

    /**
     * TEST: Item 2 Title Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  "Accordion item 02"                    │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │           ↑                                             │
     *   │   AccordionTitle text must match exactly                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item has correct title text
     */
    test("should have correct title for item 2", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const title = ui.item2.locator('[data-name="AccordionTitle"]');
      await expect(title).toHaveText("Accordion item 02");
    });

    /**
     * TEST: Item 3 Title Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  "Accordion item 03"                    │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │           ↑                                             │
     *   │   AccordionTitle text must match exactly                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third item has correct title text
     */
    test("should have correct title for item 3", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const title = ui.item3.locator('[data-name="AccordionTitle"]');
      await expect(title).toHaveText("Accordion item 03");
    });

    /**
     * TEST: Title is H4 Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 data-name="AccordionTitle">                       │
     *   │      Accordion item 01                                  │
     *   │   </h4>                                                 │
     *   │    ↑                                                    │
     *   │   Must be <h4> for semantic HTML structure              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title uses correct heading element (h4)
     */
    test("title should be h4 element", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const title = ui.item1.locator('[data-name="AccordionTitle"]');
      const tagName = await title.evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("h4");
    });
  });

  /**
   * TOGGLE BEHAVIOR TESTS - Click interactions
   *
   * TOGGLE FLOW:
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   STEP 1: Initial State        STEP 2: Click to close                   │
   * │   ┌───────────────────────┐    ┌───────────────────────┐                │
   * │   │ Item 01            ▲  │    │ Item 01            ▼  │                │
   * │   ├───────────────────────┤    └───────────────────────┘                │
   * │   │ Content visible...    │ 🖱️                                          │
   * │   └───────────────────────┘ ──►  (content collapses)                    │
   * │                                                                         │
   * │   STEP 3: Click to reopen                                               │
   * │   ┌───────────────────────┐                                             │
   * │   │ Item 01            ▲  │                                             │
   * │   ├───────────────────────┤                                             │
   * │   │ Content visible...    │  ← Back to open                             │
   * │   └───────────────────────┘                                             │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   *
   * MULTIPLE ITEMS OPEN (type="multiple" behavior):
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   ┌───────────────────────┐                                             │
   * │   │ ☑ Item 01          ▲  │  ← OPEN                                     │
   * │   ├───────────────────────┤                                             │
   * │   │ Content 1...          │                                             │
   * │   └───────────────────────┘                                             │
   * │   ┌───────────────────────┐                                             │
   * │   │ ☑ Item 02          ▲  │  ← OPEN (simultaneously)                    │
   * │   ├───────────────────────┤                                             │
   * │   │ Content 2...          │                                             │
   * │   └───────────────────────┘                                             │
   * │   ┌───────────────────────┐                                             │
   * │   │ ☑ Item 03          ▲  │  ← OPEN (all three!)                        │
   * │   ├───────────────────────┤                                             │
   * │   │ Content 3...          │                                             │
   * │   └───────────────────────┘                                             │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Toggle Behavior", () => {
    /**
     * TEST: Click Toggle Open/Close
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   OPEN (initial)        🖱️ CLICK         CLOSED         │
     *   │   ┌─────────────────┐   ────────►   ┌─────────────────┐ │
     *   │   │ Item 01      ▲  │               │ Item 01      ▼  │ │
     *   │   ├─────────────────┤               └─────────────────┘ │
     *   │   │ Content...      │                                   │
     *   │   └─────────────────┘               🖱️ CLICK            │
     *   │                                     ────────►           │
     *   │                                                         │
     *   │   OPEN (again)                                          │
     *   │   ┌─────────────────┐                                   │
     *   │   │ Item 01      ▲  │                                   │
     *   │   ├─────────────────┤                                   │
     *   │   │ Content...      │                                   │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger toggles item open/closed state
     */
    test("clicking trigger should toggle item", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // item1 starts open
      expect(await ui.isItemOpen(ui.input1)).toBe(true);

      // Click to close
      await ui.toggleItem(ui.trigger1);
      expect(await ui.isItemOpen(ui.input1)).toBe(false);

      // Click to open again
      await ui.toggleItem(ui.trigger1);
      expect(await ui.isItemOpen(ui.input1)).toBe(true);
    });

    /**
     * TEST: Open Second Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE                         AFTER CLICK            │
     *   │   ┌─────────────────┐            ┌─────────────────┐    │
     *   │   │ ☐ Item 02    ▼  │   🖱️ ──►   │ ☑ Item 02    ▲  │    │
     *   │   └─────────────────┘            ├─────────────────┤    │
     *   │                                  │ Content...      │    │
     *   │   :checked = false               └─────────────────┘    │
     *   │                                  :checked = true        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second item can be opened via click
     */
    test("can open second item", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      expect(await ui.isItemOpen(ui.input2)).toBe(false);

      await ui.toggleItem(ui.trigger2);
      expect(await ui.isItemOpen(ui.input2)).toBe(true);
    });

    /**
     * TEST: Open Third Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE                         AFTER CLICK            │
     *   │   ┌─────────────────┐            ┌─────────────────┐    │
     *   │   │ ☐ Item 03    ▼  │   🖱️ ──►   │ ☑ Item 03    ▲  │    │
     *   │   └─────────────────┘            ├─────────────────┤    │
     *   │                                  │ Content...      │    │
     *   │   :checked = false               └─────────────────┘    │
     *   │                                  :checked = true        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third item can be opened via click
     */
    test("can open third item", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      expect(await ui.isItemOpen(ui.input3)).toBe(false);

      await ui.toggleItem(ui.trigger3);
      expect(await ui.isItemOpen(ui.input3)).toBe(true);
    });

    /**
     * TEST: Multiple Items Open Simultaneously
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ALL THREE ITEMS OPEN AT ONCE:                         │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☑ Accordion item 01                     │  ▲  │     │
     *   │   ├─────────────────────────────────────────┴─────┤     │
     *   │   │   Content 1...                                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☑ Accordion item 02                     │  ▲  │     │
     *   │   ├─────────────────────────────────────────┴─────┤     │
     *   │   │   Content 2...                                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☑ Accordion item 03                     │  ▲  │     │
     *   │   ├─────────────────────────────────────────┴─────┤     │
     *   │   │   Content 3...                                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Multiple accordion mode allows all items open
     */
    test("multiple items can be open simultaneously", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Open all items
      await ui.toggleItem(ui.trigger2);
      await ui.toggleItem(ui.trigger3);

      expect(await ui.isItemOpen(ui.input1)).toBe(true);
      expect(await ui.isItemOpen(ui.input2)).toBe(true);
      expect(await ui.isItemOpen(ui.input3)).toBe(true);
    });

    /**
     * TEST: Close All Items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ALL ITEMS CLOSED:                                     │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 01                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 02                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ☐ Accordion item 03                     │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                                                         │
     *   │   All checkboxes unchecked, all content collapsed       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All items can be closed simultaneously
     */
    test("can close all items", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Close item 1 (starts open)
      await ui.toggleItem(ui.trigger1);

      expect(await ui.isItemOpen(ui.input1)).toBe(false);
      expect(await ui.isItemOpen(ui.input2)).toBe(false);
      expect(await ui.isItemOpen(ui.input3)).toBe(false);
    });

    /**
     * TEST: Opening Item Expands Content (CSS Grid Animation)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE (closed)            AFTER (open)               │
     *   │   ┌─────────────────┐        ┌─────────────────┐        │
     *   │   │ Item 02      ▼  │  🖱️ ►  │ Item 02      ▲  │        │
     *   │   └─────────────────┘        ├─────────────────┤        │
     *   │   <article height=0>         │ Description... │         │
     *   │                              └─────────────────┘        │
     *   │                              <article height>0>         │
     *   │                                                         │
     *   │   Content wrapper: height 0 → height > 0                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Opening item expands content wrapper height
     */
    test("opening item should make content visible", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Item 2 starts closed - content wrapper has 0 height
      const contentWrapper = ui.content2;
      const initialHeight = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(initialHeight).toBe(0);

      // Open it
      await ui.toggleItem(ui.trigger2);

      // Wait for animation and check content is expanded
      await page.waitForTimeout(500);
      const expandedHeight = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(expandedHeight).toBeGreaterThan(0);
    });

    /**
     * TEST: Closing Item Collapses Content (CSS Grid Animation)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   BEFORE (open)              AFTER (closed)             │
     *   │   ┌─────────────────┐        ┌─────────────────┐        │
     *   │   │ Item 01      ▲  │  🖱️ ►  │ Item 01      ▼  │        │
     *   │   ├─────────────────┤        └─────────────────┘        │
     *   │   │ Description... │         <article height=0>         │
     *   │   └─────────────────┘                                   │
     *   │   <article height>0>                                    │
     *   │                                                         │
     *   │   Content wrapper: height > 0 → height 0                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Closing item collapses content wrapper to 0
     */
    test("closing item should hide content", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Item 1 starts open - content wrapper has non-zero height
      const contentWrapper = ui.content1;
      const initialHeight = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(initialHeight).toBeGreaterThan(0);

      // Close it
      await ui.toggleItem(ui.trigger1);

      // Wait for animation and check content is collapsed
      await page.waitForTimeout(500);
      const collapsedHeight = await contentWrapper.evaluate(
        (el) => el.getBoundingClientRect().height
      );
      expect(collapsedHeight).toBe(0);
    });
  });

  /**
   * CONTENT TESTS - Description and inner content
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <article>  ← Content wrapper                                  │
   * │     ┌─────────────────────────────────────────────────────┐     │
   * │     │  <div data-name="AccordionDescription">             │     │
   * │     │     "This is the Accordion description. It can      │     │
   * │     │      be used to display additional information..."  │     │
   * │     └─────────────────────────────────────────────────────┘     │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Content", () => {
    /**
     * TEST: AccordionDescription in Each Item
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Item 1: <div data-name="AccordionDescription">        │
     *   │   Item 2: <div data-name="AccordionDescription">        │
     *   │   Item 3: <div data-name="AccordionDescription">        │
     *   │                                                         │
     *   │   count === 3 (one per item)                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each item has a description element
     */
    test("should have AccordionDescription in each item", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const descriptions = ui.accordion.locator(
        '[data-name="AccordionDescription"]'
      );
      const count = await descriptions.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: Description Contains Expected Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  "This is the Accordion description..."       │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                    ↑                                    │
     *   │   Text content must include expected string             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description contains correct text content
     */
    test("description should contain expected text", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const description = ui.item1.locator('[data-name="AccordionDescription"]');
      await expect(description).toContainText("This is the Accordion description");
    });
  });

  /**
   * CHEVRON ICON TESTS - Rotation animation indicator
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   CLOSED STATE                        OPEN STATE                        │
   * │   ┌───────────────────────────┐       ┌───────────────────────────┐     │
   * │   │ Accordion item 01     ▼   │       │ Accordion item 01     ▲   │     │
   * │   └───────────────────────────┘       ├───────────────────────────┤     │
   * │                           │           │ Content...                │     │
   * │                           │           └───────────────────────────┘     │
   * │                           │                               │             │
   * │                           ▼                               ▼             │
   * │                      rotate: 0°                     rotate: 180°        │
   * │                                                                         │
   * │   CSS: [&:has(>input:checked)>label>svg:last-child]:rotate-180          │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Chevron Icon", () => {
    /**
     * TEST: Chevron Icon Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  Accordion item 01                      │ <svg>│    │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                                                  ↑      │
     *   │                                      Chevron icon must  │
     *   │                                      be visible         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has a visible chevron (arrow) icon
     */
    test("trigger should have chevron icon", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const chevron = ui.trigger1.locator("svg");
      await expect(chevron).toBeVisible();
    });

    /**
     * TEST: Chevron Rotation Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CLOSED STATE          OPEN STATE                      │
     *   │   ┌─────────────┬───┐   ┌─────────────┬───┐             │
     *   │   │ Item 01     │ ▼ │   │ Item 01     │ ▲ │             │
     *   │   └─────────────┴───┘   └─────────────┴───┘             │
     *   │                  ↑                      ↑               │
     *   │             rotate(0°)            rotate(180°)          │
     *   │                                                         │
     *   │   CSS: [&:has(>input:checked)>label>svg:last-child]     │
     *   │        :rotate-180                                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item has rotation class for chevron animation
     */
    test("chevron should rotate when item is open", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Item should have rotate class applied via CSS selector
      // [&:has(>input:checked)>label>svg:last-child]:rotate-180
      await expect(ui.item1).toHaveClass(
        /\[&:has\(>input:checked\)>label>svg:last-child\]:rotate-180/
      );
    });

    /**
     * TEST: Chevron is SVG Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label>                                               │
     *   │     <h4>Title</h4>                                      │
     *   │     <svg> ← must be SVG element                         │
     *   │       <path>...</path>                                  │
     *   │     </svg>                                              │
     *   │   </label>                                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chevron icon is an SVG element
     */
    test("chevron should be svg element", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const chevron = ui.trigger1.locator("svg");
      const tagName = await chevron.evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("svg");
    });
  });

  /**
   * STYLING TESTS - Tailwind classes and visual appearance
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   ACCORDION CONTAINER                                                   │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .divide-y           ← Separator lines between items        │       │
   * │   │  .max-w-[400px]      ← Maximum width constraint             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   TRIGGER LABEL                                                         │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .flex .justify-between .items-center   ← Flex layout       │       │
   * │   │  .p-3                                   ← Padding           │       │
   * │   │  .cursor-pointer                        ← Clickable         │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   TITLE                                                                 │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .text-sm .font-medium                  ← Typography        │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   DESCRIPTION                                                           │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  .text-muted-foreground                 ← Muted color       │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Styling", () => {
    /**
     * TEST: Accordion Divider Lines
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Item 1                                         │   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │  Item 2                                         │   │
     *   │   ├─────────────────────────────────────────────────┤   │
     *   │   │  Item 3                                         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑          ↑                                     │
     *   │   divide-y class creates horizontal separators          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Accordion has divide-y class for separators
     */
    test("accordion should have divide-y class for separators", async ({
      page,
    }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.accordion).toHaveClass(/divide-y/);
    });

    /**
     * TEST: Accordion Max Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ◄───────── max-w-[400px] ──────────►                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │            Accordion Container                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Accordion has max-width constraint
     */
    test("accordion should have max-width", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.accordion).toHaveClass(/max-w-\[400px\]/);
    });

    /**
     * TEST: Trigger Cursor Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  Accordion item 01                  👆  │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                                          ↑              │
     *   │                            cursor: pointer (clickable)  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has pointer cursor to indicate clickable
     */
    test("trigger should have cursor-pointer", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.trigger1).toHaveClass(/cursor-pointer/);
    });

    /**
     * TEST: Trigger Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label class="flex justify-between items-center">     │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │  Title                                  │ Icon│     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │    ↑                                           ↑        │
     *   │   flex-start              space-between    flex-end     │
     *   │                      (justify-between)                  │
     *   │                      (items-center)                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger uses flex layout with proper alignment
     */
    test("trigger should have flex layout", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.trigger1).toHaveClass(/flex/);
      await expect(ui.trigger1).toHaveClass(/justify-between/);
      await expect(ui.trigger1).toHaveClass(/items-center/);
    });

    /**
     * TEST: Trigger Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ┌─────────────────────────────────────────────┐ │   │
     *   │   │ │         p-3 (padding: 0.75rem)              │ │   │
     *   │   │ │    Accordion item 01                     ▼  │ │   │
     *   │   │ └─────────────────────────────────────────────┘ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has correct padding class
     */
    test("trigger should have padding", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.trigger1).toHaveClass(/p-3/);
    });

    /**
     * TEST: Title Typography
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 class="text-sm font-medium">                      │
     *   │      Accordion item 01                                  │
     *   │   </h4>                                                 │
     *   │                                                         │
     *   │   text-sm:      font-size: 0.875rem                     │
     *   │   font-medium:  font-weight: 500                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has correct typography classes
     */
    test("title should have text-sm and font-medium", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const title = ui.item1.locator('[data-name="AccordionTitle"]');
      await expect(title).toHaveClass(/text-sm/);
      await expect(title).toHaveClass(/font-medium/);
    });

    /**
     * TEST: Description Muted Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  "This is the Accordion description..."       │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                         ↑                               │
     *   │   text-muted-foreground class for subtle styling        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description has muted foreground color class
     */
    test("description should have muted foreground color", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const description = ui.item1.locator('[data-name="AccordionDescription"]');
      await expect(description).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: Item Full Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ◄─────────────────── w-full ───────────────────►      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │               AccordionItem                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   width: 100% of parent container                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Accordion items span full width
     */
    test("item should have full width", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.item1).toHaveClass(/w-full/);
    });
  });

  /**
   * ANIMATION TESTS - Transition and grid-based collapse
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   GRID-BASED COLLAPSE ANIMATION:                                        │
   * │                                                                         │
   * │   <article class="grid transition-all duration-400">                    │
   * │     │                                                                   │
   * │     ├── CLOSED: grid-rows-[0fr]  ──►  height: 0                         │
   * │     │                                                                   │
   * │     └── OPEN:   grid-rows-[1fr]  ──►  height: auto                      │
   * │                                                                         │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   Frame 0%     Frame 50%      Frame 100%                    │       │
   * │   │   ┌────┐       ┌────────┐     ┌────────────────┐            │       │
   * │   │   │    │       │        │     │    Content     │            │       │
   * │   │   └────┘       │        │     │    visible     │            │       │
   * │   │   0fr          └────────┘     └────────────────┘            │       │
   * │   │                 0.5fr              1fr                      │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   CHEVRON ROTATION:                                                     │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │  <svg class="transition-all duration-300">                  │       │
   * │   │     │                                                       │       │
   * │   │     ├── CLOSED: rotate(0deg)    ▼                           │       │
   * │   │     │                                                       │       │
   * │   │     └── OPEN:   rotate(180deg)  ▲                           │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Animation", () => {
    /**
     * TEST: Content Transition Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <article class="transition-all duration-400">         │
     *   │     ...content...                                       │
     *   │   </article>                                            │
     *   │                                                         │
     *   │   transition-all: animate all properties                │
     *   │   duration-400:   400ms animation duration              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content wrapper has smooth transition classes
     */
    test("content wrapper should have transition classes", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // The RootContent article has transition-all
      const content = ui.item1.locator("article");
      await expect(content).toHaveClass(/transition-all/);
      await expect(content).toHaveClass(/duration-400/);
    });

    /**
     * TEST: Grid-Based Collapse Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CSS Grid technique for height animation:              │
     *   │                                                         │
     *   │   CLOSED: grid-rows-[0fr]     OPEN: grid-rows-[1fr]     │
     *   │   ┌────────────────────┐      ┌────────────────────┐    │
     *   │   │  (height: 0)       │      │  Content visible   │    │
     *   │   └────────────────────┘      │  with auto height  │    │
     *   │                               └────────────────────┘    │
     *   │                                                         │
     *   │   <article class="grid">...</article>                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content uses CSS grid for collapse animation
     */
    test("content should use grid for collapse animation", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const content = ui.item1.locator("article");
      await expect(content).toHaveClass(/grid/);
    });

    /**
     * TEST: Chevron Transition Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg class="transition-all duration-300">             │
     *   │     (chevron icon)                                      │
     *   │   </svg>                                                │
     *   │                                                         │
     *   │   ROTATION ANIMATION:                                   │
     *   │   ▼ (0°) ────── 300ms ──────► ▲ (180°)                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chevron icon has smooth rotation transition
     */
    test("chevron should have transition classes", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      const chevron = ui.trigger1.locator("svg");
      await expect(chevron).toHaveClass(/transition-all/);
      await expect(chevron).toHaveClass(/duration-300/);
    });

    /**
     * TEST: Article Has Overflow Hidden for CSS Grid Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <article class="grid overflow-hidden grid-rows-[0fr]">│
     *   │     <div class="min-h-[0]">                             │
     *   │       Content clipped by grid animation                 │
     *   │     </div>                                              │
     *   │   </article>                                            │
     *   │        ↑                                                │
     *   │   overflow-hidden on article prevents content           │
     *   │   from showing during grid-rows animation               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content wrapper (article) has overflow-hidden
     */
    test("content should have overflow-hidden for collapse", async ({
      page,
    }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // The article itself has overflow-hidden for the grid animation
      await expect(ui.content1).toHaveClass(/overflow-hidden/);
    });
  });

  /**
   * ACCESSIBILITY TESTS - Keyboard navigation and focus states
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   KEYBOARD NAVIGATION:                                                  │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   [Tab] ──► Focus moves to checkbox input                   │       │
   * │   │   [Space/Enter] ──► Toggle accordion item                   │       │
   * │   │   [Tab] ──► Focus moves to next item                        │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * │   FOCUS STATES:                                                         │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   input:focus-visible ──► label gets ring-2 via peer class  │       │
   * │   │                                                             │       │
   * │   │   ┌─────────────────────────────────────────┐               │       │
   * │   │   │ ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐ │ ← Focus ring  │       │
   * │   │   │   Accordion item 01                ▲   │               │       │
   * │   │   │ └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘ │               │       │
   * │   │   └─────────────────────────────────────────┘               │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Accessibility", () => {
    /**
     * TEST: Keyboard Focusable Trigger
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [Tab] key focuses the hidden checkbox:                │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┬─────┐     │
     *   │   │ ┊ Accordion item 01 (focused)        ┊  │  ▼  │     │
     *   │   └─────────────────────────────────────────┴─────┘     │
     *   │                     ↑                                   │
     *   │   <input type="checkbox" /> receives focus              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hidden checkbox input is focusable via keyboard
     */
    test("trigger should be focusable via keyboard", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Focus the hidden input (which is keyboard accessible)
      await ui.input1.focus();
      await expect(ui.input1).toBeFocused();
    });

    /**
     * TEST: Focus Visible Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus shows ring on trigger label:           │
     *   │                                                         │
     *   │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐          │
     *   │   ┊  ┌─────────────────────────────────┬───┐ ┊          │
     *   │   ┊  │  Accordion item 01              │ ▼ │ ┊          │
     *   │   ┊  └─────────────────────────────────┴───┘ ┊          │
     *   │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘          │
     *   │                       ↑                                 │
     *   │   peer-focus-visible:ring-2 shows focus ring            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger shows focus ring via peer class
     */
    test("trigger should have focus-visible ring styles", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.trigger1).toHaveClass(/peer-focus-visible:ring-2/);
    });

    /**
     * TEST: Accordion Item Full Width (A11y)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ◄─────────────────── w-full ───────────────────►      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  AccordionItem (full clickable area)            │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Full width ensures large touch/click target           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Item spans full width for accessibility
     */
    test("accordion item should have full width", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      await expect(ui.item1).toHaveClass(/w-full/);
    });
  });

  /**
   * KEYBOARD INTERACTION TESTS - Space and Enter key support
   *
   * ┌─────────────────────────────────────────────────────────────────────────┐
   * │                                                                         │
   * │   KEYBOARD TOGGLE FLOW:                                                 │
   * │                                                                         │
   * │   ┌─────────────────────────────────────────────────────────────┐       │
   * │   │                                                             │       │
   * │   │   1. [Tab] to focus input                                   │       │
   * │   │   ┌─────────────────────────────────┐                       │       │
   * │   │   │ ☐ Accordion item 02          ▼  │ ← Focused             │       │
   * │   │   └─────────────────────────────────┘                       │       │
   * │   │                                                             │       │
   * │   │   2. Press [Space] or [Enter]                               │       │
   * │   │   ┌─────────────────────────────────┐                       │       │
   * │   │   │ ☑ Accordion item 02          ▲  │ ← Toggled open        │       │
   * │   │   ├─────────────────────────────────┤                       │       │
   * │   │   │ Content now visible...          │                       │       │
   * │   │   └─────────────────────────────────┘                       │       │
   * │   │                                                             │       │
   * │   └─────────────────────────────────────────────────────────────┘       │
   * │                                                                         │
   * └─────────────────────────────────────────────────────────────────────────┘
   */
  test.describe("Keyboard Interaction", () => {
    /**
     * TEST: Space Key Toggle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   1. Focus input       2. Press [Space]                 │
     *   │   ┌─────────────────┐  ┌─────────────────┐              │
     *   │   │ ☐ Item 02    ▼  │  │ ☑ Item 02    ▲  │              │
     *   │   └─────────────────┘  ├─────────────────┤              │
     *   │   (focused, closed)    │ Content...      │  (open)      │
     *   │                        └─────────────────┘              │
     *   │                                                         │
     *   │   3. Press [Space] again                                │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ☐ Item 02    ▼  │  (closed again)                   │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Space key toggles accordion item state
     */
    test("should toggle item with Space key", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Focus on second item (starts closed)
      await ui.input2.focus();
      expect(await ui.isItemOpen(ui.input2)).toBe(false);

      // Press Space to toggle
      await page.keyboard.press("Space");
      expect(await ui.isItemOpen(ui.input2)).toBe(true);

      // Press Space again to close
      await page.keyboard.press("Space");
      expect(await ui.isItemOpen(ui.input2)).toBe(false);
    });

    /**
     * TEST: Tab Key Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [Tab] moves focus forward through items:              │
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 01       │  ← focused (start)                │
     *   │   └─────────────────┘                                   │
     *   │           │ [Tab]                                       │
     *   │           ▼                                             │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 02       │  ← focused                        │
     *   │   └─────────────────┘                                   │
     *   │           │ [Tab]                                       │
     *   │           ▼                                             │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 03       │  ← focused                        │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key moves focus to next accordion item
     */
    test("should navigate between items with Tab", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Focus first input
      await ui.input1.focus();
      await expect(ui.input1).toBeFocused();

      // Tab to second input
      await page.keyboard.press("Tab");
      await expect(ui.input2).toBeFocused();

      // Tab to third input
      await page.keyboard.press("Tab");
      await expect(ui.input3).toBeFocused();
    });

    /**
     * TEST: Shift+Tab Backward Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [Shift+Tab] moves focus backward through items:       │
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 03       │  ← focused (start)                │
     *   │   └─────────────────┘                                   │
     *   │           │ [Shift+Tab]                                 │
     *   │           ▼                                             │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 02       │  ← focused                        │
     *   │   └─────────────────┘                                   │
     *   │           │ [Shift+Tab]                                 │
     *   │           ▼                                             │
     *   │   ┌─────────────────┐                                   │
     *   │   │ ► Item 01       │  ← focused                        │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Shift+Tab moves focus to previous accordion item
     */
    test("should navigate backwards with Shift+Tab", async ({ page }) => {
      const ui = new AccordionPage(page);
      await ui.goto();

      // Focus third input
      await ui.input3.focus();
      await expect(ui.input3).toBeFocused();

      // Shift+Tab to second input
      await page.keyboard.press("Shift+Tab");
      await expect(ui.input2).toBeFocused();

      // Shift+Tab to first input
      await page.keyboard.press("Shift+Tab");
      await expect(ui.input1).toBeFocused();
    });
  });
});
