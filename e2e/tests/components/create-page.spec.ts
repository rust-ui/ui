import { Locator, Page, test, expect } from "@playwright/test";

/**
 * ============================================================================
 * CREATE PAGE - VISUAL OVERVIEW
 * ============================================================================
 *
 * PAGE ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │  /create                                                                │
 * │  ┌──────────────┐  ┌─────────────────────────────────────────────────┐ │
 * │  │  Customizer  │  │  ThemesBlocks (live preview)                    │ │
 * │  │  ┌──────────┐│  │  ┌──────────────┐  ┌──────────────┐            │ │
 * │  │  │ Customize││  │  │ Total Revenue│  │ Subscriptions│            │ │
 * │  │  │          ││  │  │  LineChart   │  │  BarChart    │            │ │
 * │  │  │ Theme [▼]││  │  └──────────────┘  └──────────────┘            │ │
 * │  │  │          ││  │                                                 │ │
 * │  │  │ Radius   ││  │  ┌──────────────────────────────────────────┐  │ │
 * │  │  │ 0 .3 .5  ││  │  │  Exercise Minutes  (LineChart)           │  │ │
 * │  │  │          ││  │  └──────────────────────────────────────────┘  │ │
 * │  │  │ Font  [▼]││  └─────────────────────────────────────────────────┘ │
 * │  │  │          ││                                                       │
 * │  │  │--preset  ││                                                       │
 * │  │  │Copy code ││                                                       │
 * │  │  └──────────┘│                                                       │
 * │  └──────────────┘                                                       │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CUSTOMIZER CONTROLS:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │  ColorThemePicker:   [Default ▼]  → selects: Amber, Blue, Cyan, ...    │
 * │  RadiusPicker:       [0][0.3][0.5][0.75][1]  → active = bg-primary     │
 * │  FontPicker:         [Inter ▼]    → selects: Inter, Geist, ...         │
 * │  Preset button:      "--preset a1I" → copies to clipboard              │
 * │  Copy code button:   opens CopyCodeDialog with CSS snippet             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * THEME CHANGE FLOW:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │  1. User selects "Amber" in ColorThemePicker                           │
 * │  2. Rust/WASM injects --chart-1..5 as inline styles on <html>          │
 * │  3. data-color-theme="Amber" set on <html>                             │
 * │  4. MutationObserver fires → charts destroyed + re-rendered            │
 * │  5. resolveColor() converts oklch → rgb for ApexCharts                 │
 * │  6. Charts render with new theme colors                                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class CreatePage {
  readonly page: Page;

  // Customizer sidebar
  readonly customizer: Locator;
  readonly customizerHeading: Locator;
  readonly customizerDescription: Locator;

  // Controls
  readonly colorThemeSelect: Locator;
  readonly colorThemeSelectTrigger: Locator;
  readonly radiusButtons: Locator;
  readonly fontSelect: Locator;
  readonly presetButton: Locator;
  readonly copyCodeButton: Locator;

  // Charts
  readonly lineChartContainers: Locator;
  readonly barChartContainer: Locator;

  constructor(page: Page) {
    this.page = page;

    this.customizer = page.locator("aside").first();
    this.customizerHeading = this.customizer.locator("h2");
    this.customizerDescription = this.customizer.locator("p").first();

    this.colorThemeSelect = this.customizer.locator('[data-name="Select"]').first();
    this.colorThemeSelectTrigger = this.customizer.locator('[data-name="SelectTrigger"]').first();
    this.radiusButtons = this.customizer.getByRole("button").filter({ hasNotText: /copy|preset|--/i });
    this.fontSelect = this.customizer.locator('[data-name="Select"]').nth(1);
    this.presetButton = this.customizer.locator("button").filter({ hasText: /--preset/ });
    this.copyCodeButton = this.customizer.locator("button").filter({ hasText: /copy code/i });

    this.lineChartContainers = page.locator('[data-name="LineChart"]');
    this.barChartContainer = page.locator('[data-name="BarChart"]').first();
  }

  async goto(preset?: string) {
    const url = preset ? `/create?preset=${preset}` : "/create";
    await this.page.goto(url);
  }

  async waitForChartToRender(container: Locator, timeout = 20000) {
    await expect(container.locator("svg").first()).toBeVisible({ timeout });
  }

  async getHtmlAttribute(attr: string): Promise<string | null> {
    return this.page.evaluate((a) => document.documentElement.getAttribute(a), attr);
  }

  async getCssVar(varName: string): Promise<string> {
    return this.page.evaluate(
      (v) => getComputedStyle(document.documentElement).getPropertyValue(v).trim(),
      varName
    );
  }

  async waitForCssVar(varName: string, expected: string, timeout = 10000) {
    await this.page.waitForFunction(
      ({ v, e }) => getComputedStyle(document.documentElement).getPropertyValue(v).trim() === e,
      { v: varName, e: expected },
      { timeout }
    );
  }

  /** Wait for WASM to hydrate — preset button text is set reactively on mount. */
  async waitForHydration() {
    await expect(this.presetButton).toContainText("--preset", { timeout: 15000 });
  }

  /**
   * The SelectContent is rendered as a portal outside <aside> in the DOM.
   * Use page-level locator filtered by data-state="open".
   */
  openSelectContent(): Locator {
    return this.page.locator('[data-name="SelectContent"][data-state="open"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Create Page", () => {
  test.describe("Layout", () => {
    /**
     * TEST: Customizer Sidebar Visible
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  <aside>                                            │
     *   │    "Customize"           ← h2 heading               │
     *   │    "Pick a style..."     ← description              │
     *   │                                                     │
     *   │    [controls...]                                    │
     *   │  </aside>                                           │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Customizer sidebar is rendered with heading
     */
    test("should show customizer sidebar", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await expect(ui.customizer).toBeVisible();
      await expect(ui.customizerHeading).toContainText("Customize");
      await expect(ui.customizerDescription).toContainText("Pick a style");
    });

    /**
     * TEST: All Customizer Controls Present
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Customizer                                         │
     *   │  ┌───────────────────────────────────────────────┐  │
     *   │  │  Theme    [Default ▼]   ← ColorThemePicker    │  │
     *   │  │  Radius   [0][.3][.5][.75][1]  ← RadiusPicker │  │
     *   │  │  Font     [Inter  ▼]   ← FontPicker            │  │
     *   │  │  [--preset ...]        ← preset copy button   │  │
     *   │  │  [Copy code]           ← CSS copy button      │  │
     *   │  └───────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: All controls are present in the sidebar
     */
    test("should show all customizer controls", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await expect(ui.colorThemeSelectTrigger).toBeVisible();
      await expect(ui.radiusButtons.first()).toBeVisible();
      await expect(ui.presetButton).toBeVisible();
      await expect(ui.copyCodeButton).toBeVisible();
    });
  });

  test.describe("Color Theme Picker", () => {
    /**
     * TEST: Color Theme Dropdown Opens
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Before click:        After click:                  │
     *   │  ┌──────────────┐     ┌──────────────┐             │
     *   │  │ Default    ▼ │ ──► │ Default    ▼ │             │
     *   │  └──────────────┘     ├──────────────┤             │
     *   │                       │ Default      │             │
     *   │                       │ Amber      ● │             │
     *   │                       │ Blue       ● │             │
     *   │                       │ ...          │             │
     *   │                       └──────────────┘             │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking trigger opens the color theme dropdown
     */
    test("color theme dropdown should open", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      await ui.colorThemeSelectTrigger.click();

      const content = ui.openSelectContent();
      await expect(content).toBeVisible();
      await expect(content.getByText("Amber", { exact: true })).toBeVisible();
      await expect(content.getByText("Blue", { exact: true })).toBeVisible();
    });

    /**
     * TEST: Selecting Theme Updates data-color-theme on <html>
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  <html data-color-theme="Amber">                    │
     *   │          ↑                                          │
     *   │    set by Rust/WASM after color theme change        │
     *   │                                                     │
     *   │  Select "Amber" → data-color-theme = "Amber"        │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Choosing a theme injects data-color-theme on <html>
     */
    test("selecting theme should update data-color-theme on html", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      await ui.colorThemeSelectTrigger.click();
      await ui.openSelectContent().getByText("Amber", { exact: true }).click();

      await expect(page.locator("html")).toHaveAttribute("data-color-theme", "Amber");
    });

    /**
     * TEST: Selecting Theme Updates CSS Variables
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  <html style="--chart-1: oklch(0.879 0.169 91.6)">  │
     *   │                  ↑                                  │
     *   │    inline style injected by Rust/WASM               │
     *   │                                                     │
     *   │  Default: --chart-1 = (theme default value)         │
     *   │  Amber:   --chart-1 = oklch(0.879 0.169 91.605)     │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: --chart-1 CSS var changes after theme selection
     */
    test("selecting theme should update --chart-1 CSS variable", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      const before = await ui.getCssVar("--chart-1");

      await ui.colorThemeSelectTrigger.click();
      await ui.openSelectContent().getByText("Blue", { exact: true }).click();

      const after = await ui.getCssVar("--chart-1");
      expect(after).not.toBe(before);
    });

    /**
     * TEST: Selecting Theme Updates URL Preset
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Before: /create?preset=aaa                         │
     *   │  After:  /create?preset=<new_code>                  │
     *   │                         ↑                           │
     *   │    encodes theme + radius + color + font            │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: URL ?preset= param updates when theme changes
     */
    test("selecting theme should update URL preset param", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      const urlBefore = page.url();

      await ui.colorThemeSelectTrigger.click();
      await ui.openSelectContent().getByText("Emerald", { exact: true }).click();

      const urlAfter = page.url();
      expect(urlAfter).toContain("preset=");
      expect(urlAfter).not.toBe(urlBefore);
    });
  });

  test.describe("Radius Picker", () => {
    /**
     * TEST: Radius Buttons Are Visible
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Radius                                             │
     *   │  ┌─────────────────────────────────────────────┐   │
     *   │  │  [0] [0.3] [0.5] [0.75] [1]                 │   │
     *   │  │   ↑    ↑     ↑     ↑     ↑  all visible?    │   │
     *   │  └─────────────────────────────────────────────┘   │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: All 5 radius option buttons are rendered
     */
    test("should show all radius buttons", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await expect(ui.customizer.getByRole("button", { name: "0", exact: true })).toBeVisible();
      await expect(ui.customizer.getByRole("button", { name: "0.3", exact: true })).toBeVisible();
      await expect(ui.customizer.getByRole("button", { name: "0.5", exact: true })).toBeVisible();
      await expect(ui.customizer.getByRole("button", { name: "0.75", exact: true })).toBeVisible();
      await expect(ui.customizer.getByRole("button", { name: "1", exact: true })).toBeVisible();
    });

    /**
     * TEST: Clicking Radius Updates URL
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Click [1] radius button                            │
     *   │                ↓                                    │
     *   │  URL: /create?preset=<code_with_radius_1>           │
     *   │                ↑                                    │
     *   │    preset encodes the new radius value              │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking a radius button updates the URL preset
     */
    test("clicking radius should update URL preset", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      await ui.customizer.getByRole("button", { name: "1", exact: true }).click();

      await page.waitForURL(/preset=/, { timeout: 10000 });
      expect(page.url()).toContain("preset=");
    });

    /**
     * TEST: Clicking Radius Updates --radius CSS Variable
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  <html style="--radius: 1rem">                      │
     *   │                       ↑                             │
     *   │    injected by Rust/WASM after radius click         │
     *   │                                                     │
     *   │  Click [1] → --radius = "1rem"                      │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: --radius CSS var updates to the clicked value
     */
    test("clicking radius should update --radius CSS variable", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      await ui.customizer.getByRole("button", { name: "1", exact: true }).click();

      await ui.waitForCssVar("--radius", "1rem");
    });
  });

  test.describe("Preset & Copy", () => {
    /**
     * TEST: Preset Code Button Shows Current Preset
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  ┌─────────────────────────────────────────────┐   │
     *   │  │  --preset a1I                               │   │
     *   │  │     ↑ matches ?preset= URL param            │   │
     *   │  └─────────────────────────────────────────────┘   │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Preset button text contains "--preset"
     */
    test("preset button should show preset code", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto("a1I");

      await expect(ui.presetButton).toContainText("--preset");
      await expect(ui.presetButton).toContainText("a1I");
    });

    /**
     * TEST: Preset Code Updates When Settings Change
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Before: [--preset aaa]                             │
     *   │  After:  [--preset xyz]  ← different code           │
     *   │                  ↑                                  │
     *   │    encodes new color theme selection                │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Preset code in button changes after theme change
     */
    test("preset code should update when theme changes", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      const beforeText = await ui.presetButton.textContent();

      await ui.colorThemeSelectTrigger.click();
      await ui.openSelectContent().getByText("Rose", { exact: true }).click();

      const afterText = await ui.presetButton.textContent();
      expect(afterText).not.toBe(beforeText);
    });

    /**
     * TEST: Copy Code Button Opens Dialog
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Click [Copy code]                                  │
     *   │           ↓                                         │
     *   │  ┌─────────────────────────────────────────────┐   │
     *   │  │  Dialog                                     │   │
     *   │  │  ┌─────────────────────────────────────┐   │   │
     *   │  │  │  :root {                            │   │   │
     *   │  │  │    --background: ...;               │   │   │
     *   │  │  │    --primary: ...;                  │   │   │
     *   │  │  └─────────────────────────────────────┘   │   │
     *   │  └─────────────────────────────────────────────┘   │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Copy code button opens a dialog with CSS content
     */
    test("copy code button should open dialog", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await ui.copyCodeButton.click();

      const dialog = page.locator('[data-name="DialogContent"]').first();
      await expect(dialog).toBeVisible();
    });
  });

  test.describe("Charts Render", () => {
    /**
     * TEST: Bar Chart Renders SVG
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Subscriptions Card                                 │
     *   │  ┌───────────────────────────────────────────────┐  │
     *   │  │  <div data-name="BarChart">                   │  │
     *   │  │    <svg> ← ApexCharts renders this            │  │
     *   │  │      <rect /> ← bars                          │  │
     *   │  │    </svg>                                     │  │
     *   │  │  </div>                                       │  │
     *   │  └───────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts renders SVG in the BarChart container
     */
    test("bar chart should render SVG", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await ui.waitForChartToRender(ui.barChartContainer);
    });

    /**
     * TEST: Line Charts Render SVG
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Total Revenue + Exercise Minutes cards             │
     *   │  ┌───────────────────────────────────────────────┐  │
     *   │  │  <div data-name="LineChart">                  │  │
     *   │  │    <svg> ← ApexCharts renders this            │  │
     *   │  │      <path /> ← line + markers                │  │
     *   │  │    </svg>                                     │  │
     *   │  │  </div>                                       │  │
     *   │  └───────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: ApexCharts renders SVG in LineChart containers
     */
    test("line charts should render SVG", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();

      await ui.waitForChartToRender(ui.lineChartContainers.first());
    });

    /**
     * TEST: Charts Re-Render After Theme Change
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  1. Charts render with default color                │
     *   │  2. Select "Amber" theme                            │
     *   │  3. MutationObserver fires, charts re-init          │
     *   │  4. Charts still have SVG (re-rendered successfully)│
     *   │                                                     │
     *   │  BarChart: <svg> still present after theme change   │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Charts remain rendered after a color theme change
     */
    test("charts should re-render after theme change", async ({ page }) => {
      const ui = new CreatePage(page);
      await ui.goto();
      await ui.waitForHydration();

      // Wait for initial render
      await ui.waitForChartToRender(ui.barChartContainer);

      // Change theme
      await ui.colorThemeSelectTrigger.click();
      await ui.openSelectContent().getByText("Amber", { exact: true }).click();

      // Charts should still be rendered after theme change
      await ui.waitForChartToRender(ui.barChartContainer);
      await ui.waitForChartToRender(ui.lineChartContainers.first());
    });
  });

  test.describe("Preset URL", () => {
    /**
     * TEST: Preset in URL Pre-selects Theme
     * ─────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────┐
     *   │  Navigate to /create?preset=a1I                     │
     *   │                              ↑                      │
     *   │    encodes: Neutral + 0.5 + Amber + Inter           │
     *   │                                                     │
     *   │  data-color-theme="Amber" on <html> (from SSR/WASM) │
     *   └─────────────────────────────────────────────────────┘
     *
     *   Validates: Opening page with preset applies the encoded theme
     */
    test("preset in URL should apply encoded theme", async ({ page }) => {
      const ui = new CreatePage(page);
      // a1I encodes Amber color theme
      await ui.goto("a1I");

      await expect(page.locator("html")).toHaveAttribute("data-color-theme", "Amber");
    });
  });
});
