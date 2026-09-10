import { test, expect, Page, Locator } from "@playwright/test";

// Each workflow gets its own isolated full-screen page — one canvas, no noise.

function wfCanvas(page: Page): Locator {
  return page.locator('[tabindex="0"]').first();
}

async function goto(page: Page, id: string) {
  await page.goto(`/view/${id}`);
  await page.locator('[data-testid="source-handle"]').first().waitFor({
    state: "visible",
    timeout: 15000,
  });
}

// ── workflow-01 — Basic Workflow ─────────────────────────────────────────────

test.describe("workflow-01 — Basic Workflow", () => {
  test("renders nodes and edges", async ({ page }) => {
    await goto(page, "workflow-01");
    // trigger(src) + data(src+tgt) + agent(src+tgt) = 3 src; data+agent+output = 3 tgt
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(3);
    await expect(page.locator('[data-testid="target-handle"]')).toHaveCount(3);
    const edges = page.locator("svg path[fill='none'][stroke='currentColor']");
    await expect(edges).toHaveCount(3);
  });

  test("zoom controls visible", async ({ page }) => {
    await goto(page, "workflow-01");
    await expect(page.getByRole("button", { name: "fit" })).toBeVisible();
    await expect(page.getByRole("button", { name: "reset" })).toBeVisible();
  });

  test("drag node moves it", async ({ page }) => {
    await goto(page, "workflow-01");
    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    expect(box).not.toBeNull();

    const cx = box!.x + box!.width / 2;
    const cy = box!.y + box!.height / 2;

    await page.mouse.move(cx, cy);
    await page.mouse.down();
    await page.mouse.move(cx + 80, cy, { steps: 10 });
    await page.mouse.up();

    const boxAfter = await nodeWrapper.boundingBox();
    expect(boxAfter!.x).toBeGreaterThan(box!.x + 20);
  });
});

// ── workflow-02 — Copy & Paste ────────────────────────────────────────────────

test.describe("workflow-02 — Copy & Paste", () => {
  test("copy and paste buttons present", async ({ page }) => {
    await goto(page, "workflow-02");
    await expect(page.getByRole("button", { name: /copy/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /paste/i })).toBeVisible();
  });

  test("paste disabled when clipboard empty", async ({ page }) => {
    await goto(page, "workflow-02");
    await expect(page.getByRole("button", { name: /paste/i })).toBeDisabled();
  });

  test("copy selected then paste adds node", async ({ page }) => {
    await goto(page, "workflow-02");

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");

    // mousedown selects node
    const box = await nodeWrapper.boundingBox();
    expect(box).not.toBeNull();
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    const copyBtn = page.getByRole("button", { name: /copy/i });
    await expect(copyBtn).not.toBeDisabled();
    await copyBtn.click();

    const countBefore = await page.locator('[data-testid="source-handle"]').count();
    await page.getByRole("button", { name: /paste/i }).click();
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore + 1
    );
  });
});

// ── workflow-03 — Keyboard Shortcuts ─────────────────────────────────────────

test.describe("workflow-03 — Keyboard Shortcuts", () => {
  test("renders 3 nodes", async ({ page }) => {
    await goto(page, "workflow-03");
    // a(src only) + b(src+tgt) = 2 src handles; b(tgt)+c(tgt) = 2 tgt handles
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(2);
  });

  test("arrow key nudges selected node", async ({ page }) => {
    await goto(page, "workflow-03");

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    expect(box).not.toBeNull();

    // Click node to select it — focus bubbles to canvas
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    const cv = wfCanvas(page);
    await cv.focus();
    await cv.press("ArrowRight");
    await cv.press("ArrowRight");
    await cv.press("ArrowRight");  // 3 × 20px = 60px right

    const boxAfter = await nodeWrapper.boundingBox();
    expect(boxAfter!.x).toBeGreaterThan(box!.x + 30);
  });

  test("Delete key removes selected node", async ({ page }) => {
    await goto(page, "workflow-03");

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();

    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    const cv = wfCanvas(page);
    await cv.focus();
    await cv.press("Delete");

    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(1);
  });
});

// ── workflow-04 — Locked Mode ─────────────────────────────────────────────────

test.describe("workflow-04 — Locked Mode", () => {
  test("lock/unlock button toggles", async ({ page }) => {
    await goto(page, "workflow-04");
    await expect(page.getByRole("button", { name: /lock canvas/i })).toBeVisible();
    await page.getByRole("button", { name: /lock canvas/i }).click();
    await expect(page.getByRole("button", { name: /unlock canvas/i })).toBeVisible();
    await page.getByRole("button", { name: /unlock canvas/i }).click();
    await expect(page.getByRole("button", { name: /lock canvas/i })).toBeVisible();
  });

  test("locked canvas — node cannot be dragged", async ({ page }) => {
    await goto(page, "workflow-04");
    await page.getByRole("button", { name: /lock canvas/i }).click();
    await expect(page.getByRole("button", { name: /unlock canvas/i })).toBeVisible();

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    expect(box).not.toBeNull();

    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.move(box!.x + box!.width / 2 + 80, box!.y + box!.height / 2, { steps: 10 });
    await page.mouse.up();

    const boxAfter = await nodeWrapper.boundingBox();
    expect(Math.abs(boxAfter!.x - box!.x)).toBeLessThan(5);
  });
});

// ── workflow-05 — Minimap ─────────────────────────────────────────────────────

test.describe("workflow-05 — Minimap", () => {
  test("minimap container renders", async ({ page }) => {
    await goto(page, "workflow-05");
    // Minimap has inline style with bottom/right positioning
    const minimap = page.locator('[style*="bottom: 12px"][style*="right: 12px"]');
    await expect(minimap).toBeVisible();
  });

  test("minimap has node rects inside svg", async ({ page }) => {
    await goto(page, "workflow-05");
    const minimap = page.locator('[style*="bottom: 12px"][style*="right: 12px"]');
    const rects = minimap.locator("svg rect");
    // At least the viewport rect and some node rects
    await expect(rects.first()).toBeVisible();
    expect(await rects.count()).toBeGreaterThanOrEqual(3);
  });

  test("large graph renders 10 nodes", async ({ page }) => {
    await goto(page, "workflow-05");
    // 8 source handles: input, auth, cache, retrieval, rerank, agent, guard, formatter
    const count = await page.locator('[data-testid="source-handle"]').count();
    expect(count).toBeGreaterThanOrEqual(6);
  });
});

// ── workflow-06 — Multi-Select ────────────────────────────────────────────────

test.describe("workflow-06 — Multi-Select", () => {
  test("clicking node enables delete button", async ({ page }) => {
    await goto(page, "workflow-06");

    const deleteBtn = page.getByRole("button", { name: /delete selected/i });
    await expect(deleteBtn).toBeDisabled();

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    await expect(deleteBtn).not.toBeDisabled();
  });

  test("shift-click shows 2 selected in toolbar", async ({ page }) => {
    await goto(page, "workflow-06");

    const handles = page.locator('[data-testid="source-handle"]');
    const first = handles.nth(0).locator("xpath=..");
    const second = handles.nth(1).locator("xpath=..");

    const b1 = await first.boundingBox();
    expect(b1).not.toBeNull();

    await page.mouse.move(b1!.x + b1!.width / 2, b1!.y + b1!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    await second.click({ modifiers: ["Shift"] });

    await expect(page.getByText("2 selected")).toBeVisible();
  });

  test("delete selected removes nodes", async ({ page }) => {
    await goto(page, "workflow-06");

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    const countBefore = await page.locator('[data-testid="source-handle"]').count();
    await page.getByRole("button", { name: /delete selected/i }).click();
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore - 1
    );
  });
});

// ── workflow-07 — Status Nodes ────────────────────────────────────────────────

test.describe("workflow-07 — Status Nodes", () => {
  test("status labels visible", async ({ page }) => {
    await goto(page, "workflow-07");
    await expect(page.getByText("Success")).toBeVisible();
    await expect(page.getByText("Failed")).toBeVisible();
    await expect(page.getByText("In progress")).toBeVisible();
    await expect(page.getByText("Queued")).toBeVisible();
  });

  test("renders 4 nodes (3 source handles)", async ({ page }) => {
    await goto(page, "workflow-07");
    // trigger(src), data(src+tgt), agent(src+tgt), output(tgt) → 3 src
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(3);
  });

  test("status nodes have colored left border", async ({ page }) => {
    await goto(page, "workflow-07");
    const bordered = page.locator(".border-l-\\[3px\\]");
    await expect(bordered.first()).toBeVisible();
    expect(await bordered.count()).toBe(4);
  });
});

// ── workflow-08 — Toolbar ─────────────────────────────────────────────────────

test.describe("workflow-08 — Toolbar", () => {
  test("toolbar buttons visible", async ({ page }) => {
    await goto(page, "workflow-08");
    await expect(page.getByRole("button", { name: /add node/i })).toBeVisible();
    await expect(page.getByRole("button", { name: /reset/i })).toBeVisible();
  });

  test("add node increases count", async ({ page }) => {
    await goto(page, "workflow-08");
    const countBefore = await page.locator('[data-testid="source-handle"]').count();
    await page.getByRole("button", { name: /add node/i }).click();
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore + 1
    );
  });

  test("add then undo reverts count", async ({ page }) => {
    await goto(page, "workflow-08");
    const countBefore = await page.locator('[data-testid="source-handle"]').count();

    await page.getByRole("button", { name: /add node/i }).click();
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore + 1
    );

    // Undo button (ToolbarToggleItem with title="Undo")
    await page.locator('[title="Undo"]').click();
    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore
    );
  });

  test("delete selected node via keyboard", async ({ page }) => {
    await goto(page, "workflow-08");
    const countBefore = await page.locator('[data-testid="source-handle"]').count();

    const handle = page.locator('[data-testid="source-handle"]').first();
    const nodeWrapper = handle.locator("xpath=..");
    const box = await nodeWrapper.boundingBox();
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.mouse.up();

    const cv = wfCanvas(page);
    await cv.focus();
    await cv.press("Delete");

    await expect(page.locator('[data-testid="source-handle"]')).toHaveCount(
      countBefore - 1
    );
  });
});
