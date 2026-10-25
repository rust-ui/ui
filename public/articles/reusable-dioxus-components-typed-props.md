+++
title = "Reusable Dioxus Components: Typed Props, Variants, and Slots"
description = "Design reusable Dioxus components with typed props, enum variants, class merging, attribute spreading, and compound patterns. Practical Rust code and a checklist."
category = "Components"
publish_date = "2026-09-20"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Reusable Dioxus components"
keywords = ["reusable Dioxus components", "Dioxus props", "Dioxus component library", "Dioxus variants", "Dioxus compound components", "Dioxus children", "Rust UI components"]
+++

A reusable component is a stable contract: common cases are easy, semantics are preserved, and unusual cases have a clear extension point. In Dioxus, typed props make that contract explicit and compiler-checked, without a runtime schema or prop-types library.

**TL;DR:** Design the call site first. Use enums for real variants, `Option` and `#[props(default)]` for optional behavior, `children` or `Element` props for slots, a merged `class` prop as the styling escape hatch, and attribute spreading to forward the rest. Keep semantic HTML inside the primitive, and widen the API only when a second real use proves it.

- **Props are Rust types:** wrong or missing props fail at compile time
- **Variants are enums:** `Default`, `Outline`, `Destructive` instead of free-form strings
- **Slots are `Element` props or `children`:** composition scales better than prop explosions
- **Escape hatches are explicit:** `class` and spread attributes, not a dozen `*_class` props
- **Semantics live in the primitive:** a button renders a `button`, a link renders an `a`

---

## Who is this guide for?

**This guide is for developers who already have a first Dioxus screen working and now want to build components they can reuse across screens or share across projects.** If you have not built that first screen yet, start with [your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust), which covers signals, `rsx!`, and routing.

It is also useful if you maintain a component library. [Rust/UI](/docs/components) follows the patterns below for every component it ships, so you can read the source of the [Button](/docs/components/button) or [Card](/docs/components/card) as a real-world reference.

---

## What makes a Dioxus component reusable?

**A reusable component has a small, typed API, owns its visual details, and leaves room for composition.** Reuse fails in two opposite ways: components that are too rigid (every new screen needs a fork) and components that are too flexible (a prop for every CSS decision).

| Quality | Reusable | Not reusable |
|---|---|---|
| API size | A handful of meaningful props | 20 props mirroring CSS |
| Variation | Enums for repeated choices | Free-form strings and booleans |
| Extension | `class`, children, spread attributes | Forking the file per screen |
| Semantics | Correct element and ARIA by default | Caller must remember accessibility |
| Styling | Design tokens inside the component | Raw colors passed from outside |

> **Bottom line:** A component is reusable when a new caller can use it correctly without reading its implementation.

---

## How do you design a component from the call site?

**Write the usage before the implementation.** If a card needs a title, description, and optional action, express exactly that shape and see how it reads.

```rust
rsx! {
    FeatureCard {
        eyebrow: "Components",
        title: "Typed by default",
        description: "Build UI APIs the compiler can explain.",
        action: rsx! { a { href: "/docs/components", "Browse components" } },
    }
}
```

This keeps the component readable. The caller describes content and intent, while the component owns spacing, typography, borders, and responsive behavior. If the call site looks awkward, the API is wrong, and it is much cheaper to find out before you write the implementation.

Props in Dioxus are the function arguments of a `#[component]`. The compiler generates a props struct from them, which must implement `Clone` and `PartialEq`. That requirement is useful: it lets Dioxus skip re-rendering a component whose props did not change.

![Component tree where a Page passes typed props down to Button and Card, and Button sends events up through an EventHandler](/articles/reusable-dioxus-components-typed-props-asset-1.svg)

*Props flow down and events flow up. The prop types are the whole contract between a parent and its children.*

---

## When should a prop be a variant enum?

**Use an enum when a choice repeats and has a name.** `Default`, `Outline`, and `Destructive` are semantic choices that appear across an app. A boolean like `is_big` or a string like `"red"` is not.

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CardTone {
    #[default]
    Default,
    Muted,
    Accent,
}

#[component]
fn FeatureCard(
    eyebrow: String,
    title: String,
    description: String,
    #[props(default)] action: Option<Element>,
    #[props(default)] tone: CardTone,
) -> Element {
    let tone_class = match tone {
        CardTone::Default => "bg-card",
        CardTone::Muted => "bg-muted/40",
        CardTone::Accent => "bg-primary/5",
    };

    rsx! {
        article { class: "rounded-2xl border p-6 {tone_class}",
            p { class: "text-xs font-medium uppercase tracking-wide text-muted-foreground", "{eyebrow}" }
            h2 { class: "mt-3 text-xl font-semibold", "{title}" }
            p { class: "mt-2 text-sm text-muted-foreground", "{description}" }
            if let Some(action) = action {
                div { class: "mt-5", {action} }
            }
        }
    }
}
```

The component owns its tone mapping, so call sites never learn which Tailwind tokens produce the visual result. Because the match is exhaustive, adding `CardTone::Danger` later makes the compiler point at every place that needs updating.

| Prop type | Use it when | Example |
|---|---|---|
| `enum` | A named, repeated choice | `variant`, `size`, `tone` |
| `bool` | A true on/off behavior | `disabled`, `open` |
| `Option<T>` | A value that may be absent | `description`, `icon` |
| `Option<Element>` | An optional slot | `action`, `footer` |
| `String` / `#[props(into)]` | Content text | `title`, `label` |
| `EventHandler<T>` | A callback to the parent | `onclick`, `onchange` |

A small ergonomic upgrade: `#[props(into)] title: String` lets callers pass `&str` and `String` interchangeably, which keeps call sites free of `.to_string()` noise.

---

## How do you let callers add classes without breaking the component?

**Accept a `class` prop and merge it after the component's own classes.** This gives callers an escape hatch for spacing and layout without a prop per CSS property.

```rust
#[component]
fn Panel(
    title: String,
    #[props(into, default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        section { class: "rounded-2xl border bg-card p-6 {class}",
            h2 { class: "font-semibold", "{title}" }
            div { class: "mt-5", {children} }
        }
    }
}
```

Plain string concatenation has one weakness: if the component sets `p-6` and the caller passes `p-2`, both classes are present and the winner depends on CSS order. A class-merging helper such as the `tw_merge` crate resolves conflicts so the later class wins. Rust/UI components use this approach so that `class: "p-2"` reliably overrides the default padding.

Avoid prop explosions like `title_class`, `description_class`, `icon_size`, and `button_gap`. Wait until repeated consumers prove they need them, and prefer a composition slot, which usually scales better.

---

## How do you forward attributes like `id`, `aria-label`, and `onclick`?

**Use attribute spreading so the component forwards unknown attributes to its root element.** Without it, every missing attribute becomes a new prop.

```rust
#[component]
fn IconButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "inline-flex size-9 items-center justify-center rounded-md border hover:bg-accent",
            r#type: "button",
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
```

Callers can now write `IconButton { aria_label: "Close", id: "close-btn", ... }` and those attributes land on the real `button`. This is how you keep a primitive thin: it controls structure and styling, and the caller controls everything else.

---

## How do children and slots keep APIs small?

**Children are often the best extension point.** Use named props for required semantics, then accept an `Element` or `children` where callers need to place custom content.

```rust
#[component]
fn Panel(title: String, children: Element) -> Element {
    rsx! {
        section { class: "rounded-2xl border bg-card p-6",
            h2 { class: "font-semibold", "{title}" }
            div { class: "mt-5", {children} }
        }
    }
}
```

For several regions, use several `Element` props (`header`, `footer`) or move to a compound pattern.

---

## What are compound components and when do you need them?

**A compound component is a family of small components that share meaning, such as `Card`, `CardHeader`, `CardTitle`, and `CardContent`.** Use it when a single component would need too many slot props.

```rust
rsx! {
    Card {
        CardHeader {
            CardTitle { "Billing" }
            CardDescription { "Manage your plan and invoices." }
        }
        CardContent {
            p { "You are on the Pro plan." }
        }
    }
}
```

Each part is tiny and does one job, and callers arrange them freely. This is the structure of the Rust/UI [Card](/docs/components/card), and it scales to richer widgets like [Tabs](/docs/components/tabs) and [Dialog](/docs/components/dialog).

When parts must share state (the open tab, the open dialog), the root provides a context and the children read it:

```rust
#[derive(Clone, Copy)]
struct TabsState {
    active: Signal<String>,
}

#[component]
fn Tabs(default: String, children: Element) -> Element {
    let active = use_signal(|| default.clone());
    use_context_provider(|| TabsState { active });
    rsx! { div { class: "w-full", {children} } }
}

#[component]
fn TabTrigger(value: String, children: Element) -> Element {
    let mut state = use_context::<TabsState>();
    let is_active = state.active.read().as_str() == value;
    rsx! {
        button {
            role: "tab",
            aria_selected: is_active.to_string(),
            class: if is_active { "border-b-2 border-primary font-medium" } else { "text-muted-foreground" },
            onclick: move |_| state.active.set(value.clone()),
            {children}
        }
    }
}
```

The caller never passes state around, but the compiler still knows exactly what is shared.

---

## Why should semantics stay inside the primitive?

**A button should render a button, a navigation item should render a link, and a heading should keep heading order.** If every call site must remember this, some will forget.

- Render a real `button` with `type="button"` so it does not accidentally submit a form.
- Render a real `a` (or the router `Link`) for navigation so middle-click, focus, and screen readers work.
- Set `aria-*` state (`aria_selected`, `aria_expanded`, `aria_invalid`) from the same state that drives the visuals.
- Keep a visible focus style. Never remove the outline without a replacement.

Typed props do not mean rigid components. They mean the flexible parts are visible. For forms in particular, see how these primitives combine in [Dioxus forms, state, and validation](/articles/dioxus-forms-state-validation).

---

## How do you build a small component end to end?

**Start from the call site, write the enum, then write the markup last.** A `Badge` is small enough to show every step without hiding anything.

Step one is the call site you want to read in a year:

```rust
rsx! {
    Badge { "New" }
    Badge { variant: BadgeVariant::Outline, "Beta" }
    Badge { variant: BadgeVariant::Destructive, class: "ml-2", "Deprecated" }
}
```

Step two is the types that make that call site legal:

```rust
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Outline,
    Destructive,
}

impl BadgeVariant {
    fn classes(self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground",
            Self::Outline => "border border-border text-foreground",
            Self::Destructive => "bg-destructive text-white",
        }
    }
}
```

Step three is the component itself. Defaults come first, the caller's `class` comes last, and unknown attributes are forwarded to the root:

```rust
#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(into, default)] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let classes = format!(
        "inline-flex items-center rounded-md px-2 py-0.5 text-xs font-medium {} {class}",
        variant.classes()
    );

    rsx! {
        span { class: "{classes}", ..attributes, {children} }
    }
}
```

Two details are worth noticing. The enum derives `Copy`, so passing it around never needs `.clone()`, and it derives `PartialEq`, which is required for the props struct. And the class string is built from small, comparable values, so Dioxus can skip re-rendering when nothing changed. If two callers ever disagree on how a conflicting utility such as `px-2` versus `px-4` should resolve, that is the signal to adopt a class-merging helper rather than adding another prop. The real [Badge](/docs/components/badge) in Rust/UI follows the same shape.

---

## How do you test a reusable component?

**Test the logic you can isolate, and look at the rest.** The variant-to-classes mapping is a plain function, so it needs no renderer:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_variant_is_primary() {
        assert_eq!(BadgeVariant::default(), BadgeVariant::Default);
        assert!(BadgeVariant::Default.classes().contains("bg-primary"));
    }

    #[test]
    fn every_variant_sets_a_color() {
        for variant in [BadgeVariant::Default, BadgeVariant::Outline, BadgeVariant::Destructive] {
            assert!(!variant.classes().is_empty());
        }
    }
}
```

For markup, render the component to a string with `dioxus_ssr` and assert on the output, or open it in a small gallery page during development. Visual states (hover, focus, disabled, dark mode) are cheaper to review by eye than to assert. The goal is not 100 percent coverage. The goal is that a refactor of `classes()` cannot silently drop a variant.

> **Bottom line:** Keep the decision logic in plain functions and enums. They are the part of a component that is easy to test and expensive to get wrong.

---

## What are the common mistakes when building components?

| Mistake | Why it hurts | Better approach |
|---|---|---|
| Prop for every CSS decision | API grows without bound | Variant enum plus a `class` prop |
| Booleans that conflict (`primary`, `secondary`) | Impossible states compile | One `variant` enum |
| Reaching into child internals | Breaks on refactor | Compound parts or slots |
| Hard-coded colors inside | Cannot theme or support dark mode | Use tokens, see [responsive Rust UI with Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind) |
| Generalizing before the second use | Wrong abstraction | Build, use twice, then widen |
| Non-semantic root element | Keyboard and screen reader failures | Use the correct element |
| Props that are not `PartialEq` friendly | Extra re-renders | Prefer small, comparable values |

---

## How do you evolve a component API without breaking callers?

**Add optional props with defaults, deprecate before removing, and prefer enums that can grow.** A published component has callers, even inside one codebase. Adding a prop with `#[props(default)]` is invisible to existing call sites. Removing or renaming one is a compile error in every caller, which is a feature in Rust: the compiler lists everything you need to update.

When a boolean such as `loading` later needs a second meaning, you are glad you did not start with two conflicting booleans. An enum variant added to `ButtonState` breaks only exhaustive `match` expressions, and those are exactly the places you want to revisit. Mark enums `#[non_exhaustive]` if the component is shared across crates.

---

## What is a checklist for shipping a component?

1. The call site reads clearly with only the required props.
2. Variants are enums with a `Default`.
3. `class` is accepted and merged after defaults.
4. Unknown attributes are forwarded with `..attributes`.
5. The root element is semantic and keyboard accessible.
6. Colors come from tokens, so dark mode works.
7. It is used in at least two real screens.
8. It behaves the same on web and native, see [shipping cross-platform Rust UI](/articles/ship-cross-platform-rust-ui-dioxus).

---

## Frequently Asked Questions

### How do I pass children to a Dioxus component?

Declare a `children: Element` prop and render it with `{children}` inside `rsx!`. Callers then nest content inside the component's braces.

### How do I make a prop optional in Dioxus?

Use `Option<T>` for a value that may be absent, or `#[props(default)]` to fall back to `Default::default()`. Optional props can then be omitted at the call site.

### What is the difference between `Element` and `children`?

`children` is the conventional name for the nested content passed inside the component braces. A named `Element` prop (like `action`) is an extra slot you fill with `action: rsx! { ... }`. Both are the same type.

### How do I create variants like primary and outline?

Define an enum with a `Default` variant, add it as a prop with `#[props(default)]`, and `match` on it inside the component to choose classes. The compiler then forces you to handle every variant.

### How do I forward `id`, `aria-label`, and other attributes?

Add `#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>` and spread it on the root element with `..attributes`.

### How do I call a parent's callback from a child?

Use an `EventHandler<T>` prop. The child calls `handler.call(value)` and the parent supplies a closure when it renders the component.

### Should I build my own component library or use an existing one?

For common primitives (buttons, dialogs, inputs), copying from a registry saves time and you still own the code. [Rust/UI](/docs/installation) installs components into your project as source files you can edit. Build custom components for what is unique to your product.

### Why does my component re-render too often?

Check that props implement `PartialEq` cheaply, avoid creating new `String` or `Vec` values on every render for props, and keep state as local as possible so fewer components subscribe to it.

---

## Sources

- [Dioxus documentation: components and props](https://dioxuslabs.com/learn/0.7/)
- [Dioxus on GitHub](https://github.com/DioxusLabs/dioxus)
- [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [Tailwind CSS: reusing styles](https://tailwindcss.com/docs/reusing-styles)
