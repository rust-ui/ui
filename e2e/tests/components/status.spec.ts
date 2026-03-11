import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * STATUS COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="Status" class="relative inline-flex">                 │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                     ●           │   │
 * │   │   ┌───────────────────────────────────┐            ↑            │   │
 * │   │   │                                   │      StatusIndicator    │   │
 * │   │   │         Content                   │      (absolute,         │   │
 * │   │   │       (any child)                 │       top-0 right-0)    │   │
 * │   │   │                                   │                         │   │
 * │   │   └───────────────────────────────────┘                         │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * INDICATOR POSITIONING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   top-0 right-0 (default):                                              │
 * │   ┌─────────────────┐●                                                  │
 * │   │                 │                                                   │
 * │   │     Content     │                                                   │
 * │   │                 │                                                   │
 * │   └─────────────────┘                                                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * INDICATOR COLORS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Online:     Busy:       Away:       Offline:                          │
 * │     🟢          🔴          🟡           ⚫                               │
 * │   bg-green   bg-red     bg-yellow   bg-gray                             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   StatusIndicator:                                                      │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .absolute      ← Positioned relative to Status wrapper     │       │
 * │   │  .size-3        ← 12px diameter                             │       │
 * │   │  .rounded-full  ← Circular shape                            │       │
 * │   │  .bg-green-500  ← Default green color                       │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class StatusPage extends BasePage {
  protected readonly componentName = "status";

  // Status elements
  readonly status: Locator;
  readonly statusIndicator: Locator;
  readonly content: Locator;

  constructor(page: Page) {
    super(page);

    // Main status - scoped within preview
    this.status = this.preview.locator('[data-name="Status"]').first();

    // Status indicator (the dot)
    this.statusIndicator = this.status.locator('[data-name="StatusIndicator"]');

    // Content inside status
    this.content = this.status.locator("div").first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Status Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Status Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="Status"]                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                              ●    │  │
     *   │  │        Content Area                               │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The Status container element is visible on the page
     */
    test("should have Status container", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.status).toBeVisible();
    });

    /**
     * TEST: Status Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="Status">  <-- checking this attribute │
     *   │       ^^^^^^^^^^^^^^^^^^^^                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │               ...content...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name attribute equals "Status" for component identification
     */
    test("should have Status data-name attribute", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.status).toHaveAttribute("data-name", "Status");
    });

    /**
     * TEST: Status Indicator Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                              ●    │  │
     *   │  │                                              ^    │  │
     *   │  │                               StatusIndicator     │  │
     *   │  │                               (checking this)     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The status indicator dot is visible
     */
    test("should have status indicator", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toBeVisible();
    });

    /**
     * TEST: Status Content Wrapper
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │        Content (.size-16)                   │● │  │
     *   │  │  │        ^^^^^^^^^^^^^^^^^^                   │  │  │
     *   │  │  │        checking this                        │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Status wraps content with .size-16 class
     */
    test("should have content", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      // Status should wrap content
      await expect(ui.status.locator(".size-16")).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Status Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container: position: relative                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ^^^^^^^^^^^^^^^^^^^^^^^                          │  │
     *   │  │  Required for absolute positioning                │  │
     *   │  │  of StatusIndicator child                    ●    │  │
     *   │  │                                              ^    │  │
     *   │  │                           absolute (needs parent) │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Status has "relative" class for positioning context
     */
    test("status should have relative positioning", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.status).toHaveClass(/relative/);
    });

    /**
     * TEST: Status Inline-Flex Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────────────────┐
     *   │  Status (inline-flex) │  <-- shrinks to content
     *   │  ┌─────────────────┐  │
     *   │  │    Content      │● │
     *   │  └─────────────────┘  │
     *   └───────────────────────┘
     *                            ^
     *   inline-flex: component width = content width (not 100%)
     *
     *   Validates: Status uses inline-flex to size to its content
     */
    test("status should have inline-flex", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.status).toHaveClass(/inline-flex/);
    });

    /**
     * TEST: Status Indicator Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status (relative)                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                              ●    │  │
     *   │  │                                              ^    │  │
     *   │  │                                         absolute  │  │
     *   │  │                                    (checking this)│  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: StatusIndicator has "absolute" class for overlay positioning
     */
    test("status indicator should have absolute positioning", async ({
      page,
    }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toHaveClass(/absolute/);
    });

    /**
     * TEST: Status Indicator Top-Right Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                   top-0 right-0   │  │
     *   │  │                                         |         │  │
     *   │  │                                         v         │  │
     *   │  │                                         ●         │  │
     *   │  │        Content                                    │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: StatusIndicator positioned at top-right corner (top-0 right-0)
     */
    test("status indicator should be positioned at top right", async ({
      page,
    }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toHaveClass(/top-0/);
      await expect(ui.statusIndicator).toHaveClass(/right-0/);
    });

    /**
     * TEST: Status Indicator Rounded Shape
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   StatusIndicator shapes:                               │
     *   │                                                         │
     *   │   rounded-full (circle):    vs  rounded-md (square):    │
     *   │         ●                          ┌─┐                  │
     *   │         ^                          │ │                  │
     *   │    checking this                   └─┘                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: StatusIndicator has rounded-full for circular appearance
     */
    test("status indicator should have rounded-full", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toHaveClass(/rounded-full/);
    });

    /**
     * TEST: Status Indicator Default Green Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Status indicator colors:                              │
     *   │                                                         │
     *   │   Default:    Busy:       Away:       Offline:          │
     *   │     ●          ●           ●            ●               │
     *   │   green       red        yellow       gray              │
     *   │     ^                                                   │
     *   │   checking bg-green-500 (default)                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: StatusIndicator has bg-green-500 by default
     */
    test("status indicator should have green background by default", async ({
      page,
    }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toHaveClass(/bg-green-500/);
    });

    /**
     * TEST: Status Indicator Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   StatusIndicator sizing:                               │
     *   │                                                         │
     *   │   size-3 = 12px x 12px                                  │
     *   │                                                         │
     *   │   ┌────────────────────────┐                            │
     *   │   │                   ●    │  <-- 12px diameter dot     │
     *   │   │   Content        ←→    │      (size-3 class)        │
     *   │   │                  12px  │                            │
     *   │   └────────────────────────┘                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: StatusIndicator has size-3 class (12px width/height)
     */
    test("status indicator should have size-3", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      await expect(ui.statusIndicator).toHaveClass(/size-3/);
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Content Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ╭─────────────────────────────────────────────╮  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │     Content with rounded-md                 │● │  │
     *   │  │  │     ^^^^^^^^^^^^^^^^^^^^^                   │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  ╰─────────────────────────────────────────────╯  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content element has rounded-md class for rounded corners
     */
    test("content should have rounded-md", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      const content = ui.status.locator(".rounded-md");
      await expect(content).toBeVisible();
    });

    /**
     * TEST: Content Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │     Content: size-16 = 64px x 64px          │● │  │
     *   │  │  │     ←─────────── 64px ────────────→         │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content element has size-16 class (64px width/height)
     */
    test("content should have size-16", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      const content = ui.status.locator(".size-16");
      await expect(content).toBeVisible();
    });

    /**
     * TEST: Content Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Status Container                                       │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │█████████████████████████████████████████████│  │  │
     *   │  │  │█████████ bg-neutral-500 ███████████████████│● │  │
     *   │  │  │█████████ (gray background) ████████████████│  │  │
     *   │  │  │█████████████████████████████████████████████│  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content element has bg-neutral-500 background color
     */
    test("content should have bg-neutral-500", async ({ page }) => {
      const ui = new StatusPage(page);
      await ui.goto();

      const content = ui.status.locator(".bg-neutral-500");
      await expect(content).toBeVisible();
    });
  });
});
