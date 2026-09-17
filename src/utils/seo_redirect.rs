//! Real HTTP 301s for old URLs that the Dioxus router internally rewrites.
//!
//! `#[redirect(...)]` rewrites in place instead of issuing a redirect. Without
//! this, hitting e.g. `/components/accordion` renders `/docs/components/accordion`'s
//! content at HTTP 200 with a canonical tag pointing elsewhere, which Google
//! reports as "Duplicate, Google chose different canonical" rather than
//! consolidating link equity onto the canonical URL via a proper 301.
#[cfg(feature = "server")]
pub fn redirect_to_canonical(canonical_path: &str) {
    use dioxus::fullstack::http::HeaderValue;
    use dioxus::fullstack::http::header::LOCATION;
    use dioxus::fullstack::{FullstackContext, StatusCode};

    let Some(ctx) = FullstackContext::current() else { return };
    let current_path = ctx.parts_mut().uri.path().to_string();
    if current_path == canonical_path {
        return;
    }
    let Ok(location) = HeaderValue::from_str(canonical_path) else { return };
    ctx.add_response_header(LOCATION, location);
    FullstackContext::commit_http_status(StatusCode::MOVED_PERMANENTLY, None);
}
