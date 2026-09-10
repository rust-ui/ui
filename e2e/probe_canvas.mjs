import { chromium } from "@playwright/test";

const browser = await chromium.launch({ headless: false, slowMo: 300 });
const page = await browser.newPage();
await page.setViewportSize({ width: 1280, height: 720 });

await page.goto("http://localhost:8080/test-page");
await page.waitForTimeout(1500);

// ── snapshot helpers ──────────────────────────────────────────────────────────
const edgeCount = () => page.locator("svg path[fill='none']").count();
const log = (msg) => console.log(`  ${msg}`);

// ── locate handles ────────────────────────────────────────────────────────────
log("Locating source handle on 'trigger' node...");
const srcHandle = page.locator('[data-testid="source-handle"][data-node-id="trigger"]');
const tgtHandle = page.locator('[data-testid="target-handle"][data-node-id="output"]');

const srcCount = await srcHandle.count();
const tgtCount = await tgtHandle.count();
log(`source-handle[trigger] count: ${srcCount}`);
log(`target-handle[output] count: ${tgtCount}`);

if (srcCount === 0 || tgtCount === 0) {
  // Fallback: dump all data-testid elements found
  const all = await page.locator('[data-testid]').all();
  log(`All [data-testid] elements found: ${all.length}`);
  for (const el of all) {
    const testid = await el.getAttribute("data-testid");
    const nodeid = await el.getAttribute("data-node-id");
    log(`  testid=${testid} node-id=${nodeid}`);
  }
  await browser.close();
  process.exit(1);
}

const srcBox = await srcHandle.boundingBox();
const tgtBox = await tgtHandle.boundingBox();
log(`srcHandle box: ${JSON.stringify(srcBox)}`);
log(`tgtHandle box: ${JSON.stringify(tgtBox)}`);

// ── record trigger node initial position ──────────────────────────────────────
const triggerWrapper = page.locator('[data-testid="source-handle"][data-node-id="trigger"]').locator("xpath=..");
const wrapperBefore = await triggerWrapper.boundingBox();
log(`trigger wrapper x before: ${wrapperBefore?.x?.toFixed(1)}`);

// ── edges before ─────────────────────────────────────────────────────────────
const before = await edgeCount();
log(`SVG path[fill=none] count BEFORE: ${before}`);

// ── perform drag: source handle → target handle ───────────────────────────────
log("\nDragging source handle → target handle...");
const sx = srcBox.x + srcBox.width / 2;
const sy = srcBox.y + srcBox.height / 2;
const tx = tgtBox.x + tgtBox.width / 2;
const ty = tgtBox.y + tgtBox.height / 2;

await page.mouse.move(sx, sy);
await page.waitForTimeout(100);
await page.mouse.down();
await page.waitForTimeout(100);
await page.mouse.move(sx + (tx - sx) * 0.5, sy + (ty - sy) * 0.5, { steps: 8 });
await page.mouse.move(tx, ty, { steps: 8 });
await page.waitForTimeout(200);
await page.mouse.up();
await page.waitForTimeout(800); // allow Dioxus render tick

// ── edges after ──────────────────────────────────────────────────────────────
const after = await edgeCount();
log(`SVG path[fill=none] count AFTER: ${after}`);
log(`Edge delta: ${after - before} (expected +2 for main+minimap)`);

// ── node position ─────────────────────────────────────────────────────────────
const wrapperAfter = await triggerWrapper.boundingBox();
log(`trigger wrapper x after:  ${wrapperAfter?.x?.toFixed(1)}`);
const moved = Math.abs((wrapperAfter?.x ?? 0) - (wrapperBefore?.x ?? 0));
log(`Node moved: ${moved.toFixed(1)}px (should be <5 if no drag)`);

// ── screenshot ────────────────────────────────────────────────────────────────
await page.screenshot({ path: "/tmp/canvas_probe.png" });
log("\nScreenshot saved: /tmp/canvas_probe.png");

if (after > before) {
  log("\n✓ CONNECTION CREATED");
} else {
  log("\n✗ CONNECTION FAILED — investigating further...");
  // Check if connecting state started (is there a dashed primary path?)
  const preview = await page.locator("svg path[stroke*='primary']").count();
  log(`  Preview bezier count: ${preview} (0 = connect never started or already finished)`);

  // Check what element is at the source handle position
  const elemAtSrc = await page.evaluate(([x, y]) => {
    const el = document.elementFromPoint(x, y);
    return el ? `${el.tagName} class="${el.className}" testid="${el.getAttribute('data-testid')}"` : "null";
  }, [sx, sy]);
  log(`  Element at source pos: ${elemAtSrc}`);

  const elemAtTgt = await page.evaluate(([x, y]) => {
    const el = document.elementFromPoint(x, y);
    return el ? `${el.tagName} class="${el.className}" testid="${el.getAttribute('data-testid')}"` : "null";
  }, [tx, ty]);
  log(`  Element at target pos: ${elemAtTgt}`);
}

await page.waitForTimeout(2000);
await browser.close();
