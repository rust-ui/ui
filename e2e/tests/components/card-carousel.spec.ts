import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * CARD-CAROUSEL COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   CardCarousel                                                          │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  ┌─────────────────────────────────────────────────────────┐    │   │
 * │   │  │                                                         │    │   │
 * │   │  │                      IMAGE                              │    │   │
 * │   │  │                                                         │    │   │
 * │   │  │  ◀                                                 ▶    │    │   │
 * │   │  │  ↑ NavButton                               NavButton ↑  │    │   │
 * │   │  │                                                         │    │   │
 * │   │  │                    ● ○ ○ ○ ○                            │    │   │
 * │   │  │                    ↑ Indicators                         │    │   │
 * │   │  └─────────────────────────────────────────────────────────┘    │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * │   CardContent                                                           │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │  MV, Maldives                           ← CardTitle             │   │
 * │   │  4,843 kilometers away                  ← CardDescription       │   │
 * │   │  Aug 1 – 6                                                      │   │
 * │   │  $685 per night                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * CAROUSEL TRACK:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │
 * │   │ Slide 1 │ │ Slide 2 │ │ Slide 3 │ │ Slide 4 │ │ Slide 5 │           │
 * │   │  IMG    │ │  IMG    │ │  IMG    │ │  IMG    │ │  IMG    │           │
 * │   └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘           │
 * │        ↑                                                                │
 * │   Currently visible                                                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * NAVIGATION:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Click ◀ (Left):        Click ▶ (Right):                               │
 * │   Go to previous slide   Go to next slide                               │
 * │                                                                         │
 * │   Click indicator ○:                                                    │
 * │   Jump to specific slide                                                │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class CardCarouselPage extends BasePage {
  protected readonly componentName = "card-carousel";

  // Carousel elements
  readonly carousel: Locator;
  readonly carouselOverlay: Locator;
  readonly carouselNav: Locator;
  readonly navButtonLeft: Locator;
  readonly navButtonRight: Locator;
  readonly indicators: Locator;
  readonly indicatorDots: Locator;
  readonly carouselTrack: Locator;
  readonly slides: Locator;
  readonly images: Locator;

  // Card content
  readonly cardContent: Locator;
  readonly cardTitle: Locator;
  readonly cardDescription: Locator;

  constructor(page: Page) {
    super(page);

    // All locators scoped within preview
    this.carousel = this.preview.locator('[data-name="CardCarousel"]').first();
    this.carouselOverlay = this.preview.locator('[data-name="CardCarouselOverlay"]').first();
    this.carouselNav = this.preview.locator('[data-name="CardCarouselNav"]').first();
    this.navButtonLeft = this.preview.locator('[data-name="CardCarouselNavButton"]').first();
    this.navButtonRight = this.preview.locator('[data-name="CardCarouselNavButton"]').nth(1);
    this.indicators = this.preview.locator('[data-name="CardCarouselIndicators"]').first();
    this.indicatorDots = this.preview.locator('[data-name="CardCarouselIndicator"]');
    this.carouselTrack = this.preview.locator('[data-name="CardCarouselTrack"]').first();
    this.slides = this.preview.locator('[data-name="CardCarouselSlide"]');
    this.images = this.preview.locator('[data-name="CardCarouselImage"]');

    this.cardContent = this.preview.locator('[data-name="CardContent"]').first();
    this.cardTitle = this.preview.locator('[data-name="CardTitle"]').first();
    this.cardDescription = this.preview.locator('[data-name="CardDescription"]').first();
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("CardCarousel Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have carousel container
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel [data-name="CardCarousel"]               │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   (overlay, nav, track, indicators inside)        │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main carousel container element exists and is visible
     */
    test("should have carousel container", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carousel).toBeVisible();
    });

    /**
     * TEST: Should have carousel overlay
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  CardCarouselOverlay                              │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │   (semi-transparent overlay for nav/UI)     │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │         ↑                                         │  │
     *   │  │   CardCarouselOverlay - VISIBLE?                  │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Carousel overlay element is visible
     */
    test("should have carousel overlay", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carouselOverlay).toBeVisible();
    });

    /**
     * TEST: Should have carousel nav
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: hover()                                 │  │
     *   │  │   ─────────────────────────>                      │  │
     *   │  │                                                   │  │
     *   │  │   ◀        IMAGE          ▶   <── CardCarouselNav │  │
     *   │  │                                   (shows on hover) │  │
     *   │  │                                                   │  │
     *   │  │   Step 2: expect nav toBeVisible()                │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Navigation container appears on hover
     */
    test("should have carousel nav", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      // Nav is hidden until hover
      await ui.carousel.hover();
      await expect(ui.carouselNav).toBeVisible();
    });

    /**
     * TEST: Should have carousel track
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarouselTrack                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐          │  │
     *   │  │  │ S1  │ │ S2  │ │ S3  │ │ S4  │ │ S5  │          │  │
     *   │  │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘          │  │
     *   │  │     ↑                                              │  │
     *   │  │  visible                                           │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │         ↑                                               │
     *   │   CardCarouselTrack - VISIBLE?                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Carousel track (slide container) is visible
     */
    test("should have carousel track", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carouselTrack).toBeVisible();
    });
  });

  test.describe("Navigation Buttons", () => {
    /**
     * TEST: Should have left nav button (hidden when disabled)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel (on first slide)                         │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   ┌───┐                                 ┌───┐     │  │
     *   │  │   │ ◀ │ <── hidden/disabled             │ ▶ │     │  │
     *   │  │   └───┘    (aria-disabled)              └───┘     │  │
     *   │  │      ↑                                            │  │
     *   │  │   NavButtonLeft - toBeAttached() (exists in DOM)  │  │
     *   │  │   but hidden on first slide                       │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Left nav button exists (attached to DOM)
     */
    test("should have left nav button (hidden when disabled)", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      // Left nav button exists but is hidden when on first slide (aria-disabled)
      await ui.carousel.hover();
      await expect(ui.navButtonLeft).toBeAttached();
    });

    /**
     * TEST: Should have right nav button
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │   Step 1: hover()                                 │  │
     *   │  │                                                   │  │
     *   │  │                                         ┌───┐     │  │
     *   │  │             IMAGE                       │ ▶ │     │  │
     *   │  │                                         └───┘     │  │
     *   │  │                                            ↑      │  │
     *   │  │                            NavButtonRight VISIBLE │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Right nav button is visible on hover
     */
    test("should have right nav button", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await ui.carousel.hover();
      await expect(ui.navButtonRight).toBeVisible();
    });

    /**
     * TEST: Right nav button should have chevron icon
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   NavButtonRight                                        │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │                             │                       │
     *   │   │   ┌─────────────────────┐   │                       │
     *   │   │   │    <svg>            │   │                       │
     *   │   │   │      ▶ (chevron)    │   │ <── svg icon inside   │
     *   │   │   │    </svg>           │   │                       │
     *   │   │   └─────────────────────┘   │                       │
     *   │   │                             │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Right nav button contains an SVG chevron icon
     */
    test("right nav button should have chevron icon", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await ui.carousel.hover();
      await expect(ui.navButtonRight.locator("svg")).toBeVisible();
    });
  });

  test.describe("Indicators", () => {
    /**
     * TEST: Should have indicators container
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │                    IMAGE                          │  │
     *   │  │                                                   │  │
     *   │  │              ┌─────────────────┐                  │  │
     *   │  │              │  ● ○ ○ ○ ○      │                  │  │
     *   │  │              └─────────────────┘                  │  │
     *   │  │                      ↑                            │  │
     *   │  │         CardCarouselIndicators - VISIBLE?         │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Indicators container element is visible
     */
    test("should have indicators container", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.indicators).toBeVisible();
    });

    /**
     * TEST: Should have multiple indicator dots
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarouselIndicators                                │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │      ●       ○       ○       ○       ○            │ │
     *   │   │     [0]     [1]     [2]     [3]     [4]           │ │
     *   │   │                                                   │ │
     *   │   │   count > 1 ?                                     │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 1 indicator dot exists
     */
    test("should have multiple indicator dots", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const count = await ui.indicatorDots.count();
      expect(count).toBeGreaterThan(1);
    });

    /**
     * TEST: Should have 5 indicator dots for 5 images
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   5 Slides = 5 Indicator Dots                           │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │      ●       ○       ○       ○       ○            │ │
     *   │   │      1       2       3       4       5            │ │
     *   │   │                                                   │ │
     *   │   │   count === 5 ?                                   │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   1:1 mapping between slides and indicator dots         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 5 indicator dots (one per image)
     */
    test("should have 5 indicator dots for 5 images", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const count = await ui.indicatorDots.count();
      expect(count).toBe(5);
    });
  });

  test.describe("Slides", () => {
    /**
     * TEST: Should have multiple slides
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarouselTrack                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐          │  │
     *   │  │  │Slide│ │Slide│ │Slide│ │Slide│ │Slide│          │  │
     *   │  │  │  1  │ │  2  │ │  3  │ │  4  │ │  5  │          │  │
     *   │  │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘          │  │
     *   │  │                                                   │  │
     *   │  │  CardCarouselSlide count > 1 ?                    │  │
     *   │  │                                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 1 slide exists in the carousel
     */
    test("should have multiple slides", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const count = await ui.slides.count();
      expect(count).toBeGreaterThan(1);
    });

    /**
     * TEST: Should have images in slides
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarouselSlide                                     │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │                                             │  │  │
     *   │  │  │   ┌─────────────────────────────────────┐   │  │  │
     *   │  │  │   │        CardCarouselImage            │   │  │  │
     *   │  │  │   │            <img>                    │   │  │  │
     *   │  │  │   └─────────────────────────────────────┘   │  │  │
     *   │  │  │                                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │                                                   │  │
     *   │  │  CardCarouselImage count > 1 ?                    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: More than 1 image exists across slides
     */
    test("should have images in slides", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const count = await ui.images.count();
      expect(count).toBeGreaterThan(1);
    });

    /**
     * TEST: Images should have src attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarouselImage                                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   <img src="/images/maldives-1.jpg" />            │ │
     *   │   │                  ↑                                │ │
     *   │   │            src attribute exists?                  │ │
     *   │   │            src matches /.+/ ?                     │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Image must have a valid source URL                    │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First image has a non-empty src attribute
     */
    test("images should have src attribute", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const firstImage = ui.images.first();
      await expect(firstImage).toHaveAttribute("src", /.+/);
    });

    /**
     * TEST: Images should have alt attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarouselImage                                     │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   <img src="..." alt="Maldives beach" />          │ │
     *   │   │                        ↑                          │ │
     *   │   │                 alt attribute exists?             │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   │   Accessibility: Screen readers need alt text           │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First image has alt attribute for accessibility
     */
    test("images should have alt attribute", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const firstImage = ui.images.first();
      await expect(firstImage).toHaveAttribute("alt");
    });
  });

  test.describe("Card Content", () => {
    /**
     * TEST: Should have card content
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardCarousel                                          │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │         (image carousel above)                    │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │                                                         │
     *   │  CardContent [data-name="CardContent"]                 │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │  MV, Maldives                                     │  │
     *   │  │  4,843 kilometers away                            │  │
     *   │  │  Aug 1 - 6                                        │  │
     *   │  │  $685 per night                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   │         ↑                                               │
     *   │   CardContent - VISIBLE?                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card content section below carousel is visible
     */
    test("should have card content", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.cardContent).toBeVisible();
    });

    /**
     * TEST: Should have card title
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardContent                                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  MV, Maldives                               │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │         ↑                                         │  │
     *   │  │   CardTitle - contains "Maldives"?                │  │
     *   │  │                                                   │  │
     *   │  │  4,843 kilometers away                            │  │
     *   │  │  Aug 1 - 6                                        │  │
     *   │  │  $685 per night                                   │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Card title is visible and contains "Maldives"
     */
    test("should have card title", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.cardTitle).toBeVisible();
      await expect(ui.cardTitle).toContainText("Maldives");
    });

    /**
     * TEST: Should have card descriptions
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  CardContent                                           │
     *   │  ┌───────────────────────────────────────────────────┐  │
     *   │  │                                                   │  │
     *   │  │  MV, Maldives                                     │  │
     *   │  │                                                   │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  4,843 kilometers away                      │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  Aug 1 - 6                                  │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │  ┌─────────────────────────────────────────────┐  │  │
     *   │  │  │  $685 per night                             │  │  │
     *   │  │  └─────────────────────────────────────────────┘  │  │
     *   │  │         ↑                                         │  │
     *   │  │   CardDescription count > 0 ?                     │  │
     *   │  └───────────────────────────────────────────────────┘  │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: At least one CardDescription element exists
     */
    test("should have card descriptions", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      const descriptions = ui.byDataName("CardDescription");
      const count = await descriptions.count();
      expect(count).toBeGreaterThan(0);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Carousel should have relative positioning
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarousel                                          │
     *   │   ┌───────────────────────────────────────────────────┐ │
     *   │   │                                                   │ │
     *   │   │   position: relative;  <── class="relative"       │ │
     *   │   │                                                   │ │
     *   │   │   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐   │ │
     *   │   │     Absolutely positioned children:              │ │
     *   │   │   │   - Overlay                               │   │ │
     *   │   │       - Navigation buttons                        │ │
     *   │   │   │   - Indicators                            │   │ │
     *   │   │   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   │ │
     *   │   │                                                   │ │
     *   │   └───────────────────────────────────────────────────┘ │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Carousel has "relative" class for positioning context
     */
    test("carousel should have relative positioning", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carousel).toHaveClass(/relative/);
    });

    /**
     * TEST: Carousel should have overflow-hidden
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarousel (visible area)                           │
     *   │   ┌─────────────────────────────────┐                   │
     *   │   │  ┌─────┬─────┬─────┬─────┬─────┼─────┐              │
     *   │   │  │ S1  │ S2  │ S3  │ S4  │ S5  │ ... │ (hidden)     │
     *   │   │  └─────┴─────┴─────┴─────┴─────┼─────┘              │
     *   │   └─────────────────────────────────┘                   │
     *   │        ↑                            ↑                   │
     *   │   overflow: hidden clips slides outside viewport        │
     *   │                                                         │
     *   │   class="overflow-hidden"                               │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Carousel has "overflow-hidden" to clip slides
     */
    test("carousel should have overflow-hidden", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carousel).toHaveClass(/overflow-hidden/);
    });

    /**
     * TEST: Carousel should have rounded corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   CardCarousel                                          │
     *   │   ╭───────────────────────────────────────────────────╮ │
     *   │   │                                                   │ │
     *   │   │              IMAGE                                │ │
     *   │   │                                                   │ │
     *   │   ╰───────────────────────────────────────────────────╯ │
     *   │   ↑                                                   ↑ │
     *   │   rounded corners (border-radius)                       │
     *   │                                                         │
     *   │   class contains "rounded"                              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Carousel has "rounded" class for border-radius
     */
    test("carousel should have rounded corners", async ({ page }) => {
      const ui = new CardCarouselPage(page);
      await ui.goto();

      await expect(ui.carousel).toHaveClass(/rounded/);
    });
  });
});
