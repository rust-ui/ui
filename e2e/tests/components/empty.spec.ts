import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * EMPTY COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="Empty">                                               │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │                    ┌──────────────┐                             │   │
 * │   │                    │  EmptyMedia  │                             │   │
 * │   │                    │    [icon]    │                             │   │
 * │   │                    └──────────────┘                             │   │
 * │   │                                                                 │   │
 * │   │                    ┌──────────────────────────────────┐         │   │
 * │   │                    │      EmptyHeader                 │         │   │
 * │   │                    │  ┌────────────────────────────┐  │         │   │
 * │   │                    │  │ EmptyTitle                 │  │         │   │
 * │   │                    │  │  "No Projects Yet"         │  │         │   │
 * │   │                    │  └────────────────────────────┘  │         │   │
 * │   │                    │  ┌────────────────────────────┐  │         │   │
 * │   │                    │  │ EmptyDescription           │  │         │   │
 * │   │                    │  │  "You haven't created..."  │  │         │   │
 * │   │                    │  └────────────────────────────┘  │         │   │
 * │   │                    └──────────────────────────────────┘         │   │
 * │   │                                                                 │   │
 * │   │                    ┌──────────────────────────────────┐         │   │
 * │   │                    │      EmptyContent               │         │   │
 * │   │                    │  [Create Project] [Import...]   │         │   │
 * │   │                    │        Learn More               │         │   │
 * │   │                    └──────────────────────────────────┘         │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * LAYOUT:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   flex flex-col items-center justify-center                             │
 * │                                                                         │
 * │          ┌────────────┐                                                 │
 * │          │   [icon]   │  ← EmptyMedia                                   │
 * │          └────────────┘                                                 │
 * │               ↓                                                         │
 * │       No Projects Yet     ← EmptyTitle (font-semibold)                  │
 * │               ↓                                                         │
 * │   You haven't created...  ← EmptyDescription (text-muted-foreground)    │
 * │               ↓                                                         │
 * │   ┌─────────┐ ┌─────────┐                                               │
 * │   │ Create  │ │ Import  │ ← EmptyContent (buttons)                      │
 * │   └─────────┘ └─────────┘                                               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class EmptyPage extends BasePage {
  protected readonly componentName = "empty";

  // Empty elements
  readonly empty: Locator;
  readonly emptyHeader: Locator;
  readonly emptyMedia: Locator;
  readonly emptyTitle: Locator;
  readonly emptyDescription: Locator;
  readonly emptyContent: Locator;
  readonly createButton: Locator;
  readonly importButton: Locator;
  readonly learnMoreLink: Locator;

  constructor(page: Page) {
    super(page);

    // Main empty state - scoped within preview
    this.empty = this.preview.locator('[data-name="Empty"]').first();

    // Parts
    this.emptyHeader = this.empty.locator('[data-name="EmptyHeader"]');
    this.emptyMedia = this.empty.locator('[data-name="EmptyMedia"]');
    this.emptyTitle = this.empty.locator('[data-name="EmptyTitle"]');
    this.emptyDescription = this.empty.locator('[data-name="EmptyDescription"]');
    this.emptyContent = this.empty.locator('[data-name="EmptyContent"]');

    // Buttons - scoped within preview
    this.createButton = this.preview.getByRole("button", { name: "Create Project" });
    this.importButton = this.preview.getByRole("button", { name: "Import Project" });
    this.learnMoreLink = this.preview.getByText("Learn More");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Empty Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Empty Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  [data-name="Empty"]                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │                    [icon]                         │  │
     *   │  │              No Projects Yet                      │  │
     *   │  │           Description text...                     │  │
     *   │  │            [Button] [Button]                      │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The Empty container element is visible on the page
     */
    test("should have Empty container", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toBeVisible();
    });

    /**
     * TEST: Empty Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="Empty">  <-- checking this attribute  │
     *   │       ^^^^^^^^^^^^^^^^^^                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │               ...content...                       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name attribute equals "Empty" for component identification
     */
    test("should have Empty data-name attribute", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toHaveAttribute("data-name", "Empty");
    });

    /**
     * TEST: EmptyHeader Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                    [icon]                         │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  EmptyHeader  <-- checking this             │  │  │
     *   │  │  │  ┌───────────────────────────────────────┐  │  │  │
     *   │  │  │  │ EmptyTitle: "No Projects Yet"        │  │  │  │
     *   │  │  │  │ EmptyDescription: "You haven't..."   │  │  │  │
     *   │  │  │  └───────────────────────────────────────┘  │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The EmptyHeader section is visible
     */
    test("should have EmptyHeader", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyHeader).toBeVisible();
    });

    /**
     * TEST: EmptyMedia with Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │              ┌──────────────┐                     │  │
     *   │  │              │  EmptyMedia  │ <-- checking this   │  │
     *   │  │              │    ┌────┐    │                     │  │
     *   │  │              │    │ SVG│    │ <-- and this icon   │  │
     *   │  │              │    └────┘    │                     │  │
     *   │  │              └──────────────┘                     │  │
     *   │  │              No Projects Yet                      │  │
     *   │  │              ...                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: EmptyMedia element is visible and contains an SVG icon
     */
    test("should have EmptyMedia with icon", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyMedia).toBeVisible();
      const icon = ui.emptyMedia.locator("svg");
      await expect(icon).toBeVisible();
    });

    /**
     * TEST: EmptyTitle Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                    [icon]                         │  │
     *   │  │                                                   │  │
     *   │  │            ┌────────────────────┐                 │  │
     *   │  │            │    EmptyTitle      │ <-- checking    │  │
     *   │  │            │ "No Projects Yet"  │                 │  │
     *   │  │            └────────────────────┘                 │  │
     *   │  │                                                   │  │
     *   │  │           Description text...                     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The EmptyTitle element is visible
     */
    test("should have EmptyTitle", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyTitle).toBeVisible();
    });

    /**
     * TEST: EmptyDescription Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                    [icon]                         │  │
     *   │  │              No Projects Yet                      │  │
     *   │  │                                                   │  │
     *   │  │     ┌──────────────────────────────────────┐      │  │
     *   │  │     │      EmptyDescription                │      │  │
     *   │  │     │ "You haven't created any projects..." │ <--  │  │
     *   │  │     └──────────────────────────────────────┘      │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The EmptyDescription element is visible
     */
    test("should have EmptyDescription", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyDescription).toBeVisible();
    });

    /**
     * TEST: EmptyContent Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container                                        │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                    [icon]                         │  │
     *   │  │              No Projects Yet                      │  │
     *   │  │           Description text...                     │  │
     *   │  │                                                   │  │
     *   │  │     ┌──────────────────────────────────────┐      │  │
     *   │  │     │         EmptyContent                 │ <--  │  │
     *   │  │     │  [Create Project] [Import Project]   │      │  │
     *   │  │     │           Learn More                 │      │  │
     *   │  │     └──────────────────────────────────────┘      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The EmptyContent element (action area) is visible
     */
    test("should have EmptyContent", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyContent).toBeVisible();
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Title Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │                                                         │
     *   │              ╔════════════════════╗                     │
     *   │              ║ "No Projects Yet"  ║ <-- checking text   │
     *   │              ╚════════════════════╝                     │
     *   │                                                         │
     *   │           Description text...                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: EmptyTitle displays exact text "No Projects Yet"
     */
    test("title should display 'No Projects Yet'", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyTitle).toHaveText("No Projects Yet");
    });

    /**
     * TEST: Description Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │                                                         │
     *   │     ╔═══════════════════════════════════════════════╗   │
     *   │     ║ "You haven't created any projects yet..."     ║   │
     *   │     ╚═══════════════════════════════════════════════╝   │
     *   │                         ^                               │
     *   │                  checking this text                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: EmptyDescription contains helpful guidance text
     */
    test("description should have helpful text", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyDescription).toContainText(
        "You haven't created any projects yet"
      );
    });

    /**
     * TEST: Create Project Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │     ┌─────────────────┐  ┌─────────────────┐            │
     *   │     │ Create Project  │  │ Import Project  │            │
     *   │     └─────────────────┘  └─────────────────┘            │
     *   │              ^                                          │
     *   │         checking this button                            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Create Project" button is visible
     */
    test("should have Create Project button", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.createButton).toBeVisible();
    });

    /**
     * TEST: Import Project Button Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │     ┌─────────────────┐  ┌─────────────────┐            │
     *   │     │ Create Project  │  │ Import Project  │            │
     *   │     └─────────────────┘  └─────────────────┘            │
     *   │                                    ^                    │
     *   │                           checking this button          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Import Project" button is visible
     */
    test("should have Import Project button", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.importButton).toBeVisible();
    });

    /**
     * TEST: Learn More Link Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │     ┌─────────────────┐  ┌─────────────────┐            │
     *   │     │ Create Project  │  │ Import Project  │            │
     *   │     └─────────────────┘  └─────────────────┘            │
     *   │                                                         │
     *   │                   Learn More  <-- checking this link    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "Learn More" link is visible below buttons
     */
    test("should have Learn More link", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.learnMoreLink).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Empty Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Empty Container: display: flex                         │
     *   │  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                    [icon]                         │  │
     *   │  │              No Projects Yet                      │  │
     *   │  │           Description text...                     │  │
     *   │  │            [Button] [Button]                      │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  flex enables flexible layout of child elements         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Empty has "flex" class for flexbox layout
     */
    test("empty should have flex layout", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toHaveClass(/flex/);
    });

    /**
     * TEST: Empty Flex-Col Direction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  flex-col: vertical stacking                            │
     *   │                                                         │
     *   │              ┌──────────┐                               │
     *   │              │  [icon]  │  ← item 1                     │
     *   │              └────┬─────┘                               │
     *   │                   │                                     │
     *   │              ┌────▼─────┐                               │
     *   │              │  Title   │  ← item 2                     │
     *   │              └────┬─────┘                               │
     *   │                   │                                     │
     *   │              ┌────▼─────┐                               │
     *   │              │  Desc.   │  ← item 3                     │
     *   │              └────┬─────┘                               │
     *   │                   ▼                                     │
     *   │              [Buttons]     ← item 4                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Empty has "flex-col" for vertical stacking
     */
    test("empty should have flex-col", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toHaveClass(/flex-col/);
    });

    /**
     * TEST: Empty Items-Center Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  items-center: horizontal centering (cross-axis)        │
     *   │                                                         │
     *   │  ←─────────────────────────────────────────────────────→│
     *   │                         │                               │
     *   │                    [icon]│                              │
     *   │               No Projects│Yet                           │
     *   │             Description t│ext...                        │
     *   │              [Button] [Bu│tton]                         │
     *   │                         │                               │
     *   │                    center│axis                          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Empty has "items-center" to center items horizontally
     */
    test("empty should have items-center", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toHaveClass(/items-center/);
    });

    /**
     * TEST: Empty Justify-Center Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  justify-center: vertical centering (main-axis)         │
     *   │                                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   │  │
     *   │  │                    [icon]                         │  │
     *   │  │              No Projects Yet        ← centered    │  │
     *   │  │           Description text...         vertically  │  │
     *   │  │            [Button] [Button]                      │  │
     *   │  │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Empty has "justify-center" to center content vertically
     */
    test("empty should have justify-center", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.empty).toHaveClass(/justify-center/);
    });

    /**
     * TEST: Title Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │                                                         │
     *   │              ╔════════════════════╗                     │
     *   │              ║ No Projects Yet    ║  font-semibold      │
     *   │              ╚════════════════════╝  (font-weight: 600) │
     *   │                        ^                                │
     *   │                 bold/semibold text                      │
     *   │           You haven't created...  (regular weight)      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: EmptyTitle has "font-semibold" for emphasis
     */
    test("title should have font-semibold", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyTitle).toHaveClass(/font-semibold/);
    });

    /**
     * TEST: Description Muted Text Color
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │                    [icon]                               │
     *   │                                                         │
     *   │              No Projects Yet      (normal color)        │
     *   │                                                         │
     *   │     ╔═══════════════════════════════════════════════╗   │
     *   │     ║ You haven't created any projects yet...       ║   │
     *   │     ╚═══════════════════════════════════════════════╝   │
     *   │                         ^                               │
     *   │              text-muted-foreground (gray/muted color)   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: EmptyDescription uses muted color for secondary text
     */
    test("description should have text-muted-foreground", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.emptyDescription).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: Import Button Outline Variant
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Button variants:                                      │
     *   │                                                         │
     *   │   ┌─────────────────┐   ┌─────────────────┐             │
     *   │   │█████████████████│   │                 │             │
     *   │   │█Create Project██│   │ Import Project  │             │
     *   │   │█████████████████│   │                 │             │
     *   │   └─────────────────┘   └─────────────────┘             │
     *   │     solid/default          outline variant              │
     *   │                                   ^                     │
     *   │                          has border (outline)           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Import button has "border" class (outline variant)
     */
    test("Import button should have outline variant", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await expect(ui.importButton).toHaveClass(/border/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Create Project Button Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User interaction:                                     │
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │     ┌─────────────────┐                                 │
     *   │     │ Create Project  │ ← click()                       │
     *   │     └─────────────────┘                                 │
     *   │              │                                          │
     *   │              ▼                                          │
     *   │        (no error thrown)                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Create Project button can be clicked without error
     */
    test("Create Project button should be clickable", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await ui.createButton.click();
      // Should not throw error
    });

    /**
     * TEST: Import Project Button Click
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User interaction:                                     │
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │                         ┌─────────────────┐             │
     *   │                         │ Import Project  │ ← click()   │
     *   │                         └─────────────────┘             │
     *   │                                  │                      │
     *   │                                  ▼                      │
     *   │                          (no error thrown)              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Import Project button can be clicked without error
     */
    test("Import Project button should be clickable", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await ui.importButton.click();
      // Should not throw error
    });

    /**
     * TEST: Button Focus State
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard accessibility:                               │
     *   │                                                         │
     *   │                    [icon]                               │
     *   │              No Projects Yet                            │
     *   │           Description text...                           │
     *   │                                                         │
     *   │     ╔═════════════════╗  ┌─────────────────┐            │
     *   │     ║ Create Project  ║  │ Import Project  │            │
     *   │     ╚═════════════════╝  └─────────────────┘            │
     *   │              ^                                          │
     *   │         focus ring (button is focused)                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Buttons can receive keyboard focus for accessibility
     */
    test("buttons should be focusable", async ({ page }) => {
      const ui = new EmptyPage(page);
      await ui.goto();

      await ui.createButton.focus();
      await expect(ui.createButton).toBeFocused();
    });
  });
});
