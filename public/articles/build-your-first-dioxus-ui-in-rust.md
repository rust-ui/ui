+++
title = "How to Build Your First Dioxus UI in Rust (2026)"
description = "Build your first Dioxus UI in Rust: set up a project, write RSX, add signals, split components, and style with Tailwind CSS. Step by step with code."
category = "Foundations"
publish_date = "2026-09-22"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Your first Dioxus UI"
keywords = ["Dioxus tutorial", "Rust UI", "Dioxus Tailwind CSS", "Dioxus signals", "Dioxus components", "build UI in Rust"]
+++

**TL;DR:** A Dioxus UI is a Rust function that returns an `Element`. Create a project with `dx new`, write markup with `rsx!`, keep state in `use_signal`, extract a component only when it owns a real responsibility, and style with Tailwind classes. This guide builds one small screen end to end, so you finish with something that runs on web, desktop, and mobile.

- **Setup:** `cargo install dioxus-cli`, then `dx new` and `dx serve`
- **Markup:** `rsx!` inside a `#[component]` function
- **State:** `use_signal` for state, `use_memo` for derived values
- **Routing:** a `Routable` enum, checked at compile time
- **Styling:** Tailwind classes with design tokens

---

## What is Dioxus and why use it for a Rust UI?

**Dioxus is a Rust framework for building user interfaces that run on the web, desktop, and mobile from one codebase.** You describe the UI in Rust with the `rsx!` macro, hold state in signals, and the same components render in a browser (WebAssembly), in a native webview, or on a phone.

The appeal for Rust developers is that the compiler checks your UI. Props are typed, events are typed, and a missing field is a compile error instead of a blank screen at runtime. If you are still choosing a framework, the [Leptos vs Dioxus comparison](https://rustify.rs/articles/leptos-vs-dioxus-rust-frontend-2026) covers the trade-offs in depth. This article assumes you picked Dioxus and want a working first screen.

> **Bottom line:** Dioxus fits you if you want React-like components with Rust types and one codebase for web, desktop, and mobile.

## How do you set up a Dioxus project?

**Install the Dioxus CLI, create a project, and run the dev server.** You need a recent stable Rust toolchain first.

```bash
cargo install dioxus-cli
dx new my-first-ui
cd my-first-ui
dx serve
```

`dx serve` starts a dev server with hot reloading for RSX changes. Add `--platform desktop` to open the same app in a native window instead of the browser.

Dioxus 0.7 detects a `tailwind.css` file at the project root and runs Tailwind for you, so there is no separate build step to wire up. If your template did not create one, add `@import "tailwindcss";` to a `tailwind.css` file and restart `dx serve`.

## How do you write your first screen with RSX?

**Write a component function marked with `#[component]` that returns `rsx!` markup.** Start with one screen and a clear hierarchy: page shell, heading, supporting copy, and one interactive control. Keep that shape boring, because it makes styling and state bugs easy to isolate.

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

The `rsx!` tree stays close to the HTML you want. Classes are ordinary Tailwind strings, so you move from a sketch to a production layout without learning a second styling language.

## How does state work with signals?

**A signal is a reactive value: read it in RSX and Dioxus re-renders only what depends on it.** `use_signal` is the smallest useful state primitive for a screen.

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

Use local signals for local UI state: open menus, selected tabs, draft input, and optimistic feedback. Move state upward only when two sibling components need the same source of truth. When state comes from the network instead, give it an explicit type, as shown in [loading states for async data in Dioxus](/articles/async-data-loading-states-dioxus).

| State kind | Where it lives | Example |
|---|---|---|
| Local UI state | `use_signal` inside the component | Open menu, selected tab |
| Shared UI state | Signal in a parent, passed down or via context | Theme, sidebar open |
| Server state | `use_resource` or a server function | List of projects |
| Form state | A struct in a signal | Signup values and errors |

## When should you split a component?

**Split at a UI responsibility, not at every `div`.** A `PageHeader`, `ArticleCard`, or `FilterBar` can own a stable visual contract. Tiny wrappers usually add noise.

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

Props are plain Rust values, so component APIs show up in code review, autocomplete, and compiler errors. Prefer a small prop surface: add a prop when it changes behavior or content, not when it merely mirrors a wrapper class. The follow-up guide on [reusable Dioxus components with typed props](/articles/reusable-dioxus-components-typed-props) goes deeper on variants, slots, and the `class` escape hatch.

## How do you style a Dioxus app with Tailwind CSS?

**Put layout classes on the page and visual classes inside the component, and use design tokens instead of hard-coded colors.** Tokens such as `bg-background`, `text-foreground`, `border-input`, and `text-muted-foreground` make dark mode a theme concern instead of a page rewrite.

You do not have to write every primitive yourself. [Rust/UI](/docs/components) is a registry of Tailwind-styled Dioxus components you copy into your project. A [Button](/docs/components/button) or [Card](/docs/components/card) drops straight into the screen above, and the [installation guide](/docs/installation) shows the one-command setup.

The useful loop is short: render one block, check spacing at mobile width, add the next block, then extract repeated patterns. For the mobile-first part, see [responsive layouts with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind).

## Who is this guide for?

**This guide is for Rust developers who have never built a UI with Dioxus, and for frontend developers who know React and want to see how the same ideas map to Rust.** You should be comfortable with structs, enums, and `Option`. You do not need to know async Rust yet.

If you already have a project running and want to go deeper on a specific topic, jump to [reusable components with typed props](/articles/reusable-dioxus-components-typed-props), [forms and validation](/articles/dioxus-forms-state-validation), or [async loading states](/articles/async-data-loading-states-dioxus). This article is the foundation they all build on.

---

## How does Dioxus compare to React for someone who knows React?

**Dioxus deliberately borrows React's mental model: components are functions, state is local by default, and the UI is a function of state.** The differences come from Rust, not from the architecture.

| React concept | Dioxus equivalent | What changes |
|---|---|---|
| Function component | `#[component] fn Name() -> Element` | Props are typed function arguments |
| JSX | `rsx! { ... }` macro | Braces and commas instead of angle brackets |
| `useState` | `use_signal` | You mutate the signal directly, no setter function |
| `useMemo` | `use_memo` | Dependencies are tracked automatically |
| `useEffect` | `use_effect` | Dependencies are tracked automatically |
| `useContext` | `use_context` / `use_context_provider` | Context values are typed |
| React Router | `#[derive(Routable)]` enum | Routes are a Rust enum checked at compile time |
| `children` prop | `children: Element` | Same idea, typed as `Element` |

The biggest day-to-day difference is dependency tracking. In React you write dependency arrays and forget one. In Dioxus, reading a signal inside `use_memo`, `use_effect`, or `use_resource` subscribes to it automatically, so the class of "stale closure" bugs mostly disappears.

> **Bottom line:** If you can read a React component, you can read a Dioxus one. The compiler removes several categories of runtime bugs you would otherwise hit.

---

## What is the difference between use_signal, use_memo, and use_effect?

**Use `use_signal` for state you change, `use_memo` for values you derive from state, and `use_effect` for side effects that must run after rendering.** Mixing these up is the most common source of confusing beginner code.

```rust
#[component]
fn Cart() -> Element {
    let mut quantity = use_signal(|| 1);
    let unit_price = 12.5_f64;

    // Derived value: recomputed only when `quantity` changes.
    let total = use_memo(move || f64::from(quantity()) * unit_price);

    // Side effect: runs after render when `total` changes.
    use_effect(move || {
        tracing::info!("cart total is now {}", total());
    });

    rsx! {
        div { class: "flex items-center gap-4",
            button { onclick: move |_| quantity -= 1, disabled: quantity() <= 1, "-" }
            span { "{quantity}" }
            button { onclick: move |_| quantity += 1, "+" }
            p { class: "font-medium", "Total: {total:.2}" }
        }
    }
}
```

| Hook | Use it for | Do not use it for |
|---|---|---|
| `use_signal` | Values the user changes: input text, toggles, counters | Values you can compute from other state |
| `use_memo` | Derived values: totals, filtered lists, formatted strings | Side effects such as logging or network calls |
| `use_effect` | Logging, analytics, syncing to storage or the DOM | Computing values for rendering (use `use_memo`) |
| `use_resource` | Async data that depends on signals | Simple synchronous state |

A rule that prevents most bugs: if you are tempted to call `.set()` on a signal inside `use_effect` just to mirror another signal, you almost certainly want `use_memo` instead.

![Diagram of Dioxus reactive flow: a signal feeds a memo, the memo feeds rsx, rsx renders the DOM, and effects run side effects after reads](/articles/build-your-first-dioxus-ui-in-rust-asset-1.svg)

*Data flows one way: signals hold state, memos derive from it, `rsx!` describes the UI, and effects sit on the side for work that leaves the render.*

---

## How do you build a complete example screen?

**Combine one list, one filter, and one derived value into a single screen.** A small project list covers signals, `use_memo`, keyed loops, and component props in about 60 lines.

```rust
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Project {
    id: u32,
    name: String,
    done: bool,
}

#[component]
fn ProjectList() -> Element {
    let mut projects = use_signal(|| {
        vec![
            Project { id: 1, name: "Landing page".into(), done: true },
            Project { id: 2, name: "Billing".into(), done: false },
            Project { id: 3, name: "Onboarding".into(), done: false },
        ]
    });
    let mut query = use_signal(String::new);

    // Derived: filtered list, recomputed when `projects` or `query` change.
    let visible = use_memo(move || {
        let needle = query().to_lowercase();
        projects()
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&needle))
            .collect::<Vec<_>>()
    });
    let remaining = use_memo(move || projects().iter().filter(|p| !p.done).count());

    rsx! {
        section { class: "mx-auto max-w-xl space-y-4 p-6",
            h2 { class: "text-xl font-semibold", "Projects ({remaining} left)" }
            input {
                class: "w-full rounded-md border bg-background px-3 py-2",
                placeholder: "Filter projects",
                value: query(),
                oninput: move |event| query.set(event.value()),
            }
            ul { class: "space-y-2",
                for project in visible() {
                    li {
                        key: "{project.id}",
                        class: "flex items-center justify-between rounded-lg border p-3",
                        span { class: if project.done { "line-through text-muted-foreground" } else { "" },
                            "{project.name}"
                        }
                        button {
                            class: "text-sm underline",
                            onclick: move |_| {
                                if let Some(p) = projects.write().iter_mut().find(|p| p.id == project.id) {
                                    p.done = !p.done;
                                }
                            },
                            if project.done { "Reopen" } else { "Done" }
                        }
                    }
                }
            }
        }
    }
}
```

Three details are worth copying into your own code:

1. **`key` on list items.** Giving each `li` a stable `key` lets Dioxus move and update items correctly when the list changes, instead of recreating them.
2. **`projects.write()`** gives mutable access inside an event handler. The signal notifies subscribers when the write guard drops.
3. **Derived values live in `use_memo`.** `visible` and `remaining` are never stored as separate state, so they can never drift out of sync with `projects`.

To make this look polished without writing every class yourself, replace the raw `input` and `button` with the [Rust/UI Input](/docs/components/input) and [Button](/docs/components/button) components, and wrap each row in a [Card](/docs/components/card).

---

## How do you add routing to a Dioxus app?

**Define your routes as a Rust enum with `#[derive(Routable)]`, then render `Router::<Route> {}` at the root.** Each variant becomes a route, and path parameters become typed fields.

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/projects/:id")]
        ProjectDetail { id: u32 },
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        header { class: "border-b p-4",
            Link { to: Route::Home {}, class: "font-semibold", "My App" }
        }
        main { class: "p-6", Outlet::<Route> {} }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Link { to: Route::ProjectDetail { id: 1 }, "Open project 1" }
    }
}

#[component]
fn ProjectDetail(id: u32) -> Element {
    rsx! { h1 { "Project {id}" } }
}

fn main() {
    dioxus::launch(App);
}
```

Because `id` is a `u32`, a URL like `/projects/abc` simply does not match. You never parse the parameter by hand, and renaming a route is a compile error everywhere it is used. The `#[layout]` attribute wraps child routes in a shared shell, and `Outlet` marks where the child renders.

---

## What are the most common beginner mistakes in Dioxus?

**Most first-week problems come from fighting the reactive model instead of using it.** These are the ones that show up most often.

| Mistake | Symptom | Fix |
|---|---|---|
| Storing a derived value in its own signal | Two values drift out of sync | Compute it with `use_memo` |
| Forgetting `key` in a `for` loop | Inputs lose focus or state jumps between rows | Add a stable `key: "{id}"` |
| Calling `.set()` in `use_effect` to mirror state | Extra renders or loops | Use `use_memo` for derived values |
| Reading a signal outside a reactive scope | UI never updates | Read it inside `rsx!`, `use_memo`, `use_effect`, or `use_resource` |
| One giant component | Hard to read, everything re-renders together | Extract components at UI responsibilities |
| Hard-coded colors everywhere | Dark mode becomes a rewrite | Use tokens like `bg-background` |
| Cloning large data into props | Slow renders on big lists | Pass small ids, or share via context |
| Blocking work in an event handler | UI freezes | Use `spawn` for async work |

---

## How should you structure files as the app grows?

**Group by feature, not by file type.** A flat `components/` folder with 40 files gets hard to navigate. A feature folder keeps the component, its types, and its helpers together.

```text
src/
  main.rs            // launch + Route enum
  layout.rs          // AppLayout shell
  features/
    projects/
      mod.rs
      list.rs        // ProjectList
      detail.rs      // ProjectDetail
      model.rs       // Project, validation, API calls
  ui/                // shared primitives (Button, Card, Input)
```

Put shared primitives in a `ui/` module. This is exactly the shape Rust/UI uses: each component is one file you copy into `ui/` and own, so you can read and change every line.

---

## What should you build next?

**Add one real route and one loading or empty state.** That exposes the decisions that matter in a real product: navigation, state ownership, content density, and feedback. Keep each change small enough that the rendered result stays understandable.

A good order is:

1. A second route with `#[derive(Routable)]` and a shared layout.
2. A form with typed state, covered in [Dioxus forms, state, and validation](/articles/dioxus-forms-state-validation).
3. A data-backed list with loading, empty, and error states.
4. A build for a second target, covered in [shipping a cross-platform Rust UI with Dioxus](/articles/ship-cross-platform-rust-ui-dioxus).

## Frequently Asked Questions

### Is Dioxus good for beginners in Rust?

Yes, if you already know basic Rust (structs, enums, ownership). The component model feels familiar to React developers, and signals avoid most borrow-checker friction because they are `Copy` handles to shared state.

### Can Dioxus build desktop and mobile apps, not just websites?

Yes. The same components target web (WebAssembly), desktop, and mobile through the `dx` CLI. Use `dx serve --platform desktop` to try the desktop target from an existing project.

### Does Dioxus work with Tailwind CSS?

Yes. Dioxus 0.7 picks up a `tailwind.css` file in the project root and runs Tailwind automatically, so you can use utility classes directly in `class` attributes.

### What is the difference between a signal and a regular variable in Dioxus?

A regular variable is recreated on every render. A signal persists across renders and notifies Dioxus when it changes, so only the parts of the UI that read it update.

### Where can I find ready-made Dioxus components?

[Rust/UI](/docs/components) provides copy-paste, Tailwind-based Dioxus components such as buttons, cards, dialogs, and forms that you own and customize.

### How is Dioxus different from Yew or Leptos?

All three build web UIs in Rust, but Dioxus targets web, desktop, and mobile from one codebase and uses an RSX macro with signals. The [Leptos vs Dioxus comparison](https://rustify.rs/articles/leptos-vs-dioxus-rust-frontend-2026) walks through rendering model, ecosystem, and platform reach.

### Why do I need a key in a for loop?

A stable `key` tells Dioxus which item is which across renders. Without it, items can be reused incorrectly, so inputs may keep the wrong text or lose focus when the list reorders.

### How do I share state between components?

Lift the signal to a common parent and pass it down as a prop, or provide it with `use_context_provider` and read it with `use_context` in any descendant. Use context for app-wide state such as theme or the signed-in user.

## Sources

- [Dioxus documentation](https://dioxuslabs.com/learn/0.7/)
- [Dioxus on GitHub](https://github.com/DioxusLabs/dioxus)
- [Tailwind CSS documentation](https://tailwindcss.com/docs)
- [MDN: WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
