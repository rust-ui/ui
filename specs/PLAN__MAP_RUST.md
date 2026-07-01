# Map Component in Rust/Leptos — Implementation Plan

## Target Demo

**mapcn `FlyToExample`** — https://github.com/AnmolSaini16/mapcn/blob/main/src/app/(main)/(home)/_components/examples/flyto-example.tsx

Globe map (`projection: globe`) centered on Europe, with a pulsing cyan marker on New York.
A button triggers `flyTo({ center: [-74.006, 40.7128], zoom: 4, duration: 2000 })`.

```
┌─────────────────────────────────┐
│         🌍 Globe Map            │
│                                 │
│    [NORTH AMERICA]  [EUROPE]    │
│          ◉ ← pulsing marker     │
│    [SOUTH AMERICA]              │
│                                 │
│  [Fly to New York ↗]  button    │
└─────────────────────────────────┘
```

## Approach: Same Pattern as ApexCharts (RUST-UI)

Zero `wasm_bindgen`. Rust renders a `<div>` with `data-*` attributes, a JS module handles MapLibre GL.

### Rust side

```rust
<div
    data-name="MapLibre"
    data-projection="globe"
    data-center-lng="10"
    data-center-lat="50"
    data-zoom="0.6"
    data-markers=r#"[{"lng":-74.006,"lat":40.7128,"label":"New York","description":"United States"}]"#
    class="h-full w-full"
/>

// Button dispatches a custom event — no JS interop needed from Rust
<button on:click=move |_| {
    // dispatch "map:flyto" CustomEvent on the map div
    let event = /* web_sys::CustomEvent */ ...;
    map_el.dispatch_event(&event).unwrap();
}>
    { "Fly to New York" }
</button>
```

### JS side (`map_init.js`, mirrors `chart_init.js`)

```javascript
const DARK_STYLE  = "https://basemaps.cartocdn.com/gl/dark-matter-gl-style/style.json";
const LIGHT_STYLE = "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json";

const observer = new MutationObserver(() => {
    document.querySelectorAll('[data-name="MapLibre"]:not([data-initialized])').forEach(el => {
        const map = new maplibregl.Map({
            container: el,
            style: LIGHT_STYLE,
            center: [+el.dataset.centerLng, +el.dataset.centerLat],
            zoom:   +el.dataset.zoom,
            projection: el.dataset.projection === "globe" ? { type: "globe" } : undefined,
            attributionControl: { compact: true },
        });

        // Pulsing markers from JSON
        const markers = JSON.parse(el.dataset.markers || "[]");
        markers.forEach(m => {
            const dot = document.createElement("div");
            dot.innerHTML = `
                <div style="position:relative;display:flex;align-items:center;justify-content:center">
                    <div style="position:absolute;width:24px;height:24px;border-radius:50%;background:rgba(6,182,212,0.2);animation:ping 1s cubic-bezier(0,0,0.2,1) infinite"></div>
                    <div style="width:16px;height:16px;border-radius:50%;border:2px solid white;background:#06b6d4;box-shadow:0 4px 6px rgba(0,0,0,0.3)"></div>
                </div>`;
            new maplibregl.Marker({ element: dot })
                .setLngLat([m.lng, m.lat])
                .setPopup(new maplibregl.Popup().setHTML(`<b>${m.label}</b><br>${m.description}`))
                .addTo(map);
        });

        // flyTo via custom event
        el.addEventListener("map:flyto", e => {
            map.flyTo({ center: e.detail.center, zoom: e.detail.zoom, duration: 2000 });
        });

        el.setAttribute("data-initialized", "true");
    });
});
observer.observe(document.body, { childList: true, subtree: true });

// Required for pulsing animation (Tailwind's animate-ping is not available in raw JS context)
const style = document.createElement("style");
style.textContent = `@keyframes ping { 75%,100% { transform: scale(2); opacity: 0; } }`;
document.head.appendChild(style);
```

### Loading (in map layout)

```rust
<script defer src="/cdn/maplibre-gl.min.js" />
<link rel="stylesheet" href="/cdn/maplibre-gl.css" />
<script defer src="/app_components/map_init.js" />
```

For a Leptos app, download MapLibre GL JS from https://github.com/maplibre/maplibre-gl-js/releases and place in `/public/cdn/` (same as `RUST-UI` bundles ApexCharts at `/cdn/apexcharts.5.3.6.min.js`).

## Reactivity

| Need | How |
|---|---|
| Marker updates | Mutate `data-markers` attr → JS `MutationObserver` re-reads |
| Theme change | Watch `data-theme` attr (same as `chart_init.js`) |
| flyTo from button | `web_sys::CustomEvent` dispatched from Rust, caught in JS |
| Zoom/center sync | Optional: map fires custom event back, Rust reads via `leptos::ev` |

## Reference

| | |
|---|---|
| mapcn source | https://github.com/AnmolSaini16/mapcn |
| Target demo | `flyto-example.tsx` (globe + pulsing marker + flyTo button) |
| Pattern origin | `public/app_components/chart_init.js` |
| MapLibre GL JS | https://github.com/maplibre/maplibre-gl-js/releases |
