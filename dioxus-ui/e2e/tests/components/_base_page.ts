import { Locator, Page, expect } from "@playwright/test";

/**
 * Base page object with common functionality for all component pages.
 */
export abstract class BasePage {
  readonly page: Page;

  /** The demo preview container - all component tests should scope within this */
  readonly preview: Locator;

  /** Component name used in URL path (e.g., "button", "card") */
  protected abstract readonly componentName: string;

  constructor(page: Page) {
    this.page = page;
    this.preview = page.locator('[data-name="Preview"]').first();
  }

  /**
   * Navigate to the component's page.
   * @param section Optional section anchor (e.g., "card-action")
   */
  async goto(section?: string) {
    const url = section
      ? `/components/${this.componentName}#${section}`
      : `/components/${this.componentName}`;
    await this.page.goto(url);
  }

  /**
   * Get a locator by data-name attribute.
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
