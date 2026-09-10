import { test, expect, Page } from "@playwright/test";

const URL = "/test-page";

async function gotoCanvas(page: Page) {
  await page.goto(URL);
  // wait for at least one source handle to be visible
  await page.locator('[data-testid="source-handle"]').first().waitFor({ state: "visible" });
}

test.describe("NodeCanvas — click-to-connect", () => {
  test("dragging source handle creates edge, does not move node", async ({ page }) => {
    await gotoCanvas(page);

    // Locate trigger node's source handle and output node's target handle
    const sourceHandle = page.locator('[data-testid="source-handle"][data-node-id="trigger"]');
    const targetHandle = page.locator('[data-testid="target-handle"][data-node-id="output"]');

    // Record trigger node position before
    const triggerNode = page.locator('[data-testid="source-handle"][data-node-id="trigger"]').locator("xpath=../../..");
    // Use the canvas SVG edge count as proxy: initial = 3
    const svgBefore = page.locator("svg path[fill='none']");
    const edgeCountBefore = await svgBefore.count();

    const srcBox = await sourceHandle.boundingBox();
    const tgtBox = await targetHandle.boundingBox();
    expect(srcBox).not.toBeNull();
    expect(tgtBox).not.toBeNull();

    const sx = srcBox!.x + srcBox!.width / 2;
    const sy = srcBox!.y + srcBox!.height / 2;
    const tx = tgtBox!.x + tgtBox!.width / 2;
    const ty = tgtBox!.y + tgtBox!.height / 2;

    // Save trigger node's left position to verify it didn't move
    const srcParentBox = await sourceHandle.locator("xpath=..").boundingBox();
    const initialLeft = srcParentBox!.x;

    // Drag source → target
    await page.mouse.move(sx, sy);
    await page.mouse.down();
    // Move incrementally so mousemove fires on canvas
    await page.mouse.move(sx + (tx - sx) / 2, sy + (ty - sy) / 2, { steps: 10 });
    await page.mouse.move(tx, ty, { steps: 10 });
    await page.mouse.up();

    // Node must not have moved
    const srcParentBoxAfter = await sourceHandle.locator("xpath=..").boundingBox();
    expect(Math.abs(srcParentBoxAfter!.x - initialLeft)).toBeLessThan(5);

    // Edge count must have increased (trigger→output edge added)
    const edgeCountAfter = await svgBefore.count();
    expect(edgeCountAfter).toBeGreaterThan(edgeCountBefore);
  });

  test("dragging node body moves node, does not start connect", async ({ page }) => {
    await gotoCanvas(page);

    const agentSourceHandle = page.locator('[data-testid="source-handle"][data-node-id="agent"]');
    const agentParent = agentSourceHandle.locator("xpath=..");

    const box = await agentParent.boundingBox();
    expect(box).not.toBeNull();

    const cx = box!.x + box!.width / 2;
    const cy = box!.y + box!.height / 2;
    const initialX = box!.x;

    await page.mouse.move(cx, cy);
    await page.mouse.down();
    await page.mouse.move(cx + 60, cy, { steps: 10 });
    await page.mouse.up();

    const boxAfter = await agentParent.boundingBox();
    // Node should have moved right
    expect(boxAfter!.x).toBeGreaterThan(initialX + 20);

    // No connecting preview should remain (no dashed primary path)
    const preview = page.locator("svg path[stroke='hsl(var(--primary))']");
    await expect(preview).toHaveCount(0);
  });
});
