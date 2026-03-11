import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * POPOVER COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌──────────────────┐                                                  │
 * │   │  Open Popover    │  ← trigger button                                │
 * │   └──────────────────┘                                                  │
 * │            │                                                            │
 * │            │ click                                                      │
 * │            ▼                                                            │
 * │   ┌────────────────────────────────────────────────────────────────┐    │
 * │   │  [popover="auto"]                                              │    │
 * │   │  ┌──────────────────────────────────────────────────────────┐  │    │
 * │   │  │  Popover Demo                     ← PopoverTitle (h3)    │  │    │
 * │   │  │                                                          │  │    │
 * │   │  │  Interactive popover that adapts  ← PopoverDescription   │  │    │
 * │   │  │  to the viewport boundaries.                             │  │    │
 * │   │  └──────────────────────────────────────────────────────────┘  │    │
 * │   └────────────────────────────────────────────────────────────────┘    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Hidden:                                Visible:                       │
 * │   ┌──────────────────┐                   ┌──────────────────┐           │
 * │   │  Open Popover    │                   │  Open Popover    │           │
 * │   └──────────────────┘                   └──────────────────┘           │
 * │                                               │                         │
 * │   popover not visible                         ▼                         │
 * │                                          ┌────────────────┐             │
 * │                                          │  Popover Demo  │             │
 * │                                          │  Interactive...│             │
 * │                                          └────────────────┘             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Popover Content:                                                      │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .z-50       ← High stacking order                          │       │
 * │   │  .rounded-md ← Medium border radius                         │       │
 * │   │  .border     ← Border styling                               │       │
 * │   │  .shadow-md  ← Medium shadow                                │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class PopoverPage extends BasePage {
  protected readonly componentName = "popover";

  // Popover elements
  readonly trigger: Locator;
  readonly popoverContent: Locator;
  readonly popoverTitle: Locator;
  readonly popoverDescription: Locator;

  constructor(page: Page) {
    super(page);

    // Trigger button - scoped within preview
    this.trigger = this.preview.getByRole("button", { name: "Open Popover" });

    // Content - scoped within preview
    this.popoverContent = this.preview.locator('[popover="auto"]').first();
    this.popoverTitle = this.preview.locator('[data-name="PopoverTitle"]').first();
    this.popoverDescription = this.preview
      .locator('[data-name="PopoverDescription"]')
      .first();
  }

  async openPopover() {
    await this.trigger.click();
    await this.page.waitForTimeout(200);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Popover Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Trigger Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────┐                              │
     *   │   │    Open Popover      │  ← MUST BE VISIBLE           │
     *   │   └──────────────────────┘                              │
     *   │                                                         │
     *   │   role="button" name="Open Popover"                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button exists and is visible
     */
    test("should have trigger button", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: Trigger Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────┐                              │
     *   │   │   "Open Popover"     │  ← Text must match exactly   │
     *   │   └──────────────────────┘                              │
     *   │                                                         │
     *   │   Confirms button label for user interaction            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button displays "Open Popover" text
     */
    test("trigger should display 'Open Popover'", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveText("Open Popover");
    });

    /**
     * TEST: Popover Default Hidden State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Initial page load:                                    │
     *   │                                                         │
     *   │   ┌──────────────────────┐                              │
     *   │   │    Open Popover      │                              │
     *   │   └──────────────────────┘                              │
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                │
     *   │       Popover Demo                                      │
     *   │       Interactive popover...            ← NOT VISIBLE   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content is hidden on page load
     */
    test("popover should be hidden by default", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).not.toBeVisible();
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Clicking Trigger Opens Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before click:               After click:              │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │  Open Popover    │  click │  Open Popover    │      │
     *   │   └──────────────────┘  ────► └──────────────────┘      │
     *   │                                       │                 │
     *   │                               ┌───────▼──────────┐      │
     *   │                               │  Popover Demo    │      │
     *   │                               │  Interactive...  │      │
     *   │                               └──────────────────┘      │
     *   │                                       ↑                 │
     *   │                               NOW VISIBLE               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Click on trigger opens the popover content
     */
    test("clicking trigger should open popover", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await ui.openPopover();

      await expect(ui.popoverContent).toBeVisible();
    });

    /**
     * TEST: Clicking Trigger Again Closes Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open state:                 After 2nd click:          │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │  Open Popover    │  click │  Open Popover    │      │
     *   │   └──────────────────┘  ────► └──────────────────┘      │
     *   │   ┌──────────────────┐                                  │
     *   │   │  Popover Demo    │        (popover closed)          │
     *   │   │  Interactive...  │                                  │
     *   │   └──────────────────┘                                  │
     *   │                                                         │
     *   │   Toggle behavior: click to open, click again to close  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second click on trigger closes the popover
     */
    test("clicking trigger again should close popover", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await ui.openPopover();
      await ui.trigger.click();
      await page.waitForTimeout(200);

      await expect(ui.popoverContent).not.toBeVisible();
    });

    /**
     * TEST: ESC Key Closes Popover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open state:                 After ESC:                │
     *   │   ┌──────────────────┐        ┌──────────────────┐      │
     *   │   │  Open Popover    │        │  Open Popover    │      │
     *   │   └──────────────────┘        └──────────────────┘      │
     *   │   ┌──────────────────┐                                  │
     *   │   │  Popover Demo    │  [ESC]  (popover closed)         │
     *   │   │  Interactive...  │  ────►                           │
     *   │   └──────────────────┘                                  │
     *   │                                                         │
     *   │   Keyboard accessibility: ESC dismisses the popover     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressing Escape key closes the popover
     */
    test("pressing ESC should close popover", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await ui.openPopover();
      await page.keyboard.press("Escape");
      await page.waitForTimeout(200);

      await expect(ui.popoverContent).not.toBeVisible();
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Popover Title Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  "Popover Demo"  ← PopoverTitle (visible) │  │   │
     *   │   │  │                   text must match exactly │  │   │
     *   │   │  ├───────────────────────────────────────────┤  │   │
     *   │   │  │  Interactive popover...                   │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   [data-name="PopoverTitle"]                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverTitle is visible with "Popover Demo" text
     */
    test("should have title", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      await expect(ui.popoverTitle).toBeVisible();
      await expect(ui.popoverTitle).toHaveText("Popover Demo");
    });

    /**
     * TEST: Popover Description Visibility and Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  Popover Demo                             │  │   │
     *   │   │  ├───────────────────────────────────────────┤  │   │
     *   │   │  │  "Interactive popover that adapts"        │  │   │
     *   │   │  │       ↑                                   │  │   │
     *   │   │  │  PopoverDescription (visible)             │  │   │
     *   │   │  │  contains this text                       │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   [data-name="PopoverDescription"]                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverDescription is visible with expected text
     */
    test("should have description", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      await expect(ui.popoverDescription).toBeVisible();
      await expect(ui.popoverDescription).toContainText(
        "Interactive popover that adapts"
      );
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Trigger Button Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────┐                              │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░│  ← border class              │
     *   │   │░   Open Popover    ░│    (visible outline)          │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░│                              │
     *   │   └──────────────────────┘                              │
     *   │                                                         │
     *   │   class="border ..."                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button has border class
     */
    test("trigger should have border", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/border/);
    });

    /**
     * TEST: Trigger Button Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Square corners:          Rounded corners:             │
     *   │   ┌──────────────────┐     ╭──────────────────╮         │
     *   │   │  Open Popover    │     │  Open Popover    │         │
     *   │   └──────────────────┘     ╰──────────────────╯         │
     *   │                                   ↑                     │
     *   │                            rounded-md ← EXPECTED        │
     *   │                                                         │
     *   │   class="rounded-md ..."                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button has rounded-md class
     */
    test("trigger should have rounded corners", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Trigger Button Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────┐                              │
     *   │   │                      │  ← h-9 (height: 2.25rem)     │
     *   │   │    Open Popover      │    consistent button height  │
     *   │   │                      │                              │
     *   │   └──────────────────────┘                              │
     *   │          ↕ 36px                                         │
     *   │                                                         │
     *   │   class="h-9 ..."                                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button has h-9 height class
     */
    test("trigger should have h-9", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/h-9/);
    });

    /**
     * TEST: Popover Content Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭────────────────────────────────────────╮            │
     *   │   │  Popover Demo                          │            │
     *   │   │  Interactive popover...                │            │
     *   │   ╰────────────────────────────────────────╯            │
     *   │    ↑                                       ↑            │
     *   │    rounded-md corners                                   │
     *   │                                                         │
     *   │   class="rounded-md ..."                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content has rounded-md class
     */
    test("popover content should have rounded corners", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Popover Content Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────────────────────────┐            │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│            │
     *   │   │░  Popover Demo                       ░│            │
     *   │   │░  Interactive popover...             ░│            │
     *   │   │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│            │
     *   │   └────────────────────────────────────────┘            │
     *   │    ↑ border class (visible outline)                     │
     *   │                                                         │
     *   │   class="border ..."                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content has border class
     */
    test("popover content should have border", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).toHaveClass(/border/);
    });

    /**
     * TEST: Popover Content Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────────────────────────────────┐            │
     *   │   │  Popover Demo                          │            │
     *   │   │  Interactive popover...                │            │
     *   │   └────────────────────────────────────────┘            │
     *   │    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░           │
     *   │    ↑ shadow-md (medium drop shadow)                     │
     *   │                                                         │
     *   │   class="shadow-md ..."                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content has shadow-md class
     */
    test("popover content should have shadow", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).toHaveClass(/shadow-md/);
    });

    /**
     * TEST: Popover Content Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   z-index stacking:                                     │
     *   │                                                         │
     *   │   z-50    ┌────────────────────────┐ ← Popover on top   │
     *   │           │  Popover Demo          │                    │
     *   │           │  Interactive popover...│                    │
     *   │           └────────────────────────┘                    │
     *   │   z-auto  ┌────────────────────────┐                    │
     *   │           │    Page content        │                    │
     *   │           └────────────────────────┘                    │
     *   │                                                         │
     *   │   class="z-50 ..."                                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content has z-50 class
     */
    test("popover content should have z-50", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).toHaveClass(/z-50/);
    });
  });

  test.describe("Title Styling", () => {
    /**
     * TEST: Title Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │  **Popover Demo**  ← font-medium (500)    │  │   │
     *   │   │  │                      semi-bold text       │  │   │
     *   │   │  ├───────────────────────────────────────────┤  │   │
     *   │   │  │  Interactive popover...  (regular weight) │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   class="font-medium ..."                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverTitle has font-medium class
     */
    test("title should have font-medium", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      await expect(ui.popoverTitle).toHaveClass(/font-medium/);
    });

    /**
     * TEST: Title Semantic HTML Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Semantic structure:                                   │
     *   │                                                         │
     *   │   <div popover="auto">                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  <h3>Popover Demo</h3>  ← Must be H3 element    │   │
     *   │   │                          (semantic heading)     │   │
     *   │   │  <p>Interactive popover...</p>                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </div>                                                │
     *   │                                                         │
     *   │   Accessibility: proper heading hierarchy               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverTitle uses h3 element for semantics
     */
    test("title should be h3 element", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      const tagName = await ui.popoverTitle.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("h3");
    });
  });

  test.describe("Description Styling", () => {
    /**
     * TEST: Description Font Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Popover Demo (normal size)                     │   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │ Interactive popover that adapts...        │  │   │
     *   │   │  │         ↑                                 │  │   │
     *   │   │  │    text-sm (smaller font, 0.875rem)       │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   class="text-sm ..."                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverDescription has text-sm class
     */
    test("description should have text-sm", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      await expect(ui.popoverDescription).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Description Muted Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Popover Demo  (normal foreground color)        │   │
     *   │   │  ┌───────────────────────────────────────────┐  │   │
     *   │   │  │ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │  │   │
     *   │   │  │ ░ Interactive popover that adapts...     ░ │  │   │
     *   │   │  │ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │  │   │
     *   │   │  │         ↑                                 │  │   │
     *   │   │  │  text-muted-foreground (subdued gray)     │  │   │
     *   │   │  └───────────────────────────────────────────┘  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   class="text-muted-foreground ..."                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: PopoverDescription has text-muted-foreground class
     */
    test("description should have text-muted-foreground", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();
      await ui.openPopover();

      await expect(ui.popoverDescription).toHaveClass(/text-muted-foreground/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Trigger Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard navigation:                                  │
     *   │                                                         │
     *   │   [Tab] ────────────────────────────────►               │
     *   │                                                         │
     *   │   ╔══════════════════════╗                              │
     *   │   ║    Open Popover      ║  ← :focus (ring visible)     │
     *   │   ╚══════════════════════╝                              │
     *   │                                                         │
     *   │   Button must receive focus for keyboard users          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger button can receive keyboard focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await ui.trigger.focus();
      await expect(ui.trigger).toBeFocused();
    });

    /**
     * TEST: Trigger Type Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button type="button">  ← MUST have type="button"     │
     *   │   ┌──────────────────────┐                              │
     *   │   │    Open Popover      │                              │
     *   │   └──────────────────────┘                              │
     *   │   </button>                                             │
     *   │                                                         │
     *   │   Prevents form submission when inside <form>           │
     *   │   (default type="submit" would cause issues)            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has type="button" attribute
     */
    test("trigger should have type=button", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveAttribute("type", "button");
    });

    /**
     * TEST: Popover Auto Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Native Popover API:                                   │
     *   │                                                         │
     *   │   <div popover="auto">  ← MUST have this attribute      │
     *   │   ┌────────────────────────────────────────────────┐    │
     *   │   │  Popover Demo                                  │    │
     *   │   │  Interactive popover...                        │    │
     *   │   └────────────────────────────────────────────────┘    │
     *   │   </div>                                                │
     *   │                                                         │
     *   │   popover="auto" enables:                               │
     *   │   - Light dismiss (click outside closes)                │
     *   │   - ESC key closes                                      │
     *   │   - Top layer rendering                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Popover content uses native popover="auto"
     */
    test("popover should have popover=auto attribute", async ({ page }) => {
      const ui = new PopoverPage(page);
      await ui.goto();

      await expect(ui.popoverContent).toHaveAttribute("popover", "auto");
    });
  });
});
