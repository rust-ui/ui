import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * PRESSABLE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="Pressable">                                           │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ┌───────────────────────────────────────────────────────┐     │   │
 * │   │   │  [Card]                                               │     │   │
 * │   │   │  ┌────────┐  ┌──────────────────────┐  ┌──────────┐   │     │   │
 * │   │   │  │  🔔    │  │ New message received │  │[Mark as  │   │     │   │
 * │   │   │  │  icon  │  │ 2 minutes ago        │  │  read]   │   │     │   │
 * │   │   │  └────────┘  └──────────────────────┘  └──────────┘   │     │   │
 * │   │   └───────────────────────────────────────────────────────┘     │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * PRESS ANIMATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Idle:                          Active (pressed):                      │
 * │   ┌───────────────────────┐      ┌─────────────────────┐                │
 * │   │                       │      │                     │                │
 * │   │     [Card content]    │      │   [Card content]    │                │
 * │   │                       │      │                     │                │
 * │   │   scale: 1            │      │   scale: 0.98       │                │
 * │   └───────────────────────┘      └─────────────────────┘                │
 * │                                                                         │
 * │   transition-transform: smooth scale animation on press                 │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .cursor-pointer       ← Clickable cursor                   │       │
 * │   │  .select-none          ← Prevent text selection             │       │
 * │   │  .transition-transform ← Smooth animation                   │       │
 * │   │  .active:scale-[0.98]  ← Scale down on press                │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class PressablePage extends BasePage {
  protected readonly componentName = "pressable";

  // Pressable elements
  readonly pressable: Locator;
  readonly card: Locator;
  readonly title: Locator;
  readonly subtitle: Locator;
  readonly button: Locator;
  readonly icon: Locator;

  constructor(page: Page) {
    super(page);

    // Main pressable - scoped within preview
    this.pressable = this.preview.locator('[data-name="Pressable"]').first();

    // Card inside pressable
    this.card = this.pressable.locator('[data-name="Card"]');

    // Content
    this.title = this.pressable.locator(".font-medium");
    this.subtitle = this.pressable.locator(".text-muted-foreground");

    // Icon and button - scoped within preview
    this.icon = this.pressable.locator("svg").first();
    this.button = this.preview.getByRole("button", { name: "Mark as read" });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Pressable Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have Pressable container
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Page                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ [Pressable] <── visible? ✓                        │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  (content inside)                           │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable container is visible on the page
     */
    test("should have Pressable container", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toBeVisible();
    });

    /**
     * TEST: Should have Pressable data-name attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="Pressable"> <── attribute check        │
     *   │       ↓                                                 │
     *   │  data-name === "Pressable" ✓                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable has correct data-name attribute
     */
    test("should have Pressable data-name attribute", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toHaveAttribute("data-name", "Pressable");
    });

    /**
     * TEST: Should contain a Card
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Card] <── visible? ✓                            │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  icon  title  subtitle  button              │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable contains a Card component
     */
    test("should contain a Card", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.card).toBeVisible();
    });

    /**
     * TEST: Should have notification icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌────────┐                                       │  │
     *   │  │  │  [svg] │  title  subtitle  button              │  │
     *   │  │  │   🔔   │                                       │  │
     *   │  │  └────────┘                                       │  │
     *   │  │       ↑                                           │  │
     *   │  │  visible? ✓                                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable has a visible notification icon
     */
    test("should have notification icon", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.icon).toBeVisible();
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Should have title text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable > Card]                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  icon  ┌──────────────────────────┐  button       │  │
     *   │  │        │ "New message received"   │               │  │
     *   │  │        │ ← title text ✓           │               │  │
     *   │  │        └──────────────────────────┘               │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title displays correct text content
     */
    test("should have title text", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.title).toHaveText("New message received");
    });

    /**
     * TEST: Should have subtitle text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable > Card]                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  icon  ┌──────────────────────────┐  button       │  │
     *   │  │        │ title                    │               │  │
     *   │  │        │ "2 minutes ago"          │               │  │
     *   │  │        │ ← subtitle text ✓        │               │  │
     *   │  │        └──────────────────────────┘               │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Subtitle displays correct timestamp text
     */
    test("should have subtitle text", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.subtitle).toHaveText("2 minutes ago");
    });

    /**
     * TEST: Should have action button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable > Card]                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  icon  title/subtitle   ┌─────────────────────┐   │  │
     *   │  │                         │ "Mark as read"      │   │  │
     *   │  │                         │  ← button ✓         │   │  │
     *   │  │                         └─────────────────────┘   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Action button is visible with correct text
     */
    test("should have action button", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.button).toBeVisible();
      await expect(ui.button).toHaveText("Mark as read");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Pressable should have cursor-pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]  class="... cursor-pointer ..."            │
     *   │                                                         │
     *   │       ┌─────────────────────────────────────┐           │
     *   │       │                                     │           │
     *   │       │     cursor: 👆 pointer              │           │
     *   │       │                                     │           │
     *   │       └─────────────────────────────────────┘           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable shows pointer cursor on hover
     */
    test("pressable should have cursor-pointer", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toHaveClass(/cursor-pointer/);
    });

    /**
     * TEST: Pressable should have select-none
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]  class="... select-none ..."               │
     *   │                                                         │
     *   │       ┌─────────────────────────────────────┐           │
     *   │       │  "New message received"             │           │
     *   │       │   ← text not selectable ✓           │           │
     *   │       │  user-select: none                  │           │
     *   │       └─────────────────────────────────────┘           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Text inside pressable cannot be selected
     */
    test("pressable should have select-none", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toHaveClass(/select-none/);
    });

    /**
     * TEST: Pressable should have active:scale styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]  class="... active:scale-[0.98] ..."       │
     *   │                                                         │
     *   │  Idle:                    Pressed:                      │
     *   │  ┌───────────────────┐    ┌─────────────────┐           │
     *   │  │                   │    │                 │           │
     *   │  │   scale: 1.0      │ →  │  scale: 0.98    │           │
     *   │  │                   │    │                 │           │
     *   │  └───────────────────┘    └─────────────────┘           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable scales down slightly when pressed
     */
    test("pressable should have active:scale styles", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toHaveClass(/active:scale-\[0.98\]/);
    });

    /**
     * TEST: Card should have flex layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Card]  class="... flex ..."                           │
     *   │                                                         │
     *   │  ┌────────┬──────────────────┬─────────────────┐        │
     *   │  │  icon  │  title/subtitle  │  button         │        │
     *   │  │ ←──────────────── flex ──────────────────→  │        │
     *   │  └────────┴──────────────────┴─────────────────┘        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card uses flexbox for horizontal layout
     */
    test("card should have flex layout", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.card).toHaveClass(/flex/);
    });

    /**
     * TEST: Card should have gap-3
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Card]  class="... gap-3 ..."                          │
     *   │                                                         │
     *   │  ┌────────┐   ┌────────────────┐   ┌─────────────┐      │
     *   │  │  icon  │←→│ title/subtitle │←→│  button     │      │
     *   │  └────────┘   └────────────────┘   └─────────────┘      │
     *   │           gap-3            gap-3                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card has gap-3 spacing between children
     */
    test("card should have gap-3", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.card).toHaveClass(/gap-3/);
    });

    /**
     * TEST: Card should have items-center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Card]  class="... items-center ..."                   │
     *   │                                                         │
     *   │  ┌────────┬──────────────────┬─────────────────┐        │
     *   │  │        │                  │                 │        │
     *   │  │  icon ─┼─ title/subtitle ─┼─ button         │ ← center│
     *   │  │        │                  │                 │        │
     *   │  └────────┴──────────────────┴─────────────────┘        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card children are vertically centered
     */
    test("card should have items-center", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.card).toHaveClass(/items-center/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Pressable should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │         ← click anywhere on pressable             │  │
     *   │  │                   ↓                               │  │
     *   │  │          no error thrown ✓                        │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Entire pressable area responds to clicks
     */
    test("pressable should be clickable", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await ui.pressable.click();
      // Should not throw error
    });

    /**
     * TEST: Button inside pressable should be clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  icon  title/subtitle   ┌─────────────────────┐   │  │
     *   │  │                         │ "Mark as read"      │   │  │
     *   │  │                         │       ← click       │   │  │
     *   │  │                         └─────────────────────┘   │  │
     *   │  │                                  ↓                │  │
     *   │  │                        no error thrown ✓          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Nested button inside pressable is independently clickable
     */
    test("button inside pressable should be clickable", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await ui.button.click();
      // Should not throw error
    });

    /**
     * TEST: Button should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                         ┌─────────────────────┐   │  │
     *   │  │                         │ "Mark as read"      │   │  │
     *   │  │                         │  ← focus() → ✓      │   │  │
     *   │  │                         └─────────────────────┘   │  │
     *   │  │                                                   │  │
     *   │  │                isFocused() === true ✓             │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button can receive keyboard focus
     */
    test("button should be focusable", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await ui.button.focus();
      await expect(ui.button).toBeFocused();
    });
  });

  test.describe("Press Animation", () => {
    /**
     * TEST: Should have transition classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [Pressable]  class="... transition-transform ..."      │
     *   │                                                         │
     *   │  Before press:              During press:               │
     *   │  ┌───────────────────┐      ┌─────────────────┐         │
     *   │  │    scale: 1.0     │ ───→ │   scale: 0.98   │         │
     *   │  └───────────────────┘      └─────────────────┘         │
     *   │                                                         │
     *   │  transition-transform enables smooth animation ✓        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Pressable has CSS transition for smooth scale animation
     */
    test("should have transition classes", async ({ page }) => {
      const ui = new PressablePage(page);
      await ui.goto();

      await expect(ui.pressable).toHaveClass(/transition-transform/);
    });
  });
});
