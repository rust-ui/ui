import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * AVATAR COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   [data-name="Avatar"]                                                  │
 * │   ┌─────────────────────────────────┐                                   │
 * │   │        ┌───────────┐            │                                   │
 * │   │        │           │            │  ← <img> AvatarImage (z-10)       │
 * │   │        │   Image   │            │    aspect-square, absolute        │
 * │   │        │           │            │                                   │
 * │   │        └───────────┘            │                                   │
 * │   │        ┌───────────┐            │                                   │
 * │   │        │    RS     │            │  ← AvatarFallback (below image)   │
 * │   │        │ (initials)│            │    Shows when image fails         │
 * │   │        └───────────┘            │                                   │
 * │   └─────────────────────────────────┘                                   │
 * │      rounded-full, overflow-hidden                                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * LAYER STACKING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Avatar Container (relative)                                           │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │                                                             │       │
 * │   │   Layer 2 (z-10): <img> AvatarImage                         │       │
 * │   │   ┌─────────────────────────────────────────────────┐       │       │
 * │   │   │                                                 │       │       │
 * │   │   │   Shows if image loads successfully             │       │       │
 * │   │   │                                                 │       │       │
 * │   │   └─────────────────────────────────────────────────┘       │       │
 * │   │                                                             │       │
 * │   │   Layer 1 (absolute): AvatarFallback                        │       │
 * │   │   ┌─────────────────────────────────────────────────┐       │       │
 * │   │   │                                                 │       │       │
 * │   │   │   Shows initials "RS" when image fails          │       │       │
 * │   │   │   bg-muted, centered content                    │       │       │
 * │   │   │                                                 │       │       │
 * │   │   └─────────────────────────────────────────────────┘       │       │
 * │   │                                                             │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SIZES:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Default (size-8 = 32px):    ┌────┐                                    │
 * │                               │ RS │                                    │
 * │                               └────┘                                    │
 * │                                                                         │
 * │   Can be customized via class prop                                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class AvatarPage extends BasePage {
  protected readonly componentName = "avatar";

  // Avatar container
  readonly avatar: Locator;

  // Avatar parts
  readonly avatarImage: Locator;
  readonly avatarFallback: Locator;

  constructor(page: Page) {
    super(page);

    // Main avatar - scoped within preview
    this.avatar = this.preview.locator('[data-name="Avatar"]').first();

    // Parts
    this.avatarImage = this.avatar.locator("img");
    this.avatarFallback = this.avatar.locator('[data-name="AvatarFallback"]');
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("Avatar Page", () => {
  /**
   * STRUCTURE TESTS - Verify component hierarchy
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Avatar (container)                                            │
   * │   ├── <img> AvatarImage (z-10, shows if loads)                  │
   * │   └── AvatarFallback (shows initials if image fails)            │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Structure", () => {
    /**
     * TEST: Avatar Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   [data-name="Avatar"]                                  │
     *   │   ┌────────────┐                                        │
     *   │   │            │                                        │
     *   │   │  MUST BE   │  ← Avatar container visible            │
     *   │   │  VISIBLE   │                                        │
     *   │   │            │                                        │
     *   │   └────────────┘                                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar container renders and is visible
     */
    test("should have Avatar container", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toBeVisible();
    });

    /**
     * TEST: Avatar Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span data-name="Avatar">                             │
     *   │          ├────────┬──────┤                              │
     *   │                   ↓                                     │
     *   │   Attribute must equal "Avatar"                         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar has correct data-name for identification
     */
    test("should have Avatar data-name attribute", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveAttribute("data-name", "Avatar");
    });

    /**
     * TEST: AvatarFallback Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Avatar Container                                      │
     *   │   ┌────────────┐                                        │
     *   │   │            │                                        │
     *   │   │     RS     │  ← AvatarFallback MUST BE VISIBLE      │
     *   │   │            │    (shows initials when image fails)   │
     *   │   └────────────┘                                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fallback element is present and visible
     */
    test("should have AvatarFallback", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarFallback).toBeVisible();
    });

    /**
     * TEST: AvatarImage Attached
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Avatar Container                                      │
     *   │   ┌────────────┐                                        │
     *   │   │ ┌────────┐ │                                        │
     *   │   │ │  <img> │ │  ← Image element MUST BE ATTACHED      │
     *   │   │ └────────┘ │    (in DOM, may or may not be visible) │
     *   │   │   (RS)     │                                        │
     *   │   └────────────┘                                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Image element exists in the DOM
     */
    test("should have AvatarImage", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarImage).toBeAttached();
    });
  });

  /**
   * FALLBACK BEHAVIOR TESTS - Image load failure handling
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   Image loads OK:           Image fails to load:                │
   * │   ┌────────────┐            ┌────────────┐                      │
   * │   │            │            │            │                      │
   * │   │   Photo    │            │     RS     │  ← Initials shown    │
   * │   │            │            │            │                      │
   * │   └────────────┘            └────────────┘                      │
   * │   (img visible)             (fallback visible)                  │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Fallback Behavior", () => {
    /**
     * TEST: Fallback Shows Initials on Image Load Failure
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Image Load Attempt:        After Load Failure:        │
     *   │   ┌────────────┐             ┌────────────┐             │
     *   │   │            │             │            │             │
     *   │   │  Loading.. │  ──────►    │     RS     │             │
     *   │   │            │   (fail)    │            │             │
     *   │   └────────────┘             └────────────┘             │
     *   │                                    ↑                    │
     *   │                              VISIBLE with text "RS"     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fallback displays initials when image fails to load
     */
    test("fallback should show initials when image fails to load", async ({
      page,
    }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      // Wait for image load error
      await page.waitForTimeout(500);

      // Fallback should be visible with initials
      await expect(ui.avatarFallback).toBeVisible();
      await expect(ui.avatarFallback).toHaveText("RS");
    });
  });

  /**
   * STYLING TESTS - Container and fallback appearance
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   AVATAR CONTAINER:                                             │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  .relative         ← Positioning context                │   │
   * │   │  .rounded-full     ← Circular shape                     │   │
   * │   │  .overflow-hidden  ← Clip children to circle            │   │
   * │   │  .size-8           ← 32x32px default                    │   │
   * │   │  .shrink-0         ← Prevent flex shrinking             │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * │   FALLBACK:                                                     │
   * │   ┌─────────────────────────────────────────────────────────┐   │
   * │   │  .absolute         ← Fill container                     │   │
   * │   │  .bg-muted         ← Muted background                   │   │
   * │   │  .flex .items-center .justify-center ← Center initials  │   │
   * │   └─────────────────────────────────────────────────────────┘   │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Styling", () => {
    /**
     * TEST: Avatar Circular Shape
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   rounded-none:     rounded-full:                       │
     *   │   ┌────────────┐    ╭────────────╮                      │
     *   │   │            │    │            │                      │
     *   │   │     RS     │    │     RS     │  ← Perfect circle    │
     *   │   │            │    │            │                      │
     *   │   └────────────┘    ╰────────────╯                      │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar has fully rounded (circular) shape
     */
    test("avatar should have rounded-full class", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveClass(/rounded-full/);
    });

    /**
     * TEST: Avatar Overflow Hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Without overflow-hidden:  With overflow-hidden:       │
     *   │   ╭────────────╮            ╭────────────╮              │
     *   │  ┌┤            ├┐           │            │              │
     *   │  ││   IMAGE    ││           │   IMAGE    │ ← Clipped    │
     *   │  ││            ││           │            │   to circle  │
     *   │  └┤            ├┘           ╰────────────╯              │
     *   │   ╰────────────╯                                        │
     *   │   (image bleeds out)                                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Content is clipped to circular boundary
     */
    test("avatar should have overflow-hidden", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveClass(/overflow-hidden/);
    });

    /**
     * TEST: Avatar Default Size
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   size-8 = 32px x 32px                                  │
     *   │                                                         │
     *   │   ┌────┐                                                │
     *   │   │ RS │  ← 32px width, 32px height                     │
     *   │   └────┘                                                │
     *   │   ←32px→                                                │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar has default 32x32px size
     */
    test("avatar should have size-8", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveClass(/size-8/);
    });

    /**
     * TEST: Avatar Relative Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <span style="position: relative">  ← Positioning ctx  │
     *   │   ╭────────────╮                                        │
     *   │   │ ┌────────┐ │  ← Image (absolute, z-10)              │
     *   │   │ │ img    │ │                                        │
     *   │   │ └────────┘ │                                        │
     *   │   │ ┌────────┐ │  ← Fallback (absolute)                 │
     *   │   │ │   RS   │ │                                        │
     *   │   │ └────────┘ │                                        │
     *   │   ╰────────────╯                                        │
     *   │   </span>                                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar has relative positioning for child layers
     */
    test("avatar should have relative positioning", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveClass(/relative/);
    });

    /**
     * TEST: Avatar No Shrink
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   In a flex container with limited space:               │
     *   │                                                         │
     *   │   Without shrink-0:        With shrink-0:               │
     *   │   ┌──────────────────┐     ┌──────────────────┐         │
     *   │   │ ╭──╮ Long text...│     │ ╭────╮ Long te...│         │
     *   │   │ ╰──╯             │     │ ╰────╯           │         │
     *   │   └──────────────────┘     └──────────────────┘         │
     *   │      ↑                         ↑                        │
     *   │   Shrinks!              Maintains size                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Avatar maintains size in flex layouts
     */
    test("avatar should have shrink-0", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatar).toHaveClass(/shrink-0/);
    });

    /**
     * TEST: Fallback Muted Background
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭────────────╮                                        │
     *   │   │░░░░░░░░░░░░│  ← bg-muted (subtle gray background)   │
     *   │   │░░░ RS ░░░░░│                                        │
     *   │   │░░░░░░░░░░░░│                                        │
     *   │   ╰────────────╯                                        │
     *   │                                                         │
     *   │   Provides visual distinction when showing fallback     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fallback has muted background color
     */
    test("fallback should have bg-muted", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarFallback).toHaveClass(/bg-muted/);
    });

    /**
     * TEST: Fallback Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Avatar (relative)                                     │
     *   │   ╭────────────╮                                        │
     *   │   │┌──────────┐│  ← Fallback: position: absolute        │
     *   │   ││          ││    Fills entire container              │
     *   │   ││    RS    ││    (inset: 0)                          │
     *   │   ││          ││                                        │
     *   │   │└──────────┘│                                        │
     *   │   ╰────────────╯                                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fallback fills container with absolute positioning
     */
    test("fallback should have absolute positioning", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarFallback).toHaveClass(/absolute/);
    });

    /**
     * TEST: Fallback Centered Content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   ╭────────────╮                                        │
     *   │   │            │                                        │
     *   │   │     RS     │  ← Initials centered both ways         │
     *   │   │            │    flex + items-center + justify-center│
     *   │   ╰────────────╯                                        │
     *   │                                                         │
     *   │   Without centering:    With centering:                 │
     *   │   ╭────────────╮        ╭────────────╮                  │
     *   │   │RS          │        │     RS     │                  │
     *   │   ╰────────────╯        ╰────────────╯                  │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Fallback initials are centered horizontally and vertically
     */
    test("fallback should have centered content", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarFallback).toHaveClass(/flex/);
      await expect(ui.avatarFallback).toHaveClass(/items-center/);
      await expect(ui.avatarFallback).toHaveClass(/justify-center/);
    });
  });

  /**
   * IMAGE TESTS - Avatar image element
   *
   * ┌─────────────────────────────────────────────────────────────────┐
   * │                                                                 │
   * │   <img class="aspect-square absolute z-10"                      │
   * │        alt="@rustify.rs"                                        │
   * │        src="..." />                                             │
   * │                                                                 │
   * │   • aspect-square  ← Maintains 1:1 ratio                        │
   * │   • absolute       ← Positioned over fallback                   │
   * │   • z-10           ← Above fallback layer                       │
   * │   • alt            ← Accessible label                           │
   * │                                                                 │
   * └─────────────────────────────────────────────────────────────────┘
   */
  test.describe("Image", () => {
    /**
     * TEST: Image Aspect Ratio
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   aspect-square: aspect-ratio: 1 / 1                    │
     *   │                                                         │
     *   │   Non-square:        Square:                            │
     *   │   ┌──────────────┐   ┌────────────┐                     │
     *   │   │              │   │            │                     │
     *   │   │   Stretched  │   │   Perfect  │  ← 1:1 ratio        │
     *   │   │              │   │            │                     │
     *   │   └──────────────┘   └────────────┘                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Image maintains square aspect ratio
     */
    test("image should have aspect-square", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarImage).toHaveClass(/aspect-square/);
    });

    /**
     * TEST: Image Absolute Positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Avatar (relative)                                     │
     *   │   ╭────────────╮                                        │
     *   │   │┌──────────┐│  ← Image: position: absolute           │
     *   │   ││          ││    Overlays the fallback               │
     *   │   ││  IMAGE   ││                                        │
     *   │   ││          ││                                        │
     *   │   │└──────────┘│                                        │
     *   │   │   (RS)     │  ← Fallback behind                     │
     *   │   ╰────────────╯                                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Image is positioned absolutely over fallback
     */
    test("image should have absolute positioning", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarImage).toHaveClass(/absolute/);
    });

    /**
     * TEST: Image Z-Index
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Layer Stack (side view):                              │
     *   │                                                         │
     *   │   z-10: ┌──────────┐  ← Image (top layer, visible)      │
     *   │         │  IMAGE   │                                    │
     *   │         └──────────┘                                    │
     *   │   z-0:  ┌──────────┐  ← Fallback (behind, hidden)       │
     *   │         │    RS    │                                    │
     *   │         └──────────┘                                    │
     *   │                                                         │
     *   │   When image loads, it appears on top of fallback       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Image has higher z-index than fallback
     */
    test("image should have z-10", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarImage).toHaveClass(/z-10/);
    });

    /**
     * TEST: Image Alt Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   <img alt="@rustify.rs" src="..." />                   │
     *   │        ├───────┬──────────┤                             │
     *   │                ↓                                        │
     *   │   Alt text must equal "@rustify.rs"                     │
     *   │                                                         │
     *   │   Accessibility benefits:                               │
     *   │   • Screen readers announce the alt text                │
     *   │   • Shows when image fails to load                      │
     *   │   • SEO improvement                                     │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Image has accessible alt attribute
     */
    test("image should have alt attribute", async ({ page }) => {
      const ui = new AvatarPage(page);
      await ui.goto();

      await expect(ui.avatarImage).toHaveAttribute("alt", "@rustify.rs");
    });
  });
});
