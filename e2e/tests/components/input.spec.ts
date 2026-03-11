import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * INPUT COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <input data-name="Input" type="text|email|password|number">           │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │   │
 * │   │  Placeholder text (text-muted-foreground)                       │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * INPUT TYPES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   TEXT:     ┌──────────────────────────────────────────┐                │
 * │             │  Default input                           │                │
 * │             └──────────────────────────────────────────┘                │
 * │                                                                         │
 * │   EMAIL:    ┌──────────────────────────────────────────┐                │
 * │             │  test@example.com                        │                │
 * │             └──────────────────────────────────────────┘                │
 * │                                                                         │
 * │   PASSWORD: ┌──────────────────────────────────────────┐                │
 * │             │  ••••••••••                              │  ← Masked      │
 * │             └──────────────────────────────────────────┘                │
 * │                                                                         │
 * │   NUMBER:   ┌──────────────────────────────────────────┐                │
 * │             │  42                                   ▲▼ │  ← Spinners    │
 * │             └──────────────────────────────────────────┘                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .h-9              ← Height (36px)                          │       │
 * │   │  .w-full           ← Full width                             │       │
 * │   │  .rounded-md       ← Rounded corners                        │       │
 * │   │  .border           ← Border                                 │       │
 * │   │  .bg-transparent   ← Transparent background                 │       │
 * │   │  .px-3 .py-1       ← Padding                                │       │
 * │   │  .shadow-xs        ← Subtle shadow                          │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * │   FOCUS STATE:                                                          │
 * │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐      │
 * │   ┊  ┌─────────────────────────────────────────────────────────┐ ┊      │
 * │   ┊  │  focus-visible:ring-2                                   │ ┊      │
 * │   ┊  └─────────────────────────────────────────────────────────┘ ┊      │
 * │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class InputPage extends BasePage {
  protected readonly componentName = "input";

  // Demo: Basic inputs
  readonly defaultInput: Locator;
  readonly emailInput: Locator;
  readonly passwordInput: Locator;
  readonly numberInput: Locator;
  readonly customStyledInput: Locator;

  // Demo: Two-way binding
  readonly bindingInput: Locator;
  readonly bindingValueDisplay: Locator;

  constructor(page: Page) {
    super(page);

    // Basic inputs by placeholder - scoped within preview
    this.defaultInput = this.preview.getByPlaceholder("Default input");
    this.emailInput = this.preview.getByPlaceholder("Email input");
    this.passwordInput = this.preview.getByPlaceholder("Password input");
    this.numberInput = this.preview.getByPlaceholder("Number input");
    this.customStyledInput = this.preview.getByPlaceholder("Custom styled input");

    // Two-way binding - scoped within preview
    this.bindingInput = this.preview.getByPlaceholder("Type here...");
    this.bindingValueDisplay = this.preview
      .locator("p.text-sm.mt-2")
      .filter({ hasText: "Value:" });
  }

  async getFirstInput(): Promise<Locator> {
    return this.preview.locator('[data-name="Input"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Input Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Input Has Correct Base Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Input"]  base styling:                    │
     *   │   ╭─────────────────────────────────────────────────────╮
     *   │   │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░│
     *   │   │  Default input                                     │
     *   │   ╰─────────────────────────────────────────────────────╯
     *   │   ↑─ rounded-md ─ border ─ bg-transparent ─ shadow-xs ─↑
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has all required base styling classes
     */
    test("should render Input with correct base classes", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/rounded-md/);
      await expect(ui.defaultInput).toHaveClass(/border/);
      await expect(ui.defaultInput).toHaveClass(/bg-transparent/);
      await expect(ui.defaultInput).toHaveClass(/shadow-xs/);
    });

    /**
     * TEST: Input Has Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <input data-name="Input">  ← ATTRIBUTE CHECK          │
     *   │          ↑                                              │
     *   │   data-name === "Input"                                 │
     *   │   (component identification)                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has proper data-name for identification
     */
    test("should have data-name attribute", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveAttribute("data-name", "Input");
    });

    /**
     * TEST: Element Is An Input Tag
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   HTML structure:                                       │
     *   │   <input type="text" ... />                             │
     *   │      ↑                                                  │
     *   │   tagName.toLowerCase() === "input"                     │
     *   │                                                         │
     *   │   (semantic HTML, not div with contenteditable)         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component uses semantic input element
     */
    test("should be an input element", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      const tagName = await ui.defaultInput.evaluate(
        (el) => el.tagName.toLowerCase()
      );
      expect(tagName).toBe("input");
    });
  });

  test.describe("Input Types", () => {
    /**
     * TEST: Default Input Has Text Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   TEXT:                                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Default input                                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   type === "text" (default input type)                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Default input uses text type
     */
    test("should have text type by default", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveAttribute("type", "text");
    });

    /**
     * TEST: Email Input Has Email Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   EMAIL:                                                │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  test@example.com                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   type === "email"                                      │
     *   │   (enables email keyboard on mobile, validation)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input uses email type
     */
    test("should have email type for email input", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.emailInput).toHaveAttribute("type", "email");
    });

    /**
     * TEST: Password Input Has Password Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   PASSWORD:                                             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ••••••••••                                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   type === "password" (masks input characters)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Password input masks characters
     */
    test("should have password type for password input", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.passwordInput).toHaveAttribute("type", "password");
    });

    /**
     * TEST: Number Input Has Number Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   NUMBER:                                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  42                                          ▲▼ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   type === "number" (numeric keyboard, spinners)        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Number input has number type with spinners
     */
    test("should have number type for number input", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.numberInput).toHaveAttribute("type", "number");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Input Has Focus-Visible Ring Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Focus state styling:                                  │
     *   │   ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐ │
     *   │   ┊  ┌─────────────────────────────────────────────┐  ┊ │
     *   │   ┊  │  [cursor blinking]                          │  ┊ │
     *   │   ┊  └─────────────────────────────────────────────┘  ┊ │
     *   │   └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘ │
     *   │        ↑ focus-visible:ring-2 (2px ring on focus)       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input shows focus ring when focused
     */
    test("should have focus-visible ring classes", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/focus-visible:ring-2/);
    });

    /**
     * TEST: Input Has Placeholder Styling Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Placeholder text styling:                             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Default input  (muted/gray color)              │   │
     *   │   │       ↑                                         │   │
     *   │   │  placeholder:text-muted-foreground              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Placeholder text uses muted color
     */
    test("should have placeholder styling class", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/placeholder:text-muted-foreground/);
    });

    /**
     * TEST: Custom Styled Input Has Purple Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Custom styling demonstration:                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Custom styled input                            │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ↑────────── border-purple-500 ────────────────────↑   │
     *   │                (purple border color)                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Custom class prop applies purple border
     */
    test("custom styled input should have purple border", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.customStyledInput).toHaveClass(/border-purple-500/);
    });

    /**
     * TEST: Input Has Correct Height Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Input height:                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Default input                                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ↕ h-9 (36px height for consistent sizing)             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has fixed height for consistency
     */
    test("should have correct height class", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/h-9/);
    });

    /**
     * TEST: Input Has Padding Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Input padding:                                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │← px-3 →│  Default input  │← px-3 →│             │   │
     *   │   │        │ ↕ py-1 ↕        │        │             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has horizontal and vertical padding
     */
    test("should have padding classes", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/px-3/);
      await expect(ui.defaultInput).toHaveClass(/py-1/);
    });
  });

  test.describe("Interactions", () => {
    /**
     * TEST: Input Accepts Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before:                                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Default input  (placeholder)                   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                      ↓ TYPE "Hello World"               │
     *   │   After:                                                │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Hello World|                                   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑ value === "Hello World"                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input accepts and stores text value
     */
    test("should accept text input", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.defaultInput.fill("Hello World");
      await expect(ui.defaultInput).toHaveValue("Hello World");
    });

    /**
     * TEST: Email Input Accepts Email
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Email input:                                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  test@example.com|                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   value === "test@example.com"                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input accepts email format values
     */
    test("should accept email input", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.emailInput.fill("test@example.com");
      await expect(ui.emailInput).toHaveValue("test@example.com");
    });

    /**
     * TEST: Password Input Masks Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Password input (visually masked):                     │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  •••••••••  (displayed as dots)                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   type="password" + value="secret123"                   │
     *   │   (value accessible programmatically, masked visually)  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Password type masks input while preserving value
     */
    test("password input should mask text", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.passwordInput.fill("secret123");
      await expect(ui.passwordInput).toHaveValue("secret123");
      // Password type masks the input visually but value is still accessible
      await expect(ui.passwordInput).toHaveAttribute("type", "password");
    });

    /**
     * TEST: Number Input Accepts Numbers
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Number input:                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  42|                                         ▲▼ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑                                                │
     *   │   value === "42" (numeric input)                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Number input accepts numeric values
     */
    test("number input should accept numbers", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.numberInput.fill("42");
      await expect(ui.numberInput).toHaveValue("42");
    });

    /**
     * TEST: Inputs Display Placeholder Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Empty inputs show placeholders:                       │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Default input  ← placeholder="Default input"   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Email input    ← placeholder="Email input"     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Placeholder attributes display hint text
     */
    test("should display placeholder text", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveAttribute(
        "placeholder",
        "Default input"
      );
      await expect(ui.emailInput).toHaveAttribute("placeholder", "Email input");
    });

    /**
     * TEST: Input Value Can Be Cleared
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Fill input                                    │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Some text|                                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                      ↓ CLEAR                            │
     *   │   Step 2: Clear input                                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  |                          (empty)             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │        ↑ value === ""                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input value can be cleared programmatically
     */
    test("should clear input value", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.defaultInput.fill("Some text");
      await expect(ui.defaultInput).toHaveValue("Some text");

      await ui.defaultInput.clear();
      await expect(ui.defaultInput).toHaveValue("");
    });
  });

  // Two-way binding tests require WASM hydration (local dev server)
  test.describe("Two-way Binding", () => {
    /**
     * TEST: Binding Input Updates Display Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Two-way binding (Leptos reactive signal):             │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Hello Leptos|          ← TYPE HERE             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                      ↓ (signal updates)                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Value: Hello Leptos    ← DISPLAY UPDATES       │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input changes propagate to display via signal
     */
    test("binding input should update display value", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.bindingInput).toBeVisible();
      await ui.bindingInput.fill("Hello Leptos");
      await expect(ui.bindingValueDisplay).toContainText("Value: Hello Leptos");
    });

    /**
     * TEST: Binding Reflects Typed Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Reactive binding flow:                                │
     *   │                                                         │
     *   │   Input ─────────────────────────────→ Signal           │
     *   │   ┌─────────────────────────┐         ┌───────────┐     │
     *   │   │  Test123|               │ ──────→ │ "Test123" │     │
     *   │   └─────────────────────────┘         └─────┬─────┘     │
     *   │                                             │           │
     *   │   Display ←─────────────────────────────────┘           │
     *   │   ┌─────────────────────────┐                           │
     *   │   │  Value: Test123         │                           │
     *   │   └─────────────────────────┘                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typed value is reflected in the display
     */
    test("binding should reflect typed value", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.bindingInput.fill("Test123");
      await expect(ui.bindingValueDisplay).toContainText("Value: Test123");
    });

    /**
     * TEST: Binding Input Is Clearable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Fill input                                    │
     *   │   ┌─────────────────────────┐  →  Value: Test value     │
     *   │   │  Test value|            │                           │
     *   │   └─────────────────────────┘                           │
     *   │                      ↓ CLEAR                            │
     *   │   Step 2: Clear input                                   │
     *   │   ┌─────────────────────────┐  →  Value:                │
     *   │   │  |                      │       ↑                   │
     *   │   └─────────────────────────┘  (empty string)           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clearing input updates binding to empty
     */
    test("binding input should be clearable", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.bindingInput.fill("Test value");
      await expect(ui.bindingValueDisplay).toContainText("Value: Test value");

      await ui.bindingInput.fill("");
      await expect(ui.bindingValueDisplay).toContainText("Value:");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Input Is Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard accessibility:                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  [Default input|]   ← TAB TO FOCUS              │   │
     *   │   │        ↑                                        │   │
     *   │   │  document.activeElement === input               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input can receive keyboard focus
     */
    test("should be focusable", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await ui.defaultInput.focus();
      await expect(ui.defaultInput).toBeFocused();
    });

    /**
     * TEST: Keyboard Navigation Between Inputs
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Tab navigation flow:                                  │
     *   │   ┌─────────────────────────┐                           │
     *   │   │  [Default input]        │ ← FOCUSED (start)         │
     *   │   └─────────────────────────┘                           │
     *   │              ↓ TAB                                      │
     *   │   ┌─────────────────────────┐                           │
     *   │   │  [Email input]          │ ← FOCUSED                 │
     *   │   └─────────────────────────┘                           │
     *   │              ↓ TAB                                      │
     *   │   ┌─────────────────────────┐                           │
     *   │   │  [Password input]       │ ← FOCUSED                 │
     *   │   └─────────────────────────┘                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key navigates between inputs sequentially
     */
    test("should support keyboard navigation between inputs", async ({
      page,
    }) => {
      const ui = new InputPage(page);
      await ui.goto();

      // Focus first input and tab to next
      await ui.defaultInput.focus();
      await expect(ui.defaultInput).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.emailInput).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.passwordInput).toBeFocused();
    });

    /**
     * TEST: All Inputs Have Full Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Full-width inputs (w-full):                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │◀─────────────── w-full ───────────────────────▶│   │
     *   │   │  Default input                                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │◀─────────────── w-full ───────────────────────▶│   │
     *   │   │  Email input                                    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │◀─────────────── w-full ───────────────────────▶│   │
     *   │   │  Password input                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All inputs expand to fill container width
     */
    test("all inputs should have full width", async ({ page }) => {
      const ui = new InputPage(page);
      await ui.goto();

      await expect(ui.defaultInput).toHaveClass(/w-full/);
      await expect(ui.emailInput).toHaveClass(/w-full/);
      await expect(ui.passwordInput).toHaveClass(/w-full/);
    });
  });
});
