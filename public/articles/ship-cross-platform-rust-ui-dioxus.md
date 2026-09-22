+++
title = "Ship One Rust UI Across Web, Desktop, and Mobile with Dioxus"
description = "Keep a Dioxus interface portable by separating route intent, platform effects, and visual primitives from day one."
category = "Production"
publish_date = "2026-09-12"
last_updated = "2026-09-12"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Ship cross-platform Rust UI"
keywords = ["Dioxus", "Rust", "cross-platform", "web", "desktop", "iOS"]
+++

Cross-platform Rust UI works best when platform differences have a clear home. Keep product intent and component structure shared. Isolate browser APIs, native packaging, and input quirks at the edge.

## Share route intent, not URL strings

Typed routes keep navigation consistent across web and native clients. A link to a component page should describe the destination as a route value, not depend on a browser URL being accepted by every platform.

```rust
Link {
    to: Route::ComponentPage { name: "button".to_string() },
    class: "text-sm font-medium underline underline-offset-4",
    "Read Button docs"
}
```

This matters in desktop and iOS webviews, where raw anchors can attempt to open a non-HTTP app URL instead of letting the router handle navigation.

## Keep platform effects behind a boundary

Clipboard, resize observers, storage, and browser history are useful, but they are not universal Rust UI primitives. Put each effect behind a small hook or component and gate target-specific APIs when needed.

```rust
#[cfg(target_arch = "wasm32")]
fn install_scroll_listener() {
    // Browser-only DOM work lives here.
}
```

The component can still expose the same intent on every target: `copy`, `scroll_to`, or `persist`. The implementation changes; the UI contract does not.

## Design for touch and pointer input

Do not make hover the only way to discover an action. Use visible labels, adequate hit areas, and pressed or focused states. A desktop dropdown may need a sheet or disclosure pattern on mobile.

Tailwind tokens and semantic primitives help here. A button with a real `button` element gets keyboard behavior; a `Link` gets router behavior; a decorative `div` gets neither for free.

## Treat assets as a build concern

Native targets often do not serve arbitrary public paths the same way as a browser. Bundle images, scripts, and icons through the framework asset mechanism when they must appear on iOS or desktop. Keep raw public files for assets that the server must expose directly, such as feeds and crawler metadata.

## Verify the real targets

Run a web build, then exercise the native target that matters. Check hard refreshes, route transitions, keyboard focus, safe areas, image loading, and loading states. A DOM assertion can pass while a screenshot shows broken CSS or a trapped overlay.

Cross-platform UI is not “one layout everywhere.” It is one product model with target-aware edges. Dioxus and Rust let you keep that boundary explicit instead of scattering platform checks through every component.
