use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn log_requests(req: Request, next: Next) -> Response {
    let path = req.uri().path();

    // Check if this is a page request (not static assets)
    if !path.starts_with("/pkg/")
        && !path.starts_with("/app/")
        && !path.ends_with(".js")
        && !path.ends_with(".css")
        && !path.ends_with(".wasm")
        && !path.ends_with(".png")
        && !path.ends_with(".jpg")
        && !path.ends_with(".jpeg")
        && !path.ends_with(".webp")
        && !path.ends_with(".svg")
        && !path.ends_with(".ico")
    {
        // For API calls, try to extract the actual page path from the request
        if path.starts_with("/api/") {
            // Look for referer header to get the actual page being accessed
            if let Some(referer) = req.headers().get("referer")
                && let Ok(referer_str) = referer.to_str()
                && let Ok(url) = url::Url::parse(referer_str)
            {
                let page_path = url.path();
                if page_path != "/" && !page_path.is_empty() {
                    let clean_path = page_path.strip_prefix('/').unwrap_or(page_path);
                    tracing::info!("Page accessed: {}", clean_path);
                }
            }
        } else {
            // Direct page access
            let clean_path = path.strip_prefix('/').unwrap_or(path);

            if !clean_path.is_empty() {
                tracing::info!("Page accessed: {}", clean_path);
            }
        }
    }

    next.run(req).await
}
