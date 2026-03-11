import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * INPUT-PHONE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   InputPhone                                                            │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ┌───────────┐ ┌─────────────────────────────────────────────┐  │   │
 * │   │  │ 🇺🇸 +1 ▼  │ │  (555) 123-4567                             │  │   │
 * │   │  └───────────┘ └─────────────────────────────────────────────┘  │   │
 * │   │       ↑                           ↑                              │   │
 * │   │  Country selector           Phone number input                   │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * │   Preview                                                               │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  +1 555-123-4567                    ← Formatted international   │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * COUNTRY SELECTOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Click to open:                                                        │
 * │   ┌───────────────────────┐                                             │
 * │   │  🇺🇸 United States +1 │                                             │
 * │   │  🇬🇧 United Kingdom +44│                                            │
 * │   │  🇨🇦 Canada +1        │                                             │
 * │   │  🇦🇺 Australia +61    │                                             │
 * │   │  ...                  │                                             │
 * │   └───────────────────────┘                                             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * PHONE FORMATTING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Input:     5551234567                                                 │
 * │   Display:   (555) 123-4567      (national format)                      │
 * │   Preview:   +1 555-123-4567     (international format)                 │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class InputPhonePage extends BasePage {
  protected readonly componentName = "input-phone";

  // InputPhone elements
  readonly countrySelector: Locator;
  readonly phoneInput: Locator;
  readonly preview: Locator;

  constructor(page: Page) {
    super(page);

    // Use role-based selectors, .first() to get the main demo (not disabled demo)
    this.countrySelector = page
      .getByRole("button", { name: "Select country" })
      .first();
    this.phoneInput = page
      .getByRole("textbox", { name: "Phone number" })
      .first();
    this.preview = page.locator("p").filter({ hasText: "Enter a phone number" });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("InputPhone Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Country Selector Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  InputPhone                                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ┌─────────────────────────────┐   │  │
     *   │  │  │ +1  ▼     │  │  Phone number input         │   │  │
     *   │  │  └───────────┘  └─────────────────────────────┘   │  │
     *   │  │       ▲                                           │  │
     *   │  │       │                                           │  │
     *   │  │  EXISTS? ◄── VALIDATES                            │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Country code selector button is visible
     */
    test("should have country selector", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await expect(ui.countrySelector).toBeVisible();
    });

    /**
     * TEST: Phone Input Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  InputPhone                                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ┌─────────────────────────────┐   │  │
     *   │  │  │ +1  ▼     │  │  (555) 123-4567             │   │  │
     *   │  │  └───────────┘  └─────────────────────────────┘   │  │
     *   │  │                          ▲                        │  │
     *   │  │                          │                        │  │
     *   │  │                    EXISTS? ◄── VALIDATES          │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Phone number input field is visible
     */
    test("should have phone input", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await expect(ui.phoneInput).toBeVisible();
    });

    /**
     * TEST: Preview Text Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ┌─────────────────────────────┐   │  │
     *   │  │  │ +1  ▼     │  │  Phone input                │   │  │
     *   │  │  └───────────┘  └─────────────────────────────┘   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  Preview                                                │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Enter a phone number"  ◄── EXISTS?              │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Preview text area is visible below the input
     */
    test("should have preview text", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await expect(ui.preview).toBeVisible();
    });
  });

  test.describe("Phone Input", () => {
    /**
     * TEST: Numeric Input Mode
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <input                                                 │
     *   │    type="text"                                          │
     *   │    inputmode="numeric"  ◄── ATTRIBUTE EXISTS?           │
     *   │  />                                                     │
     *   │                                                         │
     *   │  On mobile:                                             │
     *   │  ┌───────────────────────┐                              │
     *   │  │  1   2   3            │                              │
     *   │  │  4   5   6            │  ◄── Numeric keyboard        │
     *   │  │  7   8   9            │                              │
     *   │  │      0                │                              │
     *   │  └───────────────────────┘                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has inputmode="numeric" for mobile keyboards
     */
    test("phone input should have numeric inputmode", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await expect(ui.phoneInput).toHaveAttribute("inputmode", "numeric");
    });

    /**
     * TEST: Phone Number Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  User types: 5551234567                                 │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ┌─────────────────────────────┐   │  │
     *   │  │  │ +1  ▼     │  │  (555) 123-4567             │   │  │
     *   │  │  └───────────┘  └─────────────────────────────┘   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                          ▲                              │
     *   │                          │                              │
     *   │                  CONTAINS "555"? ◄── VALIDATES          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Phone numbers can be typed and are accepted
     */
    test("should allow typing phone number", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await ui.phoneInput.fill("5551234567");
      const value = await ui.phoneInput.inputValue();
      expect(value.replace(/\D/g, "")).toContain("555");
    });
  });

  test.describe("Country Selector", () => {
    /**
     * TEST: Country Code Display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Country Selector Button                                │
     *   │  ┌───────────────┐                                      │
     *   │  │  +1  ▼        │  ◄── HAS TEXT CONTENT?               │
     *   │  └───────────────┘                                      │
     *   │                                                         │
     *   │  Possible content:                                      │
     *   │  - Flag emoji + code (e.g., "+1")                       │
     *   │  - Country abbreviation                                 │
     *   │  - Just the dial code                                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Country selector shows some country indicator
     */
    test("should show country code", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      // Should show some country indicator (flag or code)
      const text = await ui.countrySelector.textContent();
      expect(text).toBeTruthy();
    });

    /**
     * TEST: Selector Clickability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────┐                                      │
     *   │  │  +1  ▼        │ ◄── CLICK ─┐                         │
     *   │  └───────────────┘            │                         │
     *   │                               ▼                         │
     *   │                    ┌───────────────────────┐            │
     *   │                    │  United States +1    │            │
     *   │                    │  United Kingdom +44  │            │
     *   │                    │  Canada +1           │            │
     *   │                    │  ...                 │            │
     *   │                    └───────────────────────┘            │
     *   │                                                         │
     *   │   IS ENABLED? ◄── VALIDATES                             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Country selector button is enabled and clickable
     */
    test("selector should be clickable", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      // Verify the button can be clicked (dropdown uses popover API)
      await expect(ui.countrySelector).toBeEnabled();
    });
  });

  test.describe("Preview", () => {
    /**
     * TEST: Default Preview Message
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Input (empty)                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ┌─────────────────────────────┐   │  │
     *   │  │  │ +1  ▼     │  │                             │   │  │
     *   │  │  └───────────┘  └─────────────────────────────┘   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  Preview (when empty)                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Enter a phone number"  ◄── CONTAINS TEXT?       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default message shown when no number is entered
     */
    test("should show default message when empty", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await expect(ui.preview).toContainText("Enter a phone number");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Container Max Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Page                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │  ┌─────────────────────────────┐                  │  │
     *   │  │  │◄────── max-w-sm ──────────►│                  │  │
     *   │  │  │                             │                  │  │
     *   │  │  │  [Country] [Phone Input  ]  │                  │  │
     *   │  │  │                             │                  │  │
     *   │  │  └─────────────────────────────┘                  │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │   HAS CLASS "max-w-sm"? ◄── VALIDATES                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has max-width constraint for styling
     */
    test("container should have max width", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      const container = page.locator(".max-w-sm").first();
      await expect(container).toBeVisible();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Phone Input Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Keyboard navigation / Tab key                          │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌───────────┐  ╔═════════════════════════════╗   │  │
     *   │  │  │ +1  ▼     │  ║  Phone input               ║   │  │
     *   │  │  └───────────┘  ╚═════════════════════════════╝   │  │
     *   │  │                          ▲                        │  │
     *   │  │                          │                        │  │
     *   │  │                    FOCUS RING (active)            │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │   CAN RECEIVE FOCUS? ◄── VALIDATES                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Phone input can be focused via keyboard/programmatically
     */
    test("phone input should be focusable", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await ui.phoneInput.focus();
      await expect(ui.phoneInput).toBeFocused();
    });

    /**
     * TEST: Country Selector Focusability
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Keyboard navigation / Tab key                          │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ╔═══════════╗  ┌─────────────────────────────┐   │  │
     *   │  │  ║ +1  ▼     ║  │  Phone input               │   │  │
     *   │  │  ╚═══════════╝  └─────────────────────────────┘   │  │
     *   │  │       ▲                                           │  │
     *   │  │       │                                           │  │
     *   │  │  FOCUS RING (active)                              │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │   CAN RECEIVE FOCUS? ◄── VALIDATES                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Country selector can be focused via keyboard
     */
    test("country selector should be focusable", async ({ page }) => {
      const ui = new InputPhonePage(page);
      await ui.goto();

      await ui.countrySelector.focus();
      await expect(ui.countrySelector).toBeFocused();
    });
  });
});
