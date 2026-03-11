import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SPINNER COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <svg role="status" aria-label="Loading">                              │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │         ◠                                                       │   │
 * │   │       ╱   ╲         ← Rotating animation (animate-spin)         │   │
 * │   │      ○     ○                                                    │   │
 * │   │       ╲   ╱                                                     │   │
 * │   │         ◡                                                       │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SPINNER VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default (Loader2)          Circle (LoaderCircle)                      │
 * │   ┌─────────────┐             ┌─────────────┐                           │
 * │   │     ╭─╮     │             │    ╭───╮    │                           │
 * │   │    ╱   ╲    │             │   ╱     ╲   │                           │
 * │   │   ◯     ◯   │             │  │   ○   │  │                           │
 * │   │    ╲   ╱    │             │   ╲     ╱   │                           │
 * │   │     ╰─╯     │             │    ╰───╯    │                           │
 * │   └─────────────┘             └─────────────┘                           │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ANIMATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   animate-spin: 360° rotation in 1s linear infinite                     │
 * │                                                                         │
 * │   Frame 0°       Frame 90°      Frame 180°     Frame 270°               │
 * │   ┌───────┐      ┌───────┐      ┌───────┐      ┌───────┐                │
 * │   │   ╱   │      │     ─ │      │   ╲   │      │ ─     │                │
 * │   └───────┘      └───────┘      └───────┘      └───────┘                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SpinnerPage extends BasePage {
  protected readonly componentName = "spinner";

  // Spinner elements
  readonly spinner: Locator;
  readonly spinnerCircle: Locator;
  readonly allSpinners: Locator;

  constructor(page: Page) {
    super(page);

    // Spinners - they are SVG elements with role="status" - scoped within preview
    this.allSpinners = this.preview.locator('[role="status"]');
    this.spinner = this.allSpinners.first();
    this.spinnerCircle = this.allSpinners.nth(1);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Spinner Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Spinner Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │          ◠                                              │
     *   │        ╱   ╲      <-- Spinner SVG visible on page       │
     *   │       ○     ○                                           │
     *   │        ╲   ╱                                            │
     *   │          ◡                                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The spinner element is rendered and visible
     */
    test("should have spinner elements", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinner).toBeVisible();
    });

    /**
     * TEST: Two Spinner Variants Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Variant 1 (Loader2)      Variant 2 (LoaderCircle)     │
     *   │   ┌─────────────┐          ┌─────────────┐              │
     *   │   │     ╭─╮     │          │    ╭───╮    │              │
     *   │   │    ╱   ╲    │          │   ╱     ╲   │              │
     *   │   │   ◯     ◯   │          │  │   ○   │  │              │
     *   │   │    ╲   ╱    │          │   ╲     ╱   │              │
     *   │   │     ╰─╯     │          │    ╰───╯    │              │
     *   │   └─────────────┘          └─────────────┘              │
     *   │       count = 2                                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 2 spinner variants are rendered
     */
    test("should have two spinner variants", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      const count = await ui.allSpinners.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Spinner Is SVG Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg role="status">   <-- tagName === "svg"           │
     *   │     ┌─────────────┐                                     │
     *   │     │   ╭─╮       │                                     │
     *   │     │  ◯   ◯      │     Vector graphic element          │
     *   │     │   ╰─╯       │                                     │
     *   │     └─────────────┘                                     │
     *   │   </svg>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Spinner is rendered as an SVG element
     */
    test("spinner should be an SVG element", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      const tagName = await ui.spinner.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("svg");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Spinner Has role="status"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg role="status">                                   │
     *   │        ╭───────────╮                                    │
     *   │        │ role attr │                                    │
     *   │        │ ="status" │  <-- ARIA role for live regions   │
     *   │        ╰───────────╯                                    │
     *   │   Screen readers announce status changes                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Spinner has proper ARIA role for accessibility
     */
    test("spinner should have role=status", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinner).toHaveAttribute("role", "status");
    });

    /**
     * TEST: Spinner Has aria-label="Loading"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg aria-label="Loading">                            │
     *   │        ╭─────────────────╮                              │
     *   │        │   aria-label    │                              │
     *   │        │   ="Loading"    │  <-- Accessible name         │
     *   │        ╰─────────────────╯                              │
     *   │   Screen reader: "Loading, status"                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Spinner has descriptive aria-label
     */
    test("spinner should have aria-label=Loading", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinner).toHaveAttribute("aria-label", "Loading");
    });

    /**
     * TEST: SpinnerCircle Has role="status"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   LoaderCircle variant:                                 │
     *   │   ┌─────────────┐                                       │
     *   │   │   ╭───╮     │  <svg role="status">                  │
     *   │   │  ╱     ╲    │       ╭───────────╮                   │
     *   │   │ │   ○   │   │       │ role attr │                   │
     *   │   │  ╲     ╱    │       │ ="status" │                   │
     *   │   │   ╰───╯     │       ╰───────────╯                   │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Circle spinner variant has ARIA role
     */
    test("spinnerCircle should have role=status", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinnerCircle).toHaveAttribute("role", "status");
    });

    /**
     * TEST: SpinnerCircle Has aria-label="Loading"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   LoaderCircle variant:                                 │
     *   │   ┌─────────────┐                                       │
     *   │   │   ╭───╮     │  <svg aria-label="Loading">           │
     *   │   │  ╱     ╲    │       ╭─────────────────╮             │
     *   │   │ │   ○   │   │       │   aria-label    │             │
     *   │   │  ╲     ╱    │       │   ="Loading"    │             │
     *   │   │   ╰───╯     │       ╰─────────────────╯             │
     *   │   └─────────────┘                                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Circle spinner variant has aria-label
     */
    test("spinnerCircle should have aria-label=Loading", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinnerCircle).toHaveAttribute("aria-label", "Loading");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Spinner Has animate-spin Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .animate-spin {                                       │
     *   │     animation: spin 1s linear infinite;                 │
     *   │   }                                                     │
     *   │                                                         │
     *   │   Frame 0°       Frame 90°      Frame 180°              │
     *   │   ┌───────┐      ┌───────┐      ┌───────┐               │
     *   │   │   ╱   │  ->  │     ─ │  ->  │   ╲   │  -> ...       │
     *   │   └───────┘      └───────┘      └───────┘               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Spinner has CSS animation class applied
     */
    test("spinner should have animate-spin", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinner).toHaveClass(/animate-spin/);
    });

    /**
     * TEST: Spinner Has size-4 Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .size-4 = width: 1rem; height: 1rem; (16px)           │
     *   │                                                         │
     *   │   ┌────────────────┐                                    │
     *   │   │  ◠             │                                    │
     *   │   │ ╱ ╲  16px      │  <-- Consistent sizing             │
     *   │   │ ╲ ╱            │                                    │
     *   │   │  ◡   ├─16px─┤  │                                    │
     *   │   └────────────────┘                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Spinner has correct size utility class
     */
    test("spinner should have size-4", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinner).toHaveClass(/size-4/);
    });

    /**
     * TEST: SpinnerCircle Has animate-spin Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   LoaderCircle with animation:                          │
     *   │                                                         │
     *   │   Frame 0°       Frame 90°      Frame 180°              │
     *   │   ┌───────┐      ┌───────┐      ┌───────┐               │
     *   │   │ ╭───╮ │  ->  │ ╭───╮ │  ->  │ ╭───╮ │  -> ...       │
     *   │   │ │ ○ │ │      │ │ ○ │ │      │ │ ○ │ │               │
     *   │   │ ╰─┬─╯ │      │ ╰──┬╯ │      │ ╰───╯ │               │
     *   │   └───┼───┘      └────┼──┘      └───────┘               │
     *   │       └── indicator rotates around                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Circle spinner has animation class
     */
    test("spinnerCircle should have animate-spin", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinnerCircle).toHaveClass(/animate-spin/);
    });

    /**
     * TEST: SpinnerCircle Has size-4 Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .size-4 = width: 1rem; height: 1rem; (16px)           │
     *   │                                                         │
     *   │   LoaderCircle variant:                                 │
     *   │   ┌────────────────┐                                    │
     *   │   │   ╭───╮        │                                    │
     *   │   │  ╱     ╲ 16px  │  <-- Same size as Loader2          │
     *   │   │  ╲     ╱       │                                    │
     *   │   │   ╰───╯├─16px─┤│                                    │
     *   │   └────────────────┘                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Circle spinner has correct size class
     */
    test("spinnerCircle should have size-4", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      await expect(ui.spinnerCircle).toHaveClass(/size-4/);
    });
  });

  test.describe("Animation", () => {
    /**
     * TEST: Spinner Animation Is Active
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SVG className.baseVal includes "animate-spin"         │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────┐           │
     *   │   │  class="animate-spin size-4 ..."       │           │
     *   │   │         └──────────────────┘            │           │
     *   │   │              Check via JS               │           │
     *   │   └─────────────────────────────────────────┘           │
     *   │                                                         │
     *   │   Animation cycle: 360° rotation every 1s               │
     *   │   ╭─╮  ->  ╭─╮  ->  ╭─╮  ->  ╭─╮                        │
     *   │   │/│      │─│      │\│      │─│                        │
     *   │   ╰─╯      ╰─╯      ╰─╯      ╰─╯                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Animation class is present via JS evaluation
     */
    test("spinner should be animated", async ({ page }) => {
      const ui = new SpinnerPage(page);
      await ui.goto();

      // Verify the spin animation class is applied
      const hasAnimation = await ui.spinner.evaluate((el) =>
        el.className.baseVal.includes("animate-spin")
      );
      expect(hasAnimation).toBe(true);
    });
  });
});
