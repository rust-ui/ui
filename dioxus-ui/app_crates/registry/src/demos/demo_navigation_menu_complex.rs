use dioxus::prelude::*;

use crate::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger, navigation_menu_trigger_style,
};

#[component]
fn ListItem(#[props(into)] href: String, #[props(into)] title: String, children: Element) -> Element {
    rsx! {
        li {
            a {
                href: "{href}",
                class: "block p-3 space-y-1 leading-none no-underline rounded-md transition-colors outline-none select-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                div { class: "text-sm font-medium leading-none", "{title}" }
                p { class: "text-sm leading-snug line-clamp-2 text-muted-foreground", {children} }
            }
        }
    }
}

#[component]
pub fn DemoNavigationMenuComplex() -> Element {
    rsx! {
        div { class: "flex justify-center items-start py-8 min-h-[350px]",
            NavigationMenu {
                NavigationMenuList {
                    NavigationMenuItem {
                        NavigationMenuTrigger { "Products" }
                        NavigationMenuContent {
                            ul { class: "grid gap-3 p-0 md:grid-cols-3 w-[500px]",
                                ListItem { href: "#", title: "Analytics",
                                    "Track performance, user behavior, and key metrics."
                                }
                                ListItem { href: "#", title: "Automation",
                                    "Build workflows and automate repetitive tasks."
                                }
                                ListItem { href: "#", title: "Security",
                                    "Protect your data with enterprise-grade security."
                                }
                                ListItem { href: "#", title: "Integrations",
                                    "Connect with your favorite third-party tools."
                                }
                                ListItem { href: "#", title: "API",
                                    "Programmatic access to all platform features."
                                }
                                ListItem { href: "#", title: "CLI",
                                    "Manage your project from the command line."
                                }
                            }
                        }
                    }

                    NavigationMenuItem {
                        NavigationMenuTrigger { "Resources" }
                        NavigationMenuContent {
                            ul { class: "grid gap-3 p-0 md:grid-cols-2 w-[400px]",
                                ListItem { href: "#", title: "Blog",
                                    "Articles and tutorials from our engineering team."
                                }
                                ListItem { href: "#", title: "Case Studies",
                                    "See how companies use our platform at scale."
                                }
                                ListItem { href: "#", title: "Guides",
                                    "Step-by-step guides for common use cases."
                                }
                                ListItem { href: "#", title: "Changelog",
                                    "What's new — features, improvements, and fixes."
                                }
                            }
                        }
                    }

                    NavigationMenuItem {
                        NavigationMenuLink { class: navigation_menu_trigger_style(), href: "#",
                            "Pricing"
                        }
                    }

                    NavigationMenuItem {
                        NavigationMenuLink { class: navigation_menu_trigger_style(), href: "#",
                            "Docs"
                        }
                    }
                }
            }
        }
    }
}
