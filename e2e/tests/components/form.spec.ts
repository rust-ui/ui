import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * FORM COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <form>                                                                │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  FormField                                                      │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Full Name                       ← label                  │  │   │
 * │   │  │  ┌─────────────────────────────────────────────────────┐  │  │   │
 * │   │  │  │                                                     │  │  │   │
 * │   │  │  └─────────────────────────────────────────────────────┘  │  │   │
 * │   │  │  Name must be at least 2 characters  ← error (role=alert) │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  FormField                                                      │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Email                                                    │  │   │
 * │   │  │  ┌─────────────────────────────────────────────────────┐  │  │   │
 * │   │  │  │                                                     │  │  │   │
 * │   │  │  └─────────────────────────────────────────────────────┘  │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  FormField (textarea)                                           │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Bio                                                      │  │   │
 * │   │  │  ┌─────────────────────────────────────────────────────┐  │  │   │
 * │   │  │  │                                                     │  │  │   │
 * │   │  │  │                                                     │  │  │   │
 * │   │  │  └─────────────────────────────────────────────────────┘  │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  ☐ Subscribe to newsletter                                      │   │
 * │   │                                                                 │   │
 * │   │  [Submit]                                                       │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * │   Live JSON Display:                                                    │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  <pre>                                                          │   │
 * │   │  {                                                              │   │
 * │   │    "name": "Jane Doe",                                          │   │
 * │   │    "email": "jane@example.com",                                 │   │
 * │   │    "bio": "Developer",                                          │   │
 * │   │    "subscribe": "true"                                          │   │
 * │   │  }                                                              │   │
 * │   │  </pre>                                                         │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VALIDATION FLOW (on-blur):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   1. User types:        2. User blurs:         3. User fixes:           │
 * │   ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐       │
 * │   │ J               │   │ J               │   │ John            │       │
 * │   └─────────────────┘   └═════════════════┘   └─────────────────┘       │
 * │   (no error shown)      Name must be...       (error cleared)           │
 * │                         aria-invalid=true                               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CHECKBOX STATE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Unchecked:                         Checked:                           │
 * │   ┌───┐                              ┌───┐                              │
 * │   │   │ Subscribe to newsletter      │ ✓ │ Subscribe to newsletter      │
 * │   └───┘                              └───┘                              │
 * │   data-state="unchecked"             data-state="checked"               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class FormPage extends BasePage {
  protected readonly componentName = "form";

  // Form elements
  readonly form: Locator;
  readonly nameInput: Locator;
  readonly emailInput: Locator;
  readonly bioTextarea: Locator;
  readonly subscribeCheckbox: Locator;
  readonly submitButton: Locator;

  // Labels
  readonly nameLabel: Locator;
  readonly emailLabel: Locator;
  readonly bioLabel: Locator;
  readonly subscribeLabel: Locator;

  // Error messages
  readonly nameError: Locator;
  readonly emailError: Locator;

  // Live JSON display
  readonly jsonDisplay: Locator;

  constructor(page: Page) {
    super(page);

    // Form container - scoped within preview
    this.form = this.preview.locator("form").first();

    // Inputs by label (more reliable than placeholder) - scoped within preview
    this.nameInput = this.preview.getByLabel("Full Name");
    this.emailInput = this.preview.getByLabel("Email");
    this.bioTextarea = this.preview.getByLabel("Bio");

    // Checkbox - find by the label text - scoped within preview
    this.subscribeCheckbox = this.preview.locator('[role="checkbox"]').first();
    this.subscribeLabel = this.preview.getByText("Subscribe to newsletter");

    // Submit button - scoped within preview
    this.submitButton = this.preview.getByRole("button", { name: "Submit" });

    // Labels - scoped within preview
    this.nameLabel = this.preview.locator("label").filter({ hasText: "Full Name" });
    this.emailLabel = this.preview.locator("label").filter({ hasText: "Email" });
    this.bioLabel = this.preview.locator("label").filter({ hasText: "Bio" });

    // Error messages - using role="alert" - scoped within preview
    this.nameError = this.preview
      .locator('[data-name="FormField"]')
      .filter({ has: this.preview.getByLabel("Full Name") })
      .locator('[role="alert"]');
    this.emailError = this.preview
      .locator('[data-name="FormField"]')
      .filter({ has: this.preview.getByLabel("Email") })
      .locator('[role="alert"]');

    // Live JSON display - scoped within preview
    this.jsonDisplay = this.preview.locator("pre").first();
  }

  // Override goto to navigate to test page
  async goto() {
    await this.page.goto("/test");
    // Wait for form to be present
    await this.page.waitForSelector("form");
    // Wait for hydration - WASM needs to load for interactivity
    // Check if inputs are reactive by waiting for the JSON display to be ready
    await this.page.waitForSelector("pre");
    // Small delay to ensure WASM is fully hydrated
    await this.page.waitForTimeout(500);
  }

  // Helper to get current JSON data
  async getJsonData(): Promise<Record<string, string>> {
    const text = await this.jsonDisplay.textContent();
    if (!text || text.trim() === "" || text.trim() === "{}") {
      return {};
    }
    try {
      return JSON.parse(text);
    } catch {
      return {};
    }
  }

  // Helper to wait for JSON to update
  async waitForJsonUpdate(key: string, value: string) {
    await expect(async () => {
      const data = await this.getJsonData();
      expect(data[key]).toBe(value);
    }).toPass({ timeout: 5000 });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Form Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Form Renders All Fields
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <form>                                                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Full Name     [_______________] ← nameInput      │  │
     *   │  │  Email         [_______________] ← emailInput     │  │
     *   │  │  Bio           [_______________] ← bioTextarea    │  │
     *   │  │                [               ]                  │  │
     *   │  │  [ ] Subscribe to newsletter   ← checkbox         │  │
     *   │  │  [Submit]                      ← submitButton     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All form elements are rendered and visible
     */
    test("should render form with all fields", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await expect(ui.form).toBeVisible();
      await expect(ui.nameInput).toBeVisible();
      await expect(ui.emailInput).toBeVisible();
      await expect(ui.bioTextarea).toBeVisible();
      await expect(ui.subscribeCheckbox).toBeVisible();
      await expect(ui.submitButton).toBeVisible();
    });

    /**
     * TEST: Labels Render for All Fields
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  "Full Name"  ← label visible?                    │  │
     *   │  │  [_______________]                                │  │
     *   │  │  "Email"      ← label visible?                    │  │
     *   │  │  [_______________]                                │  │
     *   │  │  "Bio"        ← label visible?                    │  │
     *   │  │  [_______________]                                │  │
     *   │  │  "Subscribe to newsletter"  ← label visible?      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All form field labels are visible
     */
    test("should render labels for all fields", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await expect(ui.nameLabel).toBeVisible();
      await expect(ui.emailLabel).toBeVisible();
      await expect(ui.bioLabel).toBeVisible();
      await expect(ui.subscribeLabel).toBeVisible();
    });

    /**
     * TEST: Input Has data-name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <input data-name="Input" ... />                        │
     *   │         ↑                                               │
     *   │     attribute = "Input"                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input has data-name="Input" for component ID
     */
    test("should have data-name attribute on input", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await expect(ui.nameInput).toHaveAttribute("data-name", "Input");
    });

    /**
     * TEST: Bio Field is Textarea Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Bio                                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ <textarea>   ← tagName = "textarea"               │  │
     *   │  │                                                   │  │
     *   │  │ (multi-line text area, not input)                 │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Bio field uses textarea element for multi-line input
     */
    test("bio field should be a textarea element", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      const tagName = await ui.bioTextarea.evaluate(
        (el) => el.tagName.toLowerCase()
      );
      expect(tagName).toBe("textarea");
    });
  });

  test.describe("Live JSON Display", () => {
    /**
     * TEST: Empty JSON Initially
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <pre>                                                  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  {}   ← empty object (no keys)                    │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │  </pre>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: JSON display starts empty before any input
     */
    test("should display empty JSON initially", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await expect(ui.jsonDisplay).toBeVisible();
      const data = await ui.getJsonData();
      expect(Object.keys(data).length).toBe(0);
    });

    /**
     * TEST: JSON Updates on Name Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full Name: [John          ]                            │
     *   │              ↓ reactive binding                         │
     *   │  <pre>                                                  │
     *   │  {                                                      │
     *   │    "name": "John"  ← live update                        │
     *   │  }                                                      │
     *   │  </pre>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typing in name field updates JSON display live
     */
    test("should update JSON when typing in name field", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("John");
      await ui.waitForJsonUpdate("name", "John");
    });

    /**
     * TEST: JSON Updates on Email Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Email: [test@example.com  ]                            │
     *   │              ↓ reactive binding                         │
     *   │  <pre>                                                  │
     *   │  {                                                      │
     *   │    "email": "test@example.com"  ← live update           │
     *   │  }                                                      │
     *   │  </pre>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typing in email field updates JSON display live
     */
    test("should update JSON when typing in email field", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.emailInput.fill("test@example.com");
      await ui.waitForJsonUpdate("email", "test@example.com");
    });

    /**
     * TEST: JSON Updates on Bio Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Bio:                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ Hello world                                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │              ↓ reactive binding                         │
     *   │  <pre>                                                  │
     *   │  {                                                      │
     *   │    "bio": "Hello world"  ← live update                  │
     *   │  }                                                      │
     *   │  </pre>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typing in bio textarea updates JSON display live
     */
    test("should update JSON when typing in bio field", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.bioTextarea.fill("Hello world");
      await ui.waitForJsonUpdate("bio", "Hello world");
    });

    /**
     * TEST: JSON Updates on Checkbox Toggle
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Click 1:          Click 2:                             │
     *   │  [ ] → [x]         [x] → [ ]                            │
     *   │                                                         │
     *   │  JSON after:       JSON after:                          │
     *   │  {                 {                                    │
     *   │   "subscribe":      "subscribe":                        │
     *   │   "true"            "false"                             │
     *   │  }                 }                                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox toggles update JSON between true/false
     */
    test("should update JSON when toggling checkbox", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.subscribeCheckbox.click();
      await ui.waitForJsonUpdate("subscribe", "true");

      await ui.subscribeCheckbox.click();
      await ui.waitForJsonUpdate("subscribe", "false");
    });

    /**
     * TEST: JSON Reflects All Field Values
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Form filled:                                           │
     *   │  Name:  [Jane Doe        ]                              │
     *   │  Email: [jane@example.com]                              │
     *   │  Bio:   [Software developer]                            │
     *   │  [x] Subscribe                                          │
     *   │              ↓ all values reflected                     │
     *   │  <pre>                                                  │
     *   │  {                                                      │
     *   │    "name": "Jane Doe",                                  │
     *   │    "email": "jane@example.com",                         │
     *   │    "bio": "Software developer",                         │
     *   │    "subscribe": "true"                                  │
     *   │  }                                                      │
     *   │  </pre>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All form values appear together in JSON display
     */
    test("should reflect all field values in JSON", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("Jane Doe");
      await ui.emailInput.fill("jane@example.com");
      await ui.bioTextarea.fill("Software developer");
      await ui.subscribeCheckbox.click();

      await expect(async () => {
        const data = await ui.getJsonData();
        expect(data.name).toBe("Jane Doe");
        expect(data.email).toBe("jane@example.com");
        expect(data.bio).toBe("Software developer");
        expect(data.subscribe).toBe("true");
      }).toPass({ timeout: 5000 });
    });
  });

  test.describe("On-Blur Validation", () => {
    /**
     * TEST: No Error While Typing Invalid Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  User typing (field focused):                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ J|                    ← cursor still in field     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  (no error shown)  ← validation waits for blur          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Error not shown while user is still typing
     */
    test("should NOT show error while typing invalid value", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Type single character (invalid - min 2 chars)
      await ui.nameInput.fill("J");

      // Error should NOT be visible yet (field not blurred)
      await expect(ui.nameError).not.toBeVisible();
    });

    /**
     * TEST: Error Shows After Blur with Invalid Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Step 1: Type "J"          Step 2: Focus email (blur)   │
     *   │  ┌─────────────────┐       ┌─────────────────┐          │
     *   │  │ J|              │  ──►  │ J               │          │
     *   │  └─────────────────┘       └═════════════════┘          │
     *   │  (no error)                "Name must be at least       │
     *   │                             2 characters"               │
     *   │                                  ↑ role="alert"         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Error appears only after field loses focus
     */
    test("should show error AFTER blurring field with invalid value", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Type single character (invalid - min 2 chars)
      await ui.nameInput.fill("J");

      // Blur the field by clicking elsewhere
      await ui.emailInput.focus();

      // Now error should be visible
      await expect(ui.nameError).toBeVisible();
      await expect(ui.nameError).toContainText(
        "Name must be at least 2 characters"
      );
    });

    /**
     * TEST: Email Validation Error After Blur
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Step 1: Type invalid      Step 2: Blur                 │
     *   │  ┌─────────────────┐       ┌─────────────────┐          │
     *   │  │ invalid-email|  │  ──►  │ invalid-email   │          │
     *   │  └─────────────────┘       └═════════════════┘          │
     *   │  (no error)                "Please enter a valid        │
     *   │                             email address"              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email format validation triggers on blur
     */
    test("should show email validation error after blur", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Type invalid email
      await ui.emailInput.fill("invalid-email");

      // Error should NOT be visible yet
      await expect(ui.emailError).not.toBeVisible();

      // Blur the field
      await ui.nameInput.focus();

      // Now error should be visible
      await expect(ui.emailError).toBeVisible();
      await expect(ui.emailError).toContainText(
        "Please enter a valid email address"
      );
    });

    /**
     * TEST: Error Clears When Value Becomes Valid
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Step 1: Error shown       Step 2: Fix value            │
     *   │  ┌─────────────────┐       ┌─────────────────┐          │
     *   │  │ J               │  ──►  │ John            │          │
     *   │  └═════════════════┘       └─────────────────┘          │
     *   │  "Name must be..."         (error cleared)              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Error disappears when input becomes valid
     */
    test("should clear error when value becomes valid", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Type invalid, blur to show error
      await ui.nameInput.fill("J");
      await ui.emailInput.focus();
      await expect(ui.nameError).toBeVisible();

      // Fix the value
      await ui.nameInput.fill("John");

      // Error should be cleared
      await expect(ui.nameError).not.toBeVisible();
    });

    /**
     * TEST: Live Validation After Field is Touched
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  After initial blur (field "touched"):                  │
     *   │                                                         │
     *   │  [J ] → error    [Jo] → no error    [J ] → error        │
     *   │   ↓                ↓                  ↓                  │
     *   │  shown           cleared            shown again         │
     *   │                                                         │
     *   │  Validation now runs on every keystroke                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Once touched, validation runs live on each change
     */
    test("should validate on each keystroke after field is touched", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Touch the field first (type and blur)
      await ui.nameInput.fill("J");
      await ui.emailInput.focus();
      await expect(ui.nameError).toBeVisible();

      // Now go back and continue typing - validation happens on each keystroke
      await ui.nameInput.fill("Jo");
      await expect(ui.nameError).not.toBeVisible();

      // Delete a character to make invalid again
      await ui.nameInput.fill("J");
      await expect(ui.nameError).toBeVisible();
    });
  });

  test.describe("aria-invalid Attribute", () => {
    /**
     * TEST: No aria-invalid Before Blur
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <input value="J" ... />                                │
     *   │         ↑                                               │
     *   │     NO aria-invalid="true" (field not yet blurred)      │
     *   │                                                         │
     *   │  Screen readers: field appears valid                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: aria-invalid not set while user is typing
     */
    test("should NOT have aria-invalid before blur", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("J");

      // aria-invalid should not be set
      await expect(ui.nameInput).not.toHaveAttribute("aria-invalid", "true");
    });

    /**
     * TEST: aria-invalid=true After Blur with Invalid Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  After blur:                                            │
     *   │  <input value="J" aria-invalid="true" ... />            │
     *   │                   ↑                                     │
     *   │              attribute set                              │
     *   │                                                         │
     *   │  Screen readers: announces field as invalid             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: aria-invalid="true" set when field is invalid
     */
    test("should have aria-invalid=true after blur with invalid value", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("J");
      await ui.emailInput.focus();

      await expect(ui.nameInput).toHaveAttribute("aria-invalid", "true");
    });

    /**
     * TEST: aria-invalid Removed When Valid
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before:                    After fix:                  │
     *   │  <input value="J"           <input value="John"         │
     *   │   aria-invalid="true"/>      ... />                     │
     *   │        ↑                           ↑                    │
     *   │   attribute set              attribute removed          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: aria-invalid removed when value becomes valid
     */
    test("should remove aria-invalid when value becomes valid", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Make field invalid and blur
      await ui.nameInput.fill("J");
      await ui.emailInput.focus();
      await expect(ui.nameInput).toHaveAttribute("aria-invalid", "true");

      // Fix the value
      await ui.nameInput.fill("John");
      await expect(ui.nameInput).not.toHaveAttribute("aria-invalid", "true");
    });
  });

  test.describe("Interactions", () => {
    /**
     * TEST: Text Input Accepts Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Full Name:                                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ Hello World                                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ input.value = "Hello World"                  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Name input accepts and stores text value
     */
    test("should accept text input", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("Hello World");
      await expect(ui.nameInput).toHaveValue("Hello World");
    });

    /**
     * TEST: Email Input Accepts Value
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Email:                                                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ test@example.com                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ input.value = "test@example.com"             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Email input accepts and stores email value
     */
    test("should accept email input", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.emailInput.fill("test@example.com");
      await expect(ui.emailInput).toHaveValue("test@example.com");
    });

    /**
     * TEST: Textarea Accepts Multi-line Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Bio:                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │ This is my bio                                    │  │
     *   │  │ With multiple lines                               │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │          ↑ textarea supports newlines (\n)              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea accepts multi-line text with newlines
     */
    test("should accept textarea input", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.bioTextarea.fill("This is my bio\nWith multiple lines");
      await expect(ui.bioTextarea).toHaveValue(
        "This is my bio\nWith multiple lines"
      );
    });

    /**
     * TEST: Checkbox Toggle on Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Initial:        After click:       After 2nd click:   │
     *   │  ┌───┐           ┌───┐              ┌───┐              │
     *   │  │   │   ──►     │ x │     ──►      │   │              │
     *   │  └───┘           └───┘              └───┘              │
     *   │  data-state=     data-state=        data-state=        │
     *   │  "unchecked"     "checked"          "unchecked"        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Checkbox toggles between checked/unchecked states
     */
    test("checkbox should toggle on click", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Initially unchecked
      await expect(ui.subscribeCheckbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );

      // Click to check
      await ui.subscribeCheckbox.click();
      await expect(ui.subscribeCheckbox).toHaveAttribute(
        "data-state",
        "checked"
      );

      // Click to uncheck
      await ui.subscribeCheckbox.click();
      await expect(ui.subscribeCheckbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );
    });

    /**
     * TEST: Input Value Can Be Cleared
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before:                    After clear:                │
     *   │  ┌─────────────────┐        ┌─────────────────┐         │
     *   │  │ Some text       │  ──►   │                 │         │
     *   │  └─────────────────┘        └─────────────────┘         │
     *   │  value="Some text"          value=""                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input value can be cleared to empty string
     */
    test("should clear input value", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.fill("Some text");
      await expect(ui.nameInput).toHaveValue("Some text");

      await ui.nameInput.clear();
      await expect(ui.nameInput).toHaveValue("");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Inputs Are Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Focus sequence:                                        │
     *   │                                                         │
     *   │  Name:  [=========]  ← :focus                           │
     *   │  Email: [         ]                                     │
     *   │  Bio:   [         ]                                     │
     *   │              ↓                                          │
     *   │  Name:  [         ]                                     │
     *   │  Email: [=========]  ← :focus                           │
     *   │  Bio:   [         ]                                     │
     *   │              ↓                                          │
     *   │  Name:  [         ]                                     │
     *   │  Email: [         ]                                     │
     *   │  Bio:   [=========]  ← :focus                           │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All inputs can receive focus programmatically
     */
    test("inputs should be focusable", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.nameInput.focus();
      await expect(ui.nameInput).toBeFocused();

      await ui.emailInput.focus();
      await expect(ui.emailInput).toBeFocused();

      await ui.bioTextarea.focus();
      await expect(ui.bioTextarea).toBeFocused();
    });

    /**
     * TEST: Keyboard Navigation Between Inputs
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Tab key navigation:                                    │
     *   │                                                         │
     *   │  Name:  [===focus===]                                   │
     *   │              │ Tab                                      │
     *   │              ▼                                          │
     *   │  Email: [===focus===]                                   │
     *   │              │ Tab                                      │
     *   │              ▼                                          │
     *   │  Bio:   [===focus===]                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Tab key moves focus through form fields in order
     */
    test("should support keyboard navigation between inputs", async ({
      page,
    }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Focus first input and tab to next
      await ui.nameInput.focus();
      await expect(ui.nameInput).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.emailInput).toBeFocused();

      await page.keyboard.press("Tab");
      await expect(ui.bioTextarea).toBeFocused();
    });

    /**
     * TEST: Checkbox Toggleable with Keyboard
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Focus checkbox, then press Space:                      │
     *   │                                                         │
     *   │  ┌───┐                    ┌───┐                         │
     *   │  │   │ Subscribe   ──►    │ x │ Subscribe               │
     *   │  └───┘                    └───┘                         │
     *   │  :focus + Space           data-state="checked"          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Space key toggles checkbox when focused
     */
    test("checkbox should be toggleable with keyboard", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await ui.subscribeCheckbox.focus();
      await expect(ui.subscribeCheckbox).toHaveAttribute(
        "data-state",
        "unchecked"
      );

      await page.keyboard.press("Space");
      await expect(ui.subscribeCheckbox).toHaveAttribute(
        "data-state",
        "checked"
      );
    });

    /**
     * TEST: Error Messages Have role=alert
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div role="alert">                                     │
     *   │    Name must be at least 2 characters                   │
     *   │  </div>                                                 │
     *   │       ↑                                                 │
     *   │   role="alert" for screen reader announcements          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Error messages use role="alert" for accessibility
     */
    test("error messages should have role=alert", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Trigger validation error
      await ui.nameInput.fill("J");
      await ui.emailInput.focus();

      await expect(ui.nameError).toHaveAttribute("role", "alert");
    });

    /**
     * TEST: Inputs Have Associated Labels via ID
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <label for="name">Full Name</label>                    │
     *   │  <input id="name" ... />                                │
     *   │         ↑                                               │
     *   │     id matches label's for attribute                    │
     *   │                                                         │
     *   │  <label for="email">Email</label>                       │
     *   │  <input id="email" ... />                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Inputs have correct IDs for label association
     */
    test("inputs should have associated labels via id", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      // Name input should have id matching label's for attribute
      const nameId = await ui.nameInput.getAttribute("id");
      expect(nameId).toBe("name");

      const emailId = await ui.emailInput.getAttribute("id");
      expect(emailId).toBe("email");
    });
  });

  test.describe("Form Submission", () => {
    /**
     * TEST: Submit Button Visible with Correct Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │  <button type="submit">Submit</button>                  │
     *   │          ↑                ↑                             │
     *   │     type="submit"     visible & clickable               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Submit button is visible with type="submit"
     */
    test("submit button should be visible", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      await expect(ui.submitButton).toBeVisible();
      await expect(ui.submitButton).toHaveAttribute("type", "submit");
    });

    /**
     * TEST: Submit Does Not Navigate Away
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before click:              After click:                │
     *   │  URL: /test                 URL: /test                  │
     *   │                                                         │
     *   │  [Submit] ──► click ──►     (same page)                 │
     *   │                                                         │
     *   │  preventDefault() stops default form submission         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Form submission is prevented (SPA behavior)
     */
    test("clicking submit should not navigate away", async ({ page }) => {
      const ui = new FormPage(page);
      await ui.goto();

      const url = page.url();
      await ui.submitButton.click();

      // Should stay on same page (preventDefault working)
      expect(page.url()).toBe(url);
    });
  });
});
