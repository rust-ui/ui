import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * INPUT-OTP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   <div data-name="InputOTP">                                            │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐              │   │
 * │   │   │  1  │ │  2  │ │  3  │ │  4  │ │  5  │ │  6  │              │   │
 * │   │   │     │ │     │ │     │ │     │ │     │ │     │              │   │
 * │   │   └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘              │   │
 * │   │      ↑       ↑       ↑       ↑       ↑       ↑                 │   │
 * │   │   InputOTPSlot (maxlength=1, text-center)                       │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * AUTO-ADVANCE BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Type "1":                   Type "2":                                 │
 * │   ┌─────┐ ┌─────┐ ┌─────┐    ┌─────┐ ┌─────┐ ┌─────┐                   │
 * │   │▌ 1 │ │     │ │     │    │  1  │ │▌ 2 │ │     │                    │
 * │   └═════┘ └─────┘ └─────┘    └─────┘ └═════┘ └─────┘                   │
 * │   focused                     auto-advances to next                     │
 * │                                                                         │
 * │   Complete:                                                             │
 * │   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                      │
 * │   │  1  │ │  2  │ │  3  │ │  4  │ │  5  │ │▌ 6 │                       │
 * │   └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └═════┘                      │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * SLOT STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   ┌─────────────────────────────────────────────────────────────┐       │
 * │   │  .size-9      ← 36px square                                 │       │
 * │   │  .border      ← Border styling                              │       │
 * │   │  .rounded-md  ← Medium border radius                        │       │
 * │   │  .text-center ← Center the digit                            │       │
 * │   │  .focus:ring  ← Focus ring on active slot                   │       │
 * │   └─────────────────────────────────────────────────────────────┘       │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class InputOtpPage extends BasePage {
  protected readonly componentName = "input-otp";

  // OTP elements
  readonly otpContainer: Locator;
  readonly otpSlots: Locator;
  readonly firstSlot: Locator;
  readonly secondSlot: Locator;
  readonly thirdSlot: Locator;

  constructor(page: Page) {
    super(page);

    // OTP container - scoped within preview
    this.otpContainer = this.preview.locator('[data-name="InputOTP"]').first();

    // OTP slots
    this.otpSlots = this.otpContainer.locator('[data-name="InputOTPSlot"]');
    this.firstSlot = this.otpSlots.nth(0);
    this.secondSlot = this.otpSlots.nth(1);
    this.thirdSlot = this.otpSlots.nth(2);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("InputOtp Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: InputOTP Container Visibility
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  data-name="InputOTP"                                   │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │     │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │                        ↑                                │
     *   │              Container should be visible                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: InputOTP container is rendered and visible on page
     */
    test("should have InputOTP container", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.otpContainer).toBeVisible();
    });

    /**
     * TEST: InputOTP Data Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <div data-name="InputOTP">  <── attribute check       │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │     │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name="InputOTP" attribute is present on container
     */
    test("should have InputOTP data-name attribute", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.otpContainer).toHaveAttribute("data-name", "InputOTP");
    });

    /**
     * TEST: Six OTP Slots
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │  1  │ │  2  │ │  3  │ │  4  │ │  5  │ │  6  │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │     ↑       ↑       ↑       ↑       ↑       ↑          │
     *   │     └───────┴───────┴───────┴───────┴───────┘          │
     *   │                  count() === 6                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 6 OTP slot inputs exist in the container
     */
    test("should have six OTP slots", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      const count = await ui.otpSlots.count();
      expect(count).toBe(6);
    });

    /**
     * TEST: Slots Have Data-Name Attribute
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  ┌─────────────────────────────────────────────────┐    │
     *   │  │ data-name="InputOTPSlot"  <── attribute check   │    │
     *   │  │ ┌─────┐                                         │    │
     *   │  │ │     │ (first slot)                            │    │
     *   │  │ └─────┘                                         │    │
     *   │  └─────────────────────────────────────────────────┘    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: data-name="InputOTPSlot" attribute is on each slot
     */
    test("slots should have data-name attribute", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveAttribute("data-name", "InputOTPSlot");
    });
  });

  test.describe("Slot Properties", () => {
    /**
     * TEST: Slots Have Maxlength=1
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <input maxlength="1">  <── attribute check            │
     *   │  ┌─────┐                                                │
     *   │  │  X  │  ← Only 1 character allowed per slot          │
     *   │  └─────┘                                                │
     *   │                                                         │
     *   │  Typing "12" in one slot:                               │
     *   │  ┌─────┐                                                │
     *   │  │  1  │  ← Second character "2" is blocked            │
     *   │  └─────┘                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each slot restricts input to single character
     */
    test("slots should have maxlength=1", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveAttribute("maxlength", "1");
    });

    /**
     * TEST: Slots Are Input Elements
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  <input>  <── tagName check                            │
     *   │  ┌─────┐                                                │
     *   │  │     │  ← Must be native <input> element             │
     *   │  └─────┘                                                │
     *   │                                                         │
     *   │  tagName.toLowerCase() === "input"                      │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slots are proper <input> HTML elements
     */
    test("slots should be input elements", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      const tagName = await ui.firstSlot.evaluate((el) =>
        el.tagName.toLowerCase()
      );
      expect(tagName).toBe("input");
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Container Has Flex Layout
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... flex ..."                                   │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │     │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │     ←────── flex row layout ──────→                     │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container uses flex layout for horizontal slot arrangement
     */
    test("container should have flex layout", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.otpContainer).toHaveClass(/flex/);
    });

    /**
     * TEST: Container Has Gap Spacing
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... gap-2 ..."                                  │
     *   │  ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐                  │
     *   │  │     │   │     │   │     │   │     │                  │
     *   │  └─────┘   └─────┘   └─────┘   └─────┘                  │
     *   │        ↑         ↑         ↑                            │
     *   │        └─────────┴─────────┘                            │
     *   │            gap-2 (8px spacing)                          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has gap-2 class for consistent slot spacing
     */
    test("container should have gap", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.otpContainer).toHaveClass(/gap-2/);
    });

    /**
     * TEST: Slots Have Border
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... border ..."                                 │
     *   │  ╔═════╗                                                │
     *   │  ║     ║  ← border visible around slot                  │
     *   │  ╚═════╝                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each slot has 'border' class for visible outline
     */
    test("slots should have border", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveClass(/border/);
    });

    /**
     * TEST: Slots Have Rounded Corners
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... rounded-md ..."                             │
     *   │  ╭─────╮                                                │
     *   │  │     │  ← rounded corners (border-radius: medium)     │
     *   │  ╰─────╯                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each slot has 'rounded-md' class for rounded corners
     */
    test("slots should have rounded corners", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveClass(/rounded-md/);
    });

    /**
     * TEST: Slots Are Square (size-9)
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... size-9 ..."                                 │
     *   │  ┌─────────┐                                            │
     *   │  │         │  ↕ 36px (size-9 = 2.25rem)                 │
     *   │  │    5    │                                            │
     *   │  │         │                                            │
     *   │  └─────────┘                                            │
     *   │  ←──────────→                                           │
     *   │     36px                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Each slot is a 36x36px square (size-9)
     */
    test("slots should be square (size-9)", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveClass(/size-9/);
    });

    /**
     * TEST: Slots Have Text Center
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... text-center ..."                            │
     *   │  ┌─────────┐                                            │
     *   │  │         │                                            │
     *   │  │    5    │  ← digit is horizontally centered          │
     *   │  │         │                                            │
     *   │  └─────────┘                                            │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Text in slots is centered with 'text-center' class
     */
    test("slots should have text-center", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveClass(/text-center/);
    });
  });

  test.describe("Interaction", () => {
    /**
     * TEST: First Slot Focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before focus():                                        │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │     │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │                                                         │
     *   │  After focus():                                         │
     *   │  ┌═════┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │▌    │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └═════┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │     ↑ focused (toBeFocused() = true)                    │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First OTP slot can receive keyboard focus
     */
    test("first slot should be focusable", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await ui.firstSlot.focus();
      await expect(ui.firstSlot).toBeFocused();
    });

    /**
     * TEST: Type in Slots
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before fill("1"):                                      │
     *   │  ┌─────┐                                                │
     *   │  │     │                                                │
     *   │  └─────┘                                                │
     *   │                                                         │
     *   │  After fill("1"):                                       │
     *   │  ┌─────┐                                                │
     *   │  │  1  │  ← value = "1"                                 │
     *   │  └─────┘                                                │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: User can type a digit into a slot
     */
    test("should be able to type in slots", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await ui.firstSlot.fill("1");
      await expect(ui.firstSlot).toHaveValue("1");
    });

    /**
     * TEST: Auto-Advance to Next Slot
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Step 1: Focus first slot                               │
     *   │  ┌═════┐ ┌─────┐ ┌─────┐                               │
     *   │  │▌    │ │     │ │     │                               │
     *   │  └═════┘ └─────┘ └─────┘                               │
     *   │                                                         │
     *   │  Step 2: Type "1"                                       │
     *   │  ┌─────┐ ┌═════┐ ┌─────┐                               │
     *   │  │  1  │ │▌    │ │     │                               │
     *   │  └─────┘ └═════┘ └─────┘                               │
     *   │            ↑ focus auto-advances to slot 2              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: After typing, focus moves to next slot automatically
     */
    test("typing should auto-advance to next slot", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await ui.firstSlot.focus();
      await page.keyboard.type("1");

      // Should auto-focus second slot
      await page.waitForTimeout(100);
      await expect(ui.secondSlot).toBeFocused();
    });

    /**
     * TEST: Fill All Slots
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Before typing:                                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │     │ │     │ │     │ │     │ │     │ │     │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │                                                         │
     *   │  After keyboard.type("123456"):                         │
     *   │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
     *   │  │  1  │ │  2  │ │  3  │ │  4  │ │  5  │ │  6  │       │
     *   │  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘       │
     *   │     ↑       ↑       ↑       ↑       ↑       ↑          │
     *   │  Each slot filled with corresponding digit              │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Typing "123456" fills all 6 slots with auto-advance
     */
    test("can fill all slots", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await ui.firstSlot.focus();
      await page.keyboard.type("123456");

      // All slots should have values
      await expect(ui.otpSlots.nth(0)).toHaveValue("1");
      await expect(ui.otpSlots.nth(1)).toHaveValue("2");
      await expect(ui.otpSlots.nth(2)).toHaveValue("3");
      await expect(ui.otpSlots.nth(3)).toHaveValue("4");
      await expect(ui.otpSlots.nth(4)).toHaveValue("5");
      await expect(ui.otpSlots.nth(5)).toHaveValue("6");
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Slots Have Focus Ring Styles
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  class="... focus:ring ..."                             │
     *   │                                                         │
     *   │  Unfocused:         Focused:                            │
     *   │  ┌─────┐            ┌═════════┐                         │
     *   │  │     │            │ ╔═════╗ │                         │
     *   │  │     │     →      │ ║     ║ │  ← focus ring visible   │
     *   │  │     │            │ ╚═════╝ │                         │
     *   │  └─────┘            └═════════┘                         │
     *   │                           ↑                             │
     *   │                  focus:ring class enables ring          │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Slots have focus:ring class for accessibility outline
     */
    test("slots should have focus ring styles", async ({ page }) => {
      const ui = new InputOtpPage(page);
      await ui.goto();

      await expect(ui.firstSlot).toHaveClass(/focus:ring/);
    });
  });
});
