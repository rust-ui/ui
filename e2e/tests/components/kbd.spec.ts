import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * KBD COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="KbdGroup">                                            │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   <kbd data-name="Kbd">  <kbd>  <kbd>  <kbd>                    │   │
 * │   │   ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐                            │   │
 * │   │   │  ⌘  │  │  ⇧  │  │  ⌥  │  │  ⌃  │                            │   │
 * │   │   └─────┘  └─────┘  └─────┘  └─────┘                            │   │
 * │   │     ↑                                                           │   │
 * │   │   bordered, rounded, text-xs                                    │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * MAC KEY SYMBOLS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐                                    │
 * │   │  ⌘  │  │  ⇧  │  │  ⌥  │  │  ⌃  │                                    │
 * │   └─────┘  └─────┘  └─────┘  └─────┘                                    │
 * │   Command   Shift   Option  Control                                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * KEY COMBINATIONS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="KbdGroup">                                            │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ┌──────┐       ┌─────┐                                        │   │
 * │   │   │ Ctrl │   +   │  B  │                                        │   │
 * │   │   └──────┘       └─────┘                                        │   │
 * │   │       ↑      ↑       ↑                                          │   │
 * │   │     Kbd   separator  Kbd                                        │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .text-xs    ← Extra small text                             │       │
 * │   │  .border     ← Border around key                            │       │
 * │   │  .rounded    ← Rounded corners                              │       │
 * │   │  .px-1       ← Horizontal padding                           │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class KbdPage extends BasePage {
  protected readonly componentName = "kbd";

  // Kbd elements
  readonly kbdGroups: Locator;
  readonly kbdKeys: Locator;
  readonly firstGroup: Locator;
  readonly secondGroup: Locator;

  constructor(page: Page) {
    super(page);

    // Kbd groups - scoped within preview
    this.kbdGroups = this.preview.locator('[data-name="KbdGroup"]');
    this.firstGroup = this.kbdGroups.nth(0);
    this.secondGroup = this.kbdGroups.nth(1);

    // Individual keys - scoped within preview
    this.kbdKeys = this.preview.locator('[data-name="Kbd"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Kbd Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: KbdGroup Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="KbdGroup">  <-- Must be visible        │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                        │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │                        │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First KbdGroup container renders and is visible
     */
    test("should have KbdGroup components", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.firstGroup).toBeVisible();
    });

    /**
     * TEST: Two KbdGroups Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────────────────────────────────────────────────────┐
     *   │  KbdGroup #1 (Mac Keys)     KbdGroup #2 (Ctrl+B)         │
     *   │  ┌─────────────────────┐    ┌─────────────────────┐      │
     *   │  │ ⌘  ⇧  ⌥  ⌃         │    │ Ctrl + B            │      │
     *   │  └─────────────────────┘    └─────────────────────┘      │
     *   │           ↑                          ↑                   │
     *   │       count = 2 total KbdGroups                          │
     *   └───────────────────────────────────────────────────────────┘
     *
     *   Validates: Page contains exactly 2 KbdGroup containers
     */
    test("should have two KbdGroups", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const count = await ui.kbdGroups.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Multiple Kbd Components Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  All <kbd data-name="Kbd"> elements across groups:     │
     *   │                                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌──────┐ ┌─────┐      │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │ │ Ctrl │ │  B  │      │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └──────┘ └─────┘      │
     *   │    1       2       3       4        5       6           │
     *   │                                                         │
     *   │  Total count > 5                                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 5 individual Kbd key elements exist
     */
    test("should have Kbd components", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const count = await ui.kbdKeys.count();
      expect(count).toBeGreaterThan(5);
    });

    /**
     * TEST: Kbd Uses Semantic <kbd> Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  HTML Element Type:                                     │
     *   │                                                         │
     *   │  <kbd data-name="Kbd">⌘</kbd>                           │
     *   │    ↑                                                    │
     *   │  tagName === "kbd" (semantic keyboard element)          │
     *   │                                                         │
     *   │  NOT: <div>, <span>, or other generic elements          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component uses proper semantic <kbd> HTML tag
     */
    test("kbd should be a kbd element", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const tagName = await ui.kbdKeys
        .first()
        .evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("kbd");
    });
  });

  test.describe("First Group - Mac Keys", () => {
    /**
     * TEST: Command Key Symbol Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  First Group - Mac Key Symbols:                        │
     *   │                                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                        │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │                        │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘                        │
     *   │    ↑                                                    │
     *   │  nth(0) === "⌘" (Command)                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First key displays Command symbol (⌘)
     */
    test("should display Command key symbol", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.firstGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(0)).toHaveText("⌘");
    });

    /**
     * TEST: Shift Key Symbol Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  First Group - Mac Key Symbols:                        │
     *   │                                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                        │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │                        │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘                        │
     *   │           ↑                                             │
     *   │         nth(1) === "⇧" (Shift)                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second key displays Shift symbol (⇧)
     */
    test("should display Shift key symbol", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.firstGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(1)).toHaveText("⇧");
    });

    /**
     * TEST: Option Key Symbol Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  First Group - Mac Key Symbols:                        │
     *   │                                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                        │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │                        │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘                        │
     *   │                   ↑                                     │
     *   │                 nth(2) === "⌥" (Option)                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third key displays Option symbol (⌥)
     */
    test("should display Option key symbol", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.firstGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(2)).toHaveText("⌥");
    });

    /**
     * TEST: Control Key Symbol Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  First Group - Mac Key Symbols:                        │
     *   │                                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                        │
     *   │  │  ⌘  │ │  ⇧  │ │  ⌥  │ │  ⌃  │                        │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘                        │
     *   │                           ↑                             │
     *   │                         nth(3) === "⌃" (Control)        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fourth key displays Control symbol (⌃)
     */
    test("should display Control key symbol", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.firstGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(3)).toHaveText("⌃");
    });
  });

  test.describe("Second Group - Ctrl+B", () => {
    /**
     * TEST: Ctrl Key Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Second Group - Key Combination:                       │
     *   │                                                         │
     *   │  ┌──────┐     ┌─────┐                                   │
     *   │  │ Ctrl │  +  │  B  │                                   │
     *   │  └──────┘     └─────┘                                   │
     *   │    ↑                                                    │
     *   │  nth(0) === "Ctrl"                                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First key in combination shows "Ctrl" text
     */
    test("should display Ctrl key", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.secondGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(0)).toHaveText("Ctrl");
    });

    /**
     * TEST: Plus Separator Between Keys
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Second Group - Key Combination Separator:             │
     *   │                                                         │
     *   │  ┌──────┐     ┌─────┐                                   │
     *   │  │ Ctrl │  +  │  B  │                                   │
     *   │  └──────┘  ↑  └─────┘                                   │
     *   │            │                                            │
     *   │      Contains "+" separator text                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Group contains "+" separator between keys
     */
    test("should have plus separator", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.secondGroup).toContainText("+");
    });

    /**
     * TEST: B Key Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Second Group - Key Combination:                       │
     *   │                                                         │
     *   │  ┌──────┐     ┌─────┐                                   │
     *   │  │ Ctrl │  +  │  B  │                                   │
     *   │  └──────┘     └─────┘                                   │
     *   │                 ↑                                       │
     *   │               nth(1) === "B"                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second key in combination shows "B" text
     */
    test("should display B key", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      const keys = ui.secondGroup.locator('[data-name="Kbd"]');
      await expect(keys.nth(1)).toHaveText("B");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Kbd Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Kbd Element Styling:                                  │
     *   │                                                         │
     *   │  ╭─────╮  <-- rounded corners (border-radius)          │
     *   │  │  ⌘  │                                                │
     *   │  ╰─────╯                                                │
     *   │                                                         │
     *   │  class includes: "rounded"                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Kbd elements have rounded corner styling
     */
    test("kbd should have rounded corners", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.kbdKeys.first()).toHaveClass(/rounded/);
    });

    /**
     * TEST: Kbd Border Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Kbd Element Border:                                   │
     *   │                                                         │
     *   │  ┌─────┐  <-- visible border around key                │
     *   │  │  ⌘  │                                                │
     *   │  └─────┘                                                │
     *   │                                                         │
     *   │  class includes: "border"                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Kbd elements have border class applied
     */
    test("kbd should have border", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.kbdKeys.first()).toHaveClass(/border/);
    });

    /**
     * TEST: Kbd Text Size (Extra Small)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Kbd Element Text Size:                                │
     *   │                                                         │
     *   │  ┌─────┐                                                │
     *   │  │  ⌘  │  <-- text-xs (extra small, ~12px)             │
     *   │  └─────┘                                                │
     *   │                                                         │
     *   │  class includes: "text-xs"                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Kbd elements use extra small text size
     */
    test("kbd should have text-xs", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.kbdKeys.first()).toHaveClass(/text-xs/);
    });

    /**
     * TEST: Kbd Horizontal Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Kbd Element Padding:                                  │
     *   │                                                         │
     *   │  ┌─────────┐                                            │
     *   │  │ ← ⌘ → │  <-- px-1 (horizontal padding)              │
     *   │  └─────────┘                                            │
     *   │    ↑   ↑                                                │
     *   │  padding-left/right: 0.25rem                            │
     *   │                                                         │
     *   │  class includes: "px-1"                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Kbd elements have horizontal padding (px-1)
     */
    test("kbd should have px-1", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.kbdKeys.first()).toHaveClass(/px-1/);
    });

    /**
     * TEST: KbdGroup Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  KbdGroup Container Layout:                            │
     *   │                                                         │
     *   │  ┌─────────────────────────────────────────┐            │
     *   │  │ ┌───┐ ┌───┐ ┌───┐ ┌───┐                 │            │
     *   │  │ │ ⌘ │ │ ⇧ │ │ ⌥ │ │ ⌃ │  ← flex row    │            │
     *   │  │ └───┘ └───┘ └───┘ └───┘                 │            │
     *   │  └─────────────────────────────────────────┘            │
     *   │                                                         │
     *   │  class includes: "flex"                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: KbdGroup uses flexbox layout
     */
    test("kbd group should have flex layout", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.firstGroup).toHaveClass(/flex/);
    });

    /**
     * TEST: KbdGroup Items Center Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  KbdGroup Vertical Alignment:                          │
     *   │                                                         │
     *   │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  (center axis)             │
     *   │      ┌───┐ ┌───┐ ┌───┐ ┌───┐                            │
     *   │      │ ⌘ │ │ ⇧ │ │ ⌥ │ │ ⌃ │  ← vertically centered    │
     *   │      └───┘ └───┘ └───┘ └───┘                            │
     *   │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                             │
     *   │                                                         │
     *   │  class includes: "items-center"                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: KbdGroup centers items vertically
     */
    test("kbd group should have items-center", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.firstGroup).toHaveClass(/items-center/);
    });

    /**
     * TEST: KbdGroup Gap Spacing
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  KbdGroup Element Spacing:                             │
     *   │                                                         │
     *   │  ┌───┐ ┌───┐ ┌───┐ ┌───┐                                │
     *   │  │ ⌘ │ │ ⇧ │ │ ⌥ │ │ ⌃ │                                │
     *   │  └───┘ └───┘ └───┘ └───┘                                │
     *   │      ↑     ↑     ↑                                      │
     *   │    gap-0.5 between each element                         │
     *   │                                                         │
     *   │  class includes: "gap-0.5"                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: KbdGroup has small gap between key elements
     */
    test("kbd group should have gap", async ({ page }) => {
      const ui = new KbdPage(page);
      await ui.goto();

      await expect(ui.firstGroup).toHaveClass(/gap-0.5/);
    });
  });
});
