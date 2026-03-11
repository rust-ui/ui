import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SWITCH COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <label tabindex="0">  ← Focusable wrapper                             │
 * │   ┌───────────────────────────────────────────────────────────────────┐ │
 * │   │                                                                   │ │
 * │   │   <input type="checkbox" class="hidden" />  ← State controller    │ │
 * │   │                                                                   │ │
 * │   │   [data-name="Switch"]  ← Visual toggle track                     │ │
 * │   │   ┌─────────────────────────────────────────┐                     │ │
 * │   │   │  ┌───────┐                              │  ← Track (w-11 h-6) │ │
 * │   │   │  │   ●   │                              │                     │ │
 * │   │   │  └───────┘                              │  ← Thumb (::after)  │ │
 * │   │   └─────────────────────────────────────────┘                     │ │
 * │   │                                                                   │ │
 * │   │   "Airplane Mode"  ← Label text                                   │ │
 * │   │                                                                   │ │
 * │   └───────────────────────────────────────────────────────────────────┘ │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   OFF (unchecked):              ON (checked):                           │
 * │   ┌─────────────────────┐       ┌─────────────────────┐                 │
 * │   │ ┌───┐               │       │               ┌───┐ │                 │
 * │   │ │ ● │               │  ──►  │               │ ● │ │                 │
 * │   │ └───┘               │       │               └───┘ │                 │
 * │   └─────────────────────┘       └─────────────────────┘                 │
 * │   bg-gray-200                   bg-primary                              │
 * │   translate-x-0                 translate-x-full                        │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * THUMB ANIMATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   CSS-based sliding animation:                                          │
 * │                                                                         │
 * │   .after:transition-all           ← Smooth transition                   │
 * │   .peer-checked:after:translate-x-full  ← Move thumb right              │
 * │   .peer-checked:bg-primary        ← Change track color                  │
 * │                                                                         │
 * │   Frame 0%        Frame 50%        Frame 100%                           │
 * │   ┌─────────┐     ┌─────────┐      ┌─────────┐                          │
 * │   │●        │ ──► │   ●     │ ──►  │        ●│                          │
 * │   └─────────┘     └─────────┘      └─────────┘                          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SwitchPage extends BasePage {
  protected readonly componentName = "switch";

  // Demo elements
  readonly switchComponent: Locator;
  readonly switchInput: Locator;
  readonly switchLabel: Locator;

  constructor(page: Page) {
    super(page);

    // Switch component (the visual toggle div)
    this.switchComponent = this.firstByDataName("Switch");
    // Hidden input for the switch - scoped within preview
    this.switchInput = this.preview.locator('input[type="checkbox"]').first();
    // Label text - scoped within preview
    this.switchLabel = this.preview.locator("label").first();
  }

  async toggle() {
    await this.switchLabel.click();
  }

  async isChecked(): Promise<boolean> {
    return this.switchInput.isChecked();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Switch Page", () => {
  /**
   * STRUCTURE TESTS - Verify switch elements
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <label>                                                       │
   * │   ├── <input type="checkbox" class="hidden" />                  │
   * │   ├── [data-name="Switch"] (visual track)                       │
   * │   └── "Airplane Mode" (label text)                              │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Switch Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Switch component presence:                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <label>                                       │   │
     *   │   │     └── [data-name="Switch"]  ← MUST BE VISIBLE │   │
     *   │   │         ┌─────────────────┐                     │   │
     *   │   │         │ ●               │                     │   │
     *   │   │         └─────────────────┘                     │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch component renders and is visible on page
     */
    test("should have switch component", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toBeVisible();
    });

    /**
     * TEST: Switch Label Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label text content:                                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  "Airplane Mode"          │   │
     *   │   │   └─────────────────┘        ↑                  │   │
     *   │   │                        CONTAINS "Airplane"      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label contains "Airplane" text
     */
    test("should have label with text", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchLabel).toContainText("Airplane");
    });

    /**
     * TEST: Hidden Checkbox Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Hidden input for state management:                    │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <label>                                       │   │
     *   │   │     ├── <input type="checkbox" class="hidden"/> │   │
     *   │   │     │          ↑                                │   │
     *   │   │     │   HIDDEN (controls state)                 │   │
     *   │   │     │                                           │   │
     *   │   │     └── [Switch]  ← Visual representation       │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hidden checkbox input exists with hidden class
     */
    test("should have hidden checkbox input", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchInput).toHaveClass(/hidden/);
    });
  });

  test.describe("Toggle Behavior", () => {
    /**
     * TEST: Switch Default Unchecked State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Initial switch state:                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  ← Thumb on LEFT          │   │
     *   │   │   └─────────────────┘                           │   │
     *   │   │   bg-gray-200                                   │   │
     *   │   │                                                 │   │
     *   │   │   isChecked() === false                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch starts in unchecked/off state
     */
    test("should be unchecked by default", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      const isChecked = await ui.isChecked();
      expect(isChecked).toBe(false);
    });

    /**
     * TEST: Switch Toggle on Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Toggle cycle (off -> on -> off):                      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐      ┌─────────────────┐  │   │
     *   │   │   │ ●               │ ──→  │               ● │  │   │
     *   │   │   └─────────────────┘      └─────────────────┘  │   │
     *   │   │   OFF (gray)               ON (primary)         │   │
     *   │   │         │                       │               │   │
     *   │   │         └───────── CLICK ───────┘               │   │
     *   │   │                     │                           │   │
     *   │   │         ┌───────────┘                           │   │
     *   │   │         ▼                                       │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  ← Back to OFF            │   │
     *   │   │   └─────────────────┘                           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking toggles switch between on/off states
     */
    test("should toggle when clicked", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      // Initially unchecked
      expect(await ui.isChecked()).toBe(false);

      // Click to check
      await ui.toggle();
      expect(await ui.isChecked()).toBe(true);

      // Click to uncheck
      await ui.toggle();
      expect(await ui.isChecked()).toBe(false);
    });

    /**
     * TEST: Switch Multiple Toggle Cycles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Repeated toggle stability (5 iterations):             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   i=0: OFF ──→ ON   (checked)                   │   │
     *   │   │   i=1: ON  ──→ OFF  (unchecked)                 │   │
     *   │   │   i=2: OFF ──→ ON   (checked)                   │   │
     *   │   │   i=3: ON  ──→ OFF  (unchecked)                 │   │
     *   │   │   i=4: OFF ──→ ON   (checked)                   │   │
     *   │   │                                                 │   │
     *   │   │   State: i % 2 === 0 ? checked : unchecked      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch state remains consistent through multiple toggles
     */
    test("should toggle multiple times", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      for (let i = 0; i < 5; i++) {
        await ui.toggle();
        const isChecked = await ui.isChecked();
        expect(isChecked).toBe(i % 2 === 0);
      }
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Switch Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Component identification attribute:                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <div data-name="Switch" ...>                  │   │
     *   │   │           ↑                                     │   │
     *   │   │    getAttribute("data-name")                    │   │
     *   │   │           === "Switch"                          │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch has correct data-name attribute for testing
     */
    test("should have Switch data-name attribute", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveAttribute("data-name", "Switch");
    });

    /**
     * TEST: Switch Pill Shape
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Switch track border radius:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   .rounded-full = full pill/capsule shape       │   │
     *   │   │                                                 │   │
     *   │   │   ╭─────────────────╮                           │   │
     *   │   │   │ ●               │  ← Fully rounded ends     │   │
     *   │   │   ╰─────────────────╯                           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch track has rounded-full class for pill shape
     */
    test("should have rounded-full class for pill shape", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveClass(/rounded-full/);
    });

    /**
     * TEST: Switch Dimensions
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Switch track size classes:                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ←───── w-11 (44px) ─────→                     │   │
     *   │   │   ╭─────────────────╮  ↑                        │   │
     *   │   │   │ ●               │  h-6 (24px)               │   │
     *   │   │   ╰─────────────────╯  ↓                        │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch has w-11 and h-6 dimension classes
     */
    test("should have proper dimensions", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveClass(/w-11/);
      await expect(ui.switchComponent).toHaveClass(/h-6/);
    });

    /**
     * TEST: Switch Primary Background When Checked
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Checked state background color:                       │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   OFF:                  ON:                     │   │
     *   │   │   ┌─────────────────┐   ┌█████████████████┐     │   │
     *   │   │   │ ●               │   │███████████████●│     │   │
     *   │   │   └─────────────────┘   └█████████████████┘     │   │
     *   │   │   bg-gray-200            peer-checked:bg-primary│   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checked switch has primary background color class
     */
    test("should have primary background when checked", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();
      await ui.toggle();

      await expect(ui.switchComponent).toHaveClass(/peer-checked:bg-primary/);
    });

    /**
     * TEST: Switch Gray Background When Unchecked
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Unchecked state background color:                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  ← .bg-gray-200           │   │
     *   │   │   └─────────────────┘                           │   │
     *   │   │   Light gray background (off/inactive state)    │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Unchecked switch has gray-200 background class
     */
    test("should have gray background when unchecked", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveClass(/bg-gray-200/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Label Tabindex for Keyboard Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label keyboard accessibility:                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <label tabindex="0">                          │   │
     *   │   │              ↑                                  │   │
     *   │   │   tabindex="0" makes label focusable            │   │
     *   │   │   via Tab key navigation                        │   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  Airplane Mode            │   │
     *   │   │   └─────────────────┘                           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label has tabindex="0" for keyboard accessibility
     */
    test("label should have tabindex for keyboard navigation", async ({
      page,
    }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchLabel).toHaveAttribute("tabindex", "0");
    });

    /**
     * TEST: Input ARIA Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA label for screen readers:                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   <input aria-label="Toggle switch" ...>        │   │
     *   │   │                  ↑                              │   │
     *   │   │   Screen reader announces:                      │   │
     *   │   │   "Toggle switch, switch, off"                  │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hidden input has aria-label for accessibility
     */
    test("input should have aria-label", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchInput).toHaveAttribute(
        "aria-label",
        "Toggle switch"
      );
    });

    /**
     * TEST: Label Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label focus capability:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   Before focus():       After focus():          │   │
     *   │   │   ┌─────────────────┐   ┏━━━━━━━━━━━━━━━━━┓     │   │
     *   │   │   │ ●               │   ┃ ●               ┃     │   │
     *   │   │   └─────────────────┘   ┗━━━━━━━━━━━━━━━━━┛     │   │
     *   │   │                         isFocused() === true    │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch label can receive keyboard focus
     */
    test("label should be focusable", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await ui.switchLabel.focus();
      await expect(ui.switchLabel).toBeFocused();
    });

    /**
     * TEST: Label Cursor Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Label cursor styling:                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   .cursor-pointer on label element              │   │
     *   │   │                                                 │   │
     *   │   │   ┌─────────────────┐                           │   │
     *   │   │   │ ●               │  Airplane Mode            │   │
     *   │   │   └─────────────────┘       ↑                   │   │
     *   │   │                            👆 pointer cursor    │   │
     *   │   │                                                 │   │
     *   │   │   Indicates clickable/interactive element       │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label has cursor-pointer class for click affordance
     */
    test("should have cursor-pointer class on label", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchLabel).toHaveClass(/cursor-pointer/);
    });
  });

  test.describe("Animation", () => {
    /**
     * TEST: Thumb Transition Animation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CSS transition for smooth thumb movement:             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   .after:transition-all on Switch element       │   │
     *   │   │                                                 │   │
     *   │   │   Frame 0%      Frame 50%      Frame 100%       │   │
     *   │   │   ┌─────────┐   ┌─────────┐    ┌─────────┐      │   │
     *   │   │   │●        │   │   ●     │    │        ●│      │   │
     *   │   │   └─────────┘   └─────────┘    └─────────┘      │   │
     *   │   │       └──────── smooth ──────────┘              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Switch has after:transition-all for smooth animation
     */
    test("should have transition-all class for thumb animation", async ({
      page,
    }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveClass(/after:transition-all/);
    });

    /**
     * TEST: Thumb Translate on Checked State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Thumb position change via translate:                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   .peer-checked:after:translate-x-full          │   │
     *   │   │                                                 │   │
     *   │   │   Unchecked:            Checked:                │   │
     *   │   │   ┌─────────────────┐   ┌─────────────────┐     │   │
     *   │   │   │ ●               │   │               ● │     │   │
     *   │   │   └─────────────────┘   └─────────────────┘     │   │
     *   │   │   translate-x-0         translate-x-full        │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checked state translates thumb to the right
     */
    test("should have translate class for checked state", async ({ page }) => {
      const ui = new SwitchPage(page);
      await ui.goto();

      await expect(ui.switchComponent).toHaveClass(
        /peer-checked:after:translate-x-full/
      );
    });
  });
});
