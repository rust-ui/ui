import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * TEXTAREA COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <textarea>                                                            │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  Type your message here.                 ← placeholder          │   │
 * │   │                                                                 │   │
 * │   │                                                                 │   │
 * │   │                                                                 │   │
 * │   │                                          ═══╗  ← resize handle  │   │
 * │   └──────────────────────────────────────────═══╝───────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STATES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default:                      Focused:                                │
 * │   ┌────────────────────┐        ┌════════════════════┐                  │
 * │   │                    │        ║                    ║  ← focus ring    │
 * │   │                    │        ║                    ║                  │
 * │   └────────────────────┘        └════════════════════┘                  │
 * │                                                                         │
 * │   Disabled:                     With Content:                           │
 * │   ┌────────────────────┐        ┌────────────────────┐                  │
 * │   │ ░░░░░░░░░░░░░░░░░░ │        │ Line 1             │                  │
 * │   │ ░░░░░░░░░░░░░░░░░░ │        │ Line 2             │                  │
 * │   └────────────────────┘        │ Line 3             │                  │
 * │   opacity: 0.5                  └────────────────────┘                  │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .w-full       ← Full width                                 │       │
 * │   │  .min-h-[60px] ← Minimum height 60px                        │       │
 * │   │  .border       ← Border styling                             │       │
 * │   │  .rounded-md   ← Medium border radius                       │       │
 * │   │  .bg-background← Background color                           │       │
 * │   │  .px-3 .py-2   ← Padding                                    │       │
 * │   │  .text-sm      ← Small text size                            │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class TextareaPage extends BasePage {
  protected readonly componentName = "textarea";

  // Textarea element
  readonly textarea: Locator;

  constructor(page: Page) {
    super(page);

    // Main textarea - scoped within preview
    this.textarea = this.preview.locator("textarea").first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Textarea Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Textarea Element Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <textarea>                                            │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Type your message here.                        │   │
     *   │   │                                                 │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </textarea>                                           │
     *   │                                                         │
     *   │   Check: Element is visible on page                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea element is rendered and visible
     */
    test("should have textarea element", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toBeVisible();
    });

    /**
     * TEST: Textarea Is Correct HTML Element
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <textarea>   <-- tagName === "textarea"               │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │    Native multiline text input element          │   │
     *   │   │    (not <input type="text"> or <div>)           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </textarea>                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Uses native textarea HTML element
     */
    test("should be a textarea HTML element", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      const tagName = await ui.textarea.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("textarea");
    });

    /**
     * TEST: Textarea Has Placeholder Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <textarea placeholder="Type your message here.">      │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │   │
     *   │   │  Type your message here.     <-- placeholder    │   │
     *   │   │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │   </textarea>                                           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Placeholder attribute guides user input
     */
    test("should have placeholder text", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveAttribute(
        "placeholder",
        "Type your message here."
      );
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Textarea Has Border Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .border = border-width: 1px                           │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ╔═════════════════════════════════════════════╗ │   │
     *   │   │ ║                                             ║ │   │
     *   │   │ ║        Border around textarea               ║ │   │
     *   │   │ ║                                             ║ │   │
     *   │   │ ╚═════════════════════════════════════════════╝ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea has visible border
     */
    test("should have border", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/border/);
    });

    /**
     * TEST: Textarea Has rounded-md Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .rounded-md = border-radius: 0.375rem (6px)           │
     *   │                                                         │
     *   │   ╭─────────────────────────────────────────────────╮   │
     *   │   │                                                 │   │
     *   │   │        Rounded corners (medium)                 │   │
     *   │   │                                                 │   │
     *   │   ╰─────────────────────────────────────────────────╯   │
     *   │    ↑                                               ↑    │
     *   │    └──── rounded corners on all sides ─────────────┘    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea has medium border radius
     */
    test("should have rounded-md", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Textarea Has bg-background Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .bg-background = background-color: var(--background)  │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │   │
     *   │   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │   │
     *   │   │ ▓▓▓▓▓   Theme-aware background color   ▓▓▓▓▓▓▓ │   │
     *   │   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea uses theme background color
     */
    test("should have bg-background", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/bg-background/);
    });

    /**
     * TEST: Textarea Has text-sm Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .text-sm = font-size: 0.875rem (14px)                 │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Type your message here.                        │   │
     *   │   │  ^^^^^^^^^^^^^^^^^^^^^^^^                       │   │
     *   │   │        14px font size                           │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea uses small text size
     */
    test("should have text-sm", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/text-sm/);
    });

    /**
     * TEST: Textarea Has min-h-[60px] Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   min-h-[60px] = min-height: 60px                       │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │                                                 │   │
     *   │   │                                                 │ 60px
     *   │   │        Minimum height guaranteed                │ min
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   Can grow taller but never shorter than 60px           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea has minimum height constraint
     */
    test("should have min-h-[60px]", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/min-h-\[60px\]/);
    });

    /**
     * TEST: Textarea Has w-full Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .w-full = width: 100%                                 │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │◄──────────────── 100% width ──────────────────►│   │
     *   │   │                                                 │   │
     *   │   │       Fills entire container width              │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea spans full container width
     */
    test("should have w-full", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/w-full/);
    });

    /**
     * TEST: Textarea Has Padding Classes
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   px-3 = padding-left/right: 0.75rem (12px)             │
     *   │   py-2 = padding-top/bottom: 0.5rem (8px)               │
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │ ┌─────────────────────────────────────────────┐ │   │
     *   │   │ │  ↑ 8px (py-2)                               │ │   │
     *   │   │ │←12px→  Text content area           ←12px→│ │   │
     *   │   │ │  ↓ 8px                                      │ │   │
     *   │   │ └─────────────────────────────────────────────┘ │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea has proper internal padding
     */
    test("should have padding", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/px-3/);
      await expect(ui.textarea).toHaveClass(/py-2/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: Textarea Is Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before focus:              After focus:               │
     *   │   ┌────────────────────┐     ╔════════════════════╗     │
     *   │   │                    │     ║                    ║     │
     *   │   │                    │ --> ║    |               ║     │
     *   │   │                    │     ║   cursor           ║     │
     *   │   └────────────────────┘     ╚════════════════════╝     │
     *   │                                   focus ring            │
     *   │                                                         │
     *   │   Check: textarea.focus() then isFocused()              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea can receive keyboard focus
     */
    test("should be focusable", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await ui.textarea.focus();
      await expect(ui.textarea).toBeFocused();
    });

    /**
     * TEST: Textarea Accepts Text Input
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Before:                    After fill():              │
     *   │   ┌────────────────────┐     ┌────────────────────┐     │
     *   │   │ Type your message  │     │ Hello, World!      │     │
     *   │   │ here. (placeholder)│ --> │                    │     │
     *   │   │                    │     │                    │     │
     *   │   └────────────────────┘     └────────────────────┘     │
     *   │                                                         │
     *   │   fill("Hello, World!") -> value === "Hello, World!"    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User can type text into textarea
     */
    test("should be able to type text", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await ui.textarea.fill("Hello, World!");
      await expect(ui.textarea).toHaveValue("Hello, World!");
    });

    /**
     * TEST: Textarea Can Be Cleared
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Step 1: Fill          Step 2: Clear                   │
     *   │   ┌────────────────┐    ┌────────────────┐              │
     *   │   │ Some text      │    │                │              │
     *   │   │                │ -> │                │              │
     *   │   └────────────────┘    └────────────────┘              │
     *   │                                                         │
     *   │   fill("Some text") -> clear() -> value === ""          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea content can be cleared
     */
    test("should be able to clear text", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await ui.textarea.fill("Some text");
      await ui.textarea.clear();
      await expect(ui.textarea).toHaveValue("");
    });

    /**
     * TEST: Textarea Accepts Multiline Text
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ┌─────────────────────────────────────────────────┐   │
     *   │   │  Line 1                                         │   │
     *   │   │  Line 2              <-- Supports newlines      │   │
     *   │   │  Line 3                                         │   │
     *   │   │                                                 │   │
     *   │   └─────────────────────────────────────────────────┘   │
     *   │                                                         │
     *   │   fill("Line 1\nLine 2\nLine 3")                        │
     *   │   value === "Line 1\nLine 2\nLine 3"                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Textarea handles multiple lines of text
     */
    test("should accept multiline text", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await ui.textarea.fill("Line 1\nLine 2\nLine 3");
      await expect(ui.textarea).toHaveValue("Line 1\nLine 2\nLine 3");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Textarea Has Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .focus-visible:ring = focus ring on keyboard focus    │
     *   │                                                         │
     *   │   Unfocused:               Focused (keyboard):          │
     *   │   ┌────────────────────┐   ╔════════════════════╗       │
     *   │   │                    │   ║ ┌────────────────┐ ║       │
     *   │   │                    │   ║ │                │ ║       │
     *   │   │                    │   ║ │                │ ║       │
     *   │   └────────────────────┘   ║ └────────────────┘ ║       │
     *   │                            ╚════════════════════╝       │
     *   │                                 ↑ focus ring            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Focus state is visible for keyboard users
     */
    test("should have focus ring styles", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/focus-visible:ring/);
    });

    /**
     * TEST: Textarea Has Disabled Styles Class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   .disabled:opacity-50 = 50% opacity when disabled      │
     *   │                                                         │
     *   │   Enabled:                 Disabled:                    │
     *   │   ┌────────────────────┐   ┌────────────────────┐       │
     *   │   │ ██████████████████ │   │ ░░░░░░░░░░░░░░░░░░ │       │
     *   │   │ ██████████████████ │   │ ░░░░░░░░░░░░░░░░░░ │       │
     *   │   │ ██████████████████ │   │ ░░░░░░░░░░░░░░░░░░ │       │
     *   │   └────────────────────┘   └────────────────────┘       │
     *   │   opacity: 1               opacity: 0.5                 │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Disabled state is visually indicated
     */
    test("should have disabled styles class", async ({ page }) => {
      const ui = new TextareaPage(page);
      await ui.goto();

      await expect(ui.textarea).toHaveClass(/disabled:opacity-50/);
    });
  });
});
