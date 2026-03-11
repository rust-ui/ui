use axum::extract::Request;
use axum::http::header::LOCATION;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

/// Middleware to enforce canonical URLs with 301 permanent redirects
///
/// # Canonical URL Strategy
///
/// **Canonical:** `https://rust-ui.com/` (no www)
///
/// **Redirects (301 Permanent):**
/// - `http://rust-ui.com/*`        → `https://rust-ui.com/*`
/// - `http://www.rust-ui.com/*`    → `https://rust-ui.com/*`
/// - `https://www.rust-ui.com/*`   → `https://rust-ui.com/*`
///
/// # SEO Benefits
///
/// - **Prevents duplicate content penalties**: Google won't index 4 versions of the same page
/// - **Consolidates link equity**: All backlinks count toward one canonical URL
/// - **Improves crawl efficiency**: Search engines don't waste time on duplicates
/// - **301 status**: Permanent redirect preserves SEO value
///
/// # Implementation
///
/// This middleware runs FIRST in the middleware chain (before authentication, caching, etc.)
/// to ensure all requests are canonicalized before processing.
pub async fn enforce_canonical_url(req: Request, next: Next) -> Response {
    // Extract the host header
    let host = req.headers().get("host").and_then(|h| h.to_str().ok()).unwrap_or("");

    // Check if we need to redirect
    let needs_redirect = host.starts_with("www.") || req.uri().scheme_str().is_some_and(|s| s != "https");

    if needs_redirect {
        // Build the canonical URL
        let path_and_query = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");

        // Remove www. prefix if present
        let canonical_host = host.strip_prefix("www.").unwrap_or(host);

        // Build canonical URL (always HTTPS, always non-www)
        let canonical_url = format!("https://{}{}", canonical_host, path_and_query);

        // Return 301 Permanent Redirect
        let Ok(location) = HeaderValue::from_str(&canonical_url) else {
            return next.run(req).await;
        };
        return (StatusCode::MOVED_PERMANENTLY, [(LOCATION, location)]).into_response();
    }

    // No redirect needed, continue to next middleware
    next.run(req).await
}

/// Production-only middleware that enforces canonical URLs
///
/// In development (localhost), we skip the redirect logic since:
/// - Local development doesn't need HTTPS enforcement
/// - localhost URLs don't have www variants
/// - Developers need to test different scenarios
pub async fn enforce_canonical_url_prod(req: Request, next: Next) -> Response {
    let host = req.headers().get("host").and_then(|h| h.to_str().ok()).unwrap_or("");

    // Skip redirect for localhost/127.0.0.1 (development)
    if host.starts_with("localhost") || host.starts_with("127.0.0.1") {
        return next.run(req).await;
    }

    // In production, enforce canonical URL
    enforce_canonical_url(req, next).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_www_detection() {
        let hosts =
            [("www.rust-ui.com", true), ("rust-ui.com", false), ("www.example.com", true), ("example.com", false)];

        for (host, should_strip) in hosts {
            let has_www = host.starts_with("www.");
            assert_eq!(has_www, should_strip, "Failed for host: {}", host);
        }
    }

    #[test]
    fn test_canonical_url_generation() {
        let cases = [
            ("www.rust-ui.com", "/", "https://rust-ui.com/"),
            ("www.rust-ui.com", "/docs/components", "https://rust-ui.com/docs/components"),
            ("www.rust-ui.com", "/blocks?page=2", "https://rust-ui.com/blocks?page=2"),
            ("rust-ui.com", "/", "https://rust-ui.com/"),
        ];

        for (host, path, expected) in cases {
            let canonical_host = host.strip_prefix("www.").unwrap_or(host);
            let result = format!("https://{}{}", canonical_host, path);
            assert_eq!(result, expected, "Failed for host: {}, path: {}", host, path);
        }
    }

    #[test]
    fn test_localhost_detection() {
        let hosts =
            [("localhost:3000", true), ("127.0.0.1:3000", true), ("rust-ui.com", false), ("www.rust-ui.com", false)];

        for (host, is_local) in hosts {
            let result = host.starts_with("localhost") || host.starts_with("127.0.0.1");
            assert_eq!(result, is_local, "Failed for host: {}", host);
        }
    }
}
