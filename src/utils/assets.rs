//! Bundled static assets for the site itself.
//!
//! `dx serve --platform ios` (and any release mobile/desktop build) serves
//! **only** assets that went through the `asset!()` macro: they are copied into
//! the bundle and content-hashed. Files that live under `public/` and are
//! referenced by a plain absolute path (`/images/thumbnails/x.webp`) are served
//! by the web dev server alone, so on device they 404 and render as a broken
//! image.
//!
//! Anything the site renders itself should resolve through this module. Plain
//! `public/` paths are fine only inside `app_crates/registry` blocks, which are
//! copy-paste output for downstream projects.

use dioxus::prelude::*;

/// Component thumbnail directory. Bundles every file under
/// `public/images/thumbnails/` (light + dark, `.webp` + `.png`).
pub const THUMBNAILS: Asset = asset!("/public/images/thumbnails");

/// Resolve a stored `/images/thumbnails/<file>` path (as emitted by the
/// registry generator into `SidenavItem`) to its bundled URL. Empty input
/// yields an empty string, so the caller's `is_empty()` guard still works.
///
/// TODO(registry-gen): once `build_registry_dioxus` can emit `asset!()`
/// directly, store `Asset` in `SidenavItem` and delete this shim.
pub fn thumbnail(path: &str) -> String {
    match path.rsplit_once('/') {
        Some((_, file)) if !file.is_empty() => format!("{THUMBNAILS}/{file}"),
        _ => String::new(),
    }
}
