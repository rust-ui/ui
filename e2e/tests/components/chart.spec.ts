import { Locator, Page, test, expect } from "@playwright/test";
import { BaseChartPage } from "./_base_page";

/**
 * ============================================================================
 * CHART COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   __ChartsLayout                                                        │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ChartsHero (navigation)                                        │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  [Area] [Bar] [Line] [Pie] [Radar] [Radial]               │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  Card Grid                                                      │   │
 * │   │  ┌─────────────────────────┐ ┌─────────────────────────┐        │   │
 * │   │  │ Card                    │ │ Card                    │        │   │
 * │   │  │ ┌─────────────────────┐ │ │ ┌─────────────────────┐ │        │   │
 * │   │  │ │ Title  [Selector ▼] │ │ │ │ Title               │ │        │   │
 * │   │  │ │ Description         │ │ │ │ Description         │ │        │   │
 * │   │  │ └─────────────────────┘ │ │ └─────────────────────┘ │        │   │
 * │   │  │ ┌─────────────────────┐ │ │ ┌─────────────────────┐ │        │   │
 * │   │  │ │    ▲                │ │ │ │    █ █              │ │        │   │
 * │   │  │ │   ╱ ╲               │ │ │ │    █ █ █            │ │        │   │
 * │   │  │ │  ╱   ╲__            │ │ │ │  █ █ █ █            │ │        │   │
 * │   │  │ └─────────────────────┘ │ │ └─────────────────────┘ │        │   │
 * │   │  │     AreaChart           │ │     BarChart            │        │   │
 * │   │  └─────────────────────────┘ └─────────────────────────┘        │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CHART TYPES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   AreaChart:    BarChart:     LineChart:    PieChart:                   │
 * │   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐                 │
 * │   │  ▲      │   │ █ █     │   │  ●      │   │  ╱──╲   │                 │
 * │   │ ╱▓╲     │   │ █ █ █   │   │ ╱ ╲     │   │ │25%│   │                 │
 * │   │╱▓▓▓╲____│   │ █ █ █ █ │   │╱   ╲____│   │  ╲──╱   │                 │
 * │   └─────────┘   └─────────┘   └─────────┘   └─────────┘                 │
 * │                                                                         │
 * │   RadarChart:   RadialChart:                                            │
 * │   ┌─────────┐   ┌─────────┐                                             │
 * │   │  ╱╲     │   │  ◐      │                                             │
 * │   │ ╱  ╲    │   │ 75%     │                                             │
 * │   │ ╲  ╱    │   │         │                                             │
 * │   └─────────┘   └─────────┘                                             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * INTERACTIVE FEATURES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Time Range Selector:                                                  │
 * │   ┌────────────────────────────────────────────────────────────────┐    │
 * │   │  Area Chart - Interactive        [Last 3 months ▼]            │    │
 * │   │  Showing total visitors                                        │    │
 * │   │                                  ┌─────────────────┐           │    │
 * │   │                                  │ Last 3 months   │           │    │
 * │   │                                  │ Last 30 days    │           │    │
 * │   │                                  │ Last 7 days     │           │    │
 * │   │                                  └─────────────────┘           │    │
 * │   └────────────────────────────────────────────────────────────────┘    │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class ChartPage extends BaseChartPage {
  protected readonly chartType = "";

  // Layout
  readonly chartsLayout: Locator;
  readonly chartsHero: Locator;

  // Area Chart 01 (Interactive)
  readonly areaChart01Card: Locator;
  readonly areaChart01Title: Locator;
  readonly areaChart01Description: Locator;
  readonly areaChart01Container: Locator;
  readonly areaChart01Select: Locator;
  readonly areaChart01SelectTrigger: Locator;

  // Generic chart locators
  readonly allAreaCharts: Locator;
  readonly allCards: Locator;

  // Other chart types
  readonly barChartContainer: Locator;
  readonly lineChartContainer: Locator;
  readonly pieChartContainer: Locator;
  readonly radarChartContainer: Locator;
  readonly radialChartContainer: Locator;

  constructor(page: Page) {
    super(page);

    // Layout - scoped within preview
    this.chartsLayout = this.preview.locator('[data-name="__ChartsLayout"]');
    this.chartsHero = this.preview.locator('[data-name="ChartsHero"]');

    // Area Chart 01 - the main interactive chart (CardTitle renders as h2)
    this.areaChart01Card = this.preview.locator('[data-name="Card"]').first();
    this.areaChart01Title = this.areaChart01Card.locator("h2");
    this.areaChart01Description = this.areaChart01Card.locator(
      '[data-name="CardDescription"]'
    );
    this.areaChart01Container = this.preview.locator('[data-name="AreaChart"]').first();
    this.areaChart01Select = this.areaChart01Card.locator(
      '[data-name="Select"]'
    );
    this.areaChart01SelectTrigger = this.areaChart01Card.locator(
      '[data-name="SelectTrigger"]'
    );

    // Generic chart locators
    this.allAreaCharts = this.preview.locator('[data-name="AreaChart"]');
    this.allCards = this.preview.locator('[data-name="Card"]');

    // Other chart types
    this.barChartContainer = this.preview.locator('[data-name="BarChart"]').first();
    this.lineChartContainer = this.preview.locator('[data-name="LineChart"]').first();
    this.pieChartContainer = this.preview.locator('[data-name="PieChart"]').first();
    this.radarChartContainer = this.preview.locator('[data-name="RadarChart"]').first();
    this.radialChartContainer = this.preview.locator('[data-name="RadialChart"]').first();
  }

  async getChartDataAttribute(
    chartContainer: Locator,
    attribute: string
  ): Promise<string | null> {
    return await chartContainer.getAttribute(`data-chart-${attribute}`);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Charts Page", () => {
  test.describe("Layout", () => {
    /**
     * TEST: Charts Layout Container Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  __ChartsLayout                                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   [Hero Navigation]                               │  │
     *   │  │                                                   │  │
     *   │  │   [Chart Cards Grid]                              │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │      ↑ data-name="__ChartsLayout" (visible?)            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main charts layout container is rendered and visible
     */
    test("should have charts layout container", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto();

      await expect(ui.chartsLayout).toBeVisible();
    });

    /**
     * TEST: Hero Navigation Visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ChartsHero                                             │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Area] [Bar] [Line] [Pie] [Radar] [Radial]       │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │       ↑ navigation links (at least one visible?)        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hero section has navigation links for chart types
     */
    test("should have charts hero navigation", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto();

      // Hero should have navigation buttons for different chart types
      const heroButtons = ui.chartsLayout.getByRole("link");
      await expect(heroButtons.first()).toBeVisible();
    });
  });

  test.describe("Area Charts", () => {
    /**
     * TEST: Area Chart Container Renders
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Title                                            │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │       ▲                                     │  │  │
     *   │  │  │      ╱▓╲     ← data-name="AreaChart"        │  │  │
     *   │  │  │     ╱▓▓▓╲____                               │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Area chart container is visible with correct data-name
     */
    test("should render area chart container", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      await expect(ui.areaChart01Container).toBeVisible();
      await expect(ui.areaChart01Container).toHaveAttribute(
        "data-name",
        "AreaChart"
      );
    });

    /**
     * TEST: Area Chart Data Attributes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="AreaChart"                             │
     *   │       data-chart-curve="..."    ← curve type            │
     *   │       data-chart-values="..."   ← chart data values     │
     *   │       data-chart-labels="...">  ← axis labels           │
     *   │                                                         │
     *   │    [Chart visualization]                                │
     *   │                                                         │
     *   │  </div>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chart has required data-chart-* attributes for config
     */
    test("area chart should have data attributes", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // Check that chart has required data attributes
      await expect(ui.areaChart01Container).toHaveAttribute("data-chart-curve");
      await expect(ui.areaChart01Container).toHaveAttribute(
        "data-chart-values"
      );
      await expect(ui.areaChart01Container).toHaveAttribute(
        "data-chart-labels"
      );
    });

    /**
     * TEST: Area Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  AreaChart Container                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <path d="..." />  ← area fill                  │  │
     *   │  │    <line />          ← axis lines                 │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG inside container
     */
    test("area chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // Wait for ApexCharts to render
      await ui.waitForChartToRender(ui.areaChart01Container);
    });

    /**
     * TEST: Area Chart Card Title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  "Area Chart"  ← <h2> CardTitle             │  │  │
     *   │  │  │   Description                               │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  [Chart content]                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card has visible title containing "Area Chart"
     */
    test("area chart card should have title", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      await expect(ui.areaChart01Title).toBeVisible();
      await expect(ui.areaChart01Title).toContainText("Area Chart");
    });

    /**
     * TEST: Area Chart Card Description
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  Area Chart                                 │  │  │
     *   │  │  │  "...visitors..."  ← CardDescription        │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  [Chart content]                                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card has description text mentioning "visitors"
     */
    test("area chart card should have description", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      await expect(ui.areaChart01Description).toBeVisible();
      await expect(ui.areaChart01Description).toContainText("visitors");
    });

    /**
     * TEST: Multiple Area Charts on Page
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card Grid                                              │
     *   │  ┌──────────────────┐  ┌──────────────────┐             │
     *   │  │  AreaChart #1    │  │  AreaChart #2    │             │
     *   │  │  ┌────────────┐  │  │  ┌────────────┐  │             │
     *   │  │  │   ▲        │  │  │  │   ▲▲       │  │             │
     *   │  │  │  ╱ ╲       │  │  │  │  ╱  ╲      │  │             │
     *   │  │  └────────────┘  │  │  └────────────┘  │             │
     *   │  └──────────────────┘  └──────────────────┘             │
     *   │       count > 1                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Page displays more than one area chart
     */
    test("should have multiple area charts on page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // Page should have multiple area charts in the grid
      const chartCount = await ui.allAreaCharts.count();
      expect(chartCount).toBeGreaterThan(1);
    });

    /**
     * TEST: Area Chart Time Range Selector
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card Header                                            │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  Area Chart - Interactive   [Last 3 months ▼]     │  │
     *   │  │  Showing total visitors          ↑                │  │
     *   │  │                           SelectTrigger           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Interactive chart has visible time range selector
     */
    test("area chart should have time range selector", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // The interactive area chart has a Select for time range
      await expect(ui.areaChart01SelectTrigger).toBeVisible();
    });

    /**
     * TEST: Time Range Selector Opens Dropdown
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before click:              After click:                │
     *   │  ┌────────────────┐         ┌────────────────┐          │
     *   │  │ Last 3 months▼ │  ──►    │ Last 3 months▼ │          │
     *   │  └────────────────┘         ├────────────────┤          │
     *   │                             │ Last 3 months  │          │
     *   │                             │ Last 30 days   │          │
     *   │                             │ Last 7 days    │          │
     *   │                             └────────────────┘          │
     *   │                                ↑ SelectContent          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking selector trigger opens dropdown content
     */
    test("time range selector should open dropdown", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // Click to open the select dropdown
      await ui.areaChart01SelectTrigger.click();

      // Should show options
      const selectContent = page.locator('[data-name="SelectContent"]');
      await expect(selectContent).toBeVisible();
    });

    /**
     * TEST: Time Range Options Available
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  SelectContent (dropdown)                               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ► "Last 3 months"  ◄── visible?                  │  │
     *   │  │  ► "Last 30 days"   ◄── visible?                  │  │
     *   │  │  ► "Last 7 days"    ◄── visible?                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All three time range options are available
     */
    test("time range options should be available", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      await ui.areaChart01SelectTrigger.click();

      // Check for time range options inside the select content
      const selectContent = page.locator('[data-name="SelectContent"]');
      await expect(
        selectContent.getByText("Last 3 months", { exact: true })
      ).toBeVisible();
      await expect(
        selectContent.getByText("Last 30 days", { exact: true })
      ).toBeVisible();
      await expect(
        selectContent.getByText("Last 7 days", { exact: true })
      ).toBeVisible();
    });
  });

  test.describe("Bar Charts", () => {
    /**
     * TEST: Bar Chart Page Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  /charts/bar-chart                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │    █ █                                      │  │  │
     *   │  │  │    █ █ █        ← BarChart container        │  │  │
     *   │  │  │  █ █ █ █                                    │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation to bar-chart page shows bar chart
     */
    test("should navigate to bar chart page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("bar-chart");

      await expect(ui.barChartContainer).toBeVisible();
    });

    /**
     * TEST: Bar Chart Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="BarChart">                             │
     *   │       ↑ attribute = "BarChart"                          │
     *   │    [Bar chart visualization]                            │
     *   │  </div>                                                 │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Bar chart container has data-name="BarChart"
     */
    test("bar chart should have data-name attribute", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("bar-chart");

      await expect(ui.barChartContainer).toHaveAttribute(
        "data-name",
        "BarChart"
      );
    });

    /**
     * TEST: Bar Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  BarChart Container                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <rect />  ← bar elements                       │  │
     *   │  │    <line />  ← axis lines                         │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG for bar chart
     */
    test("bar chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("bar-chart");

      await ui.waitForChartToRender(ui.barChartContainer);
    });
  });

  test.describe("Line Charts", () => {
    /**
     * TEST: Line Chart Page Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  /charts/line-chart                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │      ●                                      │  │  │
     *   │  │  │     ╱ ╲          ← LineChart container      │  │  │
     *   │  │  │    ╱   ╲____                                │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation to line-chart page shows line chart
     */
    test("should navigate to line chart page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("line-chart");

      await expect(ui.lineChartContainer).toBeVisible();
    });

    /**
     * TEST: Line Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  LineChart Container                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <path d="..." />  ← line path                  │  │
     *   │  │    <circle />        ← data points                │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG for line chart
     */
    test("line chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("line-chart");

      await ui.waitForChartToRender(ui.lineChartContainer);
    });
  });

  test.describe("Pie Charts", () => {
    /**
     * TEST: Pie Chart Page Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  /charts/pie-chart                                      │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │        ╱──╲                                 │  │  │
     *   │  │  │       │25%│     ← PieChart container        │  │  │
     *   │  │  │        ╲──╱                                 │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation to pie-chart page shows pie chart
     */
    test("should navigate to pie chart page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("pie-chart");

      await expect(ui.pieChartContainer).toBeVisible();
    });

    /**
     * TEST: Pie Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  PieChart Container                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <path d="..." />  ← pie slices (arc paths)     │  │
     *   │  │    <text />          ← labels/percentages         │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG for pie chart
     */
    test("pie chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("pie-chart");

      await ui.waitForChartToRender(ui.pieChartContainer);
    });
  });

  test.describe("Radar Charts", () => {
    /**
     * TEST: Radar Chart Page Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  /charts/radar-chart                                    │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │       ╱╲                                    │  │  │
     *   │  │  │      ╱  ╲        ← RadarChart container     │  │  │
     *   │  │  │      ╲  ╱                                   │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation to radar-chart page shows radar chart
     */
    test("should navigate to radar chart page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("radar-chart");

      await expect(ui.radarChartContainer).toBeVisible();
    });

    /**
     * TEST: Radar Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadarChart Container                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <polygon />  ← radar area                      │  │
     *   │  │    <line />     ← axis spokes                     │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG for radar chart
     */
    test("radar chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("radar-chart");

      await ui.waitForChartToRender(ui.radarChartContainer);
    });
  });

  test.describe("Radial Charts", () => {
    /**
     * TEST: Radial Chart Page Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  /charts/radial-chart                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │        ◐                                    │  │  │
     *   │  │  │       75%        ← RadialChart container    │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation to radial-chart page shows radial chart
     */
    test("should navigate to radial chart page", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("radial-chart");

      await expect(ui.radialChartContainer).toBeVisible();
    });

    /**
     * TEST: Radial Chart Renders SVG
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  RadialChart Container                                  │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  <svg>  ← ApexCharts generated SVG                │  │
     *   │  │    <circle />  ← background track                 │  │
     *   │  │    <path />    ← progress arc                     │  │
     *   │  │  </svg>                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts library renders SVG for radial chart
     */
    test("radial chart should render SVG", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("radial-chart");

      await ui.waitForChartToRender(ui.radialChartContainer);
    });
  });

  test.describe("Chart Navigation", () => {
    /**
     * TEST: Navigate Between Chart Types
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Hero Navigation Flow:                                  │
     *   │                                                         │
     *   │  [Area] [Bar] [Line] [Pie] [Radar] [Radial]             │
     *   │           │                 │                           │
     *   │           ▼ click           ▼ click                     │
     *   │     /bar-chart  ──►   /pie-chart                        │
     *   │                                                         │
     *   │  URL changes with each navigation                       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Links navigate to correct chart type pages
     */
    test("should navigate between chart types", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto();

      // Click on Bar Chart link
      await page.getByRole("link", { name: "Bar Chart" }).click();
      await expect(page).toHaveURL(/.*bar-chart/);

      // Click on Pie Chart link
      await page.getByRole("link", { name: "Pie Chart" }).click();
      await expect(page).toHaveURL(/.*pie-chart/);
    });

    /**
     * TEST: Chart Type Links Visible in Hero
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ChartsHero Navigation Bar                              │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  [Area Chart] [Bar Chart] [Line Chart] [Pie Chart]│  │
     *   │  │       ↑            ↑           ↑           ↑      │  │
     *   │  │    visible?     visible?    visible?    visible?  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All main chart type links are visible in hero
     */
    test("chart type links should be visible in hero", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto();

      await expect(
        page.getByRole("link", { name: "Area Chart" })
      ).toBeVisible();
      await expect(page.getByRole("link", { name: "Bar Chart" })).toBeVisible();
      await expect(
        page.getByRole("link", { name: "Line Chart" })
      ).toBeVisible();
      await expect(page.getByRole("link", { name: "Pie Chart" })).toBeVisible();
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Chart Cards Have Proper Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card                                                   │
     *   │  ╭───────────────────────────────────────────────────╮  │
     *   │  │  class="... rounded-* border-* ..."               │  │
     *   │  │  ↑                                                │  │
     *   │  │  rounded corners + border styling                 │  │
     *   │  │                                                   │  │
     *   │  │  [Chart content]                                  │  │
     *   │  ╰───────────────────────────────────────────────────╯  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cards have rounded corners and border classes
     */
    test("chart cards should have proper styling", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      // Cards should have border and rounded corners
      const firstCard = ui.allCards.first();
      await expect(firstCard).toHaveClass(/rounded/);
      await expect(firstCard).toHaveClass(/border/);
    });

    /**
     * TEST: Chart Containers Have Full Width
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Card                                                   │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │◄──────────────── w-full ────────────────────────►│  │
     *   │  │  AreaChart Container                              │  │
     *   │  │  class="... w-full ..."                           │  │
     *   │  │                                                   │  │
     *   │  │  [Chart spans full width of card]                 │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Chart container spans full width of its parent
     */
    test("chart containers should have full width", async ({ page }) => {
      const ui = new ChartPage(page);
      await ui.goto("area-chart");

      await expect(ui.areaChart01Container).toHaveClass(/w-full/);
    });
  });
});
