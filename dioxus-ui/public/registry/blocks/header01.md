


```rust
use dioxus::prelude::*;
use icons::{AlignHorizontalSpaceAround, Blocks, Compass, Frame, LogIn, Menu, Search, X};

use crate::components::demos::demo_accordion_icons::DemoAccordionIcons;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::header::{
    Header, IconWrapper, InsetCard, NavMenu, NavMenuContent, NavMenuContentInset, NavMenuFixed, NavMenuHomeLink,
    NavMenuItem, NavMenuLink, NavMenuLinkDescription, NavMenuLinkGrid, NavMenuLinkTitle, NavMenuList, NavMenuMiddle,
    NavMenuTitle, NavMenuTrigger, NavMenuWrapper,
};
use crate::components::ui::theme_toggle::ThemeToggle;

#[component]
pub fn Header01() -> Element {
    let is_mobile_menu_open = use_signal(|| false);
    let data_state = move || if is_mobile_menu_open() { "active" } else { "inactive" };

    rsx! {
        Header { data_state: data_state(),
            NavMenuFixed {
                NavMenuWrapper {
                    div { class: "flex relative flex-wrap justify-between items-center lg:py-3",
                        div { class: "flex gap-8 justify-between items-center max-lg:in-data-[state=active]:border-b max-lg:h-14 max-lg:w-full",
                            NavMenuHomeLink { href: "/", aria_label: "home",
                                span { class: "text-lg font-semibold", "Rust/UI" }
                            }
                            MobileMenuTrigger { is_open: is_mobile_menu_open }
                        }

                        NavMenuMiddle {
                            NavMenu {
                                NavMenuList {
                                    FirstNavMenu {}
                                    SecondNavMenu {}
                                    NavMenuItem { NavMenuLink { href: "/icons", "Icons" } }
                                    NavMenuItem { NavMenuLink { href: "/charts", "Charts" } }
                                }
                            }
                        }

                        DemoNavMenuRight {}
                    }

                    MobileMenu { is_open: is_mobile_menu_open }
                }
            }
        }
    }
}

#[component]
pub fn FirstNavMenu() -> Element {
    rsx! {
        NavMenuItem {
            NavMenuTrigger { href: "/docs/components", "Registry" }
            NavMenuContent { class: "min-w-lg",
                NavMenuContentInset { class: "grid gap-2 grid-cols-[auto_1fr]",
                    InsetCard {
                        NavMenuTitle { "Content" }
                        ul { class: "mt-1 space-y-2",
                            li {
                                NavMenuLinkGrid { href: "/docs/components",
                                    IconWrapper { Blocks {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Components" }
                                        NavMenuLinkDescription { "Find any components" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/icons",
                                    IconWrapper { Search {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Icons" }
                                        NavMenuLinkDescription { "More than 1600 icons" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/docs/hooks",
                                    IconWrapper { Compass {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Hooks" }
                                        NavMenuLinkDescription { "Reusable logic functions" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "p-2",
                        NavMenuTitle { "Get Started" }
                        ul { class: "mt-1",
                            li {
                                NavMenuLinkGrid { href: "/docs/introduction", class: "items-center",
                                    IconWrapper { Frame {} }
                                    NavMenuLinkTitle { "Introduction" }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/docs/installation", class: "items-center",
                                    IconWrapper { Compass {} }
                                    NavMenuLinkTitle { "Installation" }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/docs/changelog", class: "items-center",
                                    IconWrapper { AlignHorizontalSpaceAround {} }
                                    NavMenuLinkTitle { "Changelog" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SecondNavMenu() -> Element {
    rsx! {
        NavMenuItem {
            NavMenuTrigger { href: "/blocks/login", "Blocks" }
            NavMenuContent { class: "w-[300px]",
                NavMenuContentInset {
                    InsetCard {
                        h3 { class: "px-2 mb-4 text-sm font-semibold text-foreground", "Demonstrations" }
                        ul { class: "space-y-2",
                            li {
                                NavMenuLinkGrid { href: "/blocks/login",
                                    IconWrapper { LogIn {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Login" }
                                        NavMenuLinkDescription { "Login Blocks" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/blocks/sidenav",
                                    IconWrapper { Menu {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Sidenav" }
                                        NavMenuLinkDescription { "Sidenav Blocks" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/blocks/footers",
                                    IconWrapper { AlignHorizontalSpaceAround {} }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Footers" }
                                        NavMenuLinkDescription { "Footers Blocks" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "p-2 w-full",
                        NavMenuLink { href: "/blocks/login", class: "items-start font-normal underline text-primary",
                            "Browse all Blocks"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DemoNavMenuRight() -> Element {
    rsx! {
        div { class: "hidden lg:flex lg:gap-6 lg:p-0 lg:m-0 lg:bg-transparent lg:border-transparent lg:shadow-none dark:shadow-none lg:w-fit dark:lg:bg-transparent",
            div { class: "flex flex-row gap-3",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    Search { class: "size-4" }
                }
                div { class: "flex justify-center items-center mr-1",
                    ThemeToggle {}
                }
            }
        }
    }
}

#[component]
pub fn MobileMenuTrigger(is_open: Signal<bool>) -> Element {
    rsx! {
        button {
            "aria-label": "Open Menu",
            class: "block relative z-20 p-2.5 -m-2.5 -mr-3 cursor-pointer lg:hidden",
            onclick: move |_| {
                is_open.set(!is_open());
            },
            Menu { class: "m-auto duration-200 in-data-[state=active]:rotate-180 in-data-[state=active]:scale-0 in-data-[state=active]:opacity-0 size-5" }
            X { class: "absolute inset-0 m-auto opacity-0 duration-200 scale-0 -rotate-180 in-data-[state=active]:rotate-0 in-data-[state=active]:scale-100 in-data-[state=active]:opacity-100 size-5" }
        }
    }
}

#[component]
pub fn MobileMenu(is_open: Signal<bool>) -> Element {
    let display_class = if is_open() { "flex" } else { "hidden" };

    rsx! {
        div { class: "flex-col gap-4 mt-6 w-full lg:hidden mb-4 {display_class}",
            DemoAccordionIcons {}
            div { class: "flex flex-row gap-3 w-full",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    Search { class: "size-4" }
                }
                div { class: "flex justify-center items-center mr-1",
                    ThemeToggle {}
                }
            }
        }
    }
}
```