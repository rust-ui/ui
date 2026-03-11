import { Locator, Page, expect } from "@playwright/test";

/**
 * Base page object for hook pages.
 */
export abstract class BaseHookPage {
  readonly page: Page;

  /** Hook name used in URL path (e.g., "use-copy-clipboard") */
  protected abstract readonly hookName: string;

  constructor(page: Page) {
    this.page = page;
  }

  /**
   * Navigate to the hook's documentation page.
   * @param section Optional section anchor
   */
  async goto(section?: string) {
    const url = section
      ? `/docs/hooks/${this.hookName}#${section}`
      : `/docs/hooks/${this.hookName}`;
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
}
