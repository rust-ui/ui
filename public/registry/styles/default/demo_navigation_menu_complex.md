---
title: "Demo Navigation Menu Complex"
name: "demo_navigation_menu_complex"
cargo_dependencies: []
registry_dependencies: ["navigation_menu"]
type: "components:demos"
path: "demos/demo_navigation_menu_complex.rs"
---

# Demo Navigation Menu Complex

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_navigation_menu_complex
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger, navigation_menu_trigger_style,
};

#[component]
fn ListItem(#[prop(into)] href: String, #[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="block p-3 space-y-1 leading-none no-underline rounded-md transition-colors outline-none select-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground"
            >
                <div class="text-sm font-medium leading-none">{title}</div>
                <p class="text-sm leading-snug line-clamp-2 text-muted-foreground">{children()}</p>
            </a>
        </li>
    }
}

#[component]
pub fn DemoNavigationMenuComplex() -> impl IntoView {
    view! {
        <div class="flex justify-center items-start py-8 min-h-[350px]">
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-3 p-0 md:grid-cols-3 w-[500px]">
                                <ListItem href="#" title="Analytics">
                                    "Track performance, user behavior, and key metrics."
                                </ListItem>
                                <ListItem href="#" title="Automation">
                                    "Build workflows and automate repetitive tasks."
                                </ListItem>
                                <ListItem href="#" title="Security">
                                    "Protect your data with enterprise-grade security."
                                </ListItem>
                                <ListItem href="#" title="Integrations">
                                    "Connect with your favorite third-party tools."
                                </ListItem>
                                <ListItem href="#" title="API">
                                    "Programmatic access to all platform features."
                                </ListItem>
                                <ListItem href="#" title="CLI">
                                    "Manage your project from the command line."
                                </ListItem>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Resources"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px]">
                                <ListItem href="#" title="Blog">
                                    "Articles and tutorials from our engineering team."
                                </ListItem>
                                <ListItem href="#" title="Case Studies">
                                    "See how companies use our platform at scale."
                                </ListItem>
                                <ListItem href="#" title="Guides">
                                    "Step-by-step guides for common use cases."
                                </ListItem>
                                <ListItem href="#" title="Changelog">
                                    "What's new — features, improvements, and fixes."
                                </ListItem>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink class=navigation_menu_trigger_style() href="#">
                            "Pricing"
                        </NavigationMenuLink>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink class=navigation_menu_trigger_style() href="#">
                            "Docs"
                        </NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}
```
