import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * AUTO-FORM COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────┐  ┌─────────────────────────────────┐  │
 * │   │  AutoForm                   │  │  Live Form Data                 │  │
 * │   │  ┌───────────────────────┐  │  │  ┌─────────────────────────────┐│  │
 * │   │  │ Full Name             │  │  │  │ {                           ││  │
 * │   │  │ ┌───────────────────┐ │  │  │  │   "name": "John",           ││  │
 * │   │  │ │ John Doe          │ │  │  │  │   "email": "john@test.com", ││  │
 * │   │  │ └───────────────────┘ │  │  │  │   "bio": null,              ││  │
 * │   │  │                       │  │  │  │   "subscribe": false        ││  │
 * │   │  │ Email                 │  │  │  │ }                           ││  │
 * │   │  │ ┌───────────────────┐ │  │  │  └─────────────────────────────┘│  │
 * │   │  │ │ you@example.com   │ │  │  │                                 │  │
 * │   │  │ └───────────────────┘ │  │  └─────────────────────────────────┘  │
 * │   │  │                       │  │                                       │
 * │   │  │ Bio                   │  │                                       │
 * │   │  │ ┌───────────────────┐ │  │                                       │
 * │   │  │ │                   │ │  │                                       │
 * │   │  │ │ Tell us about... │ │  │                                       │
 * │   │  │ └───────────────────┘ │  │                                       │
 * │   │  │                       │  │                                       │
 * │   │  │ ☐ Subscribe to news   │  │                                       │
 * │   │  │                       │  │                                       │
 * │   │  │ ┌───────────────────┐ │  │                                       │
 * │   │  │ │     Submit        │ │  │                                       │
 * │   │  │ └───────────────────┘ │  │                                       │
 * │   │  └───────────────────────┘  │                                       │
 * │   └─────────────────────────────┘                                       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * FORM FIELDS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Field Types:                                                          │
 * │   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
 * │   │  Input (text)   │  │  Textarea       │  │  Checkbox       │         │
 * │   │  ─────────────  │  │  ─────────────  │  │  ☐ / ☑          │         │
 * │   │  name, email    │  │  bio (optional) │  │  subscribe      │         │
 * │   └─────────────────┘  └─────────────────┘  └─────────────────┘         │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VALIDATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   name:  min 2 characters                                               │
 * │   email: valid email format                                             │
 * │   bio:   optional (can be null)                                         │
 * │   subscribe: boolean                                                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class AutoFormPage extends BasePage {
  protected readonly componentName = "auto-form";

  // Form elements
  readonly autoForm: Locator;
  readonly nameInput: Locator;
  readonly emailInput: Locator;
  readonly bioTextarea: Locator;
  readonly subscribeLabel: Locator;
  readonly submitButton: Locator;
  readonly liveDataDisplay: Locator;

  constructor(page: Page) {
    super(page);

    this.autoForm = this.preview.locator("form").first();
    // AutoForm generates textboxes with labels as accessible names
    this.nameInput = this.preview.getByRole("textbox", { name: "Full Name" });
    this.emailInput = this.preview.getByRole("textbox", { name: "Email" });
    this.bioTextarea = this.preview.getByRole("textbox", { name: "Bio" });
    this.subscribeLabel = this.preview.locator('[data-name="SwitchLabel"]').first();
    this.submitButton = this.preview.getByRole("button", { name: "Submit" }).first();
    this.liveDataDisplay = this.preview.locator("pre").first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("AutoForm Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have form element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <form>                                                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   (form fields inside)                            │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  </form>                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main <form> element exists and is visible
     */
    test("should have form element", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.autoForm).toBeVisible();
    });

    /**
     * TEST: Should have name input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm                                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Full Name                                        │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ John Doe                                    │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑                                           │  │
     *   │  │   textbox[name="Full Name"] - VISIBLE?            │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input field with "Full Name" label exists
     */
    test("should have name input", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.nameInput).toBeVisible();
    });

    /**
     * TEST: Should have email input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm                                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Email                                            │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ you@example.com                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑                                           │  │
     *   │  │   textbox[name="Email"] - VISIBLE?                │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input field with "Email" label exists
     */
    test("should have email input", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.emailInput).toBeVisible();
    });

    /**
     * TEST: Should have bio textarea
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm                                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Bio                                              │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │  Tell us about yourself...                  │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑                                           │  │
     *   │  │   textbox[name="Bio"] (textarea) - VISIBLE?       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Bio textarea field with "Bio" label exists
     */
    test("should have bio textarea", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.bioTextarea).toBeVisible();
    });

    /**
     * TEST: Should have subscribe label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm                                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌────┐                                          │  │
     *   │  │   │ ○──┼──○ │  Subscribe to newsletter            │  │
     *   │  │   └────┘     ↑                                    │  │
     *   │  │         SwitchLabel - VISIBLE?                    │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Subscribe toggle switch label is visible
     */
    test("should have subscribe label", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.subscribeLabel).toBeVisible();
    });

    /**
     * TEST: Should have submit button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm                                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ...form fields...                               │  │
     *   │  │                                                   │  │
     *   │  │   ┌─────────────────────────────────────────────┐ │  │
     *   │  │   │               Submit                        │ │  │
     *   │  │   └─────────────────────────────────────────────┘ │  │
     *   │  │          ↑                                        │  │
     *   │  │   button[name="Submit"] - VISIBLE?                │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Form submit button exists and is visible
     */
    test("should have submit button", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.submitButton).toBeVisible();
    });
  });

  test.describe("Live Data Display", () => {
    /**
     * TEST: Should have live data display
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AutoForm          │  Live Form Data                   │
     *   │  ┌───────────────┐ │  ┌─────────────────────────────┐  │
     *   │  │ Full Name     │ │  │ <pre>                       │  │
     *   │  │ ───────────── │ │  │   {                         │  │
     *   │  │ Email         │ │  │     "name": "...",          │  │
     *   │  │ ───────────── │ │  │     "email": "..."          │  │
     *   │  │ Bio           │ │  │   }                         │  │
     *   │  │ ───────────── │ │  │ </pre>                      │  │
     *   │  │ [Submit]      │ │  │        ↑                    │  │
     *   │  └───────────────┘ │  │   <pre> element VISIBLE?    │  │
     *   │                    │  └─────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Live data display <pre> element is visible
     */
    test("should have live data display", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.liveDataDisplay).toBeVisible();
    });

    /**
     * TEST: Live data should show JSON format
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Live Form Data                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  {                          <── contains "{"?     │  │
     *   │  │    "name": "John",                                │  │
     *   │  │    "email": "john@test.com",                      │  │
     *   │  │    "bio": null,                                   │  │
     *   │  │    "subscribe": false                             │  │
     *   │  │  }                          <── contains "}"?     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  Valid JSON structure with braces                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Live data displays valid JSON with { and }
     */
    test("live data should show JSON format", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      const text = await ui.liveDataDisplay.textContent();
      expect(text).toContain("{");
      expect(text).toContain("}");
    });
  });

  test.describe("Form Interaction", () => {
    /**
     * TEST: Should allow typing in name field
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full Name                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: fill("Test User")                       │  │
     *   │  │   ─────────────────────────>                      │  │
     *   │  │   ┌─────────────────────────────────────────────┐ │  │
     *   │  │   │ Test User                                   │ │  │
     *   │  │   └─────────────────────────────────────────────┘ │  │
     *   │  │                                                   │  │
     *   │  │   Step 2: expect value === "Test User"            │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input accepts and retains typed value
     */
    test("should allow typing in name field", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await ui.nameInput.fill("Test User");
      await expect(ui.nameInput).toHaveValue("Test User");
    });

    /**
     * TEST: Should allow typing in email field
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Email                                                  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: fill("test@example.com")                │  │
     *   │  │   ─────────────────────────>                      │  │
     *   │  │   ┌─────────────────────────────────────────────┐ │  │
     *   │  │   │ test@example.com                            │ │  │
     *   │  │   └─────────────────────────────────────────────┘ │  │
     *   │  │                                                   │  │
     *   │  │   Step 2: expect value === "test@example.com"     │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input accepts and retains typed value
     */
    test("should allow typing in email field", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await ui.emailInput.fill("test@example.com");
      await expect(ui.emailInput).toHaveValue("test@example.com");
    });

    /**
     * TEST: Should allow typing in bio textarea
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Bio                                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: fill("This is my bio")                  │  │
     *   │  │   ─────────────────────────>                      │  │
     *   │  │   ┌─────────────────────────────────────────────┐ │  │
     *   │  │   │ This is my bio                              │ │  │
     *   │  │   │                                             │ │  │
     *   │  │   └─────────────────────────────────────────────┘ │  │
     *   │  │                                                   │  │
     *   │  │   Step 2: expect value === "This is my bio"       │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Bio textarea accepts and retains typed value
     */
    test("should allow typing in bio textarea", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await ui.bioTextarea.fill("This is my bio");
      await expect(ui.bioTextarea).toHaveValue("This is my bio");
    });

    /**
     * TEST: Should have subscribe toggle visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Subscribe to newsletter                               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │   ┌──────────────┐                              │   │
     *   │   │   │  ○────────○  │  Subscribe                   │   │
     *   │   │   └──────────────┘      ↑                       │   │
     *   │   │                    SwitchLabel - VISIBLE?       │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Subscribe toggle label is visible for interaction
     */
    test("should have subscribe toggle visible", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.subscribeLabel).toBeVisible();
    });
  });

  test.describe("Placeholders", () => {
    /**
     * TEST: Name input should have placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full Name                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ John Doe                                    │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑                                           │  │
     *   │  │   placeholder="John Doe" ?                        │  │
     *   │  │                                                   │  │
     *   │  │   <input placeholder="John Doe" />                │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input has placeholder attribute "John Doe"
     */
    test("name input should have placeholder", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.nameInput).toBeVisible();
      await expect(ui.nameInput).toHaveAttribute("placeholder", "John Doe");
    });

    /**
     * TEST: Email input should have placeholder
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Email                                                  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │ you@example.com                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │       ↑                                           │  │
     *   │  │   placeholder="you@example.com" ?                 │  │
     *   │  │                                                   │  │
     *   │  │   <input placeholder="you@example.com" />         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input has placeholder "you@example.com"
     */
    test("email input should have placeholder", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await expect(ui.emailInput).toBeVisible();
      await expect(ui.emailInput).toHaveAttribute(
        "placeholder",
        "you@example.com"
      );
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Name input should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full Name                                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: focus()                                 │  │
     *   │  │   ─────────────────────────>                      │  │
     *   │  │   ┌─────────────────────────────────────────────┐ │  │
     *   │  │   │                                     :focus  │ │  │
     *   │  │   └─────────────────────────────────────────────┘ │  │
     *   │  │          ↑                                        │  │
     *   │  │   ring/outline visible (focused state)            │  │
     *   │  │                                                   │  │
     *   │  │   Step 2: expect toBeFocused()                    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input can receive keyboard focus
     */
    test("name input should be focusable", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await ui.nameInput.focus();
      await expect(ui.nameInput).toBeFocused();
    });

    /**
     * TEST: Submit button should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: focus()                                       │
     *   │   ─────────────────────────>                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │               Submit                    :focus  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │          ↑                                              │
     *   │   ring/outline visible (focused state)                  │
     *   │                                                         │
     *   │   Step 2: expect toBeFocused()                          │
     *   │                                                         │
     *   │   Keyboard users can Tab to submit the form             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Submit button can receive keyboard focus
     */
    test("submit button should be focusable", async ({ page }) => {
      const ui = new AutoFormPage(page);
      await ui.goto();

      await ui.submitButton.focus();
      await expect(ui.submitButton).toBeFocused();
    });
  });
});
