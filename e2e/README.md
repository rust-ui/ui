# E2E Tests — Playwright + Leptos SSR

Gotchas and fixes required to make Playwright work correctly with Leptos SSR + WASM hydration.

---

## 1. Wait for WASM hydration before interacting

**Problem:** Leptos apps are server-side rendered first, then hydrated by a WASM bundle.
Playwright's `goto()` resolves when the HTML is loaded, but the WASM may not have finished
attaching event handlers yet. Clicking elements before hydration succeeds silently — the
element is visible and enabled, but the `on:click` handler is not registered yet.

**Symptom:** Click actions appear to succeed (no timeout), but reactive state never updates.

**Fix:** Override `goto()` in your page object and wait for `networkidle`:

```typescript
override async goto(section?: string) {
  await super.goto(section);
  await this.page.waitForLoadState("networkidle");
}
```

`networkidle` waits for all network activity to settle, which includes the WASM bundle
download and initialization. After this, Leptos event handlers are attached.

---

## 2. Use Playwright auto-retry assertions for reactive attributes

**Problem:** Leptos reactive attributes update asynchronously after an event fires.
Reading an attribute immediately after a click with `getAttribute()` may return the
stale SSR value — the reactive effect hasn't run yet.

**Symptom:** `expect(await locator.getAttribute("data-active")).toBe("true")` fails
intermittently or consistently, even though the DOM does eventually update.

**Fix:** Use `toBeVisible()` or `toHaveAttribute()` instead, which retry automatically
until the condition is met (up to the configured timeout):

```typescript
// Bad — reads once, no retry
expect(await page.locator('[data-active="true"]').getAttribute("data-color")).toBe("red");

// Good — retries until visible
await expect(page.locator('[data-color="red"][data-active="true"]')).toBeVisible();
```

---

## 3. `attr:` prefix in Leptos view! macro renders literally in SSR

**Problem:** In Leptos 0.8, `attr:` is the syntax for arbitrary/hyphenated attributes
in the `view!` macro. For known HTML attributes (`disabled`, `class`, etc.), Leptos strips
the prefix correctly. But for custom attributes like `data-*`, the `attr:` prefix is
output **verbatim** in the SSR HTML:

```html
<!-- Wrong — attr: rendered literally -->
<button attr:data-color="slate" attr:data-active="true">

<!-- Correct -->
<button data-color="slate" data-active="true">
```

Because the browser stores `attr:data-color` as a literal attribute name, the Playwright
selector `[data-color="slate"]` never matches.

**Fix:** Write `data-*` attributes **without** the `attr:` prefix in the `view!` macro.
Despite containing a hyphen, Leptos's macro parser handles `data-*` names directly:

```rust
// Bad
attr:data-color=name
attr:data-active=move || (active.get() == name).to_string()

// Good
data-color=name
data-active=move || (active.get() == name).to_string()
```

---

## 4. Multiple demo instances on the same page

**Problem:** Documentation pages often render a component twice — once in the main demo
block and once further down the page. Playwright strict mode rejects locators that match
more than one element.

**Symptom:** `Error: strict mode violation: locator(...) resolved to 2 elements`

**Fix:** Always use `.first()` on locators in page objects, and combine attribute
selectors to narrow matches (e.g., `[data-active="true"][data-color]` excludes unrelated
elements that also use `data-active`):

```typescript
swatch(color: string): Locator {
  return this.page.locator(`[data-color="${color}"]`).first();
}

async expectActiveColor(color: string): Promise<void> {
  await expect(
    this.page.locator(`[data-color="${color}"][data-active="true"]`).first()
  ).toBeVisible();
}
```

---

## 5. SPA navigation tests and the `gotoViaSpa()` helper

**Why:** Some bugs only appear after a Leptos SPA route transition (not on direct page load).
Specifically, the `page__fade` animation runs a `translateY` + `opacity` animation on
`#page__outlet`. If `fill-mode: forwards` is left on after the animation, the CSS transform
creates a new stacking context that breaks `position: fixed` overlays (Drawer, DropdownMenu,
Sonner, ContextMenu, etc.) — they render clipped or invisible.

`gotoViaSpa()` in `_base_page.ts` exercises this path:

1. Full-loads a different component page (e.g. `button`)
2. Clicks the sidebar link to SPA-navigate to the target component
3. Waits for the URL to settle + 300 ms for the `page__fade` animation to complete

```typescript
await ui.gotoViaSpa();           // SPA nav from button → target
await ui.gotoViaSpa("dialog");   // SPA nav from dialog → target
```

**When to add SPA tests:** Any component that uses `position: fixed` or renders into a
portal (overlays, toasts, drawers, context menus) needs an SPA navigation variant.

---

## 6. Overlay locators after SPA navigation

**Problem:** After SPA navigation, portal elements (DropdownMenu content, Sonner toaster,
Drawer) render at `<body>` level — outside the `[data-name="Preview"]` container. Locators
scoped to `this.preview` will not find them.

**Fix:** Use **page-level** locators in SPA navigation tests:

```typescript
// Bad — scoped to preview, misses portal elements after SPA nav
await expect(ui.toastContainer).toBeVisible();          // this.preview.locator(...)

// Good — page-level
await expect(page.locator('[data-sonner-toaster]').first()).toBeVisible();
```

Also: `page.locator('[data-name="DropdownMenuContent"]').first()` may resolve to the
**wrong** instance after SPA nav (DOM order changes). Scope to the open one instead:

```typescript
const openContent = page.locator('[data-name="DropdownMenuContent"][data-state="open"]');
await expect(openContent).toBeVisible();
await page.keyboard.press("Escape");
await expect(openContent).not.toBeVisible();
```

---

## 7. Hydration panic on direct page load (WASM dies)

**Problem:** `theme_toggle.rs` causes an unrecoverable Leptos hydration error on every
direct page load. This kills the WASM instance — click handlers stop working, reactive
state never updates, and toasts/overlays never appear.

**Consequence:** You cannot test interactive behaviour (reactive state, toasts, overlays)
via direct `page.goto()`. The element is in the DOM (SSR-rendered) but inert.

**Fix:** Use `gotoViaSpa()` for any test that clicks a button and expects reactive output.
SPA navigation skips hydration entirely — the component renders fresh in CSR mode with a
live WASM instance.

---

## 8. Lazy-loaded Sonner JS

**Problem:** `lazy_load_sonner.js` delays loading `sonner.js` until the
`[data-name="SonnerTrigger"]` element appears in the DOM (via MutationObserver). After SPA
navigation the trigger appears ~immediately, but the async script load may not finish within
the 300 ms `gotoViaSpa()` wait.

**Fix:** Wait for `window.LazySonner.loaded` before clicking the trigger:

```typescript
await page.waitForFunction(() => (window as any).LazySonner?.loaded === true, { timeout: 5000 });
await ui.triggerToast();
```

---

## 9. Custom Leptos Toaster vs Sonner — different selectors

The app has **two distinct toast systems**:

| System | Trigger | DOM marker | Locator |
|---|---|---|---|
| **Sonner** (JS) | `SonnerTrigger` button | `data-sonner-toast="true"` on `<li>` | `[data-sonner-toast="true"]` |
| **Custom Leptos Toaster** | `show_toast()` in click handler | No `data-*` attr — plain `<div>` with CSS vars | `[style*="leptoaster-info-background-color"] span` |

The Sonner **container** (`[data-sonner-toaster]`) is always rendered globally (6 instances
for the positions demo) — always use `.first()` to avoid strict-mode violations.

The custom Leptos Toaster renders toasts as plain `<div>` elements with inline styles
using `--leptoaster-*` CSS variables. Match by CSS variable content, not text, to avoid
false-matching identical text in documentation code blocks:

```typescript
// Bad — also matches code snippet text in docs
await expect(page.getByText("This is a toast!")).toBeVisible();

// Good — specific to the toast element
await expect(page.locator('[style*="leptoaster-info-background-color"] span')).toBeVisible();
```

---

## 10. Running the tests

```bash
# Start the dev server first
LEPTOS_SITE_ADDR="127.0.0.1:4000" LEPTOS_RELOAD_PORT="4001" cargo leptos watch --hot-reload

# Run all tests (use --workers=1 to avoid Leptos reactive panic under parallel load)
cd e2e && BASE_URL="http://127.0.0.1:4000" pnpm test -- --workers=1

# Run only SPA navigation tests
BASE_URL="http://127.0.0.1:4000" pnpm test -- --grep "SPA Navigation" --workers=1

# Run a single spec
BASE_URL="http://127.0.0.1:4000" pnpm test tests/hooks/use-history.spec.ts
```

> **Note:** The server panics under parallel Playwright workers due to Leptos "reactive
> value disposed" errors. Always use `--workers=1` after a fresh server start.
