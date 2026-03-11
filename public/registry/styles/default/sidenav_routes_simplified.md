---
title: "Sidenav Routes Simplified"
name: "sidenav_routes_simplified"
cargo_dependencies: []
registry_dependencies: ["sidenav"]
type: "components:blocks"
path: "blocks/sidenav_routes_simplified.rs"
---

# Sidenav Routes Simplified

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sidenav_routes_simplified
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route};
#[allow(unused_imports)]
use leptos_router::{MatchNestedRoutes, StaticSegment, WildcardSegment};

use super::sidenav_inset_right::SidenavInsetRight;
use super::sidenav_routes::DocsRoutes;
use crate::components::ui::sidenav::SidenavVariant;

#[component(transparent)]
pub fn SidenavRoutesSimplified(
    #[prop(into, optional)] data_variant: Option<SidenavVariant>,
) -> impl MatchNestedRoutes + Clone {
    view! {
        // * Layout with @sidenav_inset_right
        <ParentRoute
            path=StaticSegment(DocsRoutes::base_segment())
            view=move || {
                if let Some(variant) = data_variant {
                    view! { <SidenavInsetRight data_variant=variant /> }
                } else {
                    view! { <SidenavInsetRight /> }
                }
            }
        >
            <Route path=StaticSegment("") view=|| () />

            // Components section - simplified with WildcardSegment
            <ParentRoute path=StaticSegment(DocsRoutes::Components.as_ref()) view=|| view! { <Outlet /> }>
                <Route path=StaticSegment("") view=|| () />
                <Route path=WildcardSegment("component_path") view=|| () />
            </ParentRoute>

            // Hooks section - simplified with WildcardSegment
            <ParentRoute path=StaticSegment(DocsRoutes::Hooks.as_ref()) view=|| view! { <Outlet /> }>
                <Route path=StaticSegment("") view=|| () />
                <Route path=WildcardSegment("hook_path") view=|| () />
            </ParentRoute>
        </ParentRoute>
    }
    .into_inner()
}
```
