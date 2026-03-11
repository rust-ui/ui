import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * BREADCRUMB COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <nav data-name="Breadcrumb">                                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  <ol data-name="BreadcrumbList">                                │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │                                                           │  │   │
 * │   │  │  [Item]  >  [Item]  >  [...]  >  [Item]  >  [Page]        │  │   │
 * │   │  │    ↑        ↑           ↑         ↑          ↑            │  │   │
 * │   │  │  Link    Separator   Ellipsis   Link    Current Page      │  │   │
 * │   │  │                                                           │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * PARTS BREAKDOWN:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌──────────────┐   ┌─────────────┐   ┌──────────────┐                 │
 * │   │ BreadcrumbItem│   │ RootSeparator│   │   RootPage   │                 │
 * │   │              │   │             │   │              │                 │
 * │   │  [Home]      │   │   [>]       │   │  Breadcrumb  │                 │
 * │   │              │   │             │   │              │                 │
 * │   │  <li>        │   │ aria-hidden │   │ aria-current │                 │
 * │   │  inline-flex │   │ role=pres.  │   │ aria-disabled│                 │
 * │   └──────────────┘   └─────────────┘   └──────────────┘                 │
 * │                                                                         │
 * │   ┌──────────────┐   ┌─────────────┐                                    │
 * │   │BreadcrumbLink│   │ RootEllipsis │                                    │
 * │   │              │   │             │                                    │
 * │   │  <a href>    │   │   [...]     │                                    │
 * │   │              │   │             │                                    │
 * │   │  hover:text- │   │ aria-hidden │                                    │
 * │   │  foreground  │   │ role=pres.  │                                    │
 * │   └──────────────┘   └─────────────┘                                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * EXAMPLE USAGE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Home  >  Components  >  ...  >  Button  >  Breadcrumb                 │
 * │     │         │           │         │            │                      │
 * │     └─────────┴───────────┴─────────┴────────────┘                      │
 * │          Links (clickable)        Current (disabled)                    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class BreadcrumbPage extends BasePage {
  protected readonly componentName = "breadcrumb";

  // Breadcrumb container
  readonly breadcrumb: Locator;

  // Breadcrumb parts
  readonly breadcrumbList: Locator;
  readonly breadcrumbItems: Locator;
  readonly breadcrumbLinks: Locator;
  readonly breadcrumbSeparators: Locator;
  readonly breadcrumbPage: Locator;
  readonly breadcrumbEllipsis: Locator;

  // Specific items
  readonly homeLink: Locator;
  readonly buttonLink: Locator;

  constructor(page: Page) {
    super(page);

    // Demo breadcrumb inside the Preview container
    this.breadcrumb = this.preview.locator('[data-name="Breadcrumb"]').first();

    // Parts
    this.breadcrumbList = this.breadcrumb.locator(
      '[data-name="BreadcrumbList"]'
    );
    this.breadcrumbItems = this.breadcrumb.locator(
      '[data-name="BreadcrumbItem"]'
    );
    this.breadcrumbLinks = this.breadcrumb.locator(
      '[data-name="BreadcrumbLink"]'
    );
    this.breadcrumbSeparators = this.breadcrumb.locator(
      '[data-name="RootSeparator"]'
    );
    this.breadcrumbPage = this.breadcrumb.locator('[data-name="RootPage"]');
    this.breadcrumbEllipsis = this.breadcrumb.locator(
      '[data-name="RootEllipsis"]'
    );

    // Specific items
    this.homeLink = this.breadcrumb.getByRole("link", { name: "Home" });
    this.buttonLink = this.breadcrumb.getByRole("link", { name: "Button" });
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Breadcrumb Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Breadcrumb Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav data-name="Breadcrumb">                          │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Home  >  Components  >  ...  >  Breadcrumb     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </nav>                                                │
     *   │                                                         │
     *   │   Check: Container is visible on page                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The breadcrumb navigation container is rendered
     */
    test("should have Breadcrumb container", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumb).toBeVisible();
    });

    /**
     * TEST: Breadcrumb data-name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav data-name="Breadcrumb">                          │
     *   │        └───────────────────────┘                        │
     *   │              ▲                                          │
     *   │              │                                          │
     *   │        Attribute check                                  │
     *   │        data-name === "Breadcrumb"                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component has correct data-name for testing
     */
    test("should have Breadcrumb data-name attribute", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumb).toHaveAttribute("data-name", "Breadcrumb");
    });

    /**
     * TEST: Breadcrumb Is nav Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav>   <-- tagName === "nav"                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Semantic navigation landmark                   │   │
     *   │   │  for screen readers and assistive tech          │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </nav>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Breadcrumb uses semantic nav element
     */
    test("breadcrumb should be a nav element", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const tagName = await ui.breadcrumb.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("nav");
    });

    /**
     * TEST: BreadcrumbList Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <nav>                                                 │
     *   │     <ol data-name="BreadcrumbList">                     │
     *   │     ┌─────────────────────────────────────────────┐     │
     *   │     │  <li>Home</li>                              │     │
     *   │     │  <li>></li>                                 │     │
     *   │     │  <li>Components</li>                        │     │
     *   │     │  ...                                        │     │
     *   │     └─────────────────────────────────────────────┘     │
     *   │     </ol>                                               │
     *   │   </nav>                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The ordered list container is visible
     */
    test("should have BreadcrumbList", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toBeVisible();
    });

    /**
     * TEST: BreadcrumbList Is ol Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol>   <-- tagName === "ol" (ordered list)            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  1. Home                                        │   │
     *   │   │  2. Components      Semantic ordered list       │   │
     *   │   │  3. Button          for breadcrumb hierarchy    │   │
     *   │   │  4. Breadcrumb                                  │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </ol>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: List uses semantic ol element for hierarchy
     */
    test("breadcrumbList should be an ol element", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const tagName = await ui.breadcrumbList.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("ol");
    });
  });

  test.describe("Items", () => {
    /**
     * TEST: Multiple BreadcrumbItems Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐             │
     *   │   │ Item │ > │ Item │ > │ Item │ > │ Item │             │
     *   │   │  1   │   │  2   │   │  3   │   │ ...  │             │
     *   │   └──────┘   └──────┘   └──────┘   └──────┘             │
     *   │                                                         │
     *   │   count >= 3  (at least Home, middle items, current)    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least 3 breadcrumb items are rendered
     */
    test("should have multiple BreadcrumbItems", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const count = await ui.breadcrumbItems.count();
      expect(count).toBeGreaterThanOrEqual(3);
    });

    /**
     * TEST: Items Are li Elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol>                                                  │
     *   │     <li>   <-- tagName === "li"                         │
     *   │     ┌─────────────┐                                     │
     *   │     │    Home     │   List item element                 │
     *   │     └─────────────┘                                     │
     *   │     </li>                                               │
     *   │     <li>...</li>                                        │
     *   │   </ol>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each item uses semantic li element
     */
    test("items should be li elements", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const tagName = await ui.breadcrumbItems
        .first()
        .evaluate((el) => el.tagName.toLowerCase());
      expect(tagName).toBe("li");
    });
  });

  test.describe("Links", () => {
    /**
     * TEST: Home Link Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌────────────┐                                        │
     *   │   │   [Home]   │  >  Components  >  ...  >  Page        │
     *   │   └────────────┘                                        │
     *   │        ▲                                                │
     *   │        │                                                │
     *   │   First link in breadcrumb trail                        │
     *   │   Check: Link with name "Home" is visible               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Home link is present and visible
     */
    test("should have Home link", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.homeLink).toBeVisible();
    });

    /**
     * TEST: Home Link href Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <a href="/">   <-- href === "/"                       │
     *   │     Home                                                │
     *   │   </a>                                                  │
     *   │                                                         │
     *   │   Click flow:                                           │
     *   │   [Home] ──click──> Navigate to root "/"                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Home link points to root URL
     */
    test("Home link should have correct href", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.homeLink).toHaveAttribute("href", "/");
    });

    /**
     * TEST: Button Link Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Home  >  Components  >  ...  >  ┌────────────┐        │
     *   │                                   │  [Button]  │  > ... │
     *   │                                   └────────────┘        │
     *   │                                        ▲                │
     *   │                                        │                │
     *   │   Link with name "Button" in trail                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button link is present and visible
     */
    test("should have Button link", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.buttonLink).toBeVisible();
    });

    /**
     * TEST: Button Link href Contains "button"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <a href="/docs/components/button">                    │
     *   │     Button                                              │
     *   │   </a>                                                  │
     *   │            └─────────────────────┘                      │
     *   │                      ▲                                  │
     *   │         href.includes("button")                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Button link points to button-related URL
     */
    test("Button link should have correct href", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const href = await ui.buttonLink.getAttribute("href");
      expect(href).toContain("button");
    });

    /**
     * TEST: Links Have Hover Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Normal state:        Hover state:                     │
     *   │   ┌────────────┐       ┌────────────┐                   │
     *   │   │   Home     │  -->  │   Home     │                   │
     *   │   │ (muted)    │       │ (highlight)│                   │
     *   │   └────────────┘       └────────────┘                   │
     *   │                                                         │
     *   │   class includes: hover:text-foreground                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Links have hover color transition class
     */
    test("links should have hover styles", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.homeLink).toHaveClass(/hover:text-foreground/);
    });
  });

  test.describe("Separators", () => {
    /**
     * TEST: Separator Elements Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Home  [>]  Components  [>]  Button  [>]  Page         │
     *   │          │                │            │                │
     *   │          └────────────────┴────────────┘                │
     *   │                     ▲                                   │
     *   │             Separator elements                          │
     *   │             count > 0                                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least one separator exists
     */
    test("should have separator elements", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const count = await ui.breadcrumbSeparators.count();
      expect(count).toBeGreaterThan(0);
    });

    /**
     * TEST: Separators Have aria-hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <li aria-hidden="true">                               │
     *   │     [>]   <-- Visual separator only                     │
     *   │   </li>                                                 │
     *   │                                                         │
     *   │   Screen reader:                                        │
     *   │   "Home, Components, Button, Page"                      │
     *   │      (separators are hidden from AT)                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separators hidden from assistive technology
     */
    test("separators should have aria-hidden", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbSeparators.first()).toHaveAttribute(
        "aria-hidden",
        "true"
      );
    });

    /**
     * TEST: Separators Have role="presentation"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <li role="presentation">                              │
     *   │     [>]   <-- Presentational role                       │
     *   │   </li>                                                 │
     *   │                                                         │
     *   │   Role indicates element is decorative,                 │
     *   │   not part of content structure                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Separators have presentational ARIA role
     */
    test("separators should have role=presentation", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbSeparators.first()).toHaveAttribute(
        "role",
        "presentation"
      );
    });

    /**
     * TEST: Separators Contain Chevron Icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <li data-name="RootSeparator">                        │
     *   │     <svg>    <-- Chevron icon                           │
     *   │       ╱                                                 │
     *   │      ╱   ChevronRight icon                              │
     *   │       ╲                                                 │
     *   │     </svg>                                              │
     *   │   </li>                                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: SVG icon is present inside separator
     */
    test("separators should contain chevron icon", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      const icon = ui.breadcrumbSeparators.first().locator("svg");
      await expect(icon).toBeVisible();
    });
  });

  test.describe("Ellipsis", () => {
    /**
     * TEST: Ellipsis Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Home  >  Components  >  ┌─────────┐  >  Button        │
     *   │                           │  [...]  │                   │
     *   │                           └─────────┘                   │
     *   │                                ▲                        │
     *   │                                │                        │
     *   │   Ellipsis indicates collapsed/hidden items             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Ellipsis element is visible in trail
     */
    test("should have ellipsis element", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbEllipsis).toBeVisible();
    });

    /**
     * TEST: Ellipsis Has aria-hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span aria-hidden="true">                             │
     *   │     [...]   <-- Ellipsis hidden from AT                 │
     *   │   </span>                                               │
     *   │                                                         │
     *   │   Screen readers skip this visual indicator             │
     *   │   (has sr-only text alternative separately)             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Ellipsis hidden from assistive technology
     */
    test("ellipsis should have aria-hidden", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbEllipsis).toHaveAttribute(
        "aria-hidden",
        "true"
      );
    });

    /**
     * TEST: Ellipsis Has role="presentation"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span role="presentation">                            │
     *   │     [...]   <-- Decorative/presentational               │
     *   │   </span>                                               │
     *   │                                                         │
     *   │   Indicates element is visual only,                     │
     *   │   not semantic content                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Ellipsis has presentational ARIA role
     */
    test("ellipsis should have role=presentation", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbEllipsis).toHaveAttribute(
        "role",
        "presentation"
      );
    });
  });

  test.describe("Current Page", () => {
    /**
     * TEST: Current Page Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Home  >  Components  >  Button  >  ┌──────────────┐   │
     *   │                                      │  Breadcrumb  │   │
     *   │                                      └──────────────┘   │
     *   │                                             ▲           │
     *   │                                             │           │
     *   │   Current page (last item, not a link)                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page element is visible
     */
    test("should have current page element", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toBeVisible();
    });

    /**
     * TEST: Current Page Has aria-current="page"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span aria-current="page">                            │
     *   │     Breadcrumb                                          │
     *   │   </span>                                               │
     *   │                                                         │
     *   │   Screen reader: "Breadcrumb, current page"             │
     *   │   Indicates user's current location                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page has aria-current attribute
     */
    test("current page should have aria-current=page", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveAttribute("aria-current", "page");
    });

    /**
     * TEST: Current Page Has aria-disabled="true"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span aria-disabled="true">                           │
     *   │     Breadcrumb                                          │
     *   │   </span>                                               │
     *   │                                                         │
     *   │   ┌─────────┐    ┌─────────────┐                        │
     *   │   │ Button  │ -> │ Breadcrumb  │ (not clickable)        │
     *   │   │ (link)  │    │ (disabled)  │                        │
     *   │   └─────────┘    └─────────────┘                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page is marked as disabled
     */
    test("current page should have aria-disabled=true", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveAttribute("aria-disabled", "true");
    });

    /**
     * TEST: Current Page Has role="link"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span role="link" aria-disabled="true">               │
     *   │     Breadcrumb                                          │
     *   │   </span>                                               │
     *   │                                                         │
     *   │   Semantic structure: appears as link                   │
     *   │   but is disabled (current location)                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page has link role for consistency
     */
    test("current page should have role=link", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveAttribute("role", "link");
    });

    /**
     * TEST: Current Page Displays "Breadcrumb"
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Home  >  Components  >  ...  >  Button  >             │
     *   │                                                         │
     *   │                              ┌──────────────────┐       │
     *   │                              │   "Breadcrumb"   │       │
     *   │                              └──────────────────┘       │
     *   │                                      ▲                  │
     *   │                              Text content check         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page shows correct text content
     */
    test("current page should display 'Breadcrumb'", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveText("Breadcrumb");
    });

    /**
     * TEST: Current Page Has font-normal Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Links (muted):          Current page:                 │
     *   │   ┌────────────┐          ┌────────────┐                │
     *   │   │   Home     │    >     │ Breadcrumb │                │
     *   │   │ (default)  │          │font-normal │                │
     *   │   └────────────┘          └────────────┘                │
     *   │                                                         │
     *   │   .font-normal = font-weight: 400                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page has normal font weight
     */
    test("current page should have font-normal", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveClass(/font-normal/);
    });

    /**
     * TEST: Current Page Has text-foreground Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Links (muted):          Current page:                 │
     *   │   ┌────────────┐          ┌────────────┐                │
     *   │   │   Home     │    >     │ Breadcrumb │                │
     *   │   │ (muted)    │          │(foreground)│                │
     *   │   └────────────┘          └────────────┘                │
     *   │        ░░░                     ███                      │
     *   │    muted color            full contrast                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Current page has prominent text color
     */
    test("current page should have text-foreground", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbPage).toHaveClass(/text-foreground/);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: List Has flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol class="flex ...">                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ┌──────┐ ┌───┐ ┌──────┐ ┌───┐ ┌──────┐         │   │
     *   │   │ │ Item │ │ > │ │ Item │ │ > │ │ Item │ ...     │   │
     *   │   │ └──────┘ └───┘ └──────┘ └───┘ └──────┘         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │       ←────── flex row layout ──────→                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: List uses flexbox layout
     */
    test("list should have flex layout", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toHaveClass(/flex/);
    });

    /**
     * TEST: List Has flex-wrap Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Normal (single line):                                 │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ Home > Components > Button > Breadcrumb         │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Wrapped (narrow viewport):                            │
     *   │   ┌─────────────────────────┐                           │
     *   │   │ Home > Components >     │                           │
     *   │   │ Button > Breadcrumb     │  <-- flex-wrap            │
     *   │   └─────────────────────────┘                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: List can wrap to multiple lines
     */
    test("list should have flex-wrap", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toHaveClass(/flex-wrap/);
    });

    /**
     * TEST: List Has items-center Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌──────────────────────────────────────────────────┐  │
     *   │   │                    ┌───┐                         │  │
     *   │   │ ┌──────┐   ┌───┐   │ > │   ┌──────┐              │  │
     *   │   │ │ Home │ ─ │ > │ ─ │   │ ─ │ Page │   centered   │  │
     *   │   │ └──────┘   └───┘   │   │   └──────┘      ▲       │  │
     *   │   │                    └───┘                 │       │  │
     *   │   └─────────────────────────────────────items-center─┘  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items are vertically centered
     */
    test("list should have items-center", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toHaveClass(/items-center/);
    });

    /**
     * TEST: List Has text-muted-foreground Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <ol class="text-muted-foreground">                    │
     *   │                                                         │
     *   │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                        │
     *   │   Home  >  Components  >  Button  >  Breadcrumb         │
     *   │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                        │
     *   │              ▲                                          │
     *   │         Muted/subdued text color                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: List uses muted text color
     */
    test("list should have text-muted-foreground", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toHaveClass(/text-muted-foreground/);
    });

    /**
     * TEST: List Has text-sm Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   text-sm = font-size: 0.875rem (14px)                  │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Home > Components > Button > Breadcrumb        │   │
     *   │   │  ^^^^   ^^^^^^^^^^   ^^^^^^   ^^^^^^^^^^        │   │
     *   │   │         14px small text size                    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: List uses small text size
     */
    test("list should have text-sm", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbList).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Items Have inline-flex Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <li class="inline-flex ...">                          │
     *   │   ┌─────────────────────────┐                           │
     *   │   │ ┌──────┐    ┌───┐       │                           │
     *   │   │ │ Icon │    │ > │       │  inline-flex allows       │
     *   │   │ │  +   │ +  │   │       │  icon + text alignment    │
     *   │   │ │ Text │    │   │       │                           │
     *   │   │ └──────┘    └───┘       │                           │
     *   │   └─────────────────────────┘                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items use inline-flex display
     */
    test("items should have inline-flex", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbItems.first()).toHaveClass(/inline-flex/);
    });

    /**
     * TEST: Items Have gap-1 Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   gap-1 = gap: 0.25rem (4px)                            │
     *   │                                                         │
     *   │   <li class="gap-1">                                    │
     *   │   ┌─────────────────────┐                               │
     *   │   │ ┌────┐ 4px ┌────┐   │                               │
     *   │   │ │Icon│◄───►│Text│   │  Spacing between children    │
     *   │   │ └────┘     └────┘   │                               │
     *   │   └─────────────────────┘                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items have small gap between children
     */
    test("items should have gap-1", async ({ page }) => {
      const ui = new BreadcrumbPage(page);
      await ui.goto();

      await expect(ui.breadcrumbItems.first()).toHaveClass(/gap-1/);
    });
  });
});
