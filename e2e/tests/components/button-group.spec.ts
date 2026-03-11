import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * BUTTON-GROUP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="ButtonGroup">                                         │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ┌──────────┬──────────┬──────────┐                            │   │
 * │   │   │  First   │  Second  │  Third   │                            │   │
 * │   │   └──────────┴──────────┴──────────┘                            │   │
 * │   │        ↑          ↑          ↑                                  │   │
 * │   │   rounded-l   rounded-none  rounded-r                           │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * BORDER RADIUS LOGIC:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   First Button:        Middle Button:       Last Button:                │
 * │   ┌──────────┐         ┌──────────┐         ┌──────────┐                │
 * │   │ rounded-l│         │rounded-  │         │ rounded-r│                │
 * │   │ rounded- │         │ none     │         │ rounded- │                │
 * │   │ r-none   │         │          │         │ l-none   │                │
 * │   └──────────┘         └──────────┘         └──────────┘                │
 * │                                                                         │
 * │   Visual:                                                               │
 * │   ╭──────────┬──────────┬──────────╮                                    │
 * │   │  First   │  Second  │  Third   │                                    │
 * │   ╰──────────┴──────────┴──────────╯                                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * BUTTON VARIANT (outline):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .border        ← Border styling                            │       │
 * │   │  .bg-background ← Background matches theme                  │       │
 * │   │  hover:...      ← Hover state styles                        │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ButtonGroupPage extends BasePage {
  protected readonly componentName = "button-group";

  // Button group elements
  readonly buttonGroup: Locator;
  readonly buttons: Locator;
  readonly firstButton: Locator;
  readonly secondButton: Locator;
  readonly thirdButton: Locator;

  constructor(page: Page) {
    super(page);

    // Main button group within preview
    this.buttonGroup = this.preview.locator('[data-name="ButtonGroup"]').first();

    // Buttons within the group
    this.buttons = this.buttonGroup.getByRole("button");
    this.firstButton = this.buttonGroup.getByRole("button", { name: "First" });
    this.secondButton = this.buttonGroup.getByRole("button", { name: "Second" });
    this.thirdButton = this.buttonGroup.getByRole("button", { name: "Third" });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("ButtonGroup Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: ButtonGroup Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="ButtonGroup">  <-- Must be visible    │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ButtonGroup container renders and is visible
     */
    test("should have ButtonGroup container", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.buttonGroup).toBeVisible();
    });

    /**
     * TEST: ButtonGroup Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="ButtonGroup">                         │
     *   │       ↑                                                 │
     *   │  attribute: data-name === "ButtonGroup"                │
     *   │                                                         │
     *   │  Used for: Component identification and testing        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has correct data-name attribute
     */
    test("should have ButtonGroup data-name attribute", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.buttonGroup).toHaveAttribute("data-name", "ButtonGroup");
    });

    /**
     * TEST: Three Buttons Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button count inside group:                            │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │       1          2          3                           │
     *   │                                                         │
     *   │  Total count === 3                                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 3 buttons exist in the group
     */
    test("should have three buttons", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      const count = await ui.buttons.count();
      expect(count).toBe(3);
    });

    /**
     * TEST: All Buttons Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button visibility check:                              │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │      ✓          ✓          ✓                            │
     *   │   visible    visible    visible                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All three buttons are visible on page
     */
    test("buttons should have correct labels", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.firstButton).toBeVisible();
      await expect(ui.secondButton).toBeVisible();
      await expect(ui.thirdButton).toBeVisible();
    });
  });

  test.describe("Button Text", () => {
    /**
     * TEST: First Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button text content:                                  │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │      ↑                                                  │
     *   │  text === "First"                                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First button displays "First" text
     */
    test("first button should have text 'First'", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.firstButton).toHaveText("First");
    });

    /**
     * TEST: Second Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button text content:                                  │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │                 ↑                                       │
     *   │            text === "Second"                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second button displays "Second" text
     */
    test("second button should have text 'Second'", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.secondButton).toHaveText("Second");
    });

    /**
     * TEST: Third Button Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button text content:                                  │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │                            ↑                            │
     *   │                       text === "Third"                  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Third button displays "Third" text
     */
    test("third button should have text 'Third'", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.thirdButton).toHaveText("Third");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: ButtonGroup Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ButtonGroup Container Layout:                         │
     *   │                                                         │
     *   │  ┌─────────────────────────────────────────┐            │
     *   │  │ ┌──────┐┌──────┐┌──────┐                │            │
     *   │  │ │First ││Second││Third │  ← flex row   │            │
     *   │  │ └──────┘└──────┘└──────┘                │            │
     *   │  └─────────────────────────────────────────┘            │
     *   │                                                         │
     *   │  class includes: "flex"                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ButtonGroup uses flexbox layout
     */
    test("button group should have flex layout", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.buttonGroup).toHaveClass(/flex/);
    });

    /**
     * TEST: Buttons Outline Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Button variant styling (outline):                     │
     *   │                                                         │
     *   │  ┌──────────┐                                           │
     *   │  │  First   │  ← border + bg-background                │
     *   │  └──────────┘                                           │
     *   │                                                         │
     *   │  Outline variant characteristics:                       │
     *   │  - border: visible border                               │
     *   │  - bg-background: theme background color                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Buttons use outline variant styling
     */
    test("buttons should have outline variant", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await expect(ui.firstButton).toHaveClass(/border/);
      await expect(ui.firstButton).toHaveClass(/bg-background/);
    });

    /**
     * TEST: First Button Left Rounded
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Border radius - first button:                         │
     *   │                                                         │
     *   │  ╭──────────┬──────────┬──────────╮                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  ╰──────────┴──────────┴──────────╯                     │
     *   │  ↑                                                      │
     *   │  rounded-l-md (left corners rounded)                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First button has rounded left corners
     */
    test("first button should have left rounding only", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      // Check computed border-radius (parent CSS selectors control rounding)
      const borderRadius = await ui.firstButton.evaluate((el) => {
        const style = getComputedStyle(el);
        return {
          topLeft: style.borderTopLeftRadius,
          topRight: style.borderTopRightRadius,
          bottomRight: style.borderBottomRightRadius,
        };
      });
      expect(borderRadius.topLeft).not.toBe("0px");
      expect(borderRadius.topRight).toBe("0px");
      expect(borderRadius.bottomRight).toBe("0px");
    });

    /**
     * TEST: First Button Right Not Rounded
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Border radius - first button right side:              │
     *   │                                                         │
     *   │  ╭──────────┬──────────┬──────────╮                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  ╰──────────┴──────────┴──────────╯                     │
     *   │           ↑                                             │
     *   │  rounded-r-none (right corners flat for seamless join) │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First button has no rounded right corners
     */
    test("button group should have child rounding selectors", async ({
      page,
    }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      // ButtonGroup uses CSS child selectors for rounding, not classes on buttons
      const className = await ui.buttonGroup.getAttribute("class");
      expect(className).toContain("[&>*:not(:first-child)]:rounded-l-none");
      expect(className).toContain("[&>*:not(:last-child)]:rounded-r-none");
    });

    /**
     * TEST: Middle Button No Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Border radius - middle button:                        │
     *   │                                                         │
     *   │  ╭──────────┬──────────┬──────────╮                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  ╰──────────┴──────────┴──────────╯                     │
     *   │              ↑        ↑                                 │
     *   │         rounded-none (all corners flat)                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Middle button has no rounded corners
     */
    test("middle button should have no rounded corners", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      // Check computed border-radius (parent CSS selectors control rounding)
      const borderRadius = await ui.secondButton.evaluate((el) => {
        const style = getComputedStyle(el);
        return {
          topLeft: style.borderTopLeftRadius,
          topRight: style.borderTopRightRadius,
          bottomLeft: style.borderBottomLeftRadius,
          bottomRight: style.borderBottomRightRadius,
        };
      });
      expect(borderRadius.topLeft).toBe("0px");
      expect(borderRadius.topRight).toBe("0px");
      expect(borderRadius.bottomLeft).toBe("0px");
      expect(borderRadius.bottomRight).toBe("0px");
    });

    /**
     * TEST: Last Button Right Rounded
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Border radius - last button:                          │
     *   │                                                         │
     *   │  ╭──────────┬──────────┬──────────╮                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  ╰──────────┴──────────┴──────────╯                     │
     *   │                                  ↑                      │
     *   │             rounded-r-md (right corners rounded)        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Last button has rounded right corners
     */
    test("last button should have right rounding only", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      // Check computed border-radius (parent CSS selectors control rounding)
      const borderRadius = await ui.thirdButton.evaluate((el) => {
        const style = getComputedStyle(el);
        return {
          topLeft: style.borderTopLeftRadius,
          topRight: style.borderTopRightRadius,
          bottomLeft: style.borderBottomLeftRadius,
        };
      });
      expect(borderRadius.topLeft).toBe("0px");
      expect(borderRadius.bottomLeft).toBe("0px");
      expect(borderRadius.topRight).not.toBe("0px");
    });

    /**
     * TEST: Last Button Left Not Rounded
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Border radius - last button left side:                │
     *   │                                                         │
     *   │  ╭──────────┬──────────┬──────────╮                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  ╰──────────┴──────────┴──────────╯                     │
     *   │                        ↑                                │
     *   │  rounded-l-none (left corners flat for seamless join)  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Last button has no rounded left corners
     */
    test("buttons should have border removed between them", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      // ButtonGroup removes left border from non-first children to avoid double borders
      const className = await ui.buttonGroup.getAttribute("class");
      expect(className).toContain("[&>*:not(:first-child)]:border-l-0");
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Buttons Clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Click interaction:                                    │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │  First   │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │      ↑                                                  │
     *   │    click()                                              │
     *   │     👆                                                  │
     *   │                                                         │
     *   │  No error thrown = clickable                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Buttons respond to click events without error
     */
    test("buttons should be clickable", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await ui.firstButton.click();
      // Should not throw error
    });

    /**
     * TEST: Buttons Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Focus state:                                          │
     *   │                                                         │
     *   │  ┌──────────┬──────────┬──────────┐                     │
     *   │  │ [First]  │  Second  │  Third   │                     │
     *   │  └──────────┴──────────┴──────────┘                     │
     *   │      ↑                                                  │
     *   │   focused (has focus ring)                              │
     *   │                                                         │
     *   │  isFocused === true                                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Buttons can receive keyboard focus
     */
    test("buttons should be focusable", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await ui.firstButton.focus();
      await expect(ui.firstButton).toBeFocused();
    });

    /**
     * TEST: Tab Navigation Between Buttons
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Keyboard Tab navigation:                              │
     *   │                                                         │
     *   │  Step 1:  [First]  Second   Third    (focus on First)  │
     *   │              │                                          │
     *   │              ↓ Tab                                      │
     *   │  Step 2:   First  [Second]  Third    (focus on Second) │
     *   │                      │                                  │
     *   │                      ↓ Tab                              │
     *   │  Step 3:   First   Second  [Third]   (focus on Third)  │
     *   │                                                         │
     *   │  Tab key moves focus left → right through group        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key navigates through all buttons in order
     */
    test("can tab between buttons", async ({ page }) => {
      const ui = new ButtonGroupPage(page);
      await ui.goto();

      await ui.firstButton.focus();
      await page.keyboard.press("Tab");
      await expect(ui.secondButton).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.thirdButton).toBeFocused();
    });
  });
});
