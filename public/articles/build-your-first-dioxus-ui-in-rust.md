+++
title = "Build Your First Dioxus UI in Rust"
description = "A practical path from a blank Dioxus app to a small, composable Rust interface with typed props, signals, and Tailwind CSS."
category = "Foundations"
publish_date = "2026-09-22"
last_updated = "2026-09-22"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Your first Dioxus UI"
keywords = ["Dioxus", "Rust", "UI", "Tailwind CSS", "signals"]
+++

Building a UI in Rust feels different when the first goal is not learning every framework feature. Start with one screen, one state transition, and one component boundary. Dioxus gives you a Rust function that returns an `Element`; the rest is composition.

## Start with a small screen

A useful first screen has a visible hierarchy: page shell, heading, supporting copy, and one interactive control. Keep that shape boring. It makes styling and state bugs easy to isolate.

```rust
use dioxus::prelude::*;

#[component]
fn Welcome() -> Element {
    rsx! {
        main { class: "mx-auto max-w-xl px-6 py-16",
            p { class: "text-sm font-medium text-muted-foreground", "Rust/UI" }
            h1 { class: "mt-3 text-4xl font-semibold tracking-tight", "Build with Rust" }
            p { class: "mt-4 text-muted-foreground",
                "Typed components, reactive state, and a UI you can ship everywhere."
            }
        }
    }
}
```

The `rsx!` tree stays close to the HTML you want. Classes remain ordinary Tailwind strings, so you can move from a sketch to a production layout without inventing a second styling language.

## Add state with a signal

Signals are the smallest useful state primitive for a Dioxus screen. Read a signal in RSX and Dioxus tracks the dependency. Set it from an event and only the affected view needs to update.

```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            class: "rounded-md bg-primary px-4 py-2 text-primary-foreground",
            onclick: move |_| count += 1,
            "Clicked {count} times"
        }
    }
}
```

Use local signals for local UI state: open menus, selected tabs, draft input, and optimistic feedback. Move state upward only when two sibling components need the same source of truth.

## Split at a meaningful boundary

Component boundaries should describe UI responsibilities, not every `div`. A `PageHeader`, `ArticleCard`, or `FilterBar` can own a stable visual contract. Tiny wrappers usually add noise.

```rust
#[component]
fn PageHeader(title: String, description: String) -> Element {
    rsx! {
        header { class: "max-w-2xl",
            h1 { class: "text-3xl font-semibold tracking-tight", "{title}" }
            p { class: "mt-3 text-lg text-muted-foreground", "{description}" }
        }
    }
}
```

Props are Rust values. This means component APIs are visible in code review, autocomplete, and compiler errors. Prefer a small prop surface. Add a prop when it changes behavior or content, not when it merely mirrors a wrapper class.

## Keep styling composable

Start with layout classes on the page and visual classes inside the component. Use design tokens such as `bg-background`, `text-foreground`, `border-input`, and `text-muted-foreground` instead of hard-coded colors. Dark mode then becomes a theme concern, not a page rewrite.

The useful loop is short: render one block, check spacing at mobile width, add the next block, then extract repeated patterns. Rust gives you structure; Dioxus keeps the feedback loop close to the browser.

## What to build next

Once the first screen works, add one real route and one loading or empty state. That exposes the decisions that matter in a real product: navigation, state ownership, content density, and feedback. Keep each change small enough that the rendered result stays understandable.
