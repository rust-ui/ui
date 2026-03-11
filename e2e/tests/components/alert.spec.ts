import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * ALERT COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Alert"]                                                   │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  ┌───┐                                                      │       │
 * │   │  │ ! │  ← SVG Icon (absolute positioned, left-4, top-4)     │       │
 * │   │  └───┘                                                      │       │
 * │   │         ┌─────────────────────────────────────────────┐     │       │
 * │   │         │  <h4> AlertTitle                            │     │       │
 * │   │         │  "Heads up !"                               │     │       │
 * │   │         └─────────────────────────────────────────────┘     │       │
 * │   │         ┌─────────────────────────────────────────────┐     │       │
 * │   │         │  <p> AlertDescription                       │     │       │
 * │   │         │  "You can add components to your app..."    │     │       │
 * │   │         └─────────────────────────────────────────────┘     │       │
 * │   │                                                             │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ICON POSITIONING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [&>svg]:absolute     ← Icon uses absolute positioning                 │
 * │   [&>svg]:left-4       ← 16px from left edge                            │
 * │   [&>svg]:top-4        ← 16px from top edge                             │
 * │   [&>svg]:text-foreground  ← Uses foreground color                      │
 * │   [&>svg~*]:pl-7       ← Content after icon has 28px left padding       │
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  ┌───┐                                                      │       │
 * │   │  │ i │←─ left-4                                             │       │
 * │   │  └───┘                                                      │       │
 * │   │    ↑     │←─────────── pl-7 ──────────────►│                │       │
 * │   │  top-4   │  Title and description content  │                │       │
 * │   │          │                                 │                │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * VARIANTS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:                                                              │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  bg-background text-foreground border                       │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * │   Destructive:                                                          │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  border-destructive/50 text-destructive                     │       │
 * │   │  [&>svg]:text-destructive                                   │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class AlertPage extends BasePage {
  protected readonly componentName = "alert";

  // Alert component
  readonly alert: Locator;

  // Alert parts
  readonly alertIcon: Locator;
  readonly alertTitle: Locator;
  readonly alertDescription: Locator;

  constructor(page: Page) {
    super(page);

    // Main alert - scoped within preview
    this.alert = this.preview.locator('[data-name="Alert"]').first();

    // Parts - scoped within alert (which is within preview)
    this.alertIcon = this.alert.locator("svg").first();
    this.alertTitle = this.alert.locator('[data-name="AlertTitle"]');
    this.alertDescription = this.alert.locator(
      '[data-name="AlertDescription"]',
    );
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Alert Page", () => {
  /**
   * STRUCTURE TESTS - Verify component hierarchy
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Alert (div)                                                   │
   * │   ├── <svg> Icon                                                │
   * │   ├── <h4> AlertTitle                                           │
   * │   └── <p> AlertDescription                                      │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Alert Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Alert"]                                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │          MUST BE VISIBLE                        │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert component renders and is visible on the page
     */
    test("should have Alert component", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toBeVisible();
    });

    /**
     * TEST: Alert Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="Alert">                               │
     *   │        ├────────┬──────┤                                │
     *   │             ↓                                           │
     *   │   Attribute must equal "Alert"                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has correct data-name attribute for identification
     */
    test("should have Alert data-name attribute", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveAttribute("data-name", "Alert");
    });

    /**
     * TEST: Alert Element Type
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div> ← Must be a div element                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │         Alert Content                           │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </div>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert uses semantic div element for container
     */
    test("alert should be a div element", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      const tagName = await ui.alert.evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("div");
    });

    /**
     * TEST: Alert Icon Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───┐                                                 │
     *   │   │ ! │  ← SVG icon MUST BE VISIBLE                     │
     *   │   └───┘                                                 │
     *   │         Title...                                        │
     *   │         Description...                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert displays an icon for visual context
     */
    test("should have icon", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertIcon).toBeVisible();
    });

    /**
     * TEST: Alert Title Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [!]  ┌─────────────────────────────────────────────┐  │
     *   │        │  AlertTitle                                 │  │
     *   │        │  MUST BE VISIBLE                            │  │
     *   │        └─────────────────────────────────────────────┘  │
     *   │        Description...                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has a visible title element
     */
    test("should have title", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toBeVisible();
    });

    /**
     * TEST: Alert Description Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [!]  Title...                                         │
     *   │        ┌─────────────────────────────────────────────┐  │
     *   │        │  AlertDescription                           │  │
     *   │        │  MUST BE VISIBLE                            │  │
     *   │        └─────────────────────────────────────────────┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has a visible description element
     */
    test("should have description", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertDescription).toBeVisible();
    });
  });

  /**
   * CONTENT TESTS - Text and data attributes
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  <h4 data-name="AlertTitle">                            │   │
   * │   │     "Heads up !"                                        │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  <p data-name="AlertDescription">                       │   │
   * │   │     "You can add components to your app using the cli." │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Content", () => {
    /**
     * TEST: Title Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4>                                                  │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "Heads up !"                                   │   │
     *   │   │       ↑                                         │   │
     *   │   │  MUST MATCH EXACTLY                             │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </h4>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title displays the expected text content
     */
    test("title should have correct text", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveText("Heads up !");
    });

    /**
     * TEST: Description Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <p>                                                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "You can add components to your app using      │   │
     *   │   │   the cli."                                     │   │
     *   │   │       ↑                                         │   │
     *   │   │  MUST CONTAIN THIS TEXT                         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </p>                                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description contains the expected explanatory text
     */
    test("description should have correct text", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertDescription).toContainText(
        "You can add components to your app using the cli.",
      );
    });

    /**
     * TEST: Title Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 data-name="AlertTitle">                           │
     *   │       ├────────┬──────────┤                             │
     *   │                ↓                                        │
     *   │   Attribute must equal "AlertTitle"                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has correct data-name for component identification
     */
    test("title should have AlertTitle data-name", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveAttribute("data-name", "AlertTitle");
    });

    /**
     * TEST: Description Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <p data-name="AlertDescription">                      │
     *   │      ├────────┬───────────────┤                         │
     *   │               ↓                                         │
     *   │   Attribute must equal "AlertDescription"               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description has correct data-name for identification
     */
    test("description should have AlertDescription data-name", async ({
      page,
    }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertDescription).toHaveAttribute(
        "data-name",
        "AlertDescription",
      );
    });
  });

  /**
   * TITLE ELEMENT TESTS - Heading styling
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <h4 class="font-medium tracking-tight leading-none mb-1">    │
   * │      "Heads up !"                                               │
   * │   </h4>                                                         │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Title Element", () => {
    /**
     * TEST: Title Is H4 Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4>  ← tagName must be "h4"                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Heads up !                                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </h4>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title uses semantic h4 heading element
     */
    test("title should be h4 element", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      const tagName = await ui.alertTitle.evaluate((el) =>
        el.tagName.toLowerCase(),
      );
      expect(tagName).toBe("h4");
    });

    /**
     * TEST: Title Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 class="font-medium ...">                          │
     *   │              ├─────────┤                                │
     *   │                   ↓                                     │
     *   │   Font weight: 500 (medium)                             │
     *   │                                                         │
     *   │   Normal:  Heads up !                                   │
     *   │   Medium:  Heads up !  ← Slightly bolder                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has medium font weight for emphasis
     */
    test("title should have font-medium class", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveClass(/font-medium/);
    });

    /**
     * TEST: Title Letter Spacing
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 class="tracking-tight ...">                       │
     *   │              ├────────────┤                             │
     *   │                    ↓                                    │
     *   │   Letter spacing: -0.025em (tighter)                    │
     *   │                                                         │
     *   │   Normal:   H e a d s   u p                             │
     *   │   Tight:    Heads up  ← Letters closer together         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has tight letter spacing for compact look
     */
    test("title should have tracking-tight class", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveClass(/tracking-tight/);
    });

    /**
     * TEST: Title Line Height
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <h4 class="leading-none ...">                         │
     *   │              ├───────────┤                              │
     *   │                   ↓                                     │
     *   │   Line height: 1 (no extra space)                       │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Heads up !  ← line-height matches font size    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has minimal line height for precise spacing
     */
    test("title should have leading-none class", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveClass(/leading-none/);
    });

    /**
     * TEST: Title Bottom Margin
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Heads up !                                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                 ↓ mb-1 (4px gap)                        │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Description text...                            │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has margin below to separate from description
     */
    test("title should have margin-bottom", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertTitle).toHaveClass(/mb-1/);
    });
  });

  /**
   * DESCRIPTION ELEMENT TESTS - Body text styling
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <p class="text-sm">                                           │
   * │      "You can add components to your app using the cli."        │
   * │   </p>                                                          │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Description Element", () => {
    /**
     * TEST: Description Is P Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <p>  ← tagName must be "p"                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  You can add components to your app using...    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </p>                                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description uses semantic paragraph element
     */
    test("description should be p element", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      const tagName = await ui.alertDescription.evaluate((el) =>
        el.tagName.toLowerCase(),
      );
      expect(tagName).toBe("p");
    });

    /**
     * TEST: Description Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <p class="text-sm ...">                               │
     *   │             ├──────┤                                    │
     *   │                 ↓                                       │
     *   │   Font size: 14px (0.875rem)                            │
     *   │                                                         │
     *   │   text-base:  You can add components...                 │
     *   │   text-sm:    You can add components...  ← Smaller      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Description has smaller text for visual hierarchy
     */
    test("description should have text-sm class", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alertDescription).toHaveClass(/text-sm/);
    });
  });

  /**
   * ALERT CONTAINER STYLING TESTS - Box appearance
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  .relative         ← Positioning context for icon       │   │
   * │   │  .w-full           ← Full width                         │   │
   * │   │  .border           ← Border around alert                │   │
   * │   │  .rounded-lg       ← Rounded corners                    │   │
   * │   │  .px-4 .py-3       ← Padding                            │   │
   * │   │  .text-sm          ← Text size                          │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Alert Container Styling", () => {
    /**
     * TEST: Alert Has Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐  │
     *   │   │                                                  │  │
     *   │   │         Alert content                            │  │
     *   │   │                                                  │  │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘  │
     *   │        ↑ border class creates visible edge              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert container has visible border
     */
    test("should have border", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/border/);
    });

    /**
     * TEST: Alert Has Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭─────────────────────────────────────────────────╮   │
     *   │   │                                                 │   │
     *   │   │         Alert content                           │   │
     *   │   │                                                 │   │
     *   │   ╰─────────────────────────────────────────────────╯   │
     *   │   ↑                                                 ↑   │
     *   │   rounded-lg = 8px border radius                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has large rounded corners
     */
    test("should have rounded-lg corners", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/rounded-lg/);
    });

    /**
     * TEST: Alert Horizontal Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │←px-4→│                              │←px-4→│    │   │
     *   │   │      │      Alert content           │      │    │   │
     *   │   │  16px│                              │16px  │    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has 16px horizontal padding
     */
    test("should have horizontal padding", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/px-4/);
    });

    /**
     * TEST: Alert Vertical Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │           ↑ py-3 (12px)                         │   │
     *   │   │                                                 │   │
     *   │   │         Alert content                           │   │
     *   │   │                                                 │   │
     *   │   │           ↓ py-3 (12px)                         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has 12px vertical padding
     */
    test("should have vertical padding", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/py-3/);
    });

    /**
     * TEST: Alert Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div class="text-sm ...">                             │
     *   │               ├──────┤                                  │
     *   │                  ↓                                      │
     *   │   Base font size: 14px for all text inside              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert container sets base text size
     */
    test("should have text-sm class", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Alert Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div class="relative">  ← Positioning context         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌───┐                                          │   │
     *   │   │  │ ! │ ← absolute positioned relative to this   │   │
     *   │   │  └───┘                                          │   │
     *   │   │        Content...                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </div>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert has relative positioning for icon placement
     */
    test("should have relative positioning", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/relative/);
    });

    /**
     * TEST: Alert Full Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Parent Container                                      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │←─────────────── w-full ────────────────────────►│   │
     *   │   │                                                 │   │
     *   │   │   Alert spans full width of parent              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alert expands to fill container width
     */
    test("should have full width", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/w-full/);
    });
  });

  /**
   * ICON POSITIONING TESTS - Absolute icon placement
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  ┌───┐                                                  │   │
   * │   │  │ ! │ ← absolute, left-4, top-4                        │   │
   * │   │  └───┘                                                  │   │
   * │   │        │←────── pl-7 ──────►│                           │   │
   * │   │        │  Content area       │                           │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * │   CSS selectors on Alert container:                             │
   * │   • [&>svg]:absolute                                            │
   * │   • [&>svg]:left-4                                              │
   * │   • [&>svg]:top-4                                               │
   * │   • [&>svg]:text-foreground                                     │
   * │   • [&>svg~*]:pl-7                                              │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Icon Positioning", () => {
    /**
     * TEST: Icon Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Alert (position: relative)                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ┌───┐                                          │   │
     *   │   │  │ ! │ ← position: absolute                     │   │
     *   │   │  └───┘   [&>svg]:absolute                       │   │
     *   │   │        Content...                               │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is positioned absolutely within container
     */
    test("icon should be positioned absolutely", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      // Alert has class that positions svg absolutely
      await expect(ui.alert).toHaveClass(/\[&>svg\]:absolute/);
    });

    /**
     * TEST: Icon Left Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │←16px→┌───┐                                      │   │
     *   │   │      │ ! │  left-4 = 16px from left edge        │   │
     *   │   │      └───┘                                      │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is positioned 16px from left edge
     */
    test("icon should be positioned at left-4", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/\[&>svg\]:left-4/);
    });

    /**
     * TEST: Icon Top Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │      ↑                                          │   │
     *   │   │    16px  top-4 = 16px from top edge             │   │
     *   │   │      ↓                                          │   │
     *   │   │      ┌───┐                                      │   │
     *   │   │      │ ! │                                      │   │
     *   │   │      └───┘                                      │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is positioned 16px from top edge
     */
    test("icon should be positioned at top-4", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/\[&>svg\]:top-4/);
    });

    /**
     * TEST: Icon Text Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───┐                                                 │
     *   │   │ ! │ ← color: var(--foreground)                      │
     *   │   └───┘   [&>svg]:text-foreground                       │
     *   │                                                         │
     *   │   Icon color matches the theme foreground color         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon uses foreground color for visibility
     */
    test("icon should have foreground text color", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      await expect(ui.alert).toHaveClass(/\[&>svg\]:text-foreground/);
    });

    /**
     * TEST: Content Padding After Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───┐ │←──── pl-7 (28px) ────►│                       │
     *   │   │ ! │ │  Title               │                        │
     *   │   └───┘ │  Description         │                        │
     *   │         │                      │                        │
     *   │         ↑                                               │
     *   │   [&>svg~*]:pl-7  (sibling elements get left padding)   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content after icon has padding to avoid overlap
     */
    test("content after icon should have left padding", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      // Elements after svg should have pl-7
      await expect(ui.alert).toHaveClass(/\[&>svg~\*\]:pl-7/);
    });
  });

  /**
   * ICON TESTS - SVG element verification
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   ┌───┐                                                         │
   * │   │ ! │  ← <svg> element, typically an info/warning icon        │
   * │   └───┘                                                         │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Icon", () => {
    /**
     * TEST: Icon Is SVG Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg>  ← tagName must be "svg"                        │
     *   │   ┌───┐                                                 │
     *   │   │ ! │   Scalable vector icon                          │
     *   │   └───┘                                                 │
     *   │   </svg>                                                │
     *   │                                                         │
     *   │   Benefits of SVG:                                      │
     *   │   • Scales without pixelation                           │
     *   │   • Color can be changed via CSS                        │
     *   │   • Small file size                                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon is a proper SVG element for scalability
     */
    test("icon should be an SVG element", async ({ page }) => {
      const ui = new AlertPage(page);
      await ui.goto();

      const tagName = await ui.alertIcon.evaluate((el) =>
        el.tagName.toLowerCase(),
      );
      expect(tagName).toBe("svg");
    });
  });
});
