#![recursion_limit = "256"]

mod middlewares;

use app::app::App;
use app::shell::shell;
use axum::routing::get_service;
use axum::{Router, middleware};
use dotenv::dotenv;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower_http::services::ServeFile;

use crate::middlewares::cache_control::add_cache_headers;
use crate::middlewares::canonical_redirect::enforce_canonical_url_prod;
use crate::middlewares::deprecated_routes::handle_deprecated_routes;
use crate::middlewares::log_requests::log_requests;

// TODO 🚑 Shortfix to register all public/docs/*.md files to Router. I've tried using dynamic path but it was not working... It happens when server start so no runtime overhead. Find a better solution later.
fn add_all_markdown_routes<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    use std::fs;

    fn collect_md_files(dir: &str, files: &mut Vec<String>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                collect_md_files(&path.to_string_lossy(), files)?;
            } else if path.extension().is_some_and(|ext| ext == "md") {
                files.push(path.to_string_lossy().to_string());
            }
        }
        Ok(())
    }

    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| ".".to_owned());

    // When LEPTOS_SITE_ROOT is set (production), public/* is already at site root
    // When not set (dev), we're in project root and need "public/" prefix
    let (docs_path, registry_styles_path, registry_blocks_path, index_path) =
        if std::env::var("LEPTOS_SITE_ROOT").is_ok() {
            (
                format!("{}/docs", site_root),
                format!("{}/registry/styles/default", site_root),
                format!("{}/registry/blocks", site_root),
                format!("{}/index", site_root),
            )
        } else {
            (
                format!("{}/public/docs", site_root),
                format!("{}/public/registry/styles/default", site_root),
                format!("{}/public/registry/blocks", site_root),
                format!("{}/index", site_root),
            )
        };

    let mut files = Vec::new();
    let _ = collect_md_files(&docs_path, &mut files);
    let _ = collect_md_files(&registry_styles_path, &mut files);
    let _ = collect_md_files(&registry_blocks_path, &mut files);
    let _ = collect_md_files(&index_path, &mut files);

    // Determine the prefix to strip based on environment
    let prefix_to_strip =
        if std::env::var("LEPTOS_SITE_ROOT").is_ok() { site_root } else { format!("{}/public", site_root) };

    files.into_iter().fold(router, |router, file_path| {
        let route_path = file_path.strip_prefix(&prefix_to_strip).unwrap_or(&file_path).to_owned();
        router.route_service(&route_path, get_service(ServeFile::new(file_path)))
    })
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    dotenv().ok();

    let conf = get_configuration(None).map_err(|err| {
        tracing::error!("Failed to get configuration: {err:?}");
        err
    })?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = add_all_markdown_routes(Router::new())
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(middleware::from_fn(enforce_canonical_url_prod)) // 🔗 FIRST: Canonicalize URLs
        .layer(middleware::from_fn(add_cache_headers))
        .layer(middleware::from_fn(handle_deprecated_routes))
        .layer(middleware::from_fn(log_requests))
        .with_state(leptos_options);

    log!("📂 Router configured with markdown routes");

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|err| {
        tracing::error!("Failed to bind to address {addr}: {err:?}");
        err
    })?;

    axum::serve(listener, app.into_make_service()).await.map_err(|err| {
        tracing::error!("Failed to serve application: {err:?}");
        err
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_markdown_route_path_stripping_dev() {
        // Simulate dev environment (LEPTOS_SITE_ROOT not set)
        let site_root = ".";
        let prefix_to_strip = format!("{}/public", site_root);

        // Simulate absolute file paths that would be collected
        let test_cases = vec![
            ("./public/docs/components/button.md", "/docs/components/button.md"),
            ("./public/docs/components/table.md", "/docs/components/table.md"),
            ("./public/registry/styles/default/button.md", "/registry/styles/default/button.md"),
        ];

        for (file_path, expected_route) in test_cases {
            let route_path = file_path.strip_prefix(&prefix_to_strip).unwrap_or(file_path).to_string();
            assert_eq!(
                route_path, expected_route,
                "Dev environment route path should be correctly stripped. Got: {}, Expected: {}",
                route_path, expected_route
            );
        }
    }

    #[test]
    fn test_markdown_route_path_stripping_prod() {
        // Simulate production environment (LEPTOS_SITE_ROOT is set)
        let site_root = "/var/www/app";
        let prefix_to_strip = site_root;

        let test_cases = vec![
            ("/var/www/app/docs/components/button.md", "/docs/components/button.md"),
            ("/var/www/app/docs/components/table.md", "/docs/components/table.md"),
            ("/var/www/app/registry/styles/default/button.md", "/registry/styles/default/button.md"),
        ];

        for (file_path, expected_route) in test_cases {
            let route_path = file_path.strip_prefix(prefix_to_strip).unwrap_or(file_path).to_string();
            assert_eq!(
                route_path, expected_route,
                "Prod environment route path should be correctly stripped. Got: {}, Expected: {}",
                route_path, expected_route
            );
        }
    }

    #[test]
    fn test_markdown_routes_dont_contain_absolute_paths() {
        // Verify that routes never contain full system paths
        let site_root = ".";
        let prefix_to_strip = format!("{}/public", site_root);

        let file_path = "./public/docs/components/button.md";
        let route_path = file_path.strip_prefix(&prefix_to_strip).unwrap_or(file_path).to_string();

        // Route should not contain "public" in the path
        assert!(
            !route_path.contains("./public"),
            "Route path should not contain './public' prefix. Got: {}",
            route_path
        );

        // Route should start with /
        assert!(route_path.starts_with('/'), "Route path should start with /. Got: {}", route_path);
    }
}
