# BUGFIX: `/docs/*` and `/icons` return 404 on hard refresh

Status: fixed in `src/main.rs` (`mod server`) + `Cargo.toml` (adds `tower-http`).
Scope: dev server (`dx serve`) and any served-SSR deployment. A fully
pre-rendered static export is not affected.

## Symptom

- Client-side SPA navigation to `/docs/components/alert`, `/docs/hooks/use-random`,
  `/docs`, `/icons` works fine.
- A hard refresh or a direct hit of the same URL returns HTTP 404.
- Sibling routes that do not share a name with a `public/` sub-directory
  (`/components/alert`, `/create`, `/charts/area-chart`, `/blocks/login`) load
  fine on refresh.

## Evidence

Captured against `dx serve` before the fix:

| Request                                | Result | Note                                    |
|----------------------------------------|--------|-----------------------------------------|
| `GET /docs/components/alert`           | 404    | route exists in `Route` enum            |
| `GET /docs/components/alert.md`        | 200    | real file `public/docs/components/alert.md` |
| `GET /docs/components`                 | 307    | redirected, still never renders         |
| `GET /icons`                           | 404    | route exists (`PageIcons`)              |
| `GET /components/alert`                | 200    | no `public/components/` dir, so SSR runs |
| `GET /create`                         | 200    | no `public/create/` dir                 |

The `.md` sibling returning 200 while the route returns 404 is the tell: the
request for `/docs/...` is being answered by a static file handler rooted at
`public/docs/`, not by the app.

## Root cause

`dioxus-server` 0.7.10, `DioxusRouterExt::serve_static_assets` calls
`serve_dir_cached` (`~/.cargo/registry/src/*/dioxus-server-0.7.10/src/server.rs`).

In debug builds (`#[cfg(debug_assertions)]` branch) it walks the top level of
`public/` and, for each entry, mounts it as a nested service:

```rust
// dioxus-server-0.7.10/src/server.rs, serve_dir_cached()
for entry in std::fs::read_dir(&public_path)? {
    let path = entry?.path();
    let route = format!("/{}", file_name);
    router = router.nest_service(&route, ServeDir::new(&path));
}
```

`nest_service("/docs", ServeDir::new("public/docs"))` makes axum route the
**entire `/docs/*` subtree** into that `ServeDir`. `ServeDir` here has no
`not_found_service`, so any path with no matching file on disk terminates the
match with a 404. The router's `.fallback(render_handler)` (the SSR renderer)
is never reached for anything under `/docs/`.

Our `Route` enum has `/docs/components`, `/docs/components/:name`,
`/docs/hooks`, `/docs/hooks/:name`, and `/icons`. Directories `public/docs/`
and `public/icons/` both exist (the first holds the raw `.md` docs served for
LLM/tooling consumers and for parity with `leptos-ui`; the second holds real
favicons and logos). Those routes are therefore shadowed on any cold request.

Why SPA navigation still works: the client `Router::<Route>` resolves routes
in memory and never issues these HTTP requests.

Why release / static export is unaffected: the
`#[cfg(not(debug_assertions))]` branch of `serve_dir_cached` recurses file by
file instead of mounting directory `nest_service`s, and a static export writes
every route to its own `index.html` on disk.

Why `leptos-ui` with the same `public/` layout does not have this bug:
`leptos_axum` wires static files as `ServeDir::new(dir).fallback(handler)`, so
a missing file falls through to the app.

## Fix

Stop using `dioxus::launch` on the server. Build the axum router by hand via
`dioxus::serve` (the documented 0.7 extension point for a custom router) and
wire `ServeDir` as the router **fallback** instead of a set of per-directory
nested services:

```rust
Router::new()
    .register_server_functions()
    .fallback_service(ServeDir::new(public_path()).fallback(ssr))
    .with_state(state)
```

Now a real file under `public/` still wins, and every other path (including
`/docs/components/alert` and `/icons`) falls through to the Dioxus SSR handler.
This is the same shape `leptos_axum` uses.

`public_path()` mirrors the private `dioxus_server::public_path()`: honour
`DIOXUS_PUBLIC_PATH` if set, else `<current_exe dir>/public`.

`apply_base_path` (which `dioxus-server` applies internally) is skipped on
purpose: this site configures no base path.

### Files changed

- `src/main.rs`: server bootstrap moved from `dioxus::launch(App)` to
  `dioxus::serve(|| async { Ok(server::router()) })`, plus new `mod server`.
  Full rationale is in the doc comment on `mod server`.
- `Cargo.toml`: `tower-http = { version = "0.6", features = ["fs"], optional = true }`
  added, and `dep:tower-http` added to the `server` feature. (`tower-http` was
  already a transitive dependency; this just makes `ServeDir` importable.)

## Options considered and rejected

- **Move the colliding directories** (`public/docs` out of the web root, rename
  `public/icons` to `public/images/icons`). Smaller in code but needs a change
  in the separate private registry generator, breaks serving raw `.md` at its
  natural URL, diverges from `leptos-ui`, and only fixes these two paths rather
  than the whole collision class.
- **Patch `dioxus-server` upstream** to give its static handler a not-found
  fallback. Correct fix, but out of our control and slow to land. If it lands,
  `mod server` here can be deleted and `dioxus::launch(App)` restored.

## Follow-up

Track upstream `dioxus-server`. When its debug static handler gains a
not-found fallback, revert `src/main.rs` to plain `dioxus::launch(App)` and
drop the `tower-http` dependency.
