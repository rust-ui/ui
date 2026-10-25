+++
title = "Async Data and Loading States in Dioxus: A Practical Guide"
description = "Handle loading, empty, error, and success states in Dioxus with use_resource, a typed state enum, and skeletons that avoid layout jumps. Code included."
category = "Interaction"
publish_date = "2026-09-14"
last_updated = "2026-10-25"
author = "Max Wells"
author_role = "Creator of Rust/UI and Rustify"
author_image = "/articles/author-max-wells.webp"
short_title = "Async UI states"
keywords = ["Dioxus use_resource", "Dioxus async", "Dioxus loading state", "Rust UI data fetching", "Dioxus skeleton", "Dioxus Suspense"]
+++

**TL;DR:** Treat async data as four UI states, not one spinner: loading, empty, error, and ready. In Dioxus, `use_resource` runs the request and re-runs it when its inputs change, while a small enum or a `match` on the resource makes every state visible. Render loading UI with roughly the final shape, and a skeleton keeps the page from jumping.

## Who is this guide for?

This guide is for Rust developers who have built a first Dioxus screen and now need real data in it. You should be comfortable with signals and `rsx!`; if not, start with [building your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust). Jump to the `use_resource` section if you only need the fetch pattern, to the stale response section if you are debugging a flicker, and to the fullstack section if your data comes from server functions.

---

## What states does async UI need to handle?

**Every request has at least four outcomes: loading, empty, failed, and ready.** Users read each one differently, so each needs its own copy and often its own action.

| State | What the user sees | Typical action |
|---|---|---|
| Loading | Skeleton or inline status | Wait, or cancel |
| Empty | "No projects yet" | Create the first item |
| Failed | What broke and what to try | Retry, edit a filter, check the connection |
| Ready | The data | Normal use |

An empty list is a successful response, not an error. Giving it the error treatment is one of the most common async UI mistakes.

> **Bottom line:** Decide the copy and action for all four states before you write the request code.

![State diagram: idle moves to loading, then to ready, empty or failed; failed can retry back to loading](/articles/async-data-loading-states-dioxus-asset-1.svg)

*Every request leaves Loading through exactly one of three doors. Failed is the only state with a way back, which is why it needs a retry action.*

## How do you fetch data with use_resource in Dioxus?

**`use_resource` runs an async closure, exposes its result as a signal-like value, and re-runs when any signal read inside it changes.** That makes it the default tool for data owned by a component.

```rust
use dioxus::prelude::*;

#[component]
fn Projects() -> Element {
    let mut query = use_signal(String::new);

    // Re-runs whenever `query` changes. The previous run is dropped.
    let projects = use_resource(move || async move { fetch_projects(query()).await });

    rsx! {
        input {
            class: "rounded-md border px-3 py-2",
            placeholder: "Search projects",
            value: query(),
            oninput: move |event| query.set(event.value()),
        }
        match &*projects.read_unchecked() {
            None => rsx! { ProjectSkeleton {} },
            Some(Err(error)) => rsx! { ErrorPanel { message: error.to_string() } },
            Some(Ok(items)) if items.is_empty() => rsx! { EmptyProjects {} },
            Some(Ok(items)) => rsx! { ProjectList { projects: items.clone() } },
        }
    }
}
```

The `match` covers all four outcomes in one place, and the compiler forces you to handle each branch. For a manual refresh, call `projects.restart()` from a button so users understand what triggered the request.

## When should you use a typed state enum instead?

**Use an enum when state is driven by events, not just by inputs.** A form submit, a "load more" button, or a polling loop do not fit a single resource well. An enum prevents impossible combinations such as "loading and error" at once.

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

```rust
let mut state = use_signal(|| LoadState::<Vec<Project>>::Idle);

let load = move |_| {
    spawn(async move {
        state.set(LoadState::Loading);
        match fetch_projects(String::new()).await {
            Ok(projects) if projects.is_empty() => state.set(LoadState::Empty),
            Ok(projects) => state.set(LoadState::Ready(projects)),
            Err(error) => state.set(LoadState::Failed(error.to_string())),
        }
    });
};
```

Keep the work close to the component that owns the data, so the request lifecycle has one visible owner.

## How do you avoid layout jumps while loading?

**Reserve roughly the same space as the finished UI.** Skeleton rows work well for lists, and a small inline status works better for a compact panel. Avoid replacing a whole page with a spinner when only one region is loading.

```rust
#[component]
fn ProjectSkeleton() -> Element {
    rsx! {
        div { class: "space-y-3", "aria-busy": "true",
            for _ in 0..3 {
                div { class: "h-12 animate-pulse rounded-xl bg-muted" }
            }
        }
    }
}
```

You do not need to build this by hand. Rust/UI ships a ready-made [Skeleton](/docs/components/skeleton) and [Spinner](/docs/components/spinner), plus an [Alert](/docs/components/alert) for error panels, all styled with Tailwind tokens so they follow your theme.

Keep error actions specific: retry the request, edit a filter, or check a connection. "Something went wrong" is a fallback, not a recovery path.

## How do you handle stale responses?

**Never let a slow response for an old query overwrite newer data.** With `use_resource` this is handled for you: when a dependency changes, the previous future is dropped and a new one starts. With a manual `spawn`, you must guard it yourself.

```rust
let mut latest = use_signal(|| 0u32);

let search = move |text: String| {
    let id = latest() + 1;
    latest.set(id);
    spawn(async move {
        let result = fetch_projects(text).await;
        if latest() == id {
            // Only the newest request is allowed to write.
            state.set(to_state(result));
        }
    });
};
```

Also debounce fast input, such as search as you type, so you do not fire a request on every keystroke.

## How do Suspense and ErrorBoundary fit in?

**Use them when many components wait on data and you want one fallback for the group.** `use_resource` gives you per-component control and a manual `match`. Dioxus also offers `SuspenseBoundary` and `ErrorBoundary`, which let a subtree suspend while its data loads and catch errors thrown by children, so you do not repeat the same `match` everywhere.

| Tool | Best for | Trade-off |
|---|---|---|
| `use_resource` + `match` | One component that owns its data | You write all four branches |
| `SuspenseBoundary` | A section with several async children | One shared fallback, less control per child |
| `ErrorBoundary` | Unexpected failures in a subtree | Good for "something broke", weak for specific recovery |
| Typed `LoadState` enum | Event-driven flows (submit, load more) | You manage transitions by hand |

A practical rule: use boundaries for coarse, page-level fallbacks, and keep explicit `match` arms where the copy and recovery action differ per state. The user-facing empty state ("Create your first project") almost never belongs in a generic boundary.

---

## What changes with server functions and fullstack?

**Nothing in the UI, and a lot in where the data comes from.** A server function is an async function the client calls like any other. From the component's point of view it is the same four states.

```rust
#[server]
async fn list_projects(query: String) -> Result<Vec<Project>, ServerFnError> {
    // Runs on the server: database access, secrets, authorization.
    Ok(db::search_projects(&query).await?)
}

#[component]
fn Projects() -> Element {
    let query = use_signal(String::new);
    let projects = use_resource(move || async move { list_projects(query()).await });
    // The same match as before handles loading, empty, failed, ready.
    rsx! { /* ... */ }
}
```

What does change is initial render. With server-side rendering you want data ready in the first HTML, so the page does not flash a skeleton for content that was available on the server. Dioxus provides `use_server_future` for this, which resolves during SSR and hydrates on the client. For client-only data, such as results that depend on the browser, stay with `use_resource`.

---

## How do you debounce search input?

**Wait until the user pauses, then fire one request.** Without debouncing, "dioxus" fires six requests for six keystrokes, and four of them are dropped by the time they return.

```rust
let mut raw = use_signal(String::new);
let mut settled = use_signal(String::new);

// Copy `raw` into `settled` only after 300ms of silence.
use_effect(move || {
    let text = raw();
    spawn(async move {
        sleep_ms(300).await; // gloo_timers on web, tokio::time on native
        if raw() == text {
            settled.set(text);
        }
    });
});

let results = use_resource(move || async move { search(settled()).await });
```

The resource reads `settled`, not `raw`, so it only re-runs for the debounced value. The comparison `raw() == text` is the same stale-guard idea used for manual requests: a newer keystroke makes the older timer a no-op. Note that this is one of the few legitimate `use_effect` uses, because it schedules work outside the render.

---

## What are the common async UI mistakes?

| Mistake | Symptom | Fix |
|---|---|---|
| One spinner for the whole page | Page jumps, user loses context | Skeleton only for the region that loads |
| Empty list shown as an error | Users think the app is broken | Separate `Empty` state with a creation action |
| No retry on failure | User reloads the page | Retry button that calls `restart()` |
| Unwrapping the result | Panic on a network error | `match` on `Result`, never `unwrap()` in UI code |
| Writing from a stale response | Old results replace new ones | Rely on `use_resource`, or guard with a request id |
| Request on every keystroke | Rate limits, flicker | Debounce, then read the settled value |
| Spinner with no timeout or message | Infinite loading | Show an error after a deadline |

---

## How do you keep accessibility intact while loading?

**Tell assistive technology what is happening.** A skeleton is silent unless you announce it. Mark the loading region with `aria-busy="true"`, and put errors in a container with `role="alert"` so they are announced when they appear. Keep focus where it was: moving focus into a spinner steals it from the control the user just used.

```rust
rsx! {
    div { "aria-live": "polite",
        match &*projects.read_unchecked() {
            None => rsx! { p { class: "sr-only", "Loading projects" } ProjectSkeleton {} },
            Some(Err(error)) => rsx! { div { role: "alert", "{error}" } },
            Some(Ok(items)) => rsx! { ProjectList { projects: items.clone() } },
        }
    }
}
```

The visually hidden `sr-only` text gives screen reader users the same information a sighted user gets from the skeleton shape.

---

## How do optimistic updates work with async state?

**Update the UI first, then reconcile when the server answers.** Toggling a favorite or ticking a todo should feel instant. Waiting for a round trip makes the interface feel slower than the network actually is.

```rust
let mut items = use_signal(Vec::<Todo>::new);

let toggle = move |id: u32| {
    // 1. Apply the change locally and remember the previous value.
    let previous = items.read().clone();
    items.with_mut(|list| {
        if let Some(todo) = list.iter_mut().find(|todo| todo.id == id) {
            todo.done = !todo.done;
        }
    });

    // 2. Ask the server. On failure, roll back and tell the user.
    spawn(async move {
        if save_todo(id).await.is_err() {
            items.set(previous);
            // show an inline error near the row, not a global banner
        }
    });
};
```

The rollback is the part people forget. An optimistic update without a rollback path is a lie that survives until the next refresh. Think of it as a loan against the server's answer: cheap when it is repaid, expensive when it is not. Keep optimistic updates for actions that almost always succeed and are cheap to undo. For anything irreversible, such as deleting an account or sending a payment, show a pending state and wait for confirmation.

---

## How should retries and timeouts behave?

**Retry transient failures automatically, and ask the user before repeating anything that has side effects.** A failed `GET` for a list is safe to retry. A failed `POST` that charges a card is not, unless the server supports idempotency keys.

- Retry reads two or three times with growing delays (for example 250ms, then 1s), then show the error state.
- Never retry silently forever. The user should see a result or a clear failure within a few seconds.
- Distinguish a network failure ("check your connection") from a server error ("we could not load this, try again") and from a validation error (a field problem, which belongs next to the field, as covered in [Dioxus forms, state, and validation](/articles/dioxus-forms-state-validation)).
- Put a deadline on the request. A hung connection that never resolves leaves the skeleton on screen indefinitely, which is the worst of the four states because it looks like loading but never finishes.

Showing the underlying error text is useful in development and often wrong in production. Map errors to a small set of user-facing messages, and log the detail.

---

## What does a complete example look like?

**A list screen that handles all four states, search, and retry in one component.** Pulling the earlier pieces together, the structure is the same for almost every data screen:

1. A signal for the query (the input of the request).
2. A `use_resource` that reads the query and calls the fetch function.
3. A `match` with four arms: skeleton, error with retry, empty with a call to action, list.
4. A retry button that calls `restart()`.
5. Small presentational components for each state, so the main component reads like a table of contents.

Keeping each state as its own component (`ProjectSkeleton`, `ErrorPanel`, `EmptyProjects`, `ProjectList`) is what makes the screen maintainable. When design changes the empty state, you edit one file and the fetch logic never moves. It also lets you reuse the pieces: the same `ErrorPanel` serves every screen in the app. Build those panels on tokens and the [Alert](/docs/components/alert) component so they pick up your theme, and your product feels consistent without extra work.

Treat the states as part of the design, not as an afterthought handed to whoever wires up the request. A designer who has only drawn the happy path has drawn a quarter of the screen. Finally, test the states directly. Temporarily make the fetch function return an empty list, then an error, then sleep for several seconds. If any of the four screens looks unfinished, that is the screen real users will eventually meet.

---

## How do async states fit into a full screen?

Async states are one piece of a screen. The same discipline of one source of truth and one visible error path shows up in [Dioxus forms, state, and validation](/articles/dioxus-forms-state-validation). Layout stability under loading depends on the patterns in [responsive Rust UI with Dioxus and Tailwind](/articles/responsive-rust-ui-with-dioxus-tailwind). If you are new to signals, start with [building your first Dioxus UI](/articles/build-your-first-dioxus-ui-in-rust).

## Frequently Asked Questions

### What is use_resource in Dioxus?

`use_resource` is a hook that runs an async closure and exposes the result reactively. It re-runs when any signal read inside the closure changes, which makes it the standard way to fetch data tied to component state.

### How do I show a loading spinner in Dioxus?

Match on the resource: `None` means the request is still running, so render a spinner or skeleton there. Prefer a skeleton that matches the final layout to avoid layout shift.

### How do I retry a failed request in Dioxus?

Call `restart()` on the resource from a button's `onclick`. For enum-based state, call the same load function again and set the state back to `Loading` first.

### Should an empty list be treated as an error?

No. An empty list is a successful response and needs different copy and a different action, such as "Create your first project". Model it as its own state.

### How do I avoid race conditions with overlapping requests?

`use_resource` drops the previous future when its inputs change. For manual `spawn` calls, store a request id in a signal and only apply the result if the id still matches.

### Can I use async data with server functions in Dioxus fullstack?

Yes. Server functions are async functions you can call from `use_resource` the same way as any other async call, and the same four states apply.

### Should I use SuspenseBoundary or a manual match?

Use `SuspenseBoundary` for a coarse fallback over several async children. Use a manual `match` where the empty and error states need their own copy and actions, which is most real screens.

### How do I debounce a search box in Dioxus?

Keep a raw input signal and a settled signal. Copy raw into settled after a short delay only if the text has not changed, and make the resource read the settled signal.

## Sources

- [Dioxus: asynchronous tasks and resources](https://dioxuslabs.com/learn/0.7/)
- [Dioxus on GitHub](https://github.com/DioxusLabs/dioxus)
- [Web.dev: Cumulative Layout Shift](https://web.dev/articles/cls)
- [WAI-ARIA: alert role](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/alert_role)
