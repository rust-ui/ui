+++
title = "Dioxus Router: Typed Routes, Layouts, and Nested Navigation"
description = "Learn the Dioxus Router with typed routes, dynamic segments, nested layouts, Outlet, redirects, and programmatic navigation. Real Rust examples for web and desktop."
category = "Foundations"
publish_date = "2026-10-25"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Dioxus Router guide"
keywords = ["Dioxus router", "Dioxus routing", "Dioxus typed routes", "Dioxus layout Outlet", "Dioxus navigation", "Routable derive", "Dioxus Link"]
+++

The Dioxus Router is a type-safe router: you describe every page as a variant of a Rust enum that derives `Routable`, and the compiler checks every link, parameter, and redirect. A misspelled route is a build error instead of a 404 in production.

**TL;DR:** Define one `enum Route` with `#[derive(Routable)]`. Give each variant a `#[route("/path")]` attribute, use `:param` segments for dynamic data, and wrap groups of pages in `#[layout(...)]` components that render children through `Outlet::<Route> {}`. Navigate with `Link { to: Route::... }` or `use_navigator()`, never with raw `<a href>` for internal pages.

- **Type safety:** `Route::ArticlePage { slug }` replaces string URLs, so renames fail at compile time.
- **Layouts:** nested `#[layout]` blocks share headers, sidebars, and footers without repeating them.
- **Dynamic routes:** `:slug` segments become typed fields on the variant.
- **Redirects:** `#[redirect]` keeps old URLs alive, which protects SEO.
- **Cross-platform:** the same enum drives web, desktop, and mobile navigation.

---

## Who is this guide for?

This guide is for Dioxus developers who have a working component and now need multiple pages: docs sites, dashboards, or multi-screen apps. You should know `rsx!` and components; if not, start with [your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust). Jump to layouts if you repeat a header on every page, to dynamic segments for detail pages, and to the SEO section if the site is public.

---

## What is a typed route in Dioxus?

**A typed route is an enum variant that represents a page, and its fields are the URL parameters.** The router parses the URL into the variant and renders the component with the same name.

```rust
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/articles")]
    Articles {},
    #[route("/articles/:slug")]
    Article { slug: String },
}

#[component]
fn Home() -> Element {
    rsx! { h1 { "Home" } }
}

#[component]
fn Articles() -> Element {
    rsx! { h1 { "Articles" } }
}

#[component]
fn Article(slug: String) -> Element {
    rsx! { h1 { "Reading {slug}" } }
}

fn app() -> Element {
    rsx! { Router::<Route> {} }
}
```

Each variant needs a component with the same name, and the component's props must match the variant's fields. If you add a field to a variant and forget the component, the build fails. This is the central benefit compared with string-based routers, where the same mistake shows up when a user clicks.

| Approach | Link checked at compile time | Params typed | Refactor safety |
|---|---|---|---|
| Typed `Routable` enum (Dioxus) | Yes | Yes | Compiler finds every use |
| String paths (`"/articles/" + slug`) | No | No | Search and hope |
| Manual `match` on the URL | Partly | No | Easy to forget a branch |

> **Bottom line:** Treat the `Route` enum as the sitemap of your app. If a page is not in the enum, it does not exist, and every link to it is verified by the compiler.

---

## How do layouts and Outlet work?

**A layout is a component that wraps a group of routes and renders the active child with `Outlet::<Route> {}`.** Put shared chrome there: header, sidebar, footer, and providers.

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[layout(ArticlesLayout)]
            #[route("/articles")]
            Articles {},
            #[route("/articles/:slug")]
            Article { slug: String },
        #[end_layout]
    #[end_layout]
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        header { class: "border-b border-border p-4",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Articles {}, class: "ml-4", "Articles" }
        }
        main { Outlet::<Route> {} }
        footer { class: "p-4 text-sm text-muted-foreground", "Footer" }
    }
}

#[component]
fn ArticlesLayout() -> Element {
    rsx! {
        div { class: "mx-auto max-w-3xl px-4 py-8",
            Outlet::<Route> {}
        }
    }
}
```

Layouts nest. `AppLayout` wraps everything, and `ArticlesLayout` wraps only the article pages inside it, so the reading width applies there and nowhere else. The Rust/UI site uses exactly this shape: an app layout for the header and footer, a home layout, an articles layout, and a docs layout with a sidebar.

![Route tree: an AppLayout wraps a HomeLayout and an ArticlesLayout, each rendering child routes through an Outlet](/articles/dioxus-router-typed-routes-layouts-asset-1.svg)

*Each layout renders its active child through an `Outlet`. Nested layouts compose, so the shell is written once.*

Layouts are also where state that must survive navigation belongs. A signal created in `AppLayout` is not reset when you move between child routes, because the layout component stays mounted. A signal created in a page component is dropped when the user leaves the page.

---

## How do dynamic segments and query parameters work?

**A `:name` segment becomes a field on the variant, and `?:param` captures query strings.** Both are parsed into the types you declare.

```rust
#[route("/docs/components/:name")]
Component { name: String },

#[route("/search?:q&:page")]
Search { q: String, page: u32 },
```

Any type that implements `FromStr` (and `Display` for building links) can be a segment, including integers and your own enums. If parsing fails, for example `page=abc` where a `u32` is required, the router falls through to your not-found route instead of passing garbage to your component. Because the data is typed, you can use it directly:

```rust
#[component]
fn Search(q: String, page: u32) -> Element {
    let results = use_resource(move || async move { search_api(&q, page).await });
    // loading, empty, error, ready: see the async guide
    rsx! { /* ... */ }
}
```

Fetching on a route parameter is exactly the case `use_resource` handles well: when the segment changes, the resource re-runs. The four states from [async data and loading states in Dioxus](/articles/async-data-loading-states-dioxus) apply to every detail page.

A catch-all route handles 404s. Place it last so it only matches when nothing else did:

```rust
#[route("/:..segments")]
NotFound { segments: Vec<String> },
```

Return a real not-found screen with a link home, and on server-rendered sites make sure the HTTP status is 404, not 200. Search engines treat a 200 on a missing page as a soft 404, which wastes crawl budget.

---

## How do you navigate between pages?

**Use `Link` for anything the user clicks, and `use_navigator()` for navigation triggered by logic.** `Link` renders an anchor with the correct `href`, so it works with middle-click, copy link, and crawlers.

```rust
rsx! {
    Link {
        to: Route::Article { slug: "dioxus-dark-mode-tailwind-theming".to_string() },
        class: "underline underline-offset-4",
        "Read the dark mode guide"
    }
}
```

For navigation after an action, such as redirecting to a dashboard after sign-in:

```rust
let nav = use_navigator();

let on_done = move |_| {
    nav.push(Route::Articles {});      // adds a history entry
    // nav.replace(Route::Articles {}); // replaces the current entry
};
```

Use `push` when the user should be able to press Back and return. Use `replace` for transitions where going back makes no sense, such as after a login form. For the active-link styling in navigation, compare `use_route::<Route>()` with the target. Rust/UI provides [Navigation Menu](/docs/components/navigation_menu) and [Breadcrumb](/docs/components/breadcrumb) components that you can drive with `Link`, plus [Pagination](/docs/components/pagination) for list pages.

On desktop and iOS, internal navigation through `Link` stays inside the app. A raw `<a href="/articles">` may try to open an external URL, which is why typed links matter more as you add targets, as covered in [shipping a cross-platform Rust UI](/articles/ship-cross-platform-rust-ui-dioxus).

---

## How do redirects keep old URLs working?

**Declare `#[redirect]` entries so retired URLs forward to the new ones.** This matters for public sites: external links and search results keep pointing at old paths for years.

```rust
#[redirect("/components", || Route::ComponentsIndex {})]
#[redirect("/components/:name", |name: String| Route::Component { name })]
```

A redirect closure receives the same parameters as the route and returns the new variant, so you can even add logic, such as sending a retired component name to a hook page. When you rename a URL, add a redirect in the same change. On a server-rendered site, prefer a permanent redirect status so search engines transfer ranking to the new address.

---

## How should routing support SEO and AI search?

**Give every public page one stable, descriptive URL, render real content on the server, and expose clean metadata.** Crawlers and AI answer engines read the HTML they receive; they do not wait for a client-side router to figure out the page.

- Use readable, lowercase, hyphenated slugs: `/articles/dioxus-router-typed-routes-layouts`, not `/a/123`.
- Render the page body on the server (Dioxus fullstack with server-side rendering) so the first response contains the article, not an empty shell.
- Set a unique `<title>`, meta description, and canonical URL per route. Each route component should emit its own head tags.
- Link every page from at least one other page with a real `Link`. Pages with no internal links are discovered late, if ever.
- List public URLs in a sitemap and keep it in sync with the `Route` enum.
- Keep one URL per piece of content. If two routes show the same page, add a canonical link or a redirect.

The structure of this very site follows the list: articles live under `/articles/:slug`, categories under `/articles/category/:category`, and each article page links to related ones.

| Routing choice | Effect on search and AI citation |
|---|---|
| Readable slug | Clear topic signal, better link text when cited |
| Server-rendered body | Content visible to every crawler on first fetch |
| Canonical URL | Consolidates duplicates into one ranking page |
| 301 redirect on rename | Preserves existing links and ranking |
| Soft 404 (200 status) | Wastes crawl budget, can be dropped from the index |

---

## How do you test and debug routes?

**Test the parsing and the rendering separately.** Because a route is a plain enum, you can unit test URL parsing without a browser: convert a string to a `Route` with `FromStr`, assert the variant and its fields, then convert it back with `to_string()` and assert the round trip. A round trip that does not match usually means a segment type formats differently from how it parses, for example a `String` with spaces or an enum whose `Display` does not mirror `FromStr`.

```rust
#[test]
fn article_route_round_trips() {
    let route: Route = "/articles/dioxus-dark-mode-tailwind-theming".parse().unwrap();
    assert_eq!(
        route,
        Route::Article { slug: "dioxus-dark-mode-tailwind-theming".to_string() }
    );
    assert_eq!(route.to_string(), "/articles/dioxus-dark-mode-tailwind-theming");
}
```

For rendering problems, check these in order. First, confirm the variant name matches a component name in scope. Second, confirm every layout renders an `Outlet`. Third, confirm the component props match the variant fields exactly, including types. Fourth, log `use_route::<Route>()` to see what the router actually matched, which immediately reveals a catch-all swallowing a real URL. Most routing bugs are one of those four.

When a page works on first load but breaks after client-side navigation, look for state that was initialized in the component body instead of a hook. Components are re-rendered, not re-created, when only the parameter changes, so a `use_signal(|| slug.clone())` keeps the first slug forever. Derive values from props in a `use_memo` or key the component so it remounts.

---

## How do you organize routes in a growing app?

**Group routes by feature, give each group a layout, and keep the enum flat enough to read.** A single file with a 40-variant enum is fine when the layouts show the structure. The pattern that scales is one layout per section (marketing, articles, docs, dashboard), with each layout owning the chrome and the data providers its pages need.

| Section | Layout responsibility | Typical routes |
|---|---|---|
| Marketing | Header, footer, hero spacing | `/`, `/pricing` |
| Articles | Reading width, related links | `/articles`, `/articles/:slug` |
| Docs | Sidebar, search, table of contents | `/docs/components/:name` |
| Dashboard | Auth guard, sidebar, user context | `/app`, `/app/settings` |

An auth guard is simply a layout that checks a signal or context and renders a redirect or a login screen instead of the `Outlet`. Because every protected page sits inside that layout, you write the check once and forget to protect no page. Pair this with the loading states in the async guide so the guard shows a skeleton while the session loads, not a flash of the login form.

Keep route component files small. A route component should fetch or select its data, then delegate rendering to reusable components, as described in [reusable Dioxus components with typed props](/articles/reusable-dioxus-components-typed-props). That separation keeps the router file readable and the UI testable, and it lets the same presentational component serve several routes.

Finally, think about URL design before you write the enum. URLs are a public interface: people bookmark them, search engines index them, and AI assistants cite them. Choose nouns over verbs, keep hierarchy shallow, avoid ids when a slug will do, and never reuse a URL for different content. Changing the enum later is cheap; changing indexed URLs is not, which is why redirects exist.

---

## How do responsive and themed layouts fit in?

**Layouts are the natural home for responsive shells and theme providers.** A docs layout can show a persistent sidebar on large screens and a sheet on small ones, driven by the same `Route` links in both cases. See [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind) for the breakpoint patterns, and the [Sheet](/docs/components/sheet) component for the mobile drawer. The theme class from [dark mode in Dioxus with Tailwind](/articles/dioxus-dark-mode-tailwind-theming) is applied once at the app layout, so every route inherits it with no per-page code.

---

## What are the common routing mistakes?

| Mistake | Symptom | Fix |
|---|---|---|
| Raw `<a href>` for internal links | Full reload, broken in webviews | Use `Link { to: Route::... }` |
| Missing `Outlet` in a layout | Child pages render blank | Add `Outlet::<Route> {}` |
| Catch-all route placed first | Every URL hits the 404 page | Put `/:..segments` last |
| State in a page component | State resets on navigation | Lift it into a layout or context |
| Renaming a URL without a redirect | Old links and rankings are lost | Add `#[redirect]` |
| Parsing params by hand | Panics on bad input | Use typed segments |
| One giant layout | Header logic everywhere | Nest layouts per section |

For forms that navigate on success, combine `use_navigator` with the submit state from [Dioxus forms, state, and validation](/articles/dioxus-forms-state-validation).

---

## Frequently Asked Questions

### What is the Dioxus Router?

The Dioxus Router is the official routing library for Dioxus. It maps URLs to components using a Rust enum that derives `Routable`, and it works on web, desktop, and mobile targets.

### How do I add a dynamic route in Dioxus?

Put a `:name` segment in the route attribute, such as `#[route("/articles/:slug")]`, and add a matching field on the variant, `Article { slug: String }`. The component receives `slug` as a prop.

### What does Outlet do in Dioxus?

`Outlet::<Route> {}` renders the currently active child route inside a layout. Without it, a layout shows its own markup but never the page the user navigated to.

### How do I navigate programmatically in Dioxus?

Call `use_navigator()` to get a navigator, then use `push` to add a history entry or `replace` to swap the current one. Use `Link` for ordinary clickable navigation.

### How do I handle a 404 page in Dioxus?

Add a catch-all route like `#[route("/:..segments")]` as the last variant and render a not-found screen. On server-rendered sites, also return a 404 HTTP status.

### Can I share layouts between routes?

Yes. Wrap routes in `#[layout(MyLayout)]` and close the group with `#[end_layout]`. Layouts nest, so you can have an app-wide shell and a section-specific wrapper.

### Does the Dioxus Router work on desktop and mobile?

Yes. The same `Routable` enum drives navigation in a native webview. Use `Link` and `use_navigator` instead of raw anchors so navigation stays inside the app.

### How do I keep old URLs working after a rename?

Add a `#[redirect]` entry mapping the old path to the new `Route` variant. For public websites, serve it as a permanent redirect so search engines update their index.

---

## Sources

- [Dioxus documentation: router](https://dioxuslabs.com/learn/0.7/)
- [Dioxus on GitHub](https://github.com/DioxusLabs/dioxus)
- [Google Search Central: redirects and Google Search](https://developers.google.com/search/docs/crawling-indexing/301-redirects)
- [Google Search Central: soft 404 errors](https://developers.google.com/search/docs/crawling-indexing/http-network-errors)
