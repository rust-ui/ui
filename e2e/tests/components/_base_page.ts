import { Locator, Page, expect } from "@playwright/test";

/**
 * Base page object with common functionality for all component pages.
 */
export abstract class BasePage {
  readonly page: Page;

  /** The demo preview container - all component tests should scope within this */
  readonly preview: Locator;

  /** Component name used in URL path (e.g., "button", "dialog") */
  protected abstract readonly componentName: string;

  constructor(page: Page) {
    this.page = page;
    // Demo components are rendered inside Preview container
    this.preview = page.locator('[data-name="Preview"]').first();
  }

  /**
   * Navigate to the component's documentation page.
   * @param section Optional section anchor (e.g., "reactive-button")
   */
  async goto(section?: string) {
    const url = section
      ? `/docs/components/${this.componentName}#${section}`
      : `/docs/components/${this.componentName}`;
    await this.page.goto(url);
  }

  /**
   * Get a locator by data-name attribute.
   * @param name The data-name value (e.g., "Card", "Button")
   */
  byDataName(name: string): Locator {
    return this.page.locator(`[data-name="${name}"]`);
  }

  /**
   * Get first element by data-name attribute.
   */
  firstByDataName(name: string): Locator {
    return this.byDataName(name).first();
  }

  /**
   * Wait for an element to have a specific data attribute value.
   */
  async waitForDataState(
    locator: Locator,
    state: string,
    timeout = 5000
  ): Promise<void> {
    await expect(locator).toHaveAttribute("data-state", state, { timeout });
  }

  /**
   * Wait for an element to be initialized (has data-initialized="true").
   */
  async waitForInitialized(locator: Locator, timeout = 10000): Promise<void> {
    await expect(locator).toHaveAttribute("data-initialized", "true", {
      timeout,
    });
  }
}

/**
 * Base page for chart-specific pages (different URL structure).
 */
export abstract class BaseChartPage extends BasePage {
  protected readonly componentName = "chart";

  /** Chart type used in URL path (e.g., "area-chart", "bar-chart") */
  protected abstract readonly chartType: string;

  async goto(chartType?: string) {
    const type = chartType ?? this.chartType;
    const url = type ? `/charts/${type}` : "/charts";
    await this.page.goto(url);
  }

  /**
   * Wait for chart to render (SVG appears inside container).
   */
  async waitForChartToRender(
    chartContainer: Locator,
    timeout = 10000
  ): Promise<void> {
    await expect(chartContainer.locator("svg").first()).toBeVisible({
      timeout,
    });
  }
}
