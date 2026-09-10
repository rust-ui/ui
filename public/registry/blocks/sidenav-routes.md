


```rust
use std::fmt;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Debug, PartialEq)]
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
        "view"
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SidenavRoutes::Sidenav01 => "sidenav01",
            SidenavRoutes::Sidenav02 => "sidenav02",
            SidenavRoutes::Sidenav03 => "sidenav03",
            SidenavRoutes::Sidenav04 => "sidenav04",
            SidenavRoutes::Sidenav05 => "sidenav05",
            SidenavRoutes::Sidenav06 => "sidenav06",
            SidenavRoutes::Sidenav07 => "sidenav07",
            SidenavRoutes::Sidenav08 => "sidenav08",
            SidenavRoutes::Sidenav09 => "sidenav09",
            SidenavRoutes::Sidenav10 => "sidenav10",
            SidenavRoutes::Sidenav11 => "sidenav11",
        }
    }

    pub fn all() -> Vec<SidenavRoutes> {
        vec![
            SidenavRoutes::Sidenav01,
            SidenavRoutes::Sidenav02,
            SidenavRoutes::Sidenav03,
            SidenavRoutes::Sidenav04,
            SidenavRoutes::Sidenav05,
            SidenavRoutes::Sidenav06,
            SidenavRoutes::Sidenav07,
            SidenavRoutes::Sidenav08,
            SidenavRoutes::Sidenav09,
            SidenavRoutes::Sidenav10,
            SidenavRoutes::Sidenav11,
        ]
    }

    pub fn to_title(&self) -> &'static str {
        match self {
            SidenavRoutes::Sidenav01 => "Sidenav 01",
            SidenavRoutes::Sidenav02 => "Sidenav 02",
            SidenavRoutes::Sidenav03 => "Sidenav 03",
            SidenavRoutes::Sidenav04 => "Sidenav 04",
            SidenavRoutes::Sidenav05 => "Sidenav 05",
            SidenavRoutes::Sidenav06 => "Sidenav 06",
            SidenavRoutes::Sidenav07 => "Sidenav 07",
            SidenavRoutes::Sidenav08 => "Sidenav 08",
            SidenavRoutes::Sidenav09 => "Sidenav 09",
            SidenavRoutes::Sidenav10 => "Sidenav 10",
            SidenavRoutes::Sidenav11 => "Sidenav 11",
        }
    }

    pub fn to_route(self) -> String {
        format!("{}/{}", Self::view_segment(), self.as_str())
    }

    pub fn from_path(path: &str) -> Self {
        // Iterate in reverse to match higher numbers first (sidenav10 before sidenav01)
        for route in Self::all().into_iter().rev() {
            if path.contains(route.as_str()) {
                return route;
            }
        }
        Self::Sidenav01
    }
}

impl fmt::Display for SidenavRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocsRoutes {
    Components,
    Hooks,
}

impl DocsRoutes {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocsRoutes::Components => "components",
            DocsRoutes::Hooks => "hooks",
        }
    }

    pub fn to_title(&self) -> &'static str {
        match self {
            DocsRoutes::Components => "Components",
            DocsRoutes::Hooks => "Hooks",
        }
    }

    pub fn all() -> &'static [DocsRoutes] {
        &[DocsRoutes::Components, DocsRoutes::Hooks]
    }
}

impl fmt::Display for DocsRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentsRoutes {
    Accordion,
    Alert,
    AlertDialog,
    Badge,
    Button,
    Card,
    Checkbox,
    Dialog,
    Input,
    Label,
    Separator,
    Spinner,
}

impl ComponentsRoutes {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentsRoutes::Accordion => "accordion",
            ComponentsRoutes::Alert => "alert",
            ComponentsRoutes::AlertDialog => "alert-dialog",
            ComponentsRoutes::Badge => "badge",
            ComponentsRoutes::Button => "button",
            ComponentsRoutes::Card => "card",
            ComponentsRoutes::Checkbox => "checkbox",
            ComponentsRoutes::Dialog => "dialog",
            ComponentsRoutes::Input => "input",
            ComponentsRoutes::Label => "label",
            ComponentsRoutes::Separator => "separator",
            ComponentsRoutes::Spinner => "spinner",
        }
    }

    pub fn to_title(&self) -> &'static str {
        match self {
            ComponentsRoutes::Accordion => "Accordion",
            ComponentsRoutes::Alert => "Alert",
            ComponentsRoutes::AlertDialog => "Alert Dialog",
            ComponentsRoutes::Badge => "Badge",
            ComponentsRoutes::Button => "Button",
            ComponentsRoutes::Card => "Card",
            ComponentsRoutes::Checkbox => "Checkbox",
            ComponentsRoutes::Dialog => "Dialog",
            ComponentsRoutes::Input => "Input",
            ComponentsRoutes::Label => "Label",
            ComponentsRoutes::Separator => "Separator",
            ComponentsRoutes::Spinner => "Spinner",
        }
    }

    pub fn all() -> &'static [ComponentsRoutes] {
        &[
            ComponentsRoutes::Accordion,
            ComponentsRoutes::Alert,
            ComponentsRoutes::AlertDialog,
            ComponentsRoutes::Badge,
            ComponentsRoutes::Button,
            ComponentsRoutes::Card,
            ComponentsRoutes::Checkbox,
            ComponentsRoutes::Dialog,
            ComponentsRoutes::Input,
            ComponentsRoutes::Label,
            ComponentsRoutes::Separator,
            ComponentsRoutes::Spinner,
        ]
    }
}

impl fmt::Display for ComponentsRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HooksRoutes {
    UseCopyClipboard,
    UseLockBodyScroll,
    UseRandom,
}

impl HooksRoutes {
    pub fn as_str(&self) -> &'static str {
        match self {
            HooksRoutes::UseCopyClipboard => "use-copy-clipboard",
            HooksRoutes::UseLockBodyScroll => "use-lock-body-scroll",
            HooksRoutes::UseRandom => "use-random",
        }
    }

    pub fn to_title(&self) -> &'static str {
        match self {
            HooksRoutes::UseCopyClipboard => "Use Copy Clipboard",
            HooksRoutes::UseLockBodyScroll => "Use Lock Body Scroll",
            HooksRoutes::UseRandom => "Use Random",
        }
    }

    pub fn all() -> &'static [HooksRoutes] {
        &[HooksRoutes::UseCopyClipboard, HooksRoutes::UseLockBodyScroll, HooksRoutes::UseRandom]
    }
}

impl fmt::Display for HooksRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```