import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * BADGE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <span data-name="Badge">                                              │
 * │   ┌─────────────┐                                                       │
 * │   │   Default   │  ← Inline-flex, rounded-md, font-semibold             │
 * │   └─────────────┘                                                       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:      ┌─────────────┐  bg-primary text-primary-foreground     │
 * │                 │   Default   │  border-transparent shadow              │
 * │                 └─────────────┘                                         │
 * │                                                                         │
 * │   Secondary:    ┌─────────────┐  bg-secondary text-secondary-foreground │
 * │                 │  Secondary  │  border-transparent                     │
 * │                 └─────────────┘                                         │
 * │                                                                         │
 * │   Outline:      ┌─────────────┐  border text-foreground                 │
 * │                 │   Outline   │  (no background)                        │
 * │                 └─────────────┘                                         │
 * │                                                                         │
 * │   Destructive:  ┌─────────────┐  bg-destructive text-destructive-fg     │
 * │                 │ Destructive │  border-transparent shadow              │
 * │                 └─────────────┘                                         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SIZING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default size:                                                         │
 * │   ┌──────────────────────────────────────────────┐                      │
 * │   │  text-xs                                     │                      │
 * │   │  px-2.5 (horizontal padding)                 │                      │
 * │   │  py-0.5 (vertical padding)                   │                      │
 * │   │  w-fit (intrinsic width)                     │                      │
 * │   └──────────────────────────────────────────────┘                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class BadgePage extends BasePage {
  protected readonly componentName = "badge";

  // Default badge
  readonly defaultBadge: Locator;

  // All badges on page
  readonly allBadges: Locator;

  constructor(page: Page) {
    super(page);

    // Main badge - scoped within preview
    this.defaultBadge = this.preview.locator('[data-name="Badge"]').first();

    // All badges - scoped within preview
    this.allBadges = this.preview.locator('[data-name="Badge"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Badge Page", () => {
  /**
   * STRUCTURE TESTS - Verify basic element structure
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <span data-name="Badge">                                      │
   * │   ┌─────────────┐                                               │
   * │   │   Default   │  ← Text content inside span                   │
   * │   └─────────────┘                                               │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Badge Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Badge"]                                   │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← MUST BE VISIBLE                    │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge component renders and is visible on the page
     */
    test("should have Badge component", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toBeVisible();
    });

    /**
     * TEST: Badge Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span data-name="Badge">                              │
     *   │         ├────────┬─────┤                                │
     *   │                  ↓                                      │
     *   │   Attribute must equal "Badge"                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has correct data-name for component identification
     */
    test("should have Badge data-name attribute", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveAttribute("data-name", "Badge");
    });

    /**
     * TEST: Badge Is Span Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span>  ← tagName must be "span"                      │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │   Inline element for labels           │
     *   │   └─────────────┘                                       │
     *   │   </span>                                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge uses inline span element
     */
    test("badge should be a span element", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      const tagName = await ui.defaultBadge.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("span");
    });

    /**
     * TEST: Badge Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │                                       │
     *   │   └─────────────┘                                       │
     *   │         ↑                                               │
     *   │   Text must be "Default"                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default badge displays correct text content
     */
    test("default badge should have text content", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveText("Default");
    });
  });

  /**
   * BASE STYLING TESTS - Common styles across all variants
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  .inline-flex        ← Display mode                     │   │
   * │   │  .items-center       ← Vertical alignment               │   │
   * │   │  .font-semibold      ← Text weight                      │   │
   * │   │  .rounded-md         ← Corner radius                    │   │
   * │   │  .border             ← Border (color varies by variant) │   │
   * │   │  .transition-colors  ← Smooth color transitions         │   │
   * │   │  .w-fit              ← Width fits content               │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Base Styling", () => {
    /**
     * TEST: Badge Inline-Flex Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Text ┌─────────────┐ more text                        │
     *   │        │   Badge     │                                  │
     *   │        └─────────────┘                                  │
     *   │             ↑                                           │
     *   │   inline-flex: Flows inline with text, flexbox inside   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge uses inline-flex for inline flow with flex layout
     */
    test("should have inline-flex display", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/inline-flex/);
    });

    /**
     * TEST: Badge Vertical Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← Content vertically centered        │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   items-center: align-items: center on flex container   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge content is vertically centered
     */
    test("should have items-center alignment", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/items-center/);
    });

    /**
     * TEST: Badge Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← font-weight: 600 (semibold)        │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   Normal:    Default                                    │
     *   │   Semibold:  Default  ← Bolder, more emphasis           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge text has semibold font weight for emphasis
     */
    test("should have font-semibold", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/font-semibold/);
    });

    /**
     * TEST: Badge Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭─────────────╮                                       │
     *   │   │   Default   │  ← rounded-md = 6px border-radius     │
     *   │   ╰─────────────╯                                       │
     *   │   ↑             ↑                                       │
     *   │   Slightly rounded corners                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has medium rounded corners
     */
    test("should have rounded-md corners", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Badge Has Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐                                       │
     *   │   │   Default   │  ← border (color varies by variant)   │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘                                       │
     *   │                                                         │
     *   │   Border width: 1px (default)                           │
     *   │   Border color: depends on variant                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has border styling
     */
    test("should have border", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/border/);
    });

    /**
     * TEST: Badge Color Transitions
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Normal State:      Hover State:                       │
     *   │   ┌─────────────┐    ┌─────────────┐                    │
     *   │   │   Default   │ →→ │   Default   │                    │
     *   │   └─────────────┘    └─────────────┘                    │
     *   │   (color A)    ~~~>  (color B)                          │
     *   │                                                         │
     *   │   transition-colors: smooth 150ms color animation       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has smooth color transitions for hover states
     */
    test("should have transition-colors", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/transition-colors/);
    });

    /**
     * TEST: Badge Intrinsic Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────┐  ┌─────────────────┐                        │
     *   │   │ Short │  │ Longer Content  │                        │
     *   │   └───────┘  └─────────────────┘                        │
     *   │       ↑               ↑                                 │
     *   │   w-fit: width adjusts to content                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge width fits its content
     */
    test("should have w-fit for intrinsic width", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/w-fit/);
    });
  });

  /**
   * DEFAULT VARIANT TESTS - Primary colored badge
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────┐                                               │
   * │   │   Default   │  ← bg-primary, text-primary-foreground        │
   * │   └─────────────┘    shadow, border-transparent                 │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Default Variant", () => {
    /**
     * TEST: Default Variant Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │█████████████│  ← bg-primary (filled background)     │
     *   │   │   Default   │                                       │
     *   │   │█████████████│                                       │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   Primary color: typically dark/brand color             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default badge has primary background color
     */
    test("should have primary background", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/bg-primary/);
    });

    /**
     * TEST: Default Variant Text Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← text-primary-foreground            │
     *   │   └─────────────┘    (contrasting text on primary bg)   │
     *   │                                                         │
     *   │   Ensures readable text on dark background              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default badge has contrasting text color
     */
    test("should have primary-foreground text", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/text-primary-foreground/);
    });

    /**
     * TEST: Default Variant Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │                                       │
     *   │   └─────────────┘                                       │
     *   │    ░░░░░░░░░░░░░  ← shadow (subtle elevation)           │
     *   │                                                         │
     *   │   Adds depth and visual prominence                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default badge has subtle shadow
     */
    test("should have shadow", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/shadow/);
    });

    /**
     * TEST: Default Variant Transparent Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← border-transparent                 │
     *   │   └─────────────┘    (border exists but invisible)      │
     *   │                                                         │
     *   │   Border is present for consistent sizing across        │
     *   │   variants but not visible on default variant           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default badge has invisible border
     */
    test("should have border-transparent", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/border-transparent/);
    });
  });

  /**
   * DEFAULT SIZE TESTS - Compact sizing
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────┐                                               │
   * │   │   Default   │  ← text-xs (12px)                             │
   * │   └─────────────┘    px-2.5 (10px horizontal)                   │
   * │                      py-0.5 (2px vertical)                      │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Default Size", () => {
    /**
     * TEST: Badge Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │   Default   │  ← text-xs (12px / 0.75rem)           │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   text-sm:   Default   (14px - larger)                  │
     *   │   text-xs:   Default   (12px - compact)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge uses extra-small text for compact appearance
     */
    test("should have text-xs", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/text-xs/);
    });

    /**
     * TEST: Badge Horizontal Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────┐                               │
     *   │   │←2.5→│ Default │←2.5→│                               │
     *   │   │10px │         │10px │                               │
     *   │   └─────────────────────┘                               │
     *   │                                                         │
     *   │   px-2.5 = 10px horizontal padding on each side         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has 10px horizontal padding
     */
    test("should have px-2.5 padding", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/px-2\.5/);
    });

    /**
     * TEST: Badge Vertical Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────┐                                       │
     *   │   │ ↑ 0.5 (2px) │                                       │
     *   │   │   Default   │                                       │
     *   │   │ ↓ 0.5 (2px) │                                       │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   py-0.5 = 2px vertical padding (compact height)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has minimal 2px vertical padding
     */
    test("should have py-0.5 padding", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/py-0\.5/);
    });
  });

  /**
   * ACCESSIBILITY TESTS - Focus states
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Focus State:                                                  │
   * │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐                                  │
   * │   ┊  ┌─────────────────────┐ ┊  ← focus:ring-2                  │
   * │   ┊  │      Default        │ ┊    focus:ring-ring               │
   * │   ┊  └─────────────────────┘ ┊    focus:ring-offset-2           │
   * │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘    focus:outline-hidden           │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Accessibility", () => {
    /**
     * TEST: Focus Outline Hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Browser default:         With focus:outline-hidden:   │
     *   │   ┌─────────────┐          ┌─────────────┐              │
     *   │   ║   Default   ║          │   Default   │              │
     *   │   └─────────────┘          └─────────────┘              │
     *   │        ↑                                                │
     *   │   Ugly browser outline     No outline (uses ring)       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default browser focus outline is hidden
     */
    test("should have focus:outline-hidden", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/focus:outline-hidden/);
    });

    /**
     * TEST: Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Focused State:                                        │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                                │
     *   │   ┊  ┌─────────────┐  ┊  ← focus:ring-2 (2px ring)      │
     *   │   ┊  │   Default   │  ┊    focus:ring-ring (color)      │
     *   │   ┊  └─────────────┘  ┊                                 │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Badge has visible focus ring for accessibility
     */
    test("should have focus ring styles", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/focus:ring-2/);
      await expect(ui.defaultBadge).toHaveClass(/focus:ring-ring/);
    });

    /**
     * TEST: Focus Ring Offset
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐                                │
     *   │   ┊                   ┊  ← 2px gap between ring         │
     *   │   ┊  ┌─────────────┐  ┊    and badge border             │
     *   │   ┊  │   Default   │  ┊                                 │
     *   │   ┊  └─────────────┘  ┊    focus:ring-offset-2          │
     *   │   ┊                   ┊                                 │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Focus ring has offset from badge border
     */
    test("should have focus ring offset", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto();

      await expect(ui.defaultBadge).toHaveClass(/focus:ring-offset-2/);
    });
  });

  /**
   * VARIANTS SECTION TESTS - Multiple badge types
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────┐ ┌───────────┐ ┌─────────┐ ┌─────────────┐         │
   * │   │ Default │ │ Secondary │ │ Outline │ │ Destructive │         │
   * │   └─────────┘ └───────────┘ └─────────┘ └─────────────┘         │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Variants Section", () => {
    /**
     * TEST: Multiple Badge Variants
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Variants page shows multiple badge types:             │
     *   │                                                         │
     *   │   ┌─────────┐ ┌───────────┐ ┌─────────┐ ┌─────────────┐ │
     *   │   │ Default │ │ Secondary │ │ Outline │ │ Destructive │ │
     *   │   └─────────┘ └───────────┘ └─────────┘ └─────────────┘ │
     *   │        1           2            3             4         │
     *   │                                                         │
     *   │   Count must be > 1                                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Variants page displays multiple badge variants
     */
    test("should have multiple badges on variants page", async ({ page }) => {
      const ui = new BadgePage(page);
      await ui.goto("variants");

      const count = await ui.allBadges.count();
      expect(count).toBeGreaterThan(1);
    });
  });
});
