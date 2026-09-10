


```rust
use dioxus::prelude::*;

use super::sidenav_routes::{DocsRoutes, SidenavRoutes};

/// Renders a list of links for navigating between sidenav routes.
#[component]
pub fn SidenavRoutesSelector(sidenav_route: SidenavRoutes) -> Element {
    let docs_routes = DocsRoutes::all();

    rsx! {
        div { class: "flex flex-col gap-1 p-2",
            for doc_route in docs_routes.iter().copied() {
                a {
                    key: "{doc_route}",
                    href: "/{sidenav_route}/{doc_route}",
                    class: "flex items-center gap-2 px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                    "{doc_route.to_title()}"
                }
            }
        }
    }
}
```