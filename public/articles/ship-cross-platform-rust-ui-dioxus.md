+++
title = "Ship a Cross-Platform Rust UI with Dioxus: Web, Desktop, iOS"
description = "Ship one Dioxus codebase to web, desktop, and iOS: typed routes, platform boundaries, touch input, assets, and a target checklist. Practical Rust examples."
category = "Production"
publish_date = "2026-09-12"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Cross-platform Rust UI"
keywords = ["cross-platform Rust UI", "Dioxus desktop", "Dioxus iOS", "Dioxus mobile app", "Dioxus web", "Rust desktop app", "Dioxus assets"]
+++

**TL;DR:** One Dioxus codebase can ship to web, desktop, and mobile if platform differences have a clear home. Share routes and components, put browser-only or native-only code behind small hooks, bundle assets through the framework, and test every real target with the `dx` CLI before release. This guide shows the boundaries that keep that manageable.

## Who is this guide for?

This guide is for Rust developers who have a working Dioxus screen and want to run it as a website, a desktop app, and a mobile app without forking the code. You need the basics of components and routing; if you are earlier than that, read [your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust) first. Jump to the platform edge section if your build breaks on one target, to assets if images disappear, and to the release checklist before you ship.

---

## Can one Dioxus codebase really target web, desktop, and mobile?

**Yes: the same components render in a browser via WebAssembly, in a native webview on desktop, and in a webview on iOS and Android.** You pick the target with the `dx` CLI, not with separate projects.

```bash
dx serve --platform web
dx serve --platform desktop
dx serve --platform ios   # needs Xcode and a simulator
```

Rust/UI components are Tailwind-styled Dioxus components, so they work on every one of these targets without changes. The harder question is not "does it run" but "where do platform differences live". The rest of this guide answers that.

> **Bottom line:** Share product intent and component structure. Isolate platform APIs at the edge.

![Architecture diagram: shared Dioxus code feeds web, desktop and mobile targets, each reaching platform APIs through a small cfg-gated edge](/articles/ship-cross-platform-rust-ui-dioxus-asset-1.svg)

*Most of the code lives in the shared layer. Platform differences are confined to a thin edge of `cfg`-gated hooks and bundled assets.*

## How do you share navigation across platforms?

**Use typed routes, not URL strings.** A link described as a route value does not depend on a browser URL being accepted by every platform.

```rust
Link {
    to: Route::ComponentPage { name: "button".to_string() },
    class: "text-sm font-medium underline underline-offset-4",
    "Read Button docs"
}
```

This matters in desktop and iOS webviews, where a raw `<a href>` can try to open a non-HTTP app URL instead of letting the router handle navigation. Typed routes also make renames a compile error instead of a dead link. For responsive navigation patterns on top of this, see [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind).

## Where should platform-specific code live?

**Behind a small hook or component, gated by target.** Clipboard, resize observers, storage, and browser history are useful but not universal Rust UI primitives.

```rust
#[cfg(target_arch = "wasm32")]
fn install_scroll_listener() {
    // Browser-only DOM work lives here.
}

#[cfg(not(target_arch = "wasm32"))]
fn install_scroll_listener() {
    // Native targets do nothing, or use a native API.
}
```

The component keeps one contract on every target: `copy`, `scroll_to`, or `persist`. The implementation changes; the UI does not. Rust/UI follows this pattern in hooks such as [use_copy_clipboard](/docs/hooks/use-copy-clipboard) and [use_lock_body_scroll](/docs/hooks/use-lock-body-scroll).

| Concern | Web | Desktop | Mobile |
|---|---|---|---|
| Navigation | Browser history | In-app router | In-app router |
| Clipboard | Browser API | Native or webview API | Native or webview API |
| Assets | Public files or bundled | Bundled | Bundled |
| Input | Mouse and keyboard | Mouse and keyboard | Touch |

## How do you design for touch and pointer input?

**Never make hover the only way to discover an action.** Use visible labels, adequate hit areas (around 44px on touch), and pressed or focused states. A desktop dropdown may need a sheet or disclosure pattern on mobile.

A real `button` element gets keyboard behavior, a `Link` gets router behavior, and a decorative `div` gets neither for free. Semantic primitives plus Tailwind tokens carry most of the cross-platform accessibility work.

## How should you handle assets on each target?

**Treat assets as a build concern.** Native targets do not serve arbitrary public paths the way a browser does, so bundle images, scripts, and icons through the framework asset mechanism when they must appear on desktop or iOS.

```rust
const LOGO: Asset = asset!("/assets/logo.svg");

rsx! { img { src: LOGO, alt: "Logo" } }
```

Keep raw public files only for what a server must expose directly, such as feeds and crawler metadata like `robots.txt` and the sitemap.

## How should you structure the project for multiple targets?

**Keep one shared UI crate or module, and make each target a thin entry point.** The shared part holds components, routes, and state. The entry points only choose how to launch it.

```text
my_app/
  Cargo.toml        # features: web, desktop, mobile
  src/
    main.rs         # dioxus::launch(App), same on every target
    routes.rs       # typed Routable enum
    components/     # shared UI
    platform/       # cfg-gated hooks: clipboard, storage, files
  assets/           # bundled with asset!()
```

With Dioxus 0.7, a single `dioxus::launch(App)` call is enough, and the `dx` CLI picks the renderer from the platform flag or from Cargo features. Declare the targets as features so dependencies that only make sense on one platform stay out of the others:

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

The discipline that matters is the `platform/` module. If a component imports `web_sys` directly, the desktop build breaks. If it calls `platform::copy_to_clipboard`, every target compiles and each one provides its own implementation.

---

## What changes between the web build and native builds?

**The rendering model is the same; the runtime around it is not.** Knowing the differences up front prevents most surprises.

| Area | Web (WebAssembly) | Desktop | iOS and Android |
|---|---|---|---|
| Renderer | Browser DOM | Native webview window | Native webview |
| Threads | Single main thread, async only | Real threads available | Real threads available |
| File system | Sandboxed, no direct access | Full access with permissions | App sandbox |
| Networking | Browser `fetch`, CORS applies | Native HTTP, no CORS | Native HTTP, no CORS |
| Distribution | URL | Installer or bundle | App store or TestFlight |
| Update path | Deploy and refresh | Ship a new build or updater | Store review |

Two consequences are worth stating plainly. First, code that works on the web can fail natively because it assumes the browser: `window`, `localStorage`, and CORS-shaped workarounds simply do not exist. Second, code that works natively can fail on the web because it assumes threads or a file system. Writing against a small trait or function boundary makes both failures a compile-time problem instead of a runtime one.

---

## How do you handle safe areas, notches, and system UI?

**Pad with the environment insets, and test on a device with a notch.** On iOS, content can slide under the status bar, the home indicator, or the notch if you ignore safe areas.

```rust
rsx! {
    div {
        class: "min-h-screen",
        style: "padding-top: env(safe-area-inset-top); padding-bottom: env(safe-area-inset-bottom);",
        // app content
    }
}
```

`env(safe-area-inset-*)` is plain CSS, so it works in any webview and is ignored when there is nothing to inset. Pair it with the viewport meta tag `viewport-fit=cover` so the app is allowed to draw edge to edge. Also remember the on-screen keyboard: a fixed bottom bar can end up hidden behind it, so test forms on a real phone. The responsive rules in [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind) apply unchanged inside the mobile webview.

---

## How do you persist data across targets?

**Define a storage interface and implement it per platform.** Browser `localStorage` is convenient on the web and unavailable natively, where a file or a small database is the right tool.

```rust
pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str);
}
```

Components receive a `Store` through context (`use_context_provider` at the root) and never know which one they got. Preferences, auth tokens, and drafts all go through it. Keep secrets out of plain storage on every target: use the platform keychain on native builds and short-lived, HTTP-only cookies on the web. For data that must sync across devices, the source of truth belongs on a server, which brings back the loading and error states from [async data and loading states in Dioxus](/articles/async-data-loading-states-dioxus).

---

## How do you package and release each target?

**Build a release artifact per platform, and verify it on a clean machine.** `dx serve` is for development. A release is a different build with optimizations on and assets bundled.

```bash
dx build --release --platform web       # static files for any host
dx bundle --release --platform desktop  # installer or app bundle
dx bundle --release --platform ios      # needs signing and a provisioning profile
```

- **Web:** upload the output to any static host or serve it from your own server. Set long cache headers on hashed assets.
- **Desktop:** produce a signed bundle per operating system. Unsigned apps trigger scary OS warnings that cost you users.
- **iOS and Android:** plan for signing, store listings, review time, and privacy declarations early. These take longer than the code.

Automate what you can: a CI job per target that builds the release artifact catches broken feature flags and missing assets before a human does. Run each artifact on a machine that has never seen your toolchain. Missing fonts, absent system libraries, and wrong asset paths only show up there.

---

## How do you debug a target-specific failure?

**Reproduce on the failing target first, then bisect between shared and edge code.** Most cross-platform bugs fall into three buckets, and naming the bucket saves hours.

- **It does not compile on one target.** A platform-only crate or API leaked into shared code. The compiler error names the symbol; move it behind a `cfg` and a function in `platform/`.
- **It compiles but behaves differently.** Usually an assumption about the runtime: a missing browser API, a different clipboard, a CORS rule that only exists on the web, or threading.
- **It works in development but not in the release build.** Assets not bundled, a feature flag that differs, or optimizations exposing a race. Always test the release artifact, not just `dx serve`.

Add logging that works everywhere (the `tracing` crate with a platform subscriber) so you can read the same messages in the browser console, the desktop terminal, and the device log. When a bug appears on only one platform, write a one-line note of the platform fact that caused it in the `platform/` module. That file becomes the team's map of every place the targets genuinely differ.

---

## What are the common cross-platform mistakes?

| Mistake | Symptom | Fix |
|---|---|---|
| `web_sys` calls in shared components | Desktop build fails to compile | Move behind a `platform` function with `cfg` |
| Raw `<a href>` for internal links | Navigation breaks in webviews | Use `Link { to: Route::... }` |
| Public-folder image paths | Images missing on desktop and iOS | Use `asset!()` |
| Hover-only controls | Unusable on touch | Visible labels and focus states |
| Assuming a browser clipboard | Copy button does nothing natively | Use a hook such as [use_copy_clipboard](/docs/hooks/use-copy-clipboard) |
| Testing only the web build | Native-only bugs reach users | Exercise every target before release |

---

## How do you decide which platforms to ship first?

**Start where your users are, and add targets as the shared layer proves itself.** Most teams ship the web build first because deployment is a URL, then add desktop when offline use or file access matters, and mobile when distribution through a store pays off. Each new target adds testing and release work, even when the code is shared, so choose the next one based on a real user need, not on the fact that the framework allows it.

---

## What should you verify before shipping?

**Run the web build, then exercise every native target that matters.** A DOM assertion can pass while a screenshot shows broken CSS or a trapped overlay.

1. Hard refresh on a deep route (web).
2. Route transitions and the back button on each target.
3. Keyboard focus order and visible focus rings.
4. Safe areas and notches on iOS.
5. Image and font loading from bundled assets.
6. Loading, empty, and error states, as in [async data and loading states in Dioxus](/articles/async-data-loading-states-dioxus).

Cross-platform UI is not "one layout everywhere". It is one product model with target-aware edges. If you are just starting, build the first screen with [your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust), and design components for reuse with [typed props](/articles/reusable-dioxus-components-typed-props) so they travel between targets cleanly.

## Frequently Asked Questions

### Can Dioxus build native desktop apps?

Yes. Dioxus desktop renders your components in a native webview window. Run `dx serve --platform desktop` during development and bundle with `dx bundle` for release.

### Does Dioxus support iOS and Android?

Yes, through webview-based mobile targets driven by the `dx` CLI. iOS needs Xcode and a simulator or device. Mobile support is newer than web and desktop, so test on real hardware.

### Do I need separate code for each platform?

Mostly no. Components, routes, and state are shared. You only write target-specific code for platform APIs such as clipboard, storage, or file access, and you can gate it with `#[cfg(...)]`.

### How do I use browser APIs without breaking the desktop build?

Put them in a function gated with `#[cfg(target_arch = "wasm32")]` and provide a no-op or native alternative for other targets, so the shared component calls one function everywhere.

### Why does my image not load on desktop or iOS?

Native targets may not serve your public folder like a web server. Reference images through the `asset!` macro so they are bundled with the app.

### Is Dioxus desktop the same as Tauri?

Both use a webview, but Dioxus lets you write the UI in Rust, while Tauri typically pairs a JavaScript frontend with a Rust backend. The [Dioxus vs Tauri comparison](https://rustify.rs/articles/dioxus-vs-tauri-2026) goes into the trade-offs.

### How big is a Dioxus desktop app?

Dioxus desktop uses the system webview instead of bundling a browser engine, so binaries are usually far smaller than Electron apps. Exact size depends on your dependencies and assets, so measure a release build.

### Can I share a Tailwind design system across web and mobile?

Yes. All Dioxus targets that render through a webview understand the same CSS, so Tailwind classes and design tokens work everywhere. Components from Rust/UI are built that way.

## Sources

- [Dioxus documentation](https://dioxuslabs.com/learn/0.7/)
- [Dioxus CLI on GitHub](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli)
- [MDN: WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [MDN: env() and safe-area-inset](https://developer.mozilla.org/en-US/docs/Web/CSS/env)
