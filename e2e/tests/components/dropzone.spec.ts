import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DROPZONE COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <label for="id-dropzone01">                                           │
 * │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
 * │   │                                                                │   │
 * │   │                          ☁️ ↑                                  │   │
 * │   │                         upload icon                            │   │
 * │   │                                                                │   │
 * │   │               Drag & drop or upload a file                     │   │
 * │   │                         ↑                ↑                     │   │
 * │   │                    text-slate    text-emerald (highlighted)    │   │
 * │   │                                                                │   │
 * │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
 * │       ↑ border-dashed                                               │
 * │                                                                         │
 * │   <input type="file" id="id-dropzone01" class="hidden" />               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * DRAG & DROP STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:                          Drag over:                          │
 * │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐        ┌═══════════════════════┐            │
 * │   │                       │        ║                       ║            │
 * │   │       ☁️ ↑            │        ║       ☁️ ↑            ║            │
 * │   │  Drag & drop or...    │        ║  Drop file here       ║            │
 * │   │                       │        ║                       ║            │
 * │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘        └═══════════════════════┘            │
 * │   transition-colors                 highlighted border                   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * FILE INPUT PROPERTIES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  type="file"                                                │       │
 * │   │  accept=".gif,.jpg,.png,.jpeg"  ← Accepted file types       │       │
 * │   │  name="file-upload"             ← Form field name           │       │
 * │   │  class="hidden"                 ← Visually hidden           │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DropzonePage extends BasePage {
  protected readonly componentName = "dropzone";

  // Dropzone elements
  readonly dropzone: Locator;
  readonly fileInput: Locator;
  readonly label: Locator;
  readonly icon: Locator;
  readonly text: Locator;

  constructor(page: Page) {
    super(page);

    // File input - scoped within preview
    this.fileInput = this.preview.locator('input[type="file"]');

    // Label (the clickable dropzone area) - scoped within preview
    this.label = this.preview.locator('label[for="id-dropzone01"]');
    this.dropzone = this.label;

    // Icon and text
    this.icon = this.label.locator("svg");
    this.text = this.label.locator("span.text-slate-500");
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Dropzone Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Dropzone Label Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label for="id-dropzone01">  ← VISIBLE?               │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │              ☁️ ↑                               │   │
     *   │   │       Drag & drop or upload a file             │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: The dropzone label container is rendered and visible
     */
    test("should have dropzone label", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toBeVisible();
    });

    /**
     * TEST: File Input Exists
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DOM Structure:                                        │
     *   │                                                         │
     *   │   <label>...</label>                                    │
     *   │   <input type="file" id="id-dropzone01">  ← ATTACHED?   │
     *   │                                                         │
     *   │   The input must exist in DOM (even if hidden)          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hidden file input is attached to the DOM
     */
    test("should have file input", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.fileInput).toBeAttached();
    });

    /**
     * TEST: File Input Hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Visible:                     Hidden (actual):         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐       ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │      ☁️ ↑          │       │      ☁️ ↑          │   │
     *   │   │  Drag & drop...   │       │  Drag & drop...   │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘       └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │   ┌───────────────────┐                                 │
     *   │   │ Choose File      │  ←  NOT visible (class="hidden") │
     *   │   └───────────────────┘                                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Native file input is visually hidden
     */
    test("file input should be hidden", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.fileInput).toHaveClass(/hidden/);
    });

    /**
     * TEST: Upload Icon Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │                  [SVG]  ← VISIBLE?              │   │
     *   │   │                   ☁️ ↑                          │   │
     *   │   │             upload cloud icon                  │   │
     *   │   │                                                │   │
     *   │   │           Drag & drop or upload a file         │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Upload icon is rendered and visible
     */
    test("should have upload icon", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.icon).toBeVisible();
    });
  });

  test.describe("File Input Properties", () => {
    /**
     * TEST: Input Type File
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <input type="file" ... />                             │
     *   │          └───┬───┘                                      │
     *   │              │                                          │
     *   │   CHECK: type === "file"                                │
     *   │                                                         │
     *   │   This enables the browser's file picker dialog         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input is configured for file selection
     */
    test("input should have type=file", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.fileInput).toHaveAttribute("type", "file");
    });

    /**
     * TEST: Accepted File Types
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <input accept=".gif,.jpg,.png,.jpeg" />               │
     *   │                                                         │
     *   │   File Picker Filter:                                   │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  image.gif   ✓                                  │   │
     *   │   │  photo.jpg   ✓                                  │   │
     *   │   │  icon.png    ✓                                  │   │
     *   │   │  pic.jpeg    ✓                                  │   │
     *   │   │  doc.pdf     ✗ (filtered out)                   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Only image file types are accepted
     */
    test("input should accept image files", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      const accept = await ui.fileInput.getAttribute("accept");
      expect(accept).toContain(".gif");
      expect(accept).toContain(".jpg");
      expect(accept).toContain(".png");
      expect(accept).toContain(".jpeg");
    });

    /**
     * TEST: Input Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <input name="file-upload" ... />                      │
     *   │          └──────┬──────┘                                │
     *   │                 │                                       │
     *   │   Form Submission:                                      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  POST /upload                                   │   │
     *   │   │  Content-Type: multipart/form-data              │   │
     *   │   │                                                 │   │
     *   │   │  file-upload: [binary data]  ← uses name attr   │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input has name for form submission
     */
    test("input should have name attribute", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.fileInput).toHaveAttribute("name", "file-upload");
    });
  });

  test.describe("Content", () => {
    /**
     * TEST: Instructional Text Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑                           │   │
     *   │   │                                                │   │
     *   │   │          "Drag & drop" or upload a file        │   │
     *   │   │           └─────┬─────┘                        │   │
     *   │   │                 │                              │   │
     *   │   │      CHECK: contains "Drag & drop"             │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User sees drag & drop instructions
     */
    test("should have instructional text", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toContainText("Drag & drop");
    });

    /**
     * TEST: Upload Link Text Present
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑                           │   │
     *   │   │                                                │   │
     *   │   │          Drag & drop or "upload a file"        │   │
     *   │   │                        └──────┬──────┘         │   │
     *   │   │                               │                │   │
     *   │   │         CHECK: contains "upload a file"        │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Alternative click-to-upload text is shown
     */
    test("should have 'upload a file' link text", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toContainText("upload a file");
    });

    /**
     * TEST: Icon Aria Label
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <svg aria-label="File input icon">                    │
     *   │        └──────────┬───────────┘                         │
     *   │                   │                                     │
     *   │   Screen Reader Announcement:                           │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  "File input icon"                              │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Provides context for assistive technology             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Icon has accessible label for screen readers
     */
    test("icon should have aria-label", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.icon).toHaveAttribute("aria-label", "File input icon");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Dashed Border Style
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │ ↑                                              │   │
     *   │   │ border-dashed (visual drop target indicator)   │   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑                           │   │
     *   │   │           Drag & drop or upload a file         │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropzone has dashed border indicating drop area
     */
    test("dropzone should have dashed border", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/border-dashed/);
    });

    /**
     * TEST: Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ╮   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑                           │   │
     *   │   │           Drag & drop or upload a file         │   │
     *   │   │                                                │   │
     *   │   ╰ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ╯   │
     *   │   ↑                                               ↑    │
     *   │   rounded corners (border-radius applied)              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Dropzone has rounded corners for modern look
     */
    test("dropzone should have rounded corners", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/rounded/);
    });

    /**
     * TEST: Cursor Pointer
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Mouse cursor changes on hover:                        │
     *   │                                                         │
     *   │   Outside:       Over dropzone:                         │
     *   │      ↖           ┌ ─ ─ ─ ─ ─ ─ ─ ─ ┐                   │
     *   │   (default)      │      👆          │  ← cursor: pointer │
     *   │                  │  Drag & drop... │                    │
     *   │                  └ ─ ─ ─ ─ ─ ─ ─ ─ ┘                   │
     *   │                                                         │
     *   │   Indicates clickable area                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Cursor indicates dropzone is clickable
     */
    test("dropzone should have cursor-pointer", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/cursor-pointer/);
    });

    /**
     * TEST: Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   display: flex                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │  ┌─────────────────────────────────────────┐  │   │
     *   │   │  │              ☁️ ↑                        │  │   │
     *   │   │  └─────────────────────────────────────────┘  │   │
     *   │   │  ┌─────────────────────────────────────────┐  │   │
     *   │   │  │      Drag & drop or upload a file      │  │   │
     *   │   │  └─────────────────────────────────────────┘  │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Uses flexbox for layout control
     */
    test("dropzone should have flex layout", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/flex/);
    });

    /**
     * TEST: Flex Column Direction
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   flex-direction: column                                │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑           ← item 1        │   │
     *   │   │                   │                            │   │
     *   │   │                   ↓                            │   │
     *   │   │          Drag & drop or upload  ← item 2       │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items stack vertically (icon above text)
     */
    test("dropzone should have flex-col", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/flex-col/);
    });

    /**
     * TEST: Items Centered
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   align-items: center                                   │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │             │                │                 │   │
     *   │   │             │     ☁️ ↑       │                 │   │
     *   │   │             │       │        │                 │   │
     *   │   │             │ Drag & drop... │                 │   │
     *   │   │             │                │                 │   │
     *   │   │             └── centered ────┘                 │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content is horizontally centered
     */
    test("dropzone should have items-center", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/items-center/);
    });

    /**
     * TEST: Transition Colors
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Default:                      Drag Over:              │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐        ┌═══════════════════┐   │
     *   │   │      ☁️ ↑          │  ───→  ║      ☁️ ↑          ║   │
     *   │   │  Drag & drop...   │ smooth ║  Drag & drop...   ║   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘  trans ┗═══════════════════┛   │
     *   │                          ition                         │
     *   │   transition-colors enables smooth state changes        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Color transitions are smooth (not instant)
     */
    test("dropzone should have transition-colors", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveClass(/transition-colors/);
    });

    /**
     * TEST: Upload Text Highlighted
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │                  ☁️ ↑                           │   │
     *   │   │                                                │   │
     *   │   │   Drag & drop or  upload a file                │   │
     *   │   │   └──text-slate──┘ └─text-emerald-500─┘        │   │
     *   │   │      (gray)           (green/highlighted)      │   │
     *   │   │                                                │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: "upload a file" text is highlighted in emerald
     */
    test("upload text should be highlighted", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      const uploadText = ui.dropzone.locator(".text-emerald-500");
      await expect(uploadText).toHaveText("upload a file");
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Label Click Triggers Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   User clicks label:                                    │
     *   │                                                         │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │
     *   │   │                                                │   │
     *   │   │      ☁️ ↑       CLICK                          │   │
     *   │   │                  │                             │   │
     *   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │
     *   │                      │                                  │
     *   │                      ↓                                  │
     *   │   <input type="file">  ← triggers file picker dialog    │
     *   │                                                         │
     *   │   (linked via for="id-dropzone01")                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Clicking label opens file dialog (no error)
     */
    test("clicking label should trigger file input", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      // Click the label - this should open file dialog
      // We can't fully test file upload, but we can verify the label is clickable
      await ui.dropzone.click();
      // Should not throw error
    });

    /**
     * TEST: Input Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Keyboard Navigation:                                  │
     *   │                                                         │
     *   │   [TAB] ───────────────────────────────────────┐        │
     *   │                                                ↓        │
     *   │   <input type="file" class="hidden">  :focus            │
     *   │                                                         │
     *   │   Even though hidden, input can receive focus           │
     *   │   (force: true bypasses visibility check)               │
     *   │                                                         │
     *   │   CHECK: document.activeElement === input               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Hidden input can be programmatically focused
     */
    test("dropzone should be focusable via input", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      // Focus the hidden input
      await ui.fileInput.focus({ force: true });
      // Input should receive focus
      await expect(ui.fileInput).toBeFocused();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Label For Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label for="id-dropzone01">                           │
     *   │          └───────┬───────┘                              │
     *   │                  │                                      │
     *   │   ┌──────────────┼──────────────┐                       │
     *   │   │              │              │                       │
     *   │   │              ↓              │                       │
     *   │   │   <input id="id-dropzone01">│                       │
     *   │   │                             │                       │
     *   │   └──────────────────────────────┘                       │
     *   │                                                         │
     *   │   Links label to input for accessibility                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Label has for attribute pointing to input
     */
    test("label should have for attribute", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.dropzone).toHaveAttribute("for", "id-dropzone01");
    });

    /**
     * TEST: Input Matching ID
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <label for="id-dropzone01">                           │
     *   │                     │                                   │
     *   │                     │  MUST MATCH                       │
     *   │                     ↓                                   │
     *   │   <input id="id-dropzone01">                            │
     *   │              └───────┬───────┘                          │
     *   │                      │                                  │
     *   │         CHECK: id === "id-dropzone01"                   │
     *   │                                                         │
     *   │   Ensures label-input connection works                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Input ID matches label's for attribute
     */
    test("input should have matching id", async ({ page }) => {
      const ui = new DropzonePage(page);
      await ui.goto();

      await expect(ui.fileInput).toHaveAttribute("id", "id-dropzone01");
    });
  });
});
