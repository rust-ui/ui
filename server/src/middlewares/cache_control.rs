use axum::extract::Request;
use axum::http::header::{CACHE_CONTROL, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;

pub async fn add_cache_headers(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let mut response = next.run(req).await;

    // Determine cache duration based on file type
    let cache_header = if is_immutable_asset(&path) {
        // Hashed assets (like Leptos outputs) can be cached for 1 year
        "public, max-age=31536000, immutable"
    } else if is_static_asset(&path) {
        // Other static assets: cache for 1 week
        "public, max-age=604800"
    } else {
        // HTML pages and API responses: no cache or short cache
        return response;
    };

    if let Ok(header_value) = HeaderValue::from_str(cache_header) {
        response.headers_mut().insert(CACHE_CONTROL, header_value);
    }

    response
}

fn is_immutable_asset(path: &str) -> bool {
    // Leptos generates hashed filenames for JS/WASM/CSS in /pkg/
    // These can be cached indefinitely as the hash changes when content changes
    path.starts_with("/pkg/") && (path.ends_with(".js") || path.ends_with(".wasm") || path.ends_with(".css"))
}

fn is_static_asset(path: &str) -> bool {
    // Non-hashed static assets should have reasonable cache times
    path.starts_with("/app/")
        || path.starts_with("/coming_soon/")
        || path.starts_with("/hooks/")
        || path.ends_with(".js")
        || path.ends_with(".css")
        || path.ends_with(".woff")
        || path.ends_with(".woff2")
        || path.ends_with(".ttf")
        || path.ends_with(".eot")
        || path.ends_with(".svg")
        || path.ends_with(".png")
        || path.ends_with(".jpg")
        || path.ends_with(".jpeg")
        || path.ends_with(".gif")
        || path.ends_with(".webp")
        || path.ends_with(".ico")
}
