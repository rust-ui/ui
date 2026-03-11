---
title: "Sidenav Routes"
name: "sidenav_routes"
cargo_dependencies: []
registry_dependencies: []
type: "components:blocks"
path: "blocks/sidenav_routes.rs"
---

# Sidenav Routes

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add sidenav_routes
```

## Component Code

```rust
use heck::ToTitleCase;
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum SidenavRoutes {
    Sidenav01,
    Sidenav02,
    Sidenav03,
    Sidenav04,
    Sidenav05,
    Sidenav06,
    Sidenav07,
    Sidenav08,
    Sidenav09,
    Sidenav10,
    Sidenav11,
}

impl SidenavRoutes {
    pub fn view_segment() -> &'static str {
        // * 💁‍♂️ "view" for the moment, I will switch back to "view" when all good.
        "view"
    }

    /// Detect which sidenav route is active based on URL path.
    /// Returns `Sidenav01` as default if no match is found.
    pub fn from_path(path: &str) -> Self {
        use strum::IntoEnumIterator;
        // Iterate in reverse to match higher numbers first (Sidenav10 before Sidenav01)
        Self::iter().rev().find(|route| path.contains(route.as_ref())).unwrap_or(Self::Sidenav01)
    }

    pub fn to_route(self) -> String {
        format!("{}/{}", Self::view_segment(), self.as_ref())
    }

    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum DocsRoutes {
    Components,
    Hooks,
}

impl DocsRoutes {
    pub fn base_segment() -> &'static str {
        "docs"
    }

    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum ComponentsRoutes {
    Accordion,
    Alert,
    AlertDialog,
    Button,
}

impl ComponentsRoutes {
    pub fn base_segment() -> &'static str {
        "components"
    }

    // http://localhost:3000/view/{sidenav_route}/docs/components
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), DocsRoutes::Components.as_ref())
    }

    // http://localhost:3000/view/{sidenav_route}/docs/components/XXXXXXX
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }

    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Display, AsRefStr, IntoStaticStr, EnumString, EnumIter, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum HooksRoutes {
    UseCopyClipboard,
    UseLockBodyScroll,
    UseRandom,
}

impl HooksRoutes {
    pub fn base_segment() -> &'static str {
        "hooks"
    }

    // http://localhost:3000/view/{sidenav_route}/docs/hooks
    pub fn base_url_with_sidenav(sidenav: SidenavRoutes) -> String {
        format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), DocsRoutes::Hooks.as_ref())
    }

    // http://localhost:3000/view/{sidenav_route}/docs/hooks/XXXXXXX
    pub fn to_route_with_sidenav(self, sidenav: SidenavRoutes) -> String {
        format!("{}/{}", Self::base_url_with_sidenav(sidenav), self.as_ref())
    }

    pub fn to_title(self) -> String {
        self.as_ref().to_title_case()
    }
}
```
