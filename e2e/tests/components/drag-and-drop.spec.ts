import { Locator, Page, test, expect } from "@playwright/test";
import { BasePage } from "./_base_page";

/**
 * ============================================================================
 * DRAG-AND-DROP COMPONENT - VISUAL OVERVIEW
 * ============================================================================
 *
 * COMPONENT ANATOMY:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Draggable Container                                                   │
 * │   ┌─────────────────────────────────────────────────────────────────┐   │
 * │   │                                                                 │   │
 * │   │   DraggableZone 1                    DraggableZone 2            │   │
 * │   │   ┌─────────────────────┐            ┌─────────────────────┐    │   │
 * │   │   │  ┌───────────────┐  │            │  ┌───────────────┐  │    │   │
 * │   │   │  │  Item 1       │  │            │  │  Item 3       │  │    │   │
 * │   │   │  └───────────────┘  │            │  └───────────────┘  │    │   │
 * │   │   │  ┌───────────────┐  │            │  ┌───────────────┐  │    │   │
 * │   │   │  │  Item 2       │  │            │  │  Item 4       │  │    │   │
 * │   │   │  └───────────────┘  │            │  └───────────────┘  │    │   │
 * │   │   └─────────────────────┘            └─────────────────────┘    │   │
 * │   │                                                                 │   │
 * │   └─────────────────────────────────────────────────────────────────┘   │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * DRAG BEHAVIOR:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   Step 1: Start Drag              Step 2: Dragging                      │
 * │   ┌─────────────────────┐         ┌─────────────────────┐               │
 * │   │  ┌───────────────┐  │         │  ┌ ─ ─ ─ ─ ─ ─ ─ ┐  │               │
 * │   │  │  Item 1  🖐️   │  │   ──→   │    Item 1          │   ──→          │
 * │   │  └───────────────┘  │         │  └ ─ ─ ─ ─ ─ ─ ─ ┘  │               │
 * │   └─────────────────────┘         └─────────────────────┘               │
 * │                                   opacity: 0.5 (dragging class)         │
 * │                                                                         │
 * │   Step 3: Drop                                                          │
 * │   ┌─────────────────────┐                                               │
 * │   │  ┌───────────────┐  │                                               │
 * │   │  │  Item 3       │  │   Item moved to new zone                      │
 * │   │  └───────────────┘  │                                               │
 * │   │  ┌───────────────┐  │                                               │
 * │   │  │  Item 1  ✓    │  │                                               │
 * │   │  └───────────────┘  │                                               │
 * │   └─────────────────────┘                                               │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * STYLING:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                                                                         │
 * │   .draggable.dragging {                                                 │
 * │     opacity: 0.5;         // Visual feedback during drag                │
 * │   }                                                                     │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * ============================================================================
 */

class DragAndDropPage extends BasePage {
  protected readonly componentName = "drag-and-drop";

  // Main container - scoped within preview
  readonly draggable: Locator;
  readonly draggableZones: Locator;
  readonly draggableItems: Locator;

  // Individual zones
  readonly zone1: Locator;
  readonly zone2: Locator;

  constructor(page: Page) {
    super(page);

    // All locators scoped within preview container
    this.draggable = this.preview.locator('[data-name="Draggable"]').first();
    this.draggableZones = this.preview.locator('[data-name="DraggableZone"]');
    this.draggableItems = this.preview.locator('[data-name="DraggableItem"]');

    this.zone1 = this.draggableZones.first();
    this.zone2 = this.draggableZones.nth(1);
  }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

test.describe("DragAndDrop Page", () => {
  test.describe("Structure", () => {
    /**
     * TEST: Should have draggable container
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │  Draggable Container [data-name="Draggable"]           │
     *   │  ┌───────────────────────────────────────────────────┐ │
     *   │  │                                                   │ │
     *   │  │   (zones and items inside)                        │ │
     *   │  │                                                   │ │
     *   │  └───────────────────────────────────────────────────┘ │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Main container element exists and is visible
     */
    test("should have draggable container", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.draggable).toBeVisible();
    });

    /**
     * TEST: Should have two draggable zones
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                        Zone 2                  │
     *   │   ┌───────────────┐            ┌───────────────┐        │
     *   │   │ DraggableZone │            │ DraggableZone │        │
     *   │   │   (count=1)   │            │   (count=2)   │        │
     *   │   └───────────────┘            └───────────────┘        │
     *   │                                                         │
     *   │   Total zones: 2                                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 2 DraggableZone elements exist
     */
    test("should have two draggable zones", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const count = await ui.draggableZones.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Should have four draggable items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                        Zone 2                  │
     *   │   ┌───────────────┐            ┌───────────────┐        │
     *   │   │ ┌───┐ ┌───┐   │            │ ┌───┐ ┌───┐   │        │
     *   │   │ │ 1 │ │ 2 │   │            │ │ 3 │ │ 4 │   │        │
     *   │   │ └───┘ └───┘   │            │ └───┘ └───┘   │        │
     *   │   └───────────────┘            └───────────────┘        │
     *   │                                                         │
     *   │   Total items: 4                                        │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Exactly 4 DraggableItem elements exist across all zones
     */
    test("should have four draggable items", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const count = await ui.draggableItems.count();
      expect(count).toBe(4);
    });
  });

  test.describe("Zone 1", () => {
    /**
     * TEST: Zone 1 should be visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1 (first zone)          Zone 2                   │
     *   │   ┌───────────────┐            ┌ ─ ─ ─ ─ ─ ─ ┐          │
     *   │   │               │                                     │
     *   │   │   VISIBLE?    │ <── check  │             │          │
     *   │   │               │                                     │
     *   │   └───────────────┘            └ ─ ─ ─ ─ ─ ─ ┘          │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: First DraggableZone is visible on the page
     */
    test("zone 1 should be visible", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone1).toBeVisible();
    });

    /**
     * TEST: Zone 1 should have 2 items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │  DraggableItem (1)  │    │  count = 2?           │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │  DraggableItem (2)  │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 1 contains exactly 2 DraggableItem children
     */
    test("zone 1 should have 2 items", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const items = ui.zone1.locator('[data-name="DraggableItem"]');
      const count = await items.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Zone 1 should have item with text 1
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "1"          │ <── contains text "1"?     │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "2"          │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 1 contains an item displaying text "1"
     */
    test("zone 1 should have item with text 1", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone1.getByText("1")).toBeVisible();
    });

    /**
     * TEST: Zone 1 should have item with text 2
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "1"          │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "2"          │ <── contains text "2"?     │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 1 contains an item displaying text "2"
     */
    test("zone 1 should have item with text 2", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone1.getByText("2")).toBeVisible();
    });
  });

  test.describe("Zone 2", () => {
    /**
     * TEST: Zone 2 should be visible
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                        Zone 2 (second zone)    │
     *   │   ┌ ─ ─ ─ ─ ─ ─ ┐              ┌───────────────┐        │
     *   │                                │               │        │
     *   │   │             │     check ──>│   VISIBLE?    │        │
     *   │                                │               │        │
     *   │   └ ─ ─ ─ ─ ─ ─ ┘              └───────────────┘        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Second DraggableZone is visible on the page
     */
    test("zone 2 should be visible", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone2).toBeVisible();
    });

    /**
     * TEST: Zone 2 should have 2 items
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 2                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │  DraggableItem (3)  │    │  count = 2?           │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │  DraggableItem (4)  │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 2 contains exactly 2 DraggableItem children
     */
    test("zone 2 should have 2 items", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const items = ui.zone2.locator('[data-name="DraggableItem"]');
      const count = await items.count();
      expect(count).toBe(2);
    });

    /**
     * TEST: Zone 2 should have item with text 3
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 2                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "3"          │ <── contains text "3"?     │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "4"          │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 2 contains an item displaying text "3"
     */
    test("zone 2 should have item with text 3", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone2.getByText("3")).toBeVisible();
    });

    /**
     * TEST: Zone 2 should have item with text 4
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 2                                                │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "3"          │    │                       │
     *   │   │  └─────────────────────┘    │                       │
     *   │   │  ┌─────────────────────┐    │                       │
     *   │   │  │        "4"          │ <── contains text "4"?     │
     *   │   │  └─────────────────────┘    │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Zone 2 contains an item displaying text "4"
     */
    test("zone 2 should have item with text 4", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.zone2.getByText("4")).toBeVisible();
    });
  });

  test.describe("Draggable Items", () => {
    /**
     * TEST: Items should have DraggableItem data-name
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DraggableItem                                         │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │                             │                       │
     *   │   │  data-name="DraggableItem"  │ <── attribute check   │
     *   │   │                             │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   │   <div data-name="DraggableItem">...</div>              │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items have correct data-name attribute for selection
     */
    test("items should have DraggableItem data-name", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const firstItem = ui.draggableItems.first();
      await expect(firstItem).toHaveAttribute("data-name", "DraggableItem");
    });

    /**
     * TEST: Items should have draggable class
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DraggableItem                                         │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │                             │                       │
     *   │   │   class="... draggable ..." │ <── class check       │
     *   │   │                             │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   │   CSS: .draggable { cursor: grab; }                     │
     *   │        .draggable.dragging { opacity: 0.5; }            │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items have "draggable" class for drag styling
     */
    test("items should have draggable class", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const firstItem = ui.draggableItems.first();
      await expect(firstItem).toHaveClass(/draggable/);
    });
  });

  test.describe("Styling", () => {
    /**
     * TEST: Draggable container should have max-w-2xl
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Draggable Container                                   │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │                             │                       │
     *   │   │   max-width: 42rem (672px)  │ <── max-w-2xl class   │
     *   │   │                             │                       │
     *   │   └─────────────────────────────┘                       │
     *   │   |<────────── 2xl ───────────>|                        │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Container has max-w-2xl class for width constraint
     */
    test("draggable container should have max-w-2xl", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      await expect(ui.draggable).toHaveClass(/max-w-2xl/);
    });

    /**
     * TEST: Zones should have styling
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   Zone 1                        Zone 2                  │
     *   │   ┌───────────────┐            ┌───────────────┐        │
     *   │   │               │            │               │        │
     *   │   │   VISIBLE     │            │   VISIBLE     │        │
     *   │   │   (styled)    │            │   (styled)    │        │
     *   │   │               │            │               │        │
     *   │   └───────────────┘            └───────────────┘        │
     *   │                                                         │
     *   │   Both zones should be rendered and visible             │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Both zones are visible (styled containers)
     */
    test("zones should have styling", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      // Zones should be visible containers
      await expect(ui.zone1).toBeVisible();
      await expect(ui.zone2).toBeVisible();
    });
  });

  test.describe("Accessibility", () => {
    /**
     * TEST: Items should be focusable
     * ─────────────────────────────────────────────────────────────
     *
     *   What we're testing:
     *   ┌─────────────────────────────────────────────────────────┐
     *   │                                                         │
     *   │   DraggableItem                                         │
     *   │   ┌─────────────────────────────┐                       │
     *   │   │                             │                       │
     *   │   │   [ Item 1 ]  <── focus()   │                       │
     *   │   │      ↓                      │                       │
     *   │   │   ┌─────────┐               │                       │
     *   │   │   │:focused │ ring/outline  │                       │
     *   │   │   └─────────┘               │                       │
     *   │   │                             │                       │
     *   │   └─────────────────────────────┘                       │
     *   │                                                         │
     *   │   Keyboard accessibility: Tab to navigate items         │
     *   │                                                         │
     *   └─────────────────────────────────────────────────────────┘
     *
     *   Validates: Items can receive keyboard focus for a11y
     */
    test("items should be focusable", async ({ page }) => {
      const ui = new DragAndDropPage(page);
      await ui.goto();

      const firstItem = ui.draggableItems.first();
      await firstItem.focus();
      await expect(firstItem).toBeFocused();
    });
  });
});
