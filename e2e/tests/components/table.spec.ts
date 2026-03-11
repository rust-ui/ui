import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * TABLE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <table data-name="Table">                                             │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  TableHeader (thead)                                            │   │
 * │   │  ┌─────────────────────────────────────────────────────────────┐│   │
 * │   │  │ Invoice  │ Status  │ Method      │ Amount (text-right)     ││   │
 * │   │  └─────────────────────────────────────────────────────────────┘│   │
 * │   │                                                                 │   │
 * │   │  TableBody (tbody)                                              │   │
 * │   │  ┌─────────────────────────────────────────────────────────────┐│   │
 * │   │  │ INV001   │ Paid    │ Credit Card │ $250.00                 ││   │
 * │   │  │ INV002   │ Pending │ PayPal      │ $150.00                 ││   │
 * │   │  │ INV003   │ Paid    │ Bank Trans. │ $350.00                 ││   │
 * │   │  │ ...      │ ...     │ ...         │ ...                     ││   │
 * │   │  └─────────────────────────────────────────────────────────────┘│   │
 * │   │                                                                 │   │
 * │   │  TableFooter (tfoot)                                            │   │
 * │   │  ┌─────────────────────────────────────────────────────────────┐│   │
 * │   │  │ Total                            │ $2,500.00               ││   │
 * │   │  └─────────────────────────────────────────────────────────────┘│   │
 * │   │                                                                 │   │
 * │   │  TableCaption (caption-bottom)                                  │   │
 * │   │  ┌─────────────────────────────────────────────────────────────┐│   │
 * │   │  │ A list of your recent invoices.                            ││   │
 * │   │  └─────────────────────────────────────────────────────────────┘│   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * TABLE STRUCTURE:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────┬──────────────┬──────────────┬──────────────┐          │
 * │   │ TableHead   │ TableHead    │ TableHead    │ TableHead    │ ← thead  │
 * │   ├─────────────┼──────────────┼──────────────┼──────────────┤          │
 * │   │ TableCell   │ TableCell    │ TableCell    │ TableCell    │          │
 * │   │ TableCell   │ TableCell    │ TableCell    │ TableCell    │ ← tbody  │
 * │   │ TableCell   │ TableCell    │ TableCell    │ TableCell    │          │
 * │   ├─────────────┴──────────────┼──────────────┴──────────────┤          │
 * │   │ Total                      │ $2,500.00                   │ ← tfoot  │
 * │   └────────────────────────────┴─────────────────────────────┘          │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .w-full        ← Full width                                │       │
 * │   │  .caption-bottom← Caption at bottom                         │       │
 * │   │  .text-sm       ← Small text size                           │       │
 * │   │  .font-medium   ← Header cells                              │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class TablePage extends BasePage {
  protected readonly componentName = "table";

  // Table elements
  readonly table: Locator;
  readonly tableCaption: Locator;
  readonly tableHeader: Locator;
  readonly tableBody: Locator;
  readonly tableFooter: Locator;
  readonly headerCells: Locator;
  readonly bodyRows: Locator;
  readonly bodyCells: Locator;

  constructor(page: Page) {
    super(page);

    // Main table - scoped within preview
    this.table = this.preview.locator('[data-name="Table"]').first();

    // Table parts
    this.tableCaption = this.table.locator('[data-name="TableCaption"]');
    this.tableHeader = this.table.locator('[data-name="TableHeader"]');
    this.tableBody = this.table.locator('[data-name="TableBody"]');
    this.tableFooter = this.table.locator('[data-name="TableFooter"]');

    // Cells and rows
    this.headerCells = this.tableHeader.locator('[data-name="TableHead"]');
    this.bodyRows = this.tableBody.locator('[data-name="TableRow"]');
    this.bodyCells = this.tableBody.locator('[data-name="TableCell"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Table Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Table Component Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table data-name="Table">  ← Is this visible?         │
     *   │    ...                                                 │
     *   │  </table>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table component renders and is visible on page
     */
    test("should have Table component", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.table).toBeVisible();
    });

    /**
     * TEST: Table Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table data-name="Table">  ← Check attribute value    │
     *   │         ^^^^^^^^^^^^^^^^                               │
     *   │         Must equal "Table"                             │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table has correct data-name attribute for selectors
     */
    test("should have Table data-name attribute", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.table).toHaveAttribute("data-name", "Table");
    });

    /**
     * TEST: Semantic Table Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table>  ← Is this the actual <table> HTML element?   │
     *   │  ^^^^^^^^                                              │
     *   │  Not <div> or other element                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Component uses semantic <table> element for accessibility
     */
    test("should be a table element", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const tagName = await ui.table.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("table");
    });

    /**
     * TEST: TableCaption Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table>                                               │
     *   │    ...                                                 │
     *   │    <caption data-name="TableCaption">  ← Visible?      │
     *   │      A list of your recent invoices.                   │
     *   │    </caption>                                          │
     *   │  </table>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table has a visible caption element
     */
    test("should have TableCaption", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableCaption).toBeVisible();
    });

    /**
     * TEST: TableHeader Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table>                                               │
     *   │    <thead data-name="TableHeader">  ← Visible?         │
     *   │      Invoice | Status | Method | Amount                │
     *   │    </thead>                                            │
     *   │    ...                                                 │
     *   │  </table>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table has a visible header section (thead)
     */
    test("should have TableHeader", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableHeader).toBeVisible();
    });

    /**
     * TEST: TableBody Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table>                                               │
     *   │    <thead>...</thead>                                  │
     *   │    <tbody data-name="TableBody">  ← Visible?           │
     *   │      INV001 | Paid | Credit Card | $250.00             │
     *   │      INV002 | Pending | PayPal | $150.00               │
     *   │      ...                                               │
     *   │    </tbody>                                            │
     *   │  </table>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table has a visible body section (tbody) with data rows
     */
    test("should have TableBody", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableBody).toBeVisible();
    });

    /**
     * TEST: TableFooter Presence
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <table>                                               │
     *   │    <thead>...</thead>                                  │
     *   │    <tbody>...</tbody>                                  │
     *   │    <tfoot data-name="TableFooter">  ← Visible?         │
     *   │      Total | $2,500.00                                 │
     *   │    </tfoot>                                            │
     *   │  </table>                                              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table has a visible footer section (tfoot) with totals
     */
    test("should have TableFooter", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableFooter).toBeVisible();
    });
  });

  test.describe("Caption", () => {
    /**
     * TEST: Caption Text Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <caption>                                             │
     *   │    "A list of your recent invoices."  ← Text match?    │
     *   │  </caption>                                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Caption displays the correct descriptive text
     */
    test("caption should have correct text", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableCaption).toHaveText(
        "A list of your recent invoices."
      );
    });

    /**
     * TEST: Semantic Caption Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <caption>  ← Is this the actual <caption> element?    │
     *   │  ^^^^^^^^^                                             │
     *   │  Not <div> or <span>                                   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Uses semantic <caption> for screen reader accessibility
     */
    test("caption should be a caption element", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const tagName = await ui.tableCaption.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("caption");
    });
  });

  test.describe("Header", () => {
    /**
     * TEST: Four Header Columns
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <thead>                                                │
     *   │    <tr>                                                 │
     *   │      <th>Invoice</th>   ─┐                              │
     *   │      <th>Status</th>     │  Count = 4?                  │
     *   │      <th>Method</th>     │                              │
     *   │      <th>Amount</th>    ─┘                              │
     *   │    </tr>                                                │
     *   │  </thead>                                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table header has exactly 4 column headers
     */
    test("should have four header columns", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const count = await ui.headerCells.count();
      expect(count).toBe(4);
    });

    /**
     * TEST: Invoice Column Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬───────────┬───────────┐
     *   │ Invoice   │ Status    │ Method    │ Amount    │
     *   │ ^^^^^^^^                                      │
     *   │ [0] = "Invoice"?                              │
     *   └───────────┴───────────┴───────────┴───────────┘
     *
     *   Validates: First column header text is "Invoice"
     */
    test("should have Invoice column", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.nth(0)).toHaveText("Invoice");
    });

    /**
     * TEST: Status Column Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬───────────┬───────────┐
     *   │ Invoice   │ Status    │ Method    │ Amount    │
     *   │           │ ^^^^^^                            │
     *   │           │ [1] = "Status"?                   │
     *   └───────────┴───────────┴───────────┴───────────┘
     *
     *   Validates: Second column header text is "Status"
     */
    test("should have Status column", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.nth(1)).toHaveText("Status");
    });

    /**
     * TEST: Method Column Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬───────────┬───────────┐
     *   │ Invoice   │ Status    │ Method    │ Amount    │
     *   │                       │ ^^^^^^                │
     *   │                       │ [2] = "Method"?       │
     *   └───────────┴───────────┴───────────┴───────────┘
     *
     *   Validates: Third column header text is "Method"
     */
    test("should have Method column", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.nth(2)).toHaveText("Method");
    });

    /**
     * TEST: Amount Column Header
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬───────────┬───────────┐
     *   │ Invoice   │ Status    │ Method    │ Amount    │
     *   │                                   │ ^^^^^^    │
     *   │                                   │ [3] = "Amount"?
     *   └───────────┴───────────┴───────────┴───────────┘
     *
     *   Validates: Fourth column header text is "Amount"
     */
    test("should have Amount column", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.nth(3)).toHaveText("Amount");
    });

    /**
     * TEST: Amount Column Right Alignment
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬───────────┬───────────┐
     *   │ Invoice   │ Status    │ Method    │    Amount │
     *   │                                   │ ^^^^^^^^^ │
     *   │                                   │ text-right│
     *   └───────────┴───────────┴───────────┴───────────┘
     *
     *   Validates: Amount column has text-right class for currency alignment
     */
    test("Amount column should have text-right", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.nth(3)).toHaveClass(/text-right/);
    });
  });

  test.describe("Body", () => {
    /**
     * TEST: Seven Invoice Rows
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <tbody>                                                │
     *   │    <tr>INV001...</tr>  ─┐                               │
     *   │    <tr>INV002...</tr>   │                               │
     *   │    <tr>INV003...</tr>   │                               │
     *   │    <tr>INV004...</tr>   │  Count = 7?                   │
     *   │    <tr>INV005...</tr>   │                               │
     *   │    <tr>INV006...</tr>   │                               │
     *   │    <tr>INV007...</tr>  ─┘                               │
     *   │  </tbody>                                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table body contains exactly 7 invoice data rows
     */
    test("should have seven invoice rows", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const count = await ui.bodyRows.count();
      expect(count).toBe(7);
    });

    /**
     * TEST: First Row Invoice Number
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬─────────────┬───────────┐
     *   │ INV001    │ Paid      │ Credit Card │ $250.00   │  ← Row 0
     *   │ ^^^^^^                                          │
     *   │ Contains "INV001"?                              │
     *   ├───────────┼───────────┼─────────────┼───────────┤
     *   │ INV002    │ ...       │ ...         │ ...       │
     *   └───────────┴───────────┴─────────────┴───────────┘
     *
     *   Validates: First data row displays invoice number INV001
     */
    test("first row should have INV001", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const firstRow = ui.bodyRows.nth(0);
      await expect(firstRow).toContainText("INV001");
    });

    /**
     * TEST: First Row Status
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬─────────────┬───────────┐
     *   │ INV001    │ Paid      │ Credit Card │ $250.00   │  ← Row 0
     *   │           │ ^^^^                                │
     *   │           │ Contains "Paid"?                    │
     *   ├───────────┼───────────┼─────────────┼───────────┤
     *   │ INV002    │ Pending   │ ...         │ ...       │
     *   └───────────┴───────────┴─────────────┴───────────┘
     *
     *   Validates: First data row shows "Paid" payment status
     */
    test("first row should have Paid status", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const firstRow = ui.bodyRows.nth(0);
      await expect(firstRow).toContainText("Paid");
    });

    /**
     * TEST: First Row Payment Method
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬─────────────┬───────────┐
     *   │ INV001    │ Paid      │ Credit Card │ $250.00   │  ← Row 0
     *   │                       │ ^^^^^^^^^^^             │
     *   │                       │ Contains "Credit Card"? │
     *   ├───────────┼───────────┼─────────────┼───────────┤
     *   │ INV002    │ ...       │ PayPal      │ ...       │
     *   └───────────┴───────────┴─────────────┴───────────┘
     *
     *   Validates: First data row shows "Credit Card" as payment method
     */
    test("first row should have Credit Card method", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const firstRow = ui.bodyRows.nth(0);
      await expect(firstRow).toContainText("Credit Card");
    });

    /**
     * TEST: First Row Amount
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌───────────┬───────────┬─────────────┬───────────┐
     *   │ INV001    │ Paid      │ Credit Card │ $250.00   │  ← Row 0
     *   │                                     │ ^^^^^^^   │
     *   │                                     │ Contains "$250.00"?
     *   ├───────────┼───────────┼─────────────┼───────────┤
     *   │ INV002    │ ...       │ ...         │ $150.00   │
     *   └───────────┴───────────┴─────────────┴───────────┘
     *
     *   Validates: First data row shows correct amount "$250.00"
     */
    test("first row should have $250.00 amount", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      const firstRow = ui.bodyRows.nth(0);
      await expect(firstRow).toContainText("$250.00");
    });
  });

  test.describe("Footer", () => {
    /**
     * TEST: Footer Total Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <tfoot>                                                │
     *   │    <tr>                                                 │
     *   │      <td colspan="3">Total</td>  ← Contains "Total"?    │
     *   │                      ^^^^^                              │
     *   │      <td>$2,500.00</td>                                 │
     *   │    </tr>                                                │
     *   │  </tfoot>                                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Footer displays "Total" label for summary row
     */
    test("footer should have Total label", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableFooter).toContainText("Total");
    });

    /**
     * TEST: Footer Total Amount
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <tfoot>                                                │
     *   │    <tr>                                                 │
     *   │      <td colspan="3">Total</td>                         │
     *   │      <td>$2,500.00</td>  ← Contains "$2,500.00"?        │
     *   │          ^^^^^^^^^^                                     │
     *   │    </tr>                                                │
     *   │  </tfoot>                                               │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Footer displays correct total sum of all invoices
     */
    test("footer should have total amount", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.tableFooter).toContainText("$2,500.00");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Full Width Styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │ <table class="w-full ...">                             │
     *   │        ^^^^^^^^^^^^                                    │
     *   │ ◄──────────── 100% width ─────────────►                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table spans full width of container (w-full class)
     */
    test("table should have w-full", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.table).toHaveClass(/w-full/);
    });

    /**
     * TEST: Caption Bottom Position
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │ <table class="caption-bottom ...">                     │
     *   │        ^^^^^^^^^^^^^^^                                 │
     *   │  ┌─────────────────────────────────────────────────┐   │
     *   │  │ Header Row                                      │   │
     *   │  │ Body Rows...                                    │   │
     *   │  │ Footer Row                                      │   │
     *   │  └─────────────────────────────────────────────────┘   │
     *   │  Caption appears HERE (bottom)                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Caption is positioned at bottom of table
     */
    test("table should have caption-bottom", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.table).toHaveClass(/caption-bottom/);
    });

    /**
     * TEST: Small Text Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │ <table class="text-sm ...">                            │
     *   │        ^^^^^^^^^                                       │
     *   │  ┌─────────────────────────────────────────────────┐   │
     *   │  │ Small font size text throughout table           │   │
     *   │  │ (0.875rem / 14px)                               │   │
     *   │  └─────────────────────────────────────────────────┘   │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Table uses small text size (text-sm) for compact display
     */
    test("table should have text-sm", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.table).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Header Font Weight
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │ <thead>                                                 │
     *   │   <th class="font-medium ...">Invoice</th>             │
     *   │             ^^^^^^^^^^^^                               │
     *   │   Bold-ish headers vs regular body text                │
     *   │ </thead>                                                │
     *   │ <tbody>                                                 │
     *   │   <td>INV001</td>  ← Normal weight                     │
     *   │ </tbody>                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Header cells use medium font weight for emphasis
     */
    test("header cells should have font-medium", async ({ page }) => {
      const ui = new TablePage(page);
      await ui.goto();

      await expect(ui.headerCells.first()).toHaveClass(/font-medium/);
    });
  });
});
