import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * TABS COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Tabs"]  ← flex, flex-wrap container                       │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐             │   │
 * │   │  │   Tab One    │ │   Tab Two    │ │  Tab Three   │             │   │
 * │   │  │   (active)   │ │              │ │              │             │   │
 * │   │  └──────────────┘ └──────────────┘ └──────────────┘             │   │
 * │   │  ════════════════                                               │   │
 * │   │       ↑ Active tab has border-bottom indicator                  │   │
 * │   │                                                                 │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Tab Content (order-last, w-full)                         │  │   │
 * │   │  │  ┌─────────────────────────────────────────────────────┐  │  │   │
 * │   │  │  │  <h2> Tab One Content                               │  │  │   │
 * │   │  │  │  <p> Content paragraphs...                          │  │  │   │
 * │   │  │  └─────────────────────────────────────────────────────┘  │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM (CSS-only with radio inputs):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <input type="radio" class="hidden" name="tabs" />                     │
 * │         │                                                               │
 * │         ▼                                                               │
 * │   ┌─────────────┐                                                       │
 * │   │  :checked   │ ──► Tab label shows active state (border-primary)     │
 * │   └─────────────┘     Associated content becomes visible                │
 * │                                                                         │
 * │   Radio group ensures only one tab selected at a time                   │
 * │   name="tabs" ← All radios share this name                              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TAB STRUCTURE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   For each tab:                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │                                                             │       │
 * │   │   <input type="radio" id="tab-1" class="hidden" />          │       │
 * │   │   <label for="tab-1" class="tabs__item-label">              │       │
 * │   │      "Tab One"                                              │       │
 * │   │   </label>                                                  │       │
 * │   │   <div class="tabs__item-content order-last w-full">        │       │
 * │   │      Content for Tab One...                                 │       │
 * │   │   </div>                                                    │       │
 * │   │                                                             │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * │   Content visibility controlled by:                                     │
 * │   input:checked ~ .tabs__item-content { display: block }                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class TabsPage extends BasePage {
  protected readonly componentName = "tabs";

  // Tab triggers (labels)
  readonly tabOne: Locator;
  readonly tabTwo: Locator;
  readonly tabThree: Locator;

  // Tab radio inputs
  readonly tabOneInput: Locator;
  readonly tabTwoInput: Locator;
  readonly tabThreeInput: Locator;

  // Tab content areas
  readonly allContent: Locator;

  // Tabs container
  readonly tabsContainer: Locator;

  constructor(page: Page) {
    super(page);

    // Tab labels (clickable triggers) - scoped within preview
    this.tabOne = this.preview.locator("label.tabs__item-label").filter({ hasText: "Tab One" });
    this.tabTwo = this.preview.locator("label.tabs__item-label").filter({ hasText: "Tab Two" });
    this.tabThree = this.preview.locator("label.tabs__item-label").filter({ hasText: "Tab Three" });

    // Radio inputs (hidden) - scoped within preview
    this.tabOneInput = this.preview.locator('input[type="radio"].tabs__item-input').nth(0);
    this.tabTwoInput = this.preview.locator('input[type="radio"].tabs__item-input').nth(1);
    this.tabThreeInput = this.preview.locator('input[type="radio"].tabs__item-input').nth(2);

    // Content areas - scoped within preview
    this.allContent = this.preview.locator(".tabs__item-content");

    // Tabs container
    this.tabsContainer = this.firstByDataName("Tabs");
  }

  async selectTab(tabLabel: Locator) {
    await tabLabel.click();
  }

  async getActiveTabContent(): Promise<Locator> {
    // The visible content is controlled by CSS :checked + label + content
    // Find the content that's displayed (not hidden)
    return this.allContent.filter({ has: this.page.locator(":visible") }).first();
  }

  async isTabSelected(input: Locator): Promise<boolean> {
    return input.isChecked();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Tabs Page", () => {
  /**
   * STRUCTURE TESTS - Verify tabs elements
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Tabs container (flex, flex-wrap)                              │
   * │   ├── input[type="radio"] × 3 (hidden, name="tabs")             │
   * │   ├── label.tabs__item-label × 3 (clickable triggers)           │
   * │   └── .tabs__item-content × 3 (content areas, order-last)       │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Tabs Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tabs"]  ← MUST BE VISIBLE                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Tab One │ Tab Two │ Tab Three                    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The main tabs container element exists and renders
     */
    test("should have Tabs container", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabsContainer).toBeVisible();
    });

    /**
     * TEST: Three Tab Triggers Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐    │
     *   │   │   Tab One    │ │   Tab Two    │ │  Tab Three   │    │
     *   │   │   (label)    │ │   (label)    │ │   (label)    │    │
     *   │   └──────────────┘ └──────────────┘ └──────────────┘    │
     *   │         ↑                ↑                ↑             │
     *   │    MUST BE VISIBLE  MUST BE VISIBLE  MUST BE VISIBLE    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All three tab trigger labels are rendered and visible
     */
    test("should have three tab triggers", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOne).toBeVisible();
      await expect(ui.tabTwo).toBeVisible();
      await expect(ui.tabThree).toBeVisible();
    });

    /**
     * TEST: Hidden Radio Inputs Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   input[type="radio"].tabs__item-input                  │
     *   │   ┌─────────┐  ┌─────────┐  ┌─────────┐                 │
     *   │   │ ○ (1)   │  │ ○ (2)   │  │ ○ (3)   │  ← class="hidden"│
     *   │   └─────────┘  └─────────┘  └─────────┘                 │
     *   │        ↑                                                │
     *   │   COUNT === 3                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly three hidden radio inputs exist for tab state
     */
    test("should have three hidden radio inputs", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const inputs = page.locator('input[type="radio"].tabs__item-input');
      const count = await inputs.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: Radio Inputs Share Same Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Radio Group (name="tabs"):                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ○ Tab 1  name="tabs"                           │   │
     *   │   │  ○ Tab 2  name="tabs"  ← ALL SAME NAME          │   │
     *   │   │  ○ Tab 3  name="tabs"                           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   Ensures mutual exclusivity (only one selected)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All radio inputs share name="tabs" for proper grouping
     */
    test("radio inputs should have same name attribute", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOneInput).toHaveAttribute("name", "tabs");
      await expect(ui.tabTwoInput).toHaveAttribute("name", "tabs");
      await expect(ui.tabThreeInput).toHaveAttribute("name", "tabs");
    });

    /**
     * TEST: Three Content Areas Exist
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content (3 areas):                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Content 1  │  Content 2  │  Content 3          │   │
     *   │   │  (visible)  │  (hidden)   │  (hidden)           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   COUNT === 3                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Three content areas exist for each tab's content
     */
    test("should have three content areas", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const count = await ui.allContent.count();
      expect(count).toBe(3);
    });
  });

  test.describe("Initial State", () => {
    /**
     * TEST: First Tab Selected by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Initial state on page load:                           │
     *   │   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐    │
     *   │   │   Tab One    │ │   Tab Two    │ │  Tab Three   │    │
     *   │   │   ● CHECKED  │ │   ○ unchecked│ │   ○ unchecked│    │
     *   │   └──────────────┘ └──────────────┘ └──────────────┘    │
     *   │   ════════════════                                      │
     *   │         ↑                                               │
     *   │   isChecked() === true (others false)                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First tab is pre-selected, others are unselected
     */
    test("first tab should be selected by default", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      expect(await ui.isTabSelected(ui.tabOneInput)).toBe(true);
      expect(await ui.isTabSelected(ui.tabTwoInput)).toBe(false);
      expect(await ui.isTabSelected(ui.tabThreeInput)).toBe(false);
    });

    /**
     * TEST: First Tab Content Header Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tab One │ Tab Two │ Tab Three                         │
     *   │   ════════                                              │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h2>Tab One Content</h2>  ← MUST CONTAIN TEXT  │   │
     *   │   │  <p>Content paragraphs...</p>                   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First content area displays correct header text
     */
    test("first tab content should have Tab One header", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      // First content should contain Tab One Content
      const firstContent = ui.allContent.nth(0);
      await expect(firstContent.locator("h2")).toContainText("Tab One Content");
    });
  });

  test.describe("Tab Selection", () => {
    /**
     * TEST: Clicking Tab Two Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before:  Tab One (●)  Tab Two (○)  Tab Three (○)      │
     *   │                              ↓                          │
     *   │                           CLICK                         │
     *   │                              ↓                          │
     *   │   After:   Tab One (○)  Tab Two (●)  Tab Three (○)      │
     *   │                         ════════════                    │
     *   │                              ↑                          │
     *   │                    isChecked() === true                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking Tab Two changes selection state correctly
     */
    test("clicking Tab Two should select it", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await ui.selectTab(ui.tabTwo);

      expect(await ui.isTabSelected(ui.tabOneInput)).toBe(false);
      expect(await ui.isTabSelected(ui.tabTwoInput)).toBe(true);
      expect(await ui.isTabSelected(ui.tabThreeInput)).toBe(false);
    });

    /**
     * TEST: Clicking Tab Three Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before:  Tab One (●)  Tab Two (○)  Tab Three (○)      │
     *   │                                           ↓             │
     *   │                                        CLICK            │
     *   │                                           ↓             │
     *   │   After:   Tab One (○)  Tab Two (○)  Tab Three (●)      │
     *   │                                      ══════════════     │
     *   │                                           ↑             │
     *   │                                 isChecked() === true    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking Tab Three changes selection state correctly
     */
    test("clicking Tab Three should select it", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await ui.selectTab(ui.tabThree);

      expect(await ui.isTabSelected(ui.tabOneInput)).toBe(false);
      expect(await ui.isTabSelected(ui.tabTwoInput)).toBe(false);
      expect(await ui.isTabSelected(ui.tabThreeInput)).toBe(true);
    });

    /**
     * TEST: Can Switch Back to Tab One
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Initial    Tab One (●)  Tab Two (○)           │
     *   │                           ↓                             │
     *   │   Step 2: Click Two  Tab One (○)  Tab Two (●)           │
     *   │                           ↓                             │
     *   │   Step 3: Click One  Tab One (●)  Tab Two (○)           │
     *   │                      ════════════                       │
     *   │                           ↑                             │
     *   │           Can return to previous tab selection          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab selection can be changed back and forth
     */
    test("can switch back to Tab One", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      // Switch to Tab Two
      await ui.selectTab(ui.tabTwo);
      expect(await ui.isTabSelected(ui.tabTwoInput)).toBe(true);

      // Switch back to Tab One
      await ui.selectTab(ui.tabOne);
      expect(await ui.isTabSelected(ui.tabOneInput)).toBe(true);
      expect(await ui.isTabSelected(ui.tabTwoInput)).toBe(false);
    });

    /**
     * TEST: Only One Tab Selected at a Time
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Radio group behavior (mutual exclusivity):            │
     *   │   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐    │
     *   │   │   Tab One    │ │   Tab Two    │ │  Tab Three   │    │
     *   │   │      ○       │ │      ●       │ │      ○       │    │
     *   │   └──────────────┘ └──────────────┘ └──────────────┘    │
     *   │                                                         │
     *   │   input:checked COUNT === 1  (always exactly one)       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Radio group ensures single selection at any time
     */
    test("only one tab can be selected at a time", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      // Select each tab and verify only one is selected
      await ui.selectTab(ui.tabTwo);
      const checkedCount = await page.locator('input.tabs__item-input:checked').count();
      expect(checkedCount).toBe(1);
    });
  });

  test.describe("Tab Content", () => {
    /**
     * TEST: Tab One Content Has Multiple Paragraphs
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content.nth(0):                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h2>Tab One Content</h2>                       │   │
     *   │   │  <p>Paragraph 1...</p>                          │   │
     *   │   │  <p>Paragraph 2...</p>   ← COUNT >= 3           │   │
     *   │   │  <p>Paragraph 3...</p>                          │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First tab content has at least 3 paragraph elements
     */
    test("Tab One content should have multiple paragraphs", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const firstContent = ui.allContent.nth(0);
      const paragraphs = await firstContent.locator("p").count();
      expect(paragraphs).toBeGreaterThanOrEqual(3);
    });

    /**
     * TEST: Tab Two Content Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content.nth(1):                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h2>Tab Two Content</h2>  ← MUST CONTAIN TEXT  │   │
     *   │   │  <p>Content...</p>                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second content area has correct header text
     */
    test("Tab Two content should have header", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const secondContent = ui.allContent.nth(1);
      await expect(secondContent.locator("h2")).toContainText("Tab Two Content");
    });

    /**
     * TEST: Tab Three Content Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content.nth(2):                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h2>Tab Three Content</h2>  ← MUST CONTAIN TEXT│   │
     *   │   │  <p>Content...</p>                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third content area has correct header text
     */
    test("Tab Three content should have header", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const thirdContent = ui.allContent.nth(2);
      await expect(thirdContent.locator("h2")).toContainText("Tab Three Content");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Tabs Container Has Flex Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tabs"]  class="... flex ..."              │
     *   │   ┌──────────┬──────────┬──────────┐                    │
     *   │   │  Tab 1   │  Tab 2   │  Tab 3   │  ← flex children   │
     *   │   └──────────┴──────────┴──────────┘                    │
     *   │                  ↑                                      │
     *   │   class contains "flex" for horizontal layout           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container uses flexbox for tab layout
     */
    test("tabs container should have flex class", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabsContainer).toHaveClass(/flex/);
    });

    /**
     * TEST: Tabs Container Has Flex-Wrap Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tabs"]  class="... flex-wrap ..."         │
     *   │   ┌────────┬────────┬────────┐                          │
     *   │   │ Tab 1  │ Tab 2  │ Tab 3  │  ← tabs in row           │
     *   │   ├────────┴────────┴────────┤                          │
     *   │   │    Content (wraps)       │  ← content wraps below   │
     *   │   └──────────────────────────┘                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container allows content to wrap below tabs
     */
    test("tabs container should have flex-wrap class", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabsContainer).toHaveClass(/flex-wrap/);
    });

    /**
     * TEST: Tab Trigger Has Cursor-Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   label.tabs__item-label:                               │
     *   │   ┌──────────────┐                                      │
     *   │   │   Tab One    │  class="... cursor-pointer ..."      │
     *   │   └──────────────┘                                      │
     *   │         ↑                                               │
     *   │        👆 (pointer cursor on hover)                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab triggers show pointer cursor for clickability
     */
    test("tab trigger should have cursor-pointer", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOne).toHaveClass(/cursor-pointer/);
    });

    /**
     * TEST: Tab Trigger Has Font-Bold
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   label.tabs__item-label:                               │
     *   │   ┌──────────────┐                                      │
     *   │   │  Tab One     │  class="... font-bold ..."           │
     *   │   └──────────────┘                                      │
     *   │       ↑                                                 │
     *   │   Bold text for visual emphasis                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab labels have bold font weight
     */
    test("tab trigger should have font-bold", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOne).toHaveClass(/font-bold/);
    });

    /**
     * TEST: Tab Trigger Has Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   label.tabs__item-label:                               │
     *   │   ┌──────────────────────┐                              │
     *   │   │  ┌──────────────┐    │  ← p-4 (padding all sides)   │
     *   │   │  │   Tab One    │    │                              │
     *   │   │  └──────────────┘    │                              │
     *   │   └──────────────────────┘                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab labels have proper padding for touch targets
     */
    test("tab trigger should have padding", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOne).toHaveClass(/p-4/);
    });

    /**
     * TEST: Tab Trigger Has Border-Bottom Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   label.tabs__item-label:                               │
     *   │   ┌──────────────┐                                      │
     *   │   │   Tab One    │                                      │
     *   │   └──────────────┘                                      │
     *   │   ════════════════  ← border-b-[0.2rem]                 │
     *   │                     (active indicator line)             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab labels have bottom border for active indication
     */
    test("tab trigger should have border-bottom styling", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOne).toHaveClass(/border-b-\[0\.2rem\]/);
    });

    /**
     * TEST: Content Has Order-Last Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM Order:           Visual Order (with order-last):  │
     *   │   ┌─────────────┐      ┌─────────────────────────────┐  │
     *   │   │ input       │      │ Tab 1 │ Tab 2 │ Tab 3       │  │
     *   │   │ label       │  →   ├───────────────────────────────┤  │
     *   │   │ content     │      │ Content (order-last)        │  │
     *   │   │ input       │      └─────────────────────────────┘  │
     *   │   │ label       │                                       │
     *   │   │ content     │                                       │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content areas use order-last for proper layout
     */
    test("content should have order-last class", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const firstContent = ui.allContent.nth(0);
      await expect(firstContent).toHaveClass(/order-last/);
    });

    /**
     * TEST: Content Has Full Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content:                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Content spans full container width             │   │
     *   │   │◀───────────────── w-full ─────────────────────▶│   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content areas expand to full container width
     */
    test("content should have full width", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const firstContent = ui.allContent.nth(0);
      await expect(firstContent).toHaveClass(/w-full/);
    });
  });

  test.describe("Radio Input Behavior", () => {
    /**
     * TEST: Radio Inputs Are Hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   input[type="radio"].tabs__item-input:                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ○ (hidden)  class="... hidden ..."             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   Radio buttons are visually hidden but functional      │
     *   │   (CSS-only state management via :checked)              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Radio inputs are hidden from visual display
     */
    test("radio inputs should be hidden", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      await expect(ui.tabOneInput).toHaveClass(/hidden/);
    });

    /**
     * TEST: Label Click Checks Associated Radio
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <input id="tab-2" type="radio" />                     │
     *   │   <label for="tab-2">Tab Two</label>  ← CLICK           │
     *   │                  ↓                                      │
     *   │   label[for] === input[id]  ✓                           │
     *   │                  ↓                                      │
     *   │   input.isChecked() === true  ✓                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label-input association works via for/id attributes
     */
    test("clicking label should check the associated radio", async ({
      page,
    }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      // Get the id of the second radio input
      const tabTwoId = await ui.tabTwoInput.getAttribute("id");

      // Click the label
      await ui.tabTwo.click();

      // Verify the label's for attribute matches the input id
      await expect(ui.tabTwo).toHaveAttribute("for", tabTwoId!);

      // Verify the input is checked
      expect(await ui.tabTwoInput.isChecked()).toBe(true);
    });
  });

  test.describe("Content Headers", () => {
    /**
     * TEST: Content Headers Have Proper Typography
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .tabs__item-content h2:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Tab One Content                                │   │
     *   │   │  ^^^^^^^^^^^^^^^^                               │   │
     *   │   │  text-2xl (large text)                          │   │
     *   │   │  font-bold (bold weight)                        │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content headers have correct typography classes
     */
    test("content headers should have proper typography", async ({ page }) => {
      const ui = new TabsPage(page);
      await ui.goto();

      const header = ui.allContent.nth(0).locator("h2");
      await expect(header).toHaveClass(/text-2xl/);
      await expect(header).toHaveClass(/font-bold/);
    });
  });
});
