# Charts lazy loading (TODO)

## Issue that got fixed (2026-09-11)

Home page charts didn't render on iOS; Charts page worked fine.

Root cause, two layers:
1. `themes_blocks.rs` injected `chart_init.js` via raw JS string / hardcoded
   path, bypassing `asset!()`. Dead code, removed.
2. `chart_init.js`'s own `loadApexCharts()` fallback used a hardcoded
   `/cdn/apexcharts...` path. Works on web dev server, 404s on iOS
   (fingerprinted asset paths). ApexCharts was only ever properly loaded via
   `asset!()` on the dedicated Charts route (`charts_layout.rs`), so Home
   (visited without going through Charts first) never got the library.

Fix applied: `apexcharts.5.3.6.min.js` now loads globally via
`document::Script` in `main.rs` (asset!()-resolved everywhere, works on iOS).
Removed the now-redundant per-route declaration in `charts_layout.rs`.

## Tradeoff, not yet resolved

apexcharts min.js is ~568KB, now loaded on every page/app boot instead of
only on chart-heavy routes.

## Lazy-load alternative (if perf complaint confirmed)

Keep the global load removed; instead expose the `asset!()`-resolved URL as a
JS global (set from Rust, e.g. `window.__APEXCHARTS_URL__ = "..."`), and have
`chart_init.js`'s `loadApexCharts()` read that global instead of its
hardcoded `/cdn/...` string when appending the `<script>` tag. Keeps the lazy
behavior (chart_init.js only loads it when a chart container is on the page)
while still going through the asset-fingerprinted path on iOS.

Not implemented yet, decide only if the always-global load is noticeably slow
on device.
