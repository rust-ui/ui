use dioxus::prelude::*;

use crate::app::Route;

/// Returns a closure that checks if a given path matches the current route.
///
/// For the root path "/", checks for exact match.
/// For other paths, checks if the current path starts with the given path.
pub fn use_is_current_path() -> impl Fn(&'static str) -> bool + Clone {
    let router = use_router();

    move |path: &'static str| -> bool {
        let current = router.current::<Route>().to_string();
        if path == "/" { current == "/" } else { current.starts_with(path) }
    }
}
