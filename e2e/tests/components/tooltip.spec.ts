import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * TOOLTIP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Tooltip"]  ← Wrapper (group/tooltip, relative)            │
 * │   ┌───────────────────────────────────────────────────────────────────┐ │
 * │   │                                                                   │ │
 * │   │                    TooltipContent (absolute)                      │ │
 * │   │                    ┌─────────────────────┐                        │ │
 * │   │                    │   "TOP"             │                        │ │
 * │   │                    └─────────────────────┘                        │ │
 * │   │                            ▼ TooltipArrow                         │ │
 * │   │                    ┌───────────────┐                              │ │
 * │   │                    │    Trigger    │  ← Button with hover         │ │
 * │   │                    │      ▲        │                              │ │
 * │   │                    └───────────────┘                              │ │
 * │   │                                                                   │ │
 * │   └───────────────────────────────────────────────────────────────────┘ │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * POSITION VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   TOP:                           BOTTOM:                                │
 * │   ┌───────────┐                  ┌───────────┐                          │
 * │   │   "TOP"   │                  │  Trigger  │                          │
 * │   └─────▼─────┘                  └───────────┘                          │
 * │   ┌───────────┐                  ┌─────▲─────┐                          │
 * │   │  Trigger  │                  │ "BOTTOM"  │                          │
 * │   └───────────┘                  └───────────┘                          │
 * │                                                                         │
 * │   LEFT:                          RIGHT:                                 │
 * │   ┌────────┐┌───────────┐        ┌───────────┐┌────────┐                │
 * │   │ "LEFT" ││  Trigger  │        │  Trigger  ││ "RIGHT"│                │
 * │   └────────┘└───────────┘        └───────────┘└────────┘                │
 * │            ▶                                  ◀                         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * HOVER BEHAVIOR (CSS-only):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   DEFAULT STATE:                 HOVER STATE:                           │
 * │   opacity-0                      group-hover/tooltip:opacity-100        │
 * │   pointer-events-none            (content becomes visible)              │
 * │                                                                         │
 * │   ┌───────────┐                  ┌───────────┐                          │
 * │   │  Trigger  │  ─── hover ───►  │  "TOP"    │                          │
 * │   └───────────┘                  └─────▼─────┘                          │
 * │                                  ┌───────────┐                          │
 * │                                  │  Trigger  │                          │
 * │                                  └───────────┘                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class TooltipPage extends BasePage {
  protected readonly componentName = "tooltip";

  // Tooltip containers
  readonly allTooltips: Locator;
  readonly firstTooltip: Locator;

  // Tooltip parts
  readonly firstTooltipContent: Locator;
  readonly firstTooltipArrow: Locator;

  // Trigger buttons
  readonly allTriggerButtons: Locator;

  constructor(page: Page) {
    super(page);

    // Tooltips - scoped within preview
    this.allTooltips = this.preview.locator('[data-name="Tooltip"]');
    this.firstTooltip = this.preview.locator('[data-name="Tooltip"]').first();

    // Content and arrow
    this.firstTooltipContent = this.firstTooltip.locator(
      '[data-name="TooltipContent"]'
    );
    this.firstTooltipArrow = this.firstTooltip.locator(
      '[data-name="TooltipArrow"]'
    );

    // Trigger buttons - scoped within preview
    this.allTriggerButtons = this.preview.locator('[data-name="Tooltip"] button');
  }

  async hoverTooltip(tooltip: Locator) {
    await tooltip.hover();
  }

  async unhoverTooltip() {
    // Move mouse away from tooltip
    await this.page.mouse.move(0, 0);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Tooltip Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Tooltips Work Without Provider (CSS-Only)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipProvider is no longer needed - tooltips work   │
     *   │   with pure CSS via Tailwind's group-hover.             │
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Tooltip 1   Tooltip 2   Tooltip 3   Tooltip 4    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Validates: Tooltips work without JavaScript provider  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     */
    test("should have tooltips without requiring provider", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Tooltips work with pure CSS - no provider element needed
      const count = await ui.allTooltips.count();
      expect(count).toBeGreaterThanOrEqual(4);
    });

    /**
     * TEST: Multiple Tooltips Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
     *   │   │Tooltip 1│ │Tooltip 2│ │Tooltip 3│ │Tooltip 4│       │
     *   │   │  TOP    │ │  LEFT   │ │  RIGHT  │ │ BOTTOM  │       │
     *   │   └─────────┘ └─────────┘ └─────────┘ └─────────┘       │
     *   │        ↑                                                │
     *   │   count >= 4  (at least 4 tooltips must exist)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page renders at least 4 tooltip components
     */
    test("should have multiple tooltips", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const count = await ui.allTooltips.count();
      expect(count).toBeGreaterThanOrEqual(4);
    });

    /**
     * TEST: Tooltip Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="Tooltip">  ← MUST HAVE THIS ATTR      │
     *   │   ┌───────────────────────┐                             │
     *   │   │   [Button Trigger]    │                             │
     *   │   │   [TooltipContent]    │                             │
     *   │   └───────────────────────┘                             │
     *   │   </div>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First tooltip has correct data-name attribute
     */
    test("tooltip should have Tooltip data-name", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltip).toHaveAttribute("data-name", "Tooltip");
    });

    /**
     * TEST: Button Trigger Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"]                                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │     <button>    │  ← MUST BE VISIBLE          │ │
     *   │   │   │       ▲         │    (hover trigger)          │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip contains a visible button trigger
     */
    test("tooltip should contain a button trigger", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const button = ui.firstTooltip.locator("button");
      await expect(button).toBeVisible();
    });

    /**
     * TEST: TooltipContent Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"]                                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   [data-name="TooltipContent"]  ← MUST EXIST      │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │    "TOP"        │  (hidden until hover)       │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent element is attached to DOM
     */
    test("tooltip should contain TooltipContent", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toBeAttached();
    });

    /**
     * TEST: TooltipArrow Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"]                                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │    "TOP"        │                             │ │
     *   │   │   └────────▼────────┘                             │ │
     *   │   │            ▼  ← [data-name="TooltipArrow"]        │ │
     *   │   │   ┌─────────────────┐    MUST EXIST               │ │
     *   │   │   │    [Button]     │                             │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow element is attached to DOM
     */
    test("tooltip should contain TooltipArrow", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toBeAttached();
    });
  });

  test.describe("Tooltip Wrapper Styling", () => {
    /**
     * TEST: Inline-Block Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Block:                    Inline-block:               │
     *   │   ┌────────────────────┐    ┌────────┐ ┌────────┐       │
     *   │   │      Tooltip       │    │Tooltip1│ │Tooltip2│       │
     *   │   └────────────────────┘    └────────┘ └────────┘       │
     *   │   (full width)              (side by side) ← EXPECTED   │
     *   │                                                         │
     *   │   class="inline-block ..."                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip wrapper has inline-block display class
     */
    test("tooltip should have inline-block display", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltip).toHaveClass(/inline-block/);
    });

    /**
     * TEST: Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"]  class="relative ..."           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │   position: relative  ← REQUIRED for children      │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │  TooltipContent │  position: absolute         │ │
     *   │   │   └─────────────────┘  (relative to parent)       │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │     Button      │                             │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip wrapper has relative positioning class
     */
    test("tooltip should have relative positioning", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltip).toHaveClass(/relative/);
    });

    /**
     * TEST: Transition Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tooltip wrapper needs smooth transitions:             │
     *   │                                                         │
     *   │   class="transition-all duration-300 ..."               │
     *   │            │               │                            │
     *   │            │               └── 300ms animation          │
     *   │            └── all properties transition                │
     *   │                                                         │
     *   │   Before hover ──(300ms)──► After hover                 │
     *   │   opacity: 0               opacity: 1                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip has transition-all and duration-300 classes
     */
    test("tooltip should have transition classes", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltip).toHaveClass(/transition-all/);
      await expect(ui.firstTooltip).toHaveClass(/duration-300/);
    });

    /**
     * TEST: Group Class for Hover States
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tailwind group hover pattern:                         │
     *   │                                                         │
     *   │   <div class="group/tooltip">  ← PARENT GROUP           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   <div class="group-hover/tooltip:opacity-100">   │ │
     *   │   │        ↑                                          │ │
     *   │   │   Child responds to parent hover                  │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip wrapper has group/tooltip class
     */
    test("tooltip should have group class for hover states", async ({
      page,
    }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltip).toHaveClass(/group\/tooltip/);
    });
  });

  test.describe("TooltipContent Styling", () => {
    /**
     * TEST: Content Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"] (relative)                      │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   [TooltipContent] class="absolute ..."           │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │    "TOP"        │  ← Positioned absolutely    │ │
     *   │   │   └─────────────────┘    relative to Tooltip      │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has absolute positioning class
     */
    test("content should have absolute positioning", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/absolute/);
    });

    /**
     * TEST: Content Default Hidden State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Default state (no hover):                             │
     *   │   ┌─────────────────┐                                   │
     *   │   │    "TOP"        │  opacity: 0  ← INVISIBLE          │
     *   │   └─────────────────┘                                   │
     *   │   ┌─────────────────┐                                   │
     *   │   │    [Button]     │                                   │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   │   class="opacity-0 ..."                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has opacity-0 class by default
     */
    test("content should have opacity-0 by default", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/opacity-0/);
    });

    /**
     * TEST: Content Transition Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipContent needs smooth fade transitions:         │
     *   │                                                         │
     *   │   class="transition-all duration-300 ..."               │
     *   │                                                         │
     *   │   opacity-0 ──(300ms fade)──► opacity-100               │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐           ┌─────────────┐            │
     *   │       "TOP"                 │    "TOP"    │             │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘           └─────────────┘            │
     *   │    (invisible)               (visible)                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has transition-all and duration-300
     */
    test("content should have transition classes", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/transition-all/);
      await expect(ui.firstTooltipContent).toHaveClass(/duration-300/);
    });

    /**
     * TEST: Content Pointer Events Disabled
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   pointer-events-none prevents tooltip from:            │
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │    "TOP"        │  ← Mouse events pass through      │
     *   │   └─────────────────┘                                   │
     *   │         ↓ click goes through                            │
     *   │   ┌─────────────────┐                                   │
     *   │   │    [Button]     │  ← Button still clickable         │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   │   class="pointer-events-none ..."                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has pointer-events-none class
     */
    test("content should have pointer-events-none", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/pointer-events-none/);
    });

    /**
     * TEST: Content High Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   z-index stacking:                                     │
     *   │                                                         │
     *   │   z-[1000000] ┌─────────────────┐ ← Tooltip on TOP      │
     *   │               │    "TOP"        │                       │
     *   │               └─────────────────┘                       │
     *   │   z-50        ┌─────────────────┐                       │
     *   │               │    Modal        │                       │
     *   │               └─────────────────┘                       │
     *   │   z-auto      ┌─────────────────┐                       │
     *   │               │    Content      │                       │
     *   │               └─────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has z-[1000000] class
     */
    test("content should have high z-index", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/z-\[1000000\]/);
    });

    /**
     * TEST: Content Text Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipContent text classes:                          │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┐           │
     *   │   │  "TOP"                                  │           │
     *   │   │    │                                    │           │
     *   │   │    ├── text-xs (small font size)        │           │
     *   │   │    └── text-background (light color     │           │
     *   │   │        on dark bg)                      │           │
     *   │   └─────────────────────────────────────────┘           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has text-xs and text-background
     */
    test("content should have text styling", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/text-xs/);
      await expect(ui.firstTooltipContent).toHaveClass(/text-background/);
    });

    /**
     * TEST: Content Background Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipContent background:                            │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┐           │
     *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│           │
     *   │   │▓▓▓  "TOP"  (white text on dark bg)  ▓▓▓│           │
     *   │   │▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│           │
     *   │   └─────────────────────────────────────────┘           │
     *   │                                                         │
     *   │   class="bg-foreground/90 ..."  (90% opacity)           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has bg-foreground/90 class
     */
    test("content should have background styling", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/bg-foreground\/90/);
    });

    /**
     * TEST: Content Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipContent padding:                               │
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │   py-2                        │                     │
     *   │   │ ┌───────────────────────────┐ │                     │
     *   │   │ │                           │ │ px-2.5              │
     *   │   │ │        "TOP"              │ │                     │
     *   │   │ │                           │ │                     │
     *   │   │ └───────────────────────────┘ │                     │
     *   │   │   py-2                        │                     │
     *   │   └───────────────────────────────┘                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has py-2 and px-2.5 classes
     */
    test("content should have padding", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/py-2/);
      await expect(ui.firstTooltipContent).toHaveClass(/px-2\.5/);
    });

    /**
     * TEST: Content Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TooltipContent shadow effect:                         │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┐           │
     *   │   │         "TOP"                           │           │
     *   │   └─────────────────────────────────────────┘           │
     *   │    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░            │
     *   │      ↑ shadow-lg (large drop shadow)                    │
     *   │                                                         │
     *   │   class="shadow-lg ..."                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has shadow-lg class
     */
    test("content should have shadow-lg", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/shadow-lg/);
    });

    /**
     * TEST: Content Whitespace Nowrap
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Without whitespace-nowrap:    With whitespace-nowrap: │
     *   │   ┌───────────────┐             ┌─────────────────────┐ │
     *   │   │ This is a     │             │ This is a tooltip   │ │
     *   │   │ tooltip       │             └─────────────────────┘ │
     *   │   └───────────────┘             (single line) ← EXPECTED│
     *   │   (wraps)                                               │
     *   │                                                         │
     *   │   class="whitespace-nowrap ..."                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has whitespace-nowrap class
     */
    test("content should have whitespace-nowrap", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(/whitespace-nowrap/);
    });
  });

  test.describe("Hover Behavior", () => {
    /**
     * TEST: Content Hover Opacity Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tailwind group-hover pattern:                         │
     *   │                                                         │
     *   │   Before hover:              After hover:               │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐           ┌─────────────┐            │
     *   │       "TOP"                 │    "TOP"    │             │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘           └─────────────┘            │
     *   │   opacity-0                 opacity-100                 │
     *   │   ┌─────────────┐           ┌─────────────┐            │
     *   │   │   Button    │  ─hover─► │   Button    │            │
     *   │   └─────────────┘           └─────────────┘            │
     *   │                                                         │
     *   │   class="group-hover/tooltip:opacity-100 ..."           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent has group-hover opacity class
     */
    test("content should have hover opacity class", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(
        /group-hover\/tooltip:opacity-100/
      );
    });

    /**
     * TEST: Arrow Hover Opacity Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Arrow appears with content on hover:                  │
     *   │                                                         │
     *   │   Before hover:              After hover:               │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐           ┌─────────────┐            │
     *   │       "TOP"                 │    "TOP"    │             │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘           └──────▼──────┘            │
     *   │        ▼                          ▼  ← Arrow visible    │
     *   │   (invisible)               ┌─────────────┐            │
     *   │   ┌─────────────┐           │   Button    │            │
     *   │   │   Button    │           └─────────────┘            │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   │   class="group-hover/tooltip:opacity-100 ..."           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow has group-hover opacity class
     */
    test("arrow should have hover opacity class", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toHaveClass(
        /group-hover\/tooltip:opacity-100/
      );
    });

    /**
     * TEST: Tooltip Becomes Visible on Hover (Interactive)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ACTUAL HOVER INTERACTION:                             │
     *   │                                                         │
     *   │   1. Before hover: opacity = 0 (hidden)                 │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐                                      │
     *   │       "TOP"       (computed opacity: 0)                 │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘                                      │
     *   │   ┌─────────────┐                                      │
     *   │   │   Button    │                                      │
     *   │   └─────────────┘                                      │
     *   │                                                         │
     *   │   2. After hover: opacity = 1 (visible)                 │
     *   │   ┌─────────────┐                                      │
     *   │   │    "TOP"    │  (computed opacity: 1)               │
     *   │   └─────────────┘                                      │
     *   │   ┌─────────────┐                                      │
     *   │   │   Button    │  ← MOUSE HOVER                       │
     *   │   └─────────────┘                                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering tooltip trigger shows content (CSS hover)
     */
    test("tooltip content should become visible on hover", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Before hover: content should have opacity 0
      const contentOpacityBefore = await ui.firstTooltipContent.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(contentOpacityBefore).toBe("0");

      // Hover over the tooltip wrapper (triggers group-hover)
      await ui.hoverTooltip(ui.firstTooltip);

      // Wait for CSS transition to complete (300ms duration)
      await page.waitForTimeout(350);

      // After hover: content should have opacity 1
      const contentOpacityAfter = await ui.firstTooltipContent.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(contentOpacityAfter).toBe("1");
    });

    /**
     * TEST: Tooltip Arrow Becomes Visible on Hover (Interactive)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Arrow visibility follows content:                     │
     *   │                                                         │
     *   │   Before hover:              After hover:               │
     *   │   opacity: 0                 opacity: 1                 │
     *   │        .                           ▼                    │
     *   │   (invisible)               (visible arrow)             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hovering tooltip trigger shows arrow (CSS hover)
     */
    test("tooltip arrow should become visible on hover", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Before hover: arrow should have opacity 0
      const arrowOpacityBefore = await ui.firstTooltipArrow.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(arrowOpacityBefore).toBe("0");

      // Hover over the tooltip wrapper
      await ui.hoverTooltip(ui.firstTooltip);

      // Wait for CSS transition to complete (300ms duration)
      await page.waitForTimeout(350);

      // After hover: arrow should have opacity 1
      const arrowOpacityAfter = await ui.firstTooltipArrow.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(arrowOpacityAfter).toBe("1");
    });

    /**
     * TEST: Tooltip Hides When Mouse Leaves
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   1. Hover: visible        2. Leave: hidden             │
     *   │   ┌─────────────┐          ┌ ─ ─ ─ ─ ─ ─ ┐             │
     *   │   │    "TOP"    │              "TOP"                    │
     *   │   └─────────────┘          └ ─ ─ ─ ─ ─ ─ ┘             │
     *   │   ┌─────────────┐          ┌─────────────┐             │
     *   │   │   Button    │  ──────► │   Button    │             │
     *   │   └─────────────┘  mouse   └─────────────┘             │
     *   │        ↑          leaves                                │
     *   │      HOVER                                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tooltip hides when mouse moves away
     */
    test("tooltip should hide when mouse leaves", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Hover to show tooltip
      await ui.hoverTooltip(ui.firstTooltip);

      // Wait for CSS transition to complete (300ms duration)
      await page.waitForTimeout(350);

      const opacityWhileHovered = await ui.firstTooltipContent.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(opacityWhileHovered).toBe("1");

      // Move mouse away
      await ui.unhoverTooltip();

      // Wait for CSS transition to complete
      await page.waitForTimeout(350);

      // Tooltip should be hidden again
      const opacityAfterLeave = await ui.firstTooltipContent.evaluate(
        (el) => window.getComputedStyle(el).opacity
      );
      expect(opacityAfterLeave).toBe("0");
    });

    /**
     * TEST: Pointer Events Auto on Hover
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CSS-only tooltip mechanism:                           │
     *   │                                                         │
     *   │   Before hover:                After hover:             │
     *   │   pointer-events: none         pointer-events: auto     │
     *   │   (clicks pass through)        (can interact)           │
     *   │                                                         │
     *   │   group-hover/tooltip:pointer-events-auto               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content has pointer-events-auto class for hover
     */
    test("content should have pointer-events-auto on hover class", async ({
      page,
    }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipContent).toHaveClass(
        /group-hover\/tooltip:pointer-events-auto/
      );
    });
  });

  test.describe("Position Variants", () => {
    /**
     * TEST: Top Position Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Top position layout:                                  │
     *   │                                                         │
     *   │   ┌───────────────┐                                     │
     *   │   │    "TOP"      │  ← data-position="Top"              │
     *   │   └───────▼───────┘                                     │
     *   │   ┌───────────────┐                                     │
     *   │   │    Button     │                                     │
     *   │   └───────────────┘                                     │
     *   │                                                         │
     *   │   Content appears ABOVE the trigger                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent with data-position="Top" exists
     */
    test("should have tooltip with Top position", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const topContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Top"]'
      );
      await expect(topContent).toBeAttached();
    });

    /**
     * TEST: Left Position Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Left position layout:                                 │
     *   │                                                         │
     *   │   ┌─────────┐┌───────────────┐                          │
     *   │   │ "LEFT"  ││    Button     │                          │
     *   │   └─────────┘└───────────────┘                          │
     *   │        ▶                                                │
     *   │        ↑                                                │
     *   │   data-position="Left"                                  │
     *   │                                                         │
     *   │   Content appears to the LEFT of the trigger            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent with data-position="Left" exists
     */
    test("should have tooltip with Left position", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const leftContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Left"]'
      );
      await expect(leftContent).toBeAttached();
    });

    /**
     * TEST: Right Position Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Right position layout:                                │
     *   │                                                         │
     *   │   ┌───────────────┐┌──────────┐                         │
     *   │   │    Button     ││ "RIGHT"  │                         │
     *   │   └───────────────┘└──────────┘                         │
     *   │                    ◀                                    │
     *   │                    ↑                                    │
     *   │               data-position="Right"                     │
     *   │                                                         │
     *   │   Content appears to the RIGHT of the trigger           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent with data-position="Right" exists
     */
    test("should have tooltip with Right position", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Use .first() since there are multiple Right position tooltips in the demo
      const rightContent = ui.preview
        .locator('[data-name="TooltipContent"][data-position="Right"]')
        .first();
      await expect(rightContent).toBeAttached();
    });

    /**
     * TEST: Bottom Position Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Bottom position layout:                               │
     *   │                                                         │
     *   │   ┌───────────────┐                                     │
     *   │   │    Button     │                                     │
     *   │   └───────────────┘                                     │
     *   │   ┌───────▲───────┐                                     │
     *   │   │   "BOTTOM"    │  ← data-position="Bottom"           │
     *   │   └───────────────┘                                     │
     *   │                                                         │
     *   │   Content appears BELOW the trigger                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipContent with data-position="Bottom" exists
     */
    test("should have tooltip with Bottom position", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const bottomContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Bottom"]'
      );
      await expect(bottomContent).toBeAttached();
    });
  });

  test.describe("Content Text", () => {
    /**
     * TEST: TOP Tooltip Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────┐                                     │
     *   │   │     "TOP"     │  ← Text must be exactly "TOP"       │
     *   │   └───────────────┘                                     │
     *   │   ┌───────────────┐                                     │
     *   │   │    [Button]   │                                     │
     *   │   └───────────────┘                                     │
     *   │                                                         │
     *   │   data-position="Top"                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Top tooltip displays "TOP" text
     */
    test("TOP tooltip should have correct text", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const topContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Top"]'
      );
      await expect(topContent).toHaveText("TOP");
    });

    /**
     * TEST: LEFT Tooltip Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────┐ ┌───────────────┐                         │
     *   │   │ "LEFT"  │ │   [Button]    │                         │
     *   │   └─────────┘ └───────────────┘                         │
     *   │        ↑                                                │
     *   │   Text must be exactly "LEFT"                           │
     *   │                                                         │
     *   │   data-position="Left"                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Left tooltip displays "LEFT" text
     */
    test("LEFT tooltip should have correct text", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const leftContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Left"]'
      );
      await expect(leftContent).toHaveText("LEFT");
    });

    /**
     * TEST: RIGHT Tooltip Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────┐ ┌──────────┐                        │
     *   │   │   [Button]    │ │ "RIGHT"  │                        │
     *   │   └───────────────┘ └──────────┘                        │
     *   │                          ↑                              │
     *   │               Text must be exactly "RIGHT"              │
     *   │                                                         │
     *   │   data-position="Right"                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Right tooltip displays "RIGHT" text
     */
    test("RIGHT tooltip should have correct text", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      // Use .first() since there are multiple Right position tooltips in the demo
      const rightContent = ui.preview
        .locator('[data-name="TooltipContent"][data-position="Right"]')
        .first();
      await expect(rightContent).toHaveText("RIGHT");
    });

    /**
     * TEST: BOTTOM Tooltip Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────┐                                     │
     *   │   │   [Button]    │                                     │
     *   │   └───────────────┘                                     │
     *   │   ┌───────────────┐                                     │
     *   │   │   "BOTTOM"    │  ← Text must be exactly "BOTTOM"    │
     *   │   └───────────────┘                                     │
     *   │                                                         │
     *   │   data-position="Bottom"                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Bottom tooltip displays "BOTTOM" text
     */
    test("BOTTOM tooltip should have correct text", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const bottomContent = ui.preview.locator(
        '[data-name="TooltipContent"][data-position="Bottom"]'
      );
      await expect(bottomContent).toHaveText("BOTTOM");
    });
  });

  test.describe("Trigger Buttons", () => {
    /**
     * TEST: Trigger Buttons Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐           │
     *   │   │   ▲    │ │   ◀    │ │   ▶    │ │   ▼    │           │
     *   │   │  TOP   │ │  LEFT  │ │ RIGHT  │ │ BOTTOM │           │
     *   │   └────────┘ └────────┘ └────────┘ └────────┘           │
     *   │      1          2          3          4                 │
     *   │                                                         │
     *   │   count >= 4 (at least 4 trigger buttons)               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page has at least 4 visible trigger buttons
     */
    test("trigger buttons should be visible", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const count = await ui.allTriggerButtons.count();
      expect(count).toBeGreaterThanOrEqual(4);
    });

    /**
     * TEST: Trigger Button Secondary Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Button variants:                                      │
     *   │                                                         │
     *   │   Primary:        Secondary:  ← EXPECTED                │
     *   │   ┌────────────┐  ┌────────────┐                        │
     *   │   │▓▓▓▓▓▓▓▓▓▓▓▓│  │ bg-secondary│                       │
     *   │   │   filled   │  │  outline   │                        │
     *   │   └────────────┘  └────────────┘                        │
     *   │                                                         │
     *   │   class="bg-secondary ..."                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger buttons have bg-secondary class
     */
    test("trigger buttons should have Secondary variant", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const firstButton = ui.allTriggerButtons.first();
      await expect(firstButton).toHaveClass(/bg-secondary/);
    });

    /**
     * TEST: Trigger Button Arrow Icons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Each trigger button contains an SVG arrow icon:       │
     *   │                                                         │
     *   │   ┌────────────┐                                        │
     *   │   │   <svg>    │  ← SVG icon must be visible            │
     *   │   │     ▲      │                                        │
     *   │   │   </svg>   │                                        │
     *   │   └────────────┘                                        │
     *   │                                                         │
     *   │   button > svg  (Lucide arrow icon)                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger buttons contain visible SVG icons
     */
    test("trigger buttons should contain arrow icons", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      const firstButton = ui.allTriggerButtons.first();
      const svg = firstButton.locator("svg");
      await expect(svg).toBeVisible();
    });
  });

  test.describe("No Console Errors", () => {
    /**
     * TEST: No JavaScript Errors on Page Load
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   HYDRATION BUG FIX VERIFICATION:                       │
     *   │                                                         │
     *   │   Previously, TooltipProvider used inline <script>      │
     *   │   that caused "Identifier already declared" errors      │
     *   │   during Leptos SSR hydration.                          │
     *   │                                                         │
     *   │   The tooltip now uses pure CSS (group-hover) with      │
     *   │   no JavaScript, eliminating hydration errors.          │
     *   │                                                         │
     *   │   Console should be clean:                              │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Console                                          │ │
     *   │   │  (no errors)                                      │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: No JavaScript errors occur on page load
     */
    test("should have no console errors on page load", async ({ page }) => {
      const consoleErrors: string[] = [];

      page.on("console", (msg) => {
        if (msg.type() === "error") {
          consoleErrors.push(msg.text());
        }
      });

      const ui = new TooltipPage(page);
      await ui.goto();

      // Wait for page to fully hydrate
      await page.waitForTimeout(500);

      // Filter out known non-critical errors (if any)
      const criticalErrors = consoleErrors.filter(
        (err) => !err.includes("favicon")
      );

      expect(criticalErrors).toHaveLength(0);
    });

    /**
     * TEST: No JavaScript Errors During Hover Interaction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Interaction should be error-free:                     │
     *   │                                                         │
     *   │   1. Page load           → No errors                    │
     *   │   2. Hover tooltip       → No errors                    │
     *   │   3. Mouse leave         → No errors                    │
     *   │   4. Hover different     → No errors                    │
     *   │                                                         │
     *   │   Pure CSS tooltips have no JavaScript to fail.         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: No errors during tooltip hover interactions
     */
    test("should have no console errors during hover interactions", async ({
      page,
    }) => {
      const consoleErrors: string[] = [];

      page.on("console", (msg) => {
        if (msg.type() === "error") {
          consoleErrors.push(msg.text());
        }
      });

      const ui = new TooltipPage(page);
      await ui.goto();

      // Perform multiple hover interactions
      await ui.hoverTooltip(ui.firstTooltip);
      await page.waitForTimeout(100);
      await ui.unhoverTooltip();
      await page.waitForTimeout(100);

      // Hover different tooltips
      const tooltips = await ui.allTooltips.all();
      for (const tooltip of tooltips.slice(0, 3)) {
        await ui.hoverTooltip(tooltip);
        await page.waitForTimeout(50);
      }

      await ui.unhoverTooltip();

      // Filter out known non-critical errors
      const criticalErrors = consoleErrors.filter(
        (err) => !err.includes("favicon")
      );

      expect(criticalErrors).toHaveLength(0);
    });
  });

  test.describe("Arrow Styling", () => {
    /**
     * TEST: Arrow Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Tooltip"] (relative)                      │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │    "TOP"        │                             │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   │          ▼  ← [TooltipArrow] position: absolute   │ │
     *   │   │   ┌─────────────────┐                             │ │
     *   │   │   │    [Button]     │                             │ │
     *   │   │   └─────────────────┘                             │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow has absolute positioning class
     */
    test("arrow should have absolute positioning", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toHaveClass(/absolute/);
    });

    /**
     * TEST: Arrow Default Hidden State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Default state (no hover):                             │
     *   │                                                         │
     *   │   ┌─────────────────┐                                   │
     *   │   │    "TOP"        │                                   │
     *   │   └─────────────────┘                                   │
     *   │          ▼  ← opacity: 0 (INVISIBLE)                    │
     *   │   ┌─────────────────┐                                   │
     *   │   │    [Button]     │                                   │
     *   │   └─────────────────┘                                   │
     *   │                                                         │
     *   │   class="opacity-0 ..."                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow has opacity-0 class by default
     */
    test("arrow should have opacity-0 by default", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toHaveClass(/opacity-0/);
    });

    /**
     * TEST: Arrow Transition Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Arrow needs smooth fade transitions:                  │
     *   │                                                         │
     *   │   class="transition-all duration-300 ..."               │
     *   │                                                         │
     *   │   Before hover:          After hover:                   │
     *   │   ┌─────────────┐        ┌─────────────┐               │
     *   │   │    "TOP"    │        │    "TOP"    │               │
     *   │   └─────────────┘        └──────▼──────┘               │
     *   │         .                       ▼  (visible)            │
     *   │    (invisible)            ──(300ms)──►                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow has transition-all and duration-300
     */
    test("arrow should have transition classes", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toHaveClass(/transition-all/);
      await expect(ui.firstTooltipArrow).toHaveClass(/duration-300/);
    });

    /**
     * TEST: Arrow Border Styling (Triangle Effect)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CSS triangle using borders:                           │
     *   │                                                         │
     *   │   ┌───────────────────────────────┐                     │
     *   │   │         "TOP"                 │                     │
     *   │   └───────────────────────────────┘                     │
     *   │                 ▼  ← Triangle created with:             │
     *   │                    - border-6 (6px border width)        │
     *   │                    - bg-transparent                     │
     *   │                    - border-transparent                 │
     *   │                    + specific side colored              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: TooltipArrow has border triangle styling classes
     */
    test("arrow should have border styling", async ({ page }) => {
      const ui = new TooltipPage(page);
      await ui.goto();

      await expect(ui.firstTooltipArrow).toHaveClass(/border-6/);
      await expect(ui.firstTooltipArrow).toHaveClass(/bg-transparent/);
      await expect(ui.firstTooltipArrow).toHaveClass(/border-transparent/);
    });
  });
});
