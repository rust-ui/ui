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

  /**
   * Navigate to this component's page via SPA navigation (not a full reload).
   * This exercises the page__fade route transition animation, which historically
   * broke position:fixed overlays (Drawer, ContextMenu, Sonner) by creating a
   * CSS stacking context via transform on the outlet container.
   *
   * Strategy: full-load a different component page, then click the sidebar link
   * to SPA-navigate here. The Leptos router fires retrigger_page_fade(), which
   * applies translateY + opacity animation on #page__outlet.
   *
   * @param fromComponent Component to start from (default: "button")
   */
  async gotoViaSpa(fromComponent = "button"): Promise<void> {
    // Start on a different page (full load)
    await this.page.goto(`/docs/components/${fromComponent}`);

    // Click the sidebar/nav link — triggers SPA navigation + page__fade animation
    const targetUrl = `/docs/components/${this.componentName}`;
    await this.page.click(`a[href="${targetUrl}"]`);

    // Wait for the URL to update and the animated outlet to settle
    await this.page.waitForURL(`**${targetUrl}`, { timeout: 10000 });
    // Allow the 200ms page__fade animation to complete
    await this.page.waitForTimeout(300);
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
