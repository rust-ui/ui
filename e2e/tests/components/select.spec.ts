import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * SELECT COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Select"]                                                  │
 * │   ┌───────────────────────────────────────────────────────────────────┐ │
 * │   │                                                                   │ │
 * │   │   SelectTrigger (button)                                          │ │
 * │   │   ┌─────────────────────────────────────────────────────────────┐ │ │
 * │   │   │  SelectValue          "Please select"              ▼        │ │ │
 * │   │   └─────────────────────────────────────────────────────────────┘ │ │
 * │   │                                                                   │ │
 * │   │   SelectContent (dropdown, data-state="closed|open")              │ │
 * │   │   ┌─────────────────────────────────────────────────────────────┐ │ │
 * │   │   │  SelectGroup (role="listbox")                               │ │ │
 * │   │   │  ┌─────────────────────────────────────────────────────┐    │ │ │
 * │   │   │  │  SelectOption  "Components"              ✓          │    │ │ │
 * │   │   │  ├─────────────────────────────────────────────────────┤    │ │ │
 * │   │   │  │  SelectOption  "Extensions"                         │    │ │ │
 * │   │   │  ├─────────────────────────────────────────────────────┤    │ │ │
 * │   │   │  │  SelectOption  "Icons"                              │    │ │ │
 * │   │   │  └─────────────────────────────────────────────────────┘    │ │ │
 * │   │   └─────────────────────────────────────────────────────────────┘ │ │
 * │   │                                                                   │ │
 * │   └───────────────────────────────────────────────────────────────────┘ │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATE MECHANISM:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   CLOSED:                        OPEN:                                  │
 * │   ┌─────────────────────┐        ┌─────────────────────┐                │
 * │   │ Please select    ▼  │        │ Please select    ▲  │                │
 * │   └─────────────────────┘        ├─────────────────────┤                │
 * │   data-state="closed"            │ Components       ✓  │                │
 * │   pointer-events: none           ├─────────────────────┤                │
 * │                                  │ Extensions          │                │
 * │                                  ├─────────────────────┤                │
 * │                                  │ Icons               │                │
 * │                                  └─────────────────────┘                │
 * │                                  data-state="open"                      │
 * │                                                                         │
 * │   CLOSE TRIGGERS:                                                       │
 * │   ├── Click option (selects and closes)                                 │
 * │   ├── Click trigger again                                               │
 * │   └── Press ESC                                                         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * OPTION STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   UNSELECTED:        SELECTED:           FOCUSED:                       │
 * │   ┌─────────────┐    ┌─────────────┐     ┌─────────────┐                │
 * │   │ Components  │    │ Components ✓│     │ Components  │ ← bg-accent    │
 * │   └─────────────┘    └─────────────┘     └─────────────┘                │
 * │   aria-selected=     aria-selected=      tabindex="0"                   │
 * │   "false"            "true"              (keyboard focus)               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class SelectPage extends BasePage {
  protected readonly componentName = "select";

  // Select container
  readonly select: Locator;

  // Select parts
  readonly trigger: Locator;
  readonly content: Locator;
  readonly valueDisplay: Locator;

  // Options
  readonly options: Locator;
  readonly firstOption: Locator;

  constructor(page: Page) {
    super(page);

    // Main select - scoped within preview
    this.select = this.preview.locator('[data-name="Select"]').first();

    // Parts
    this.trigger = this.select.locator('[data-name="SelectTrigger"]');
    this.content = this.select.locator('[data-name="SelectContent"]');
    this.valueDisplay = this.select.locator('[data-name="SelectValue"]');

    // Options
    this.options = this.content.locator('[data-name="SelectOption"]');
    this.firstOption = this.options.first();
  }

  async openSelect() {
    await this.trigger.click();
    await this.waitForDataState(this.content, "open");
  }

  async closeSelect() {
    await this.page.keyboard.press("Escape");
    await this.waitForDataState(this.content, "closed");
  }

  async selectOption(option: Locator) {
    await option.click();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Select Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Select Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Select"]  ← MUST BE VISIBLE               │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The main Select container element exists and renders
     */
    test("should have Select component", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.select).toBeVisible();
    });

    /**
     * TEST: Select Has Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="Select">  ← ATTRIBUTE CHECK           │
     *   │        ↑                                                │
     *   │   data-name === "Select"                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Select has proper data-name attribute for identification
     */
    test("should have Select data-name attribute", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.select).toHaveAttribute("data-name", "Select");
    });

    /**
     * TEST: SelectTrigger Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectTrigger"]  ← MUST BE VISIBLE        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   The clickable trigger button                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The trigger button element is rendered and visible
     */
    test("should have SelectTrigger", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toBeVisible();
    });

    /**
     * TEST: SelectContent Exists in DOM
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectContent"]  ← MUST BE ATTACHED       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  (dropdown content, initially hidden)             │ │
     *   │   │  Components | Extensions | Icons                  │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   In DOM but may have pointer-events: none              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown content element is in the DOM
     */
    test("should have SelectContent", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toBeAttached();
    });

    /**
     * TEST: SelectValue Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectValue"]  ← MUST BE VISIBLE          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  "Please select"                             ▼    │ │
     *   │   │       ↑                                           │ │
     *   │   │  Value display area                               │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Value display element is rendered and visible
     */
    test("should have SelectValue", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.valueDisplay).toBeVisible();
    });

    /**
     * TEST: Trigger Is a Button Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button data-name="SelectTrigger">                    │
     *   │      ↑                                                  │
     *   │   tagName.toLowerCase() === "button"                    │
     *   │                                                         │
     *   │   (semantic HTML for accessibility)                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger uses semantic button element
     */
    test("trigger should be a button element", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      const tagName = await ui.trigger.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("button");
    });

    /**
     * TEST: Trigger Has Type Button Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button type="button">  ← ATTRIBUTE CHECK             │
     *   │           ↑                                             │
     *   │   type === "button" (not "submit")                      │
     *   │                                                         │
     *   │   (prevents accidental form submission)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button has explicit type to prevent form submission
     */
    test("trigger should have type button", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveAttribute("type", "button");
    });
  });

  test.describe("Initial State", () => {
    /**
     * TEST: Content Closed by Default
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Initial page load:                                    │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   (dropdown hidden)                                     │
     *   │        ↑                                                │
     *   │   data-state === "closed"                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown content is closed initially
     */
    test("content should be closed by default", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Value Display Shows Placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectValue"]:                            │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  "Please select"  ← PLACEHOLDER TEXT              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   No selection made yet, shows placeholder              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Placeholder text shown when no option selected
     */
    test("value display should show placeholder", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.valueDisplay).toContainText("Please select");
    });

    /**
     * TEST: Content Has Pointer-Events None When Closed
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CLOSED state:                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  (dropdown content)                               │ │
     *   │   │  style="pointer-events: none"  ← PREVENTS CLICKS  │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   Click-through when closed (CSS-only interaction)      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Closed dropdown ignores pointer events
     */
    test("content should have pointer-events none when closed", async ({
      page,
    }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      const style = await ui.content.getAttribute("style");
      expect(style).toContain("pointer-events: none");
    });
  });

  test.describe("Trigger Styling", () => {
    /**
     * TEST: Trigger Has Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   ↑───────── class contains "border" ─────────────────↑ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has visible border styling
     */
    test("trigger should have border", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/border/);
    });

    /**
     * TEST: Trigger Has Background Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │██████████████████████████████████████████████████│ │
     *   │   │  class="... bg-background ..."                    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   Theme-aware background color                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger uses theme background color
     */
    test("trigger should have bg-background", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/bg-background/);
    });

    /**
     * TEST: Trigger Has Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger:                                        │
     *   │   ╭───────────────────────────────────────────────────╮ │
     *   │   │  Please select                               ▼    │ │
     *   │   ╰───────────────────────────────────────────────────╯ │
     *   │   ↑── rounded-md (medium border radius) ──────────────↑ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has medium rounded corners
     */
    test("trigger should have rounded-md", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Trigger Has Height h-9
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   ↕ h-9 (36px height for consistent sizing)             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger has fixed height for consistency
     */
    test("trigger should have height h-9", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/h-9/);
    });

    /**
     * TEST: Trigger Has Inline-Flex Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger  class="inline-flex ...":               │
     *   │   ┌─────────────────────────┐                           │
     *   │   │ [value] ──── [chevron]  │  ← inline-level flex      │
     *   │   └─────────────────────────┘                           │
     *   │        ↑                                                │
     *   │   Inline display, internal flex layout                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger uses inline-flex for layout
     */
    test("trigger should have inline-flex display", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/inline-flex/);
    });

    /**
     * TEST: Trigger Has Items-Center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger  class="items-center ...":              │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │          Please select          ▼                 │ │
     *   │   │  ← ─ ─ ─ ─ vertically centered ─ ─ ─ ─ ─ ─ ─ ─ → │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content is vertically centered within trigger
     */
    test("trigger should have items-center", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/items-center/);
    });

    /**
     * TEST: Trigger Has Justify-Between
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger  class="justify-between ...":           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select ←─────── space ───────→ ▼          │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                        ↑       │
     *   │   Value on left               Chevron on right          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Value and chevron are spaced to opposite ends
     */
    test("trigger should have justify-between", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/justify-between/);
    });

    /**
     * TEST: Trigger Contains Chevron Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectTrigger:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   │                                              ↑    │ │
     *   │   │                                         <svg>     │ │
     *   │   │                                    (chevron icon) │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown indicator icon is present
     */
    test("trigger should contain chevron icon", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      const chevron = ui.trigger.locator("svg");
      await expect(chevron).toBeVisible();
    });
  });

  test.describe("Content Styling", () => {
    /**
     * TEST: Content Has Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent  class="absolute ...":                  │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  (dropdown positioned absolutely below trigger)   │ │
     *   │   │   position: absolute                              │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown is positioned absolutely relative to trigger
     */
    test("content should have absolute positioning", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/absolute/);
    });

    /**
     * TEST: Content Has z-50 Layer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Stacking order (z-index):                             │
     *   │                                                         │
     *   │   z-50  ┌─────────────────────┐  ← SelectContent        │
     *   │         │  Components         │                         │
     *   │         │  Extensions         │  (above other content)  │
     *   │         │  Icons              │                         │
     *   │         └─────────────────────┘                         │
     *   │   z-0   ┌─────────────────────┐  ← Page content         │
     *   │         │  Other elements...  │                         │
     *   │         └─────────────────────┘                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown appears above other page content
     */
    test("content should have z-50", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/z-50/);
    });

    /**
     * TEST: Content Has Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent:                                        │
     *   │   ╭───────────────────────────────────────────────────╮ │
     *   │   │  Components                                       │ │
     *   │   │  Extensions                                       │ │
     *   │   │  Icons                                            │ │
     *   │   ╰───────────────────────────────────────────────────╯ │
     *   │   ↑─── rounded-md (medium border radius) ────────────↑  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown has medium rounded corners
     */
    test("content should have rounded-md", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Content Has Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components                                       │ │
     *   │   │  Extensions                                       │ │
     *   │   │  Icons                                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │   ↑─────────── class contains "border" ──────────────↑  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown has visible border
     */
    test("content should have border", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/border/);
    });

    /**
     * TEST: Content Has Card Background
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent  class="bg-card ...":                   │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │██████████████████████████████████████████████████│ │
     *   │   │  Components   (theme-aware card background)       │ │
     *   │   │  Extensions                                       │ │
     *   │   │██████████████████████████████████████████████████│ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown uses theme card background color
     */
    test("content should have bg-card", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/bg-card/);
    });

    /**
     * TEST: Content Has Shadow
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent  class="shadow-md ...":                 │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components                                       │ │
     *   │   │  Extensions                                       │ │
     *   │   │  Icons                                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │
     *   │        ↑ shadow-md (elevation effect)                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown has medium shadow for depth
     */
    test("content should have shadow-md", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/shadow-md/);
    });

    /**
     * TEST: Content Has Max Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent  class="max-h-[300px] ...":             │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Option 1                                         │ │
     *   │   │  Option 2                                         │ │
     *   │   │  Option 3                                         │ │
     *   │   │  ...                     ↕ max-h-[300px]           │ │
     *   │   │  Option N                                         │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   Limited height prevents oversized dropdowns           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown has maximum height constraint
     */
    test("content should have max-height", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/max-h-\[300px\]/);
    });

    /**
     * TEST: Content Has Overflow Auto
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent  class="overflow-auto ...":             │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Option 1                                       ▲ │ │
     *   │   │  Option 2                                       █ │ │
     *   │   │  Option 3                                       █ │ │
     *   │   │  ...            (scrollbar when needed)         ▼ │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   overflow-auto enables scrolling for many options      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown scrolls when content exceeds max height
     */
    test("content should have overflow-auto", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveClass(/overflow-auto/);
    });
  });

  test.describe("Open/Close Behavior", () => {
    /**
     * TEST: Clicking Trigger Opens Select
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before:                                               │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓ CLICK                            │
     *   │   After:                                                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▲    │ │
     *   │   ├───────────────────────────────────────────────────┤ │
     *   │   │  Components                                       │ │
     *   │   │  Extensions     data-state="open"                 │ │
     *   │   │  Icons                                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Click on trigger opens the dropdown
     */
    test("clicking trigger should open select", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.trigger.click();
      await expect(ui.content).toHaveAttribute("data-state", "open");
    });

    /**
     * TEST: ESC Key Closes Select
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open state:                                           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▲    │ │
     *   │   ├───────────────────────────────────────────────────┤ │
     *   │   │  Components                                       │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓ PRESS ESC                        │
     *   │   Closed state:                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   data-state="closed"                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Escape key closes the open dropdown
     */
    test("ESC should close select", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();
      await page.keyboard.press("Escape");
      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Clicking Trigger Again Closes Select
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open state:                                           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select  ← CLICK AGAIN                ▲    │ │
     *   │   ├───────────────────────────────────────────────────┤ │
     *   │   │  Components                                       │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓                                  │
     *   │   Closed state:                                         │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select                               ▼    │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑ data-state="closed" (toggle behavior)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger acts as toggle (click again closes)
     */
    test("clicking trigger again should close select", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();
      await ui.trigger.click();
      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });
  });

  test.describe("Options", () => {
    /**
     * TEST: Three Options Exist
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectOption"] count:                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components   ← Option 1                          │ │
     *   │   │  Extensions   ← Option 2                          │ │
     *   │   │  Icons        ← Option 3                          │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   COUNT === 3                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropdown contains exactly three options
     */
    test("should have three options", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      const count = await ui.options.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: Options Have Correct Text Values
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Option text content:                                  │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  nth(0): "Components"   ✓                         │ │
     *   │   │  nth(1): "Extensions"   ✓                         │ │
     *   │   │  nth(2): "Icons"        ✓                         │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each option displays correct text content
     */
    test("options should have correct values", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      await expect(ui.options.nth(0)).toContainText("Components");
      await expect(ui.options.nth(1)).toContainText("Extensions");
      await expect(ui.options.nth(2)).toContainText("Icons");
    });

    /**
     * TEST: Options Have Role Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ARIA role for accessibility:                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div role="option">Components</div>              │ │
     *   │   │       ↑                                           │ │
     *   │   │  role === "option" (for screen readers)           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Options have proper ARIA role for accessibility
     */
    test("options should have role option", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      await expect(ui.firstOption).toHaveAttribute("role", "option");
    });

    /**
     * TEST: Options Have Data-Select-Option Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Data attribute for selection logic:                   │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div data-select-option="true">Components</div>  │ │
     *   │   │            ↑                                      │ │
     *   │   │  data-select-option === "true"                    │ │
     *   │   │  (identifies clickable options)                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Options are marked as selectable items
     */
    test("options should have data-select-option attribute", async ({
      page,
    }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      await expect(ui.firstOption).toHaveAttribute("data-select-option", "true");
    });

    /**
     * TEST: Options Are Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard navigation support:                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  [Components]  tabindex="0"  ← focusable          │ │
     *   │   │   Extensions                                      │ │
     *   │   │   Icons                                           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   tabindex="0" allows keyboard focus                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Options can receive keyboard focus
     */
    test("options should be focusable", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      await expect(ui.firstOption).toHaveAttribute("tabindex", "0");
    });
  });

  test.describe("Selection", () => {
    /**
     * TEST: Clicking Option Selects It
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Open dropdown:                                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  [Components]  ← CLICK                            │ │
     *   │   │   Extensions                                      │ │
     *   │   │   Icons                                           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓                                  │
     *   │   After selection:                                      │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components  ← VALUE UPDATED                  ▼   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking an option updates the displayed value
     */
    test("clicking option should select it", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();
      await ui.options.nth(0).click();

      await expect(ui.valueDisplay).toContainText("Components");
    });

    /**
     * TEST: Selecting Option Closes Dropdown
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before selection (open):                              │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  [Components]  ← CLICK                            │ │
     *   │   │   Extensions                                      │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓                                  │
     *   │   After selection (auto-closed):                        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components                                   ▼   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑ data-state="closed"                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selection automatically closes the dropdown
     */
    test("selecting option should close dropdown", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();
      await ui.options.nth(0).click();

      await expect(ui.content).toHaveAttribute("data-state", "closed");
    });

    /**
     * TEST: Selected Option Has Aria-Selected True
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   After selecting "Components", reopen dropdown:        │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components  ✓   aria-selected="true"             │ │
     *   │   │  Extensions      aria-selected="false"            │ │
     *   │   │  Icons           aria-selected="false"            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   ARIA state reflects current selection                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selected option has proper aria-selected state
     */
    test("selected option should have aria-selected true", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      // Select first option
      await ui.openSelect();
      await ui.options.nth(0).click();

      // Reopen and check aria-selected
      await ui.openSelect();
      await expect(ui.options.nth(0)).toHaveAttribute("aria-selected", "true");
    });

    /**
     * TEST: Can Change Selection
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Select "Components"                           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Components                                   ▼   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                      ↓                                  │
     *   │   Step 2: Reopen and select "Extensions"                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Extensions  ← NEW SELECTION                  ▼   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑                                                │
     *   │   Value updates to new selection                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Selection can be changed to a different option
     */
    test("can change selection", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      // Select first option
      await ui.openSelect();
      await ui.options.nth(0).click();
      await expect(ui.valueDisplay).toContainText("Components");

      // Select second option
      await ui.openSelect();
      await ui.options.nth(1).click();
      await expect(ui.valueDisplay).toContainText("Extensions");
    });
  });

  test.describe("SelectValue Styling", () => {
    /**
     * TEST: Value Has Text-sm Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectValue  class="text-sm ...":                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select  (small text size)             ▼   │ │
     *   │   │  ↑ text-sm (14px font)                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Value text uses small font size
     */
    test("value should have text-sm", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.valueDisplay).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Value Has Muted Foreground Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectValue  class="text-muted-foreground ...":       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Please select  (muted/gray text color)       ▼   │ │
     *   │   │  ↑ placeholder styling                            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Placeholder text uses muted color
     */
    test("value should have text-muted-foreground", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.valueDisplay).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: Value Has Truncate Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectValue  class="truncate ...":                    │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  Very long option text that gets t...         ▼   │ │
     *   │   │  ↑ text-overflow: ellipsis                        │ │
     *   │   │  (prevents overflow, shows ...)                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Long text is truncated with ellipsis
     */
    test("value should have truncate class", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.valueDisplay).toHaveClass(/truncate/);
    });
  });

  test.describe("SelectGroup", () => {
    /**
     * TEST: SelectGroup Has Listbox Role
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectGroup"]  role="listbox":            │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div role="listbox">                             │ │
     *   │   │     <div role="option">Components</div>           │ │
     *   │   │     <div role="option">Extensions</div>           │ │
     *   │   │     <div role="option">Icons</div>                │ │
     *   │   │  </div>                                           │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │        ↑ ARIA listbox pattern for accessibility         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Options container has listbox role
     */
    test("should have SelectGroup with listbox role", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      const group = ui.content.locator('[data-name="SelectGroup"]');
      await expect(group).toHaveAttribute("role", "listbox");
    });

    /**
     * TEST: SelectGroup Has Aria-Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="SelectGroup"]  aria-label:                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  <div aria-label="Select options">                │ │
     *   │   │       ↑                                           │ │
     *   │   │  Screen reader: "Select options listbox"          │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Group is labeled for screen readers
     */
    test("SelectGroup should have aria-label", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();
      await ui.waitForInitialized(ui.content);

      await ui.openSelect();

      const group = ui.content.locator('[data-name="SelectGroup"]');
      await expect(group).toHaveAttribute("aria-label", "Select options");
    });
  });

  test.describe("Position", () => {
    /**
     * TEST: Content Has Data-Position Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent positioning data:                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  data-position="bottom|top|..."  ← EXISTS         │ │
     *   │   │                                                   │ │
     *   │   │  (used for positioning logic)                     │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Position attribute exists for dropdown placement
     */
    test("content should have data-position attribute", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      const position = await ui.content.getAttribute("data-position");
      expect(position).toBeTruthy();
    });

    /**
     * TEST: Content Has Data-Target Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   SelectContent target reference:                       │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  data-target="target__select"                     │ │
     *   │   │       ↑                                           │ │
     *   │   │  Links dropdown to its trigger element            │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content references its positioning target
     */
    test("content should have data-target attribute", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.content).toHaveAttribute("data-target", "target__select");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Trigger Is Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard accessibility:                               │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │  [Please select]  ← TAB TO FOCUS                  │ │
     *   │   │        ↑                                          │ │
     *   │   │  document.activeElement === trigger               │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Trigger can receive keyboard focus
     */
    test("trigger should be focusable", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await ui.trigger.focus();
      await expect(ui.trigger).toBeFocused();
    });

    /**
     * TEST: Trigger Has Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Focus visual indicator:                               │
     *   │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐ │
     *   │   ┊  ┌───────────────────────────────────────────────┐ ┊ │
     *   │   ┊  │  Please select                            ▼   │ ┊ │
     *   │   ┊  └───────────────────────────────────────────────┘ ┊ │
     *   │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘ │
     *   │        ↑ focus:ring-1 focus:ring-ring                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Focus state has visible ring indicator
     */
    test("trigger should have focus ring styles", async ({ page }) => {
      const ui = new SelectPage(page);
      await ui.goto();

      await expect(ui.trigger).toHaveClass(/focus:ring-1/);
      await expect(ui.trigger).toHaveClass(/focus:ring-ring/);
    });
  });
});
