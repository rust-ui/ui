import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DATE-PICKER COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="DatePicker">                                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  DatePickerHeader                                               │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  [<]        February 2025         [>]                     │  │   │
 * │   │  │   ↑               ↑                ↑                      │  │   │
 * │   │  │ prev         DatePickerTitle     next                     │  │   │
 * │   │  └───────────────────────────────────────────────────────────┘  │   │
 * │   │                                                                 │   │
 * │   │  <table role="grid">                                            │   │
 * │   │  ┌───────────────────────────────────────────────────────────┐  │   │
 * │   │  │  Mo  Tu  We  Th  Fr  Sa  Su  ← DatePickerWeekDay          │  │   │
 * │   │  ├───┬───┬───┬───┬───┬───┬───┤                               │  │   │
 * │   │  │   │   │   │   │   │  1│  2│                               │  │   │
 * │   │  ├───┼───┼───┼───┼───┼───┼───┤                               │  │   │
 * │   │  │  3│  4│  5│  6│  7│  8│  9│  ← DatePickerCell             │  │   │
 * │   │  ├───┼───┼───┼───┼───┼───┼───┤                               │  │   │
 * │   │  │ 10│ 11│ 12│ 13│ 14│ 15│ 16│                               │  │   │
 * │   │  └───┴───┴───┴───┴───┴───┴───┘                               │  │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * NAVIGATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [<] prev-month    February 2025    next-month [>]                     │
 * │         │                                    │                          │
 * │         └────────> January 2025              │                          │
 * │                                              │                          │
 * │                    February 2025 <───────────┘ (click next)             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CELL STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:   Selected:   Today:     Disabled:                           │
 * │   ┌─────┐    ┌═════┐     ┌─────┐    ┌─────┐                             │
 * │   │  5  │    ║  5  ║     │ ●5  │    │░ 5 ░│                             │
 * │   └─────┘    └═════┘     └─────┘    └─────┘                             │
 * │              highlighted  dot        grayed                             │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DatePickerPage extends BasePage {
  protected readonly componentName = "date-picker";

  // Date picker elements - scoped within preview container
  readonly datePicker: Locator;
  readonly header: Locator;
  readonly title: Locator;
  readonly prevButton: Locator;
  readonly nextButton: Locator;
  readonly weekDays: Locator;
  readonly dayCells: Locator;
  readonly table: Locator;

  constructor(page: Page) {
    super(page);

    // Main date picker - scoped within preview
    this.datePicker = this.preview.locator('[data-name="DatePicker"]').first();

    // Header parts
    this.header = this.datePicker.locator('[data-name="DatePickerHeader"]');
    this.title = this.datePicker.locator('[data-name="DatePickerTitle"]');
    this.prevButton = this.datePicker.locator('[title="previous-month"]');
    this.nextButton = this.datePicker.locator('[title="next-month"]');

    // Calendar parts
    this.weekDays = this.datePicker.locator('[data-name="DatePickerWeekDay"]');
    this.dayCells = this.datePicker.locator('[data-name="DatePickerCell"]');
    this.table = this.datePicker.locator("table");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("DatePicker Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: DatePicker Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Page DOM                                              │
     *   │   ├── ...                                               │
     *   │   └── <div data-name="DatePicker">  ← EXISTS & VISIBLE? │
     *   │       ├── header                                        │
     *   │       └── calendar table                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The DatePicker component renders and is visible
     */
    test("should have DatePicker container", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.datePicker).toBeVisible();
    });

    /**
     * TEST: DatePicker Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div data-name="DatePicker">                          │
     *   │        └───────┬───────┘                                │
     *   │                │                                        │
     *   │       CHECK: data-name === "DatePicker"                 │
     *   │                                                         │
     *   │   Used for testing selectors and component identification│
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component has correct data-name attribute
     */
    test("should have DatePicker data-name attribute", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.datePicker).toHaveAttribute("data-name", "DatePicker");
    });

    /**
     * TEST: Header Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  DatePickerHeader   ← VISIBLE?                  │   │
     *   │   │  ┌─────────────────────────────────────────┐    │   │
     *   │   │  │  [<]      February 2025        [>]      │    │   │
     *   │   │  └─────────────────────────────────────────┘    │   │
     *   │   │                                                 │   │
     *   │   │  ┌─────────────────────────────────────────┐    │   │
     *   │   │  │  Mo  Tu  We  Th  Fr  Sa  Su             │    │   │
     *   │   │  │  ...calendar grid...                    │    │   │
     *   │   │  └─────────────────────────────────────────┘    │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Header section with navigation is rendered
     */
    test("should have header", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.header).toBeVisible();
    });

    /**
     * TEST: Title Month and Year
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  [<]       "February 2025"           [>]      │     │
     *   │   │              └─────┬─────┘                    │     │
     *   │   │                    │                          │     │
     *   │   │       CHECK: matches /\w+\s+\d{4}/            │     │
     *   │   │       (Month name + space + 4-digit year)     │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   │   Examples: "January 2025", "December 2024"             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title displays month name and year
     */
    test("should have title with month and year", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.title).toBeVisible();
      // Title should contain month name and year
      const text = await ui.title.textContent();
      expect(text).toMatch(/\w+\s+\d{4}/);
    });

    /**
     * TEST: Navigation Buttons Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │                                               │     │
     *   │   │  [<]       February 2025           [>]        │     │
     *   │   │   ↑                                 ↑         │     │
     *   │   │ prevButton                      nextButton    │     │
     *   │   │ VISIBLE?                        VISIBLE?      │     │
     *   │   │                                               │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Both prev and next navigation buttons are visible
     */
    test("should have navigation buttons", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.prevButton).toBeVisible();
      await expect(ui.nextButton).toBeVisible();
    });

    /**
     * TEST: Calendar Table Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  [<]       February 2025           [>]        │     │
     *   │   │                                               │     │
     *   │   │  <table role="grid">   ← VISIBLE?             │     │
     *   │   │  ┌───┬───┬───┬───┬───┬───┬───┐                │     │
     *   │   │  │Mo │Tu │We │Th │Fr │Sa │Su │                │     │
     *   │   │  ├───┼───┼───┼───┼───┼───┼───┤                │     │
     *   │   │  │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │                │     │
     *   │   │  │...│...│...│...│...│...│...│                │     │
     *   │   │  └───┴───┴───┴───┴───┴───┴───┘                │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Calendar grid table is rendered and visible
     */
    test("should have calendar table", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.table).toBeVisible();
    });
  });

  test.describe("Week Days", () => {
    /**
     * TEST: Seven Week Day Headers
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Week Day Headers Row:                                 │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │Mo │Tu │We │Th │Fr │Sa │Su │                         │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │    1   2   3   4   5   6   7                            │
     *   │                                                         │
     *   │   CHECK: count === 7                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: All 7 days of the week are displayed
     */
    test("should have 7 week day headers", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      const count = await ui.weekDays.count();
      expect(count).toBe(7);
    });

    /**
     * TEST: Monday as First Day
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Week starts on Monday (ISO 8601 standard):            │
     *   │                                                         │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │Mo │Tu │We │Th │Fr │Sa │Su │                         │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │    ↑                                                    │
     *   │   first() === "Mo"                                      │
     *   │                                                         │
     *   │   (Not Sunday-first like US convention)                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Monday is the first day of the week
     */
    test("should display Monday as first day", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.weekDays.first()).toHaveText("Mo");
    });

    /**
     * TEST: Sunday as Last Day
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Week ends on Sunday:                                  │
     *   │                                                         │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │Mo │Tu │We │Th │Fr │Sa │Su │                         │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │                            ↑                            │
     *   │                       last() === "Su"                   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Sunday is the last day of the week
     */
    test("should display Sunday as last day", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.weekDays.last()).toHaveText("Su");
    });

    /**
     * TEST: Week Day Aria Labels
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <th aria-label="Monday">Mo</th>                       │
     *   │       └───────┬───────┘   └┬┘                           │
     *   │               │            │                            │
     *   │      full name for     abbreviated                      │
     *   │      screen readers    display text                     │
     *   │                                                         │
     *   │   Screen Reader:  "Monday"                              │
     *   │   Visual:         "Mo"                                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Week days have full name as aria-label
     */
    test("week days should have aria-label", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.weekDays.first()).toHaveAttribute("aria-label", "Monday");
    });
  });

  test.describe("Day Cells", () => {
    /**
     * TEST: Day Cells Count
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Calendar Grid (example February):                     │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │Mo │Tu │We │Th │Fr │Sa │Su │                         │
     *   │   ├───┼───┼───┼───┼───┼───┼───┤                         │
     *   │   │   │   │   │   │   │ 1 │ 2 │  ← row 1                │
     *   │   ├───┼───┼───┼───┼───┼───┼───┤                         │
     *   │   │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │ 9 │  ← row 2                │
     *   │   ├───┼───┼───┼───┼───┼───┼───┤                         │
     *   │   │...│...│...│...│...│...│...│  ← ... more rows        │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │                                                         │
     *   │   CHECK: count > 28 (minimum days in a month)           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Calendar displays at least 28 day cells
     */
    test("should have day cells", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      const count = await ui.dayCells.count();
      expect(count).toBeGreaterThan(28);
    });

    /**
     * TEST: Day Cells Clickable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User clicks on a day:                                 │
     *   │                                                         │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │                         │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │     ↑                                                   │
     *   │   CLICK on "1"                                          │
     *   │     │                                                   │
     *   │     └── No error thrown                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Day cells respond to click events
     */
    test("day cells should be clickable", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      // Find a visible day cell
      const visibleCell = ui.dayCells.filter({ hasText: /^1$/ }).first();
      await visibleCell.click();
      // Should not throw error
    });
  });

  test.describe("Navigation", () => {
    /**
     * TEST: Previous Month Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │                                                         │
     *   │   Before:                      After:                   │
     *   │   ┌─────────────────────┐      ┌─────────────────────┐  │
     *   │   │ [<]  February 2025  │  →   │ [<]  January 2025   │  │
     *   │   └─────────────────────┘      └─────────────────────┘  │
     *   │      ↑                                                  │
     *   │    CLICK                                                │
     *   │                                                         │
     *   │   CHECK: newTitle !== initialTitle                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking prev button navigates to previous month
     */
    test("clicking previous should change month", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();
      // Wait for WASM hydration before interactive test
      await page.waitForLoadState("networkidle");

      const initialTitle = await ui.title.textContent();
      await ui.prevButton.click();
      await page.waitForTimeout(200);
      const newTitle = await ui.title.textContent();

      expect(newTitle).not.toBe(initialTitle);
    });

    /**
     * TEST: Next Month Navigation
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Preview container                                      │
     *   │                                                         │
     *   │   Before:                      After:                   │
     *   │   ┌─────────────────────┐      ┌─────────────────────┐  │
     *   │   │  February 2025  [>] │  →   │  March 2025     [>] │  │
     *   │   └─────────────────────┘      └─────────────────────┘  │
     *   │                     ↑                                   │
     *   │                   CLICK                                 │
     *   │                                                         │
     *   │   CHECK: newTitle !== initialTitle                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking next button navigates to next month
     */
    test("clicking next should change month", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();
      // Wait for WASM hydration before interactive test
      await page.waitForLoadState("networkidle");

      const initialTitle = await ui.title.textContent();
      await ui.nextButton.click();
      await page.waitForTimeout(200);
      const newTitle = await ui.title.textContent();

      expect(newTitle).not.toBe(initialTitle);
    });

    /**
     * TEST: Previous Button Aria Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button aria-label="Go to previous month">            │
     *   │           └────────────┬────────────┘                   │
     *   │                        │                                │
     *   │   Screen Reader Announcement:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "Go to previous month, button"                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Visual: [<]  (just an icon, no visible text)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Prev button has descriptive aria-label
     */
    test("prev button should have aria-label", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.prevButton).toHaveAttribute(
        "aria-label",
        "Go to previous month"
      );
    });

    /**
     * TEST: Next Button Aria Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <button aria-label="Go to next month">                │
     *   │           └──────────┬──────────┘                       │
     *   │                      │                                  │
     *   │   Screen Reader Announcement:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "Go to next month, button"                     │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Visual: [>]  (just an icon, no visible text)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Next button has descriptive aria-label
     */
    test("next button should have aria-label", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.nextButton).toHaveAttribute(
        "aria-label",
        "Go to next month"
      );
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭─────────────────────────────────────────────────╮   │
     *   │   │                                                 │   │
     *   │   │  [<]       February 2025           [>]          │   │
     *   │   │                                                 │   │
     *   │   │  Mo  Tu  We  Th  Fr  Sa  Su                     │   │
     *   │   │  ...                                            │   │
     *   │   │                                                 │   │
     *   │   ╰─────────────────────────────────────────────────╯   │
     *   │   ↑                                                 ↑   │
     *   │   rounded-lg (large border radius)                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: DatePicker has large rounded corners
     */
    test("date picker should have rounded corners", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.datePicker).toHaveClass(/rounded-lg/);
    });

    /**
     * TEST: Border Style
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ← border class applied                          │   │
     *   │   │                                                 │   │
     *   │   │  [<]       February 2025           [>]          │   │
     *   │   │                                                 │   │
     *   │   │  Mo  Tu  We  Th  Fr  Sa  Su                     │   │
     *   │   │  ...                                            │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: DatePicker has visible border
     */
    test("date picker should have border", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.datePicker).toHaveClass(/border/);
    });

    /**
     * TEST: Padding
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ← p-3 (padding on all sides)                    │   │
     *   │   │   ┌─────────────────────────────────────────┐   │   │
     *   │   │   │  [<]    February 2025      [>]          │   │   │
     *   │   │   │                                         │   │   │
     *   │   │   │  Mo  Tu  We  Th  Fr  Sa  Su             │   │   │
     *   │   │   │  ...                                    │   │   │
     *   │   │   └─────────────────────────────────────────┘   │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: DatePicker has internal padding (p-3)
     */
    test("date picker should have padding", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.datePicker).toHaveClass(/p-3/);
    });

    /**
     * TEST: Table Grid Role
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <table role="grid">                                   │
     *   │          └───┬───┘                                      │
     *   │              │                                          │
     *   │   ARIA Grid Pattern:                                    │
     *   │   ┌───┬───┬───┬───┬───┬───┬───┐                         │
     *   │   │Mo │Tu │We │Th │Fr │Sa │Su │  ← columnheader         │
     *   │   ├───┼───┼───┼───┼───┼───┼───┤                         │
     *   │   │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │  ← gridcell             │
     *   │   └───┴───┴───┴───┴───┴───┴───┘                         │
     *   │                                                         │
     *   │   Enables keyboard navigation (arrow keys)              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table uses ARIA grid role for accessibility
     */
    test("table should have role=grid", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.table).toHaveAttribute("role", "grid");
    });

    /**
     * TEST: Day Cells Text Selection Disabled
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <td class="... select-none ...">                      │
     *   │              └─────┬─────┘                              │
     *   │                    │                                    │
     *   │   user-select: none (prevents text selection)           │
     *   │                                                         │
     *   │   User Experience:                                      │
     *   │   ┌───┬───┬───┐                                         │
     *   │   │ 1 │ 2 │ 3 │  ← Cannot select "1", "2", "3"          │
     *   │   └───┴───┴───┘                                         │
     *   │                                                         │
     *   │   Clicking/dragging across cells doesn't highlight text │
     *   │   Cleaner, more app-like feel                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Day cells have select-none class to prevent text selection
     */
    test("day cells should have select-none class", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      // Check first visible day cell has select-none class
      const firstCell = ui.dayCells.first();
      await expect(firstCell).toHaveClass(/select-none/);
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Navigation Button Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard Navigation:                                  │
     *   │                                                         │
     *   │   [TAB] ──────────────────────────────────┐             │
     *   │                                           ↓             │
     *   │   ┌───────────────────────────────────────────────┐     │
     *   │   │  [<]       February 2025           [>]        │     │
     *   │   │   ↑                                           │     │
     *   │   │  :focus (receives keyboard focus)             │     │
     *   │   └───────────────────────────────────────────────┘     │
     *   │                                                         │
     *   │   CHECK: document.activeElement === prevButton          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation buttons can receive keyboard focus
     */
    test("navigation buttons should be focusable", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await ui.prevButton.focus();
      await expect(ui.prevButton).toBeFocused();
    });

    /**
     * TEST: Title Presentation Role
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <div role="presentation">February 2025</div>          │
     *   │        └───────┬───────┘                                │
     *   │                │                                        │
     *   │   role="presentation" means:                            │
     *   │   - Element is purely decorative/visual                 │
     *   │   - Screen readers skip semantic meaning                │
     *   │   - Content still readable, but not as landmark         │
     *   │                                                         │
     *   │   (Month/year is announced via other means in grid)     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Title has presentation role (decorative)
     */
    test("title should have role=presentation", async ({ page }) => {
      const ui = new DatePickerPage(page);
      await ui.goto();

      await expect(ui.title).toHaveAttribute("role", "presentation");
    });
  });
});
