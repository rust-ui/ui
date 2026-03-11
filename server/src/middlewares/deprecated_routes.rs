//! Deprecated Routes Middleware
//!
//! **Added:** 2025-11-18
//! **Can be removed after:** 2026-11-18 (12 months minimum, safe to remove after Google deindexing)
//!
//! Returns HTTP 410 (Gone) for paths that are permanently removed, and HTTP 301
//! (Permanent Redirect) for paths whose content moved to a new URL.
//!
//! **Why 410 instead of 404?**
//! 410 tells crawlers the content is gone *forever*, prompting faster deindexing.
//! 404 is ambiguous (could be a transient issue), so crawlers retry 404s for much longer.
//!
//! **Why 301 for moved content?**
//! 301 passes link equity from the old URL to the new one, preserving SEO value.
//!
//! **To remove this middleware (after deindexing is complete):**
//! 1. Delete this file: `server/src/middlewares/deprecated_routes.rs`
//! 2. Remove the layer from `server/src/main.rs` (search for `handle_deprecated_routes`)
//! 3. Remove the module from `server/src/middlewares/mod.rs`
//! 4. Keep the `robots.txt` Disallow rules — they prevent re-crawling at negligible cost

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};

// ── 410 Gone: content permanently removed ────────────────────────────────────

/// Exact paths that are permanently gone (no replacement URL).
const GONE_EXACT: &[&str] = &[
    "/theme-generator",  // old name; route /themes has been removed, replaced by /create
    "/index.md",         // .md source file; not a public route
    "/documentation.md", // .md source file; not a public route
    "/blocks/hooks",     // removed; valid BlockRoutes are: Login/Sidenav/Headers/Footers/Faq/Integrations
    "/blocks/charts",    // removed; charts live at /charts (ChartRoutes)
    "/blocks/pricing",   // removed; pricing blocks no longer exist
    "/blocks/chat",      // removed; chat blocks no longer exist
    "/blocks/parallax",  // removed block category
    "/docs/extensions",  // removed extensions section (bare path, no trailing slash)
    "/core",             // old internal namespace (bare path; subtree covered by /core/ prefix)
    "/ui",               // old namespace (bare path; subtree covered by /ui/ prefix)
    "/extensions",       // top-level extensions (bare path; subtree covered by /extensions/ prefix)
    "/docs/ui",          // old docs namespace (bare path; subtree covered by /docs/ui/ prefix)
    "/registry/tree",    // internal registry endpoint removed
];

/// Path prefixes whose entire subtree is permanently gone.
const GONE_PREFIXES: &[&str] = &[
    "/docs/extensions/",     // extensions section removed; content was not migrated to any new path
    "/core/",                // old internal namespace, removed
    "/docs/ui/",             // old docs namespace, removed
    "/ui/",                  // old namespace, removed
    "/extensions/",          // top-level extensions route, removed
    "/theme-generator/",     // old name; route /themes has been removed, replaced by /create
    "/parallax-ipad-scroll", // removed experiment
    "/parallax-scrolls",     // removed experiment
];

/// Response body for 410 responses.
const GONE_BODY: &str =
    "This content has been permanently removed. Please visit https://rust-ui.com for current documentation.";

/// Response body for 410 on invalid registry paths.
const GONE_REGISTRY_BODY: &str = "Invalid registry path. Registry files must be accessed with .md or .json extension.";

// ── 301 Redirects: content moved to a new URL ────────────────────────────────

/// Old exact path for the component listing (no trailing slash).
const OLD_COMPONENTS: &str = "/components";

/// Old path prefix for individual component pages.
const OLD_COMPONENTS_PREFIX: &str = "/components/";

/// Old path prefix used before the /docs/ namespace was introduced.
const OLD_DEMO_COMPONENTS_PREFIX: &str = "/demo-components/";

/// Old bare /docs path (no sub-section); now the listing lives under /docs/components.
const OLD_DOCS: &str = "/docs";

/// New canonical base URL for the component documentation listing.
const NEW_DOCS_COMPONENTS: &str = "/docs/components";

/// New canonical path prefix for individual component documentation pages.
const NEW_DOCS_COMPONENTS_PREFIX: &str = "/docs/components/";

// ── Registry paths ────────────────────────────────────────────────────────────

const REGISTRY_BLOCKS_PREFIX: &str = "/registry/blocks/";
const REGISTRY_STYLES_PREFIX: &str = "/registry/styles/";

// ── View paths ────────────────────────────────────────────────────────────────

const VIEW_PREFIX: &str = "/view/";

/// Segment that appears inside /view/* paths that were generated with stale extension URLs.
const DOCS_EXTENSIONS_SEGMENT: &str = "/docs/extensions";

// ─────────────────────────────────────────────────────────────────────────────

/// What the middleware should do with a given request path.
enum RouteAction {
    /// Return HTTP 410 Gone with the supplied body text.
    Gone(&'static str),
    /// Return HTTP 301 Permanent Redirect to the supplied target path.
    Redirect(String),
    /// Let the request pass through to the next handler.
    PassThrough,
}

/// Pure function that decides what to do with a path.
/// Extracted so it can be unit-tested without an HTTP server.
fn classify_path(path: &str) -> RouteAction {
    // ── 410: permanently removed exact paths ─────────────────────────────────
    if GONE_EXACT.contains(&path) {
        return RouteAction::Gone(GONE_BODY);
    }

    // ── 410: permanently removed path prefixes ────────────────────────────────
    if GONE_PREFIXES.iter().any(|prefix| path.starts_with(prefix)) {
        return RouteAction::Gone(GONE_BODY);
    }

    // ── 301: /docs → /docs/components ────────────────────────────────────────
    // The ParentRoute at /docs has no index child; navigating there would render
    // DocsLayout with an empty outlet. Redirect to ComponentsRoutes::base_url().
    if path == OLD_DOCS {
        return RouteAction::Redirect(NEW_DOCS_COMPONENTS.to_owned());
    }

    // ── 301: /components[/*] → /docs/components[/*] ──────────────────────────
    // Component docs moved into the /docs namespace (ComponentsRoutes::base_url()).
    // The destination route is parameterised (/docs/components/:name) so all
    // component names are accepted, not just those in the ComponentsRoutes enum.
    if path == OLD_COMPONENTS {
        return RouteAction::Redirect(NEW_DOCS_COMPONENTS.to_owned());
    }
    if let Some(name) = path.strip_prefix(OLD_COMPONENTS_PREFIX) {
        return RouteAction::Redirect(format!("{NEW_DOCS_COMPONENTS_PREFIX}{name}"));
    }

    // ── 301: /demo-components/* → /docs/components/* ─────────────────────────
    // Old demo namespace was consolidated into ComponentsRoutes::base_url().
    if let Some(name) = path.strip_prefix(OLD_DEMO_COMPONENTS_PREFIX) {
        return RouteAction::Redirect(format!("{NEW_DOCS_COMPONENTS_PREFIX}{name}"));
    }

    // ── 410: stale /view/* paths that leaked a /docs/extensions segment ───────
    // These were generated by a bug in an old link helper and have no valid target.
    if path.starts_with(VIEW_PREFIX) && path.contains(DOCS_EXTENSIONS_SEGMENT) {
        return RouteAction::Gone(GONE_BODY);
    }

    // ── 410: /registry/blocks/* and /registry/styles/* without .md / .json ───
    // Only file downloads with an explicit extension are valid; bare slugs are not.
    if (path.starts_with(REGISTRY_BLOCKS_PREFIX) || path.starts_with(REGISTRY_STYLES_PREFIX))
        && !path.ends_with(".md")
        && !path.ends_with(".json")
    {
        return RouteAction::Gone(GONE_REGISTRY_BODY);
    }

    RouteAction::PassThrough
}

/// Axum middleware layer — wraps `classify_path` and converts its result into an HTTP response.
pub async fn handle_deprecated_routes(req: Request, next: Next) -> Response {
    match classify_path(req.uri().path()) {
        RouteAction::Gone(body) => (StatusCode::GONE, body).into_response(),
        RouteAction::Redirect(target) => Redirect::permanent(&target).into_response(),
        RouteAction::PassThrough => next.run(req).await,
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ───────────────────────────────────────────────────────────────

    fn is_gone(path: &str) -> bool {
        matches!(classify_path(path), RouteAction::Gone(_))
    }

    fn redirect_target(path: &str) -> Option<String> {
        match classify_path(path) {
            RouteAction::Redirect(target) => Some(target),
            _ => None,
        }
    }

    fn is_pass_through(path: &str) -> bool {
        matches!(classify_path(path), RouteAction::PassThrough)
    }

    // ── 410: exact paths ──────────────────────────────────────────────────────

    #[test]
    fn gone_exact_theme_generator() {
        assert!(is_gone("/theme-generator"));
    }

    #[test]
    fn gone_exact_index_md() {
        assert!(is_gone("/index.md"));
    }

    #[test]
    fn gone_exact_documentation_md() {
        assert!(is_gone("/documentation.md"));
    }

    #[test]
    fn gone_exact_blocks_hooks() {
        assert!(is_gone("/blocks/hooks"));
    }

    #[test]
    fn gone_exact_blocks_charts() {
        assert!(is_gone("/blocks/charts"));
    }

    #[test]
    fn gone_exact_blocks_parallax() {
        assert!(is_gone("/blocks/parallax"));
    }

    #[test]
    fn gone_exact_registry_tree() {
        assert!(is_gone("/registry/tree"));
    }

    // ── 410: prefix paths ─────────────────────────────────────────────────────

    #[test]
    fn gone_docs_extensions_exact() {
        // bare path without trailing slash
        assert!(is_gone("/docs/extensions"));
    }

    #[test]
    fn gone_docs_extensions_prefix() {
        assert!(is_gone("/docs/extensions/slider-hover"));
        assert!(is_gone("/docs/extensions/bento-grid"));
        assert!(is_gone("/docs/extensions/tabs-shadow"));
        assert!(is_gone("/docs/extensions/steps"));
        assert!(is_gone("/docs/extensions/css-pills"));
        assert!(is_gone("/docs/extensions/expandable"));
    }

    #[test]
    fn gone_core_bare_and_prefix() {
        assert!(is_gone("/core")); // bare path
        assert!(is_gone("/core/something")); // subtree
    }

    #[test]
    fn gone_ui_bare_and_prefix() {
        assert!(is_gone("/ui"));
        assert!(is_gone("/ui/something"));
    }

    #[test]
    fn gone_extensions_bare_and_prefix() {
        assert!(is_gone("/extensions"));
        assert!(is_gone("/extensions/something"));
    }

    #[test]
    fn gone_docs_ui_bare_and_prefix() {
        assert!(is_gone("/docs/ui"));
        assert!(is_gone("/docs/ui/something"));
    }

    #[test]
    fn gone_theme_generator_sub_path() {
        assert!(is_gone("/theme-generator/colors"));
    }

    #[test]
    fn gone_parallax_prefixes() {
        assert!(is_gone("/parallax-ipad-scroll/demo"));
        assert!(is_gone("/parallax-scrolls/demo"));
    }

    // ── 410: stale /view/* + /docs/extensions leak ───────────────────────────

    #[test]
    fn gone_view_with_docs_extensions_leak() {
        assert!(is_gone("/view/sidenav08/docs/extensions"));
        assert!(is_gone("/view/sidenav03/docs/extensions/orbiting-circles"));
    }

    // ── 410: registry paths without file extension ────────────────────────────

    #[test]
    fn gone_registry_blocks_bare_slug() {
        assert!(is_gone("/registry/blocks/pricing10"));
        assert!(is_gone("/registry/blocks/sidenav05"));
        assert!(is_gone("/registry/blocks/footer02"));
        assert!(is_gone("/registry/blocks/login03"));
        assert!(is_gone("/registry/blocks/sidenav-inset-right"));
        assert!(is_gone("/registry/blocks/sidenav-routes-simplified"));
    }

    #[test]
    fn gone_registry_styles_bare_slug() {
        assert!(is_gone("/registry/styles/some-style"));
    }

    // ── 301: /docs → /docs/components ────────────────────────────────────────

    #[test]
    fn redirect_bare_docs() {
        assert_eq!(redirect_target("/docs"), Some("/docs/components".to_owned()));
    }

    // ── 301: /components[/*] → /docs/components[/*] ──────────────────────────

    #[test]
    fn redirect_components_listing() {
        assert_eq!(redirect_target("/components"), Some("/docs/components".to_owned()));
    }

    #[test]
    fn redirect_components_with_name() {
        assert_eq!(redirect_target("/components/accordion"), Some("/docs/components/accordion".to_owned()));
        assert_eq!(redirect_target("/components/badge"), Some("/docs/components/badge".to_owned()));
        assert_eq!(redirect_target("/components/alert"), Some("/docs/components/alert".to_owned()));
        assert_eq!(redirect_target("/components/avatar"), Some("/docs/components/avatar".to_owned()));
        assert_eq!(redirect_target("/components/breadcrumb"), Some("/docs/components/breadcrumb".to_owned()));
        assert_eq!(redirect_target("/components/alert-dialog"), Some("/docs/components/alert-dialog".to_owned()));
    }

    // ── 301: /demo-components/* → /docs/components/* ─────────────────────────

    #[test]
    fn redirect_demo_components() {
        assert_eq!(redirect_target("/demo-components/button"), Some("/docs/components/button".to_owned()));
        assert_eq!(redirect_target("/demo-components/card"), Some("/docs/components/card".to_owned()));
    }

    // ── pass-through: valid paths ─────────────────────────────────────────────

    #[test]
    fn pass_through_home() {
        assert!(is_pass_through("/"));
    }

    #[test]
    fn pass_through_docs_components() {
        // ComponentsRoutes::base_url() = "/docs/components"
        assert!(is_pass_through("/docs/components"));
        assert!(is_pass_through("/docs/components/button"));
        assert!(is_pass_through("/docs/components/alert-dialog"));
    }

    #[test]
    fn pass_through_docs_hooks() {
        // HooksRoutes::base_url() = "/docs/hooks"
        assert!(is_pass_through("/docs/hooks"));
        assert!(is_pass_through("/docs/hooks/use-copy-clipboard"));
    }

    #[test]
    fn pass_through_blocks() {
        assert!(is_pass_through("/blocks/login"));
        assert!(is_pass_through("/blocks/sidenav"));
    }

    #[test]
    fn pass_through_view_valid() {
        // /view/* without a /docs/extensions segment should pass through
        assert!(is_pass_through("/view/login-01"));
        assert!(is_pass_through("/view/sidenav03"));
    }

    #[test]
    fn pass_through_registry_with_extension() {
        assert!(is_pass_through("/registry/blocks/button.md"));
        assert!(is_pass_through("/registry/blocks/button.json"));
        assert!(is_pass_through("/registry/styles/default/button.md"));
    }
}
