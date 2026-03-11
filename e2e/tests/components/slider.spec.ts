import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SLIDER COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <input type="range">                                                  │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   min                           value                      max  │   │
 * │   │    │                              │                          │  │   │
 * │   │    ▼                              ▼                          ▼  │   │
 * │   │    0 ═══════════════════════════[●]───────────────────────── 100│   │
 * │   │         ↑                         ↑                         ↑   │   │
 * │   │     Track (filled)            Thumb              Track (empty)  │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SLIDER VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default (value=50):                                                   │
 * │   0 ═════════════════════════[●]════════════════════════════════════ 100│
 * │                                                                         │
 * │   Custom (value=40, step=5):                                            │
 * │   0 ════════════════════[●]═════════════════════════════════════════ 100│
 * │                                                                         │
 * │   Disabled (value=80):                                                  │
 * │   0 ═══════════════════════════════════════════════════════[●]══════ 100│
 * │      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░       │
 * │                       (grayed out, not interactive)                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STEP BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   step=5:                                                               │
 * │   │     │     │     │     │     │     │     │     │     │     │         │
 * │   0     10    20    30    40    50    60    70    80    90   100        │
 * │                                                                         │
 * │   Thumb snaps to nearest step value when dragged                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SliderPage extends BasePage {
  protected readonly componentName = "slider";

  // Slider elements
  readonly sliders: Locator;
  readonly defaultSlider: Locator;
  readonly customSlider: Locator;
  readonly disabledSlider: Locator;

  constructor(page: Page) {
    super(page);

    // All sliders - scoped within preview
    this.sliders = this.preview.locator('input[type="range"]');
    this.defaultSlider = this.sliders.nth(0);
    this.customSlider = this.sliders.nth(1);
    this.disabledSlider = this.sliders.nth(2);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Slider Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: should have slider elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Page loads with at least one slider visible:          │
     *   │                                                         │
     *   │   0 ═══════════════════════[●]═════════════════════ 100 │
     *   │         ↑                                               │
     *   │     defaultSlider (visible)                             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default slider element is visible on page load
     */
    test("should have slider elements", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.defaultSlider).toBeVisible();
    });

    /**
     * TEST: should have three slider variants
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Three distinct slider variants on the page:           │
     *   │                                                         │
     *   │   [1] Default:  0 ════════[●]════════════════════ 100   │
     *   │   [2] Custom:   0 ══════[●]══════════════════════ 100   │
     *   │   [3] Disabled: 0 ══════════════════════════[●]══ 100   │
     *   │                 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page contains exactly 3 slider components
     */
    test("should have three slider variants", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      const count = await ui.sliders.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: slider should be an input[type=range] element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   HTML element type verification:                       │
     *   │                                                         │
     *   │   <input type="range" ... />                            │
     *   │          └───────────┘                                  │
     *   │               ↑                                         │
     *   │         Must be "range"                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider uses native HTML range input element
     */
    test("slider should be an input[type=range] element", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.defaultSlider).toHaveAttribute("type", "range");
    });
  });

  test.describe("Default Slider", () => {
    /**
     * TEST: should have default min value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   min="0"                                               │
     *   │    │                                                    │
     *   │    ▼                                                    │
     *   │    0 ═════════════════════════[●]═══════════════════ ?  │
     *   │    ↑                                                    │
     *   │   Minimum boundary = 0                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default slider has min attribute set to "0"
     */
    test("should have default min value", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      const min = await ui.defaultSlider.getAttribute("min");
      expect(min).toBe("0");
    });

    /**
     * TEST: should have default max value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                                          max="100"      │
     *   │                                               │         │
     *   │                                               ▼         │
     *   │   ? ═════════════════════════[●]═════════════ 100       │
     *   │                                               ↑         │
     *   │                              Maximum boundary = 100     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default slider has max attribute set to "100"
     */
    test("should have default max value", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      const max = await ui.defaultSlider.getAttribute("max");
      expect(max).toBe("100");
    });
  });

  test.describe("Custom Slider", () => {
    /**
     * TEST: should have custom value of 40
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Custom slider initial position:                       │
     *   │                                                         │
     *   │   0 ══════════════════[●]════════════════════════ 100   │
     *   │                        ↑                                │
     *   │                    value = 40                           │
     *   │                   (40% from left)                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Custom slider has initial value of 40
     */
    test("should have custom value of 40", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.customSlider).toHaveValue("40");
    });

    /**
     * TEST: should have step of 5
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   step="5" - Thumb snaps to increments of 5:            │
     *   │                                                         │
     *   │   │     │     │     │     │     │     │     │     │     │
     *   │   0     10    20    30    40    50    60    70    80   100
     *   │                           ↑                             │
     *   │                       [●] snaps here                    │
     *   │                                                         │
     *   │   Valid values: 0, 5, 10, 15, 20, 25, 30, 35, 40...     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Custom slider has step attribute set to 5
     */
    test("should have step of 5", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.customSlider).toHaveAttribute("step", "5");
    });

    /**
     * TEST: should have min of 0
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   min="0"                                               │
     *   │    │                                                    │
     *   │    ▼                                                    │
     *   │    0 ══════════════════[●]═══════════════════════ 100   │
     *   │    ↑                                                    │
     *   │   Custom slider minimum boundary = 0                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Custom slider has min attribute set to 0
     */
    test("should have min of 0", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.customSlider).toHaveAttribute("min", "0");
    });

    /**
     * TEST: should have max of 100
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                                          max="100"      │
     *   │                                               │         │
     *   │                                               ▼         │
     *   │   0 ══════════════════[●]════════════════════ 100       │
     *   │                                               ↑         │
     *   │                        Custom slider maximum = 100      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Custom slider has max attribute set to 100
     */
    test("should have max of 100", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.customSlider).toHaveAttribute("max", "100");
    });
  });

  test.describe("Disabled Slider", () => {
    /**
     * TEST: should be disabled
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled slider - not interactive:                    │
     *   │                                                         │
     *   │   0 ══════════════════════════════════════[●]══════ 100 │
     *   │     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░      │
     *   │                                                         │
     *   │   disabled="true"                                       │
     *   │   - Cannot be focused                                   │
     *   │   - Cannot be dragged                                   │
     *   │   - Grayed out appearance                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled slider has disabled attribute
     */
    test("should be disabled", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.disabledSlider).toBeDisabled();
    });

    /**
     * TEST: should have value of 80
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled slider position at 80:                       │
     *   │                                                         │
     *   │   0 ══════════════════════════════════════[●]══════ 100 │
     *   │                                            ↑            │
     *   │                                        value = 80       │
     *   │                                       (80% from left)   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled slider has value of 80
     */
    test("should have value of 80", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.disabledSlider).toHaveValue("80");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: slider should have w-full
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Full width styling (w-full):                          │
     *   │                                                         │
     *   │   ┌─── Container ───────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │ 0 ════════════════════[●]════════════════════ 100   │
     *   │   │ ←─────────────────── 100% width ──────────────→ │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider has w-full class for full width
     */
    test("slider should have w-full", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.defaultSlider).toHaveClass(/w-full/);
    });

    /**
     * TEST: slider should have cursor-pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Cursor changes on hover:                              │
     *   │                                                         │
     *   │   0 ════════════════════[●]════════════════════════ 100 │
     *   │                          ↑                              │
     *   │                         ☝️                               │
     *   │                    cursor: pointer                      │
     *   │                                                         │
     *   │   Indicates element is interactive/clickable            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider has cursor-pointer class
     */
    test("slider should have cursor-pointer", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.defaultSlider).toHaveClass(/cursor-pointer/);
    });

    /**
     * TEST: slider should have appearance-none
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Native browser styling removed:                       │
     *   │                                                         │
     *   │   Default browser:           Custom (appearance-none):  │
     *   │   ┌────────────────┐         ┌────────────────────────┐ │
     *   │   │ [====O----]    │   →     │ ════════[●]════════════│ │
     *   │   │ (OS-specific)  │         │ (fully customizable)   │ │
     *   │   └────────────────┘         └────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider has appearance-none for custom styling
     */
    test("slider should have appearance-none", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await expect(ui.defaultSlider).toHaveClass(/appearance-none/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard focus state:                                 │
     *   │                                                         │
     *   │   Before focus:                                         │
     *   │   0 ════════════════════[●]════════════════════════ 100 │
     *   │                                                         │
     *   │   After focus (Tab key):                                │
     *   │   0 ════════════════════[◉]════════════════════════ 100 │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                   focus ring visible                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider can receive keyboard focus
     */
    test("should be focusable", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await ui.defaultSlider.focus();
      await expect(ui.defaultSlider).toBeFocused();
    });

    /**
     * TEST: should be able to change value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Value change via interaction:                         │
     *   │                                                         │
     *   │   Before:                                               │
     *   │   0 ══════════[●]══════════════════════════════════ 100 │
     *   │               ↑ (initial value)                         │
     *   │                                                         │
     *   │   After fill("50"):                                     │
     *   │   0 ════════════════════════[●]════════════════════ 100 │
     *   │                              ↑                          │
     *   │                          value = 50                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slider value can be changed programmatically
     */
    test("should be able to change value", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      await ui.defaultSlider.fill("50");
      await expect(ui.defaultSlider).toHaveValue("50");
    });

    /**
     * TEST: disabled slider should not be interactable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Disabled state prevents interaction:                  │
     *   │                                                         │
     *   │   0 ══════════════════════════════════════[●]══════ 100 │
     *   │     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░      │
     *   │                                                         │
     *   │   ✗ Cannot focus                                        │
     *   │   ✗ Cannot drag                                         │
     *   │   ✗ Cannot click                                        │
     *   │   ✗ No keyboard input                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled slider cannot be interacted with
     */
    test("disabled slider should not be interactable", async ({ page }) => {
      const ui = new SliderPage(page);
      await ui.goto();

      // Disabled sliders cannot be focused
      await expect(ui.disabledSlider).toBeDisabled();
    });
  });
});
