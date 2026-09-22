+++
title = "Design Async Data and Loading States in Dioxus"
description = "Build Rust UI that handles loading, empty, error, and success states without layout jumps or hidden async behavior."
category = "Interaction"
publish_date = "2026-09-14"
last_updated = "2026-09-14"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Async UI states"
keywords = ["Dioxus", "Rust", "async", "loading states", "data fetching"]
+++

Async data is part of the UI, not a side effect hidden behind a spinner. A good screen tells users whether data is loading, missing, unavailable, or ready. Model those states before wiring the request.

## Give request state a type

An enum prevents impossible combinations such as “loading and error” at the same time. It also makes every render branch visible.

```rust
#[derive(Clone, PartialEq)]
enum LoadState<T> {
    Idle,
    Loading,
    Ready(T),
    Empty,
    Failed(String),
}
```

Use `Empty` when a successful request returns no items. Empty is not an error; it needs different copy and often a different action.

## Keep async work close to its owner

The component that owns the data should own the request lifecycle. Start work from an effect or an explicit event, then write a new state when the task completes.

```rust
let mut state = use_signal(|| LoadState::<Vec<Project>>::Idle);

use_effect(move || {
    spawn(async move {
        state.set(LoadState::Loading);
        match fetch_projects().await {
            Ok(projects) if projects.is_empty() => state.set(LoadState::Empty),
            Ok(projects) => state.set(LoadState::Ready(projects)),
            Err(error) => state.set(LoadState::Failed(error.to_string())),
        }
    });
});
```

For route-driven data, include the route input in the effect dependency. For button-driven refresh, make refresh an explicit event so users understand what caused the request.

## Render stable shapes

Loading UI should reserve roughly the same space as the finished UI. Skeleton rows work well for lists; a small inline status works better for a compact panel. Avoid replacing an entire page with a spinner when only one region is loading.

```rust
match state() {
    LoadState::Loading => rsx! { div { class: "space-y-3", for _ in 0..3 { div { class: "h-12 animate-pulse rounded-xl bg-muted" } } } },
    LoadState::Empty => rsx! { EmptyProjects {} },
    LoadState::Failed(message) => rsx! { ErrorPanel { message } },
    LoadState::Ready(projects) => rsx! { ProjectList { projects } },
    LoadState::Idle => rsx! {},
}
```

Keep error actions specific: retry the request, edit a filter, or check a connection. “Something went wrong” is a fallback, not a recovery path.

## Consider stale responses

Search and route changes can produce overlapping requests. Track the input that started a request or cancel stale work when the framework and client target allow it. Never let a slow response for an old query overwrite newer data.

Async UI feels fast when state transitions are deliberate. Rust makes the state machine explicit; Dioxus makes each transition visible in the component tree.
