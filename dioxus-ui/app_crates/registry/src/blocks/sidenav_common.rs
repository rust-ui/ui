use dioxus::prelude::*;
use icons::{BookOpen, Bot, ChevronRight, Component, Frame, Layers, Search, Settings, Sparkles, SquareTerminal};

use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::demos::demo_dropdown_menu_user::DemoDropdownMenuUser;
use crate::demos::demo_dropdown_menu_user_icon::DemoDropdownMenuUserIcon;
use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};
use crate::ui::label::Label;
use crate::ui::sheet::{Sheet, SheetContent, SheetContext, SheetDirection, SheetTrigger};
use crate::ui::sidenav::{
    DropdownMenuTriggerEllipsis, Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent,
    SidenavGroupLabel, SidenavHeader, SidenavInput, SidenavLink, SidenavMenu, SidenavMenuItem, SidenavMenuSub,
    SidenavMenuSubItem, SidenavVariant,
};

pub const COMPONENT_LINKS: &[(&str, &str)] = &[
    ("accordion", "Accordion"),
    ("alert", "Alert"),
    ("alert-dialog", "Alert Dialog"),
    ("button", "Button"),
    ("card", "Card"),
    ("checkbox", "Checkbox"),
    ("dialog", "Dialog"),
];

pub const HOOK_LINKS: &[(&str, &str)] = &[
    ("use-copy-clipboard", "Use Copy Clipboard"),
    ("use-lock-body-scroll", "Use Lock Body Scroll"),
    ("use-random", "Use Random"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SidenavPattern {
    Grouped,
    Collapsible,
    Submenus,
    Floating,
    CollapsibleSubmenus,
    Dropdown,
    Icons,
    Inset,
    Search,
    Right,
}

pub fn links_for(section: DocsRoutes, sidenav: SidenavRoutes) -> Vec<(String, String)> {
    let (base, links) = match section {
        DocsRoutes::Components => ("components", COMPONENT_LINKS),
        DocsRoutes::Hooks => ("hooks", HOOK_LINKS),
    };
    links
        .iter()
        .map(|(name, title)| {
            (
                format!("/{}/{}/{}", sidenav.to_route(), DocsRoutes::base_segment(), base) + "/" + name,
                (*title).to_string(),
            )
        })
        .collect()
}

pub fn sheet_target_id() -> Option<String> {
    try_consume_context::<SheetContext>().map(|context| context.target_id)
}

#[component]
pub fn SidenavLinkList(links: Vec<(String, String)>, #[props(default = false)] sub: bool) -> Element {
    let target_id = sheet_target_id();
    let items = rsx! {
        for (href, title) in links.iter() {
            if let Some(target_id) = target_id.as_ref() {
                div { key: "{href}-wrapper", "data-sheet-close": target_id,
                    SidenavLink { href: href.clone(), "{title}" }
                }
            } else {
                SidenavLink { key: "{href}", href: href.clone(), "{title}" }
            }
        }
    };
    if sub {
        rsx! { SidenavMenuSub { SidenavMenuSubItem { {items} } } }
    } else {
        items
    }
}

#[component]
pub fn SidenavMobileSheet(
    current_section: DocsRoutes,
    sidenav_route: SidenavRoutes,
    #[props(default = SheetDirection::Left)] direction: SheetDirection,
    #[props(default = false)] rotate_icon: bool,
    children: Element,
) -> Element {
    let icon_class = if rotate_icon { "rotate-180 size-4" } else { "size-4" };
    rsx! {
        Sheet {
            div { class: "md:hidden",
                SheetTrigger { class: "size-7",
                    icons::PanelLeft { class: icon_class }
                    span { class: "hidden", "Toggle Sidenav" }
                }
            }
            SheetContent { direction, class: "p-0 w-[18rem] bg-sidenav text-sidenav-foreground", show_close_button: false,
                div { class: "flex flex-col h-full", {children} }
            }
        }
    }
}

#[component]
pub fn SidenavStandardContent(
    current_section: DocsRoutes,
    sidenav_route: SidenavRoutes,
    pattern: SidenavPattern,
) -> Element {
    let mut query = use_signal(String::new);
    let links = links_for(current_section, sidenav_route);
    let filtered = if pattern == SidenavPattern::Search {
        let value = query().to_lowercase();
        links.iter().filter(|(_, title)| title.to_lowercase().contains(&value)).cloned().collect()
    } else {
        links.clone()
    };
    let show_search = matches!(pattern, SidenavPattern::Grouped | SidenavPattern::Submenus | SidenavPattern::Search);

    rsx! {
        SidenavHeader {
            super::sidenav_routes_selector::SidenavRoutesSelector { current_section, sidenav_route }
            if show_search {
                form {
                    SidenavGroup { data_sidenav: "group",
                        SidenavGroupContent { data_sidenav: "group-content", class: "relative",
                            Label { html_for: "search", class: "hidden", "Search" }
                            SidenavInput {
                                class: "pl-8", id: "search", placeholder: "Search the docs...",
                                value: if pattern == SidenavPattern::Search { Some(query()) } else { None },
                                oninput: if pattern == SidenavPattern::Search {
                                    Some(EventHandler::new(move |event: FormEvent| query.set(event.value())))
                                } else { None },
                            }
                            Search { class: "absolute left-2 top-1/2 opacity-50 -translate-y-1/2 pointer-events-none select-none size-4" }
                        }
                    }
                }
            }
        }
        SidenavContent {
            SidenavGroup {
                SidenavGroupLabel { "{current_section.to_title()}" }
                SidenavGroupContent {
                    match pattern {
                        SidenavPattern::Collapsible | SidenavPattern::CollapsibleSubmenus | SidenavPattern::Inset => rsx! {
                            Accordion {
                                SidenavMenuItem {
                                    AccordionItem {
                                        AccordionTrigger { open: true, class: "p-2 peer-checked:bg-accent hover:bg-accent",
                                            AccordionHeader { AccordionTitle { "{current_section.to_title()}" } }
                                        }
                                        AccordionContent { class: if pattern == SidenavPattern::Collapsible { "pt-0" } else { "p-0" },
                                            SidenavMenu { if pattern == SidenavPattern::CollapsibleSubmenus || pattern == SidenavPattern::Inset {
                                                SidenavLinkList { links: filtered.clone(), sub: true }
                                            } else {
                                                SidenavLinkList { links: filtered.clone() }
                                            } }
                                        }
                                    }
                                }
                            }
                        },
                        SidenavPattern::Dropdown => rsx! {
                            SidenavMenu {
                                SidenavMenuItem { DropdownSection { section: DocsRoutes::Components, sidenav_route } }
                                SidenavMenuItem { DropdownSection { section: DocsRoutes::Hooks, sidenav_route } }
                            }
                        },
                        SidenavPattern::Icons => rsx! {
                            SidenavMenu {
                                IconAccordionSection { icon: "book", title: current_section.to_title(), links: filtered.clone(), open: true }
                                IconAccordionSection { icon: "bot", title: "Models", links: Vec::new(), open: false }
                                IconAccordionSection { icon: "terminal", title: "Playground", links: Vec::new(), open: false }
                                IconAccordionSection { icon: "settings", title: "Settings", links: Vec::new(), open: false }
                            }
                        },
                        _ => rsx! { SidenavMenu { SidenavLinkList { links: filtered.clone() } } },
                    }
                    if pattern == SidenavPattern::Search && filtered.is_empty() && !query().is_empty() {
                        div { class: "py-4 px-2 text-sm text-muted-foreground", "No results found" }
                    }
                }
            }
            if pattern == SidenavPattern::Icons {
                SidenavGroup {
                    SidenavGroupLabel { "Projects" }
                    SidenavMenu { ProjectItem {} ProjectItem {} }
                }
            }
        }
        SidenavFooter { DemoDropdownMenuUser {} }
    }
}

#[component]
fn DropdownSection(section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    let links = links_for(section, sidenav_route);
    rsx! {
        DropdownMenu { align: DropdownMenuAlign::End,
            DropdownMenuTrigger { class: "flex overflow-hidden gap-2 items-center p-2 w-full h-8 text-sm text-left rounded-md hover:bg-sidenav-accent",
                match section { DocsRoutes::Components => rsx! { Component { class: "size-4 shrink-0" } }, DocsRoutes::Hooks => rsx! { Sparkles { class: "size-4 shrink-0" } } }
                span { class: "flex-1 truncate", "{section.to_title()}" }
                icons::ChevronRight { class: "opacity-50 size-4 shrink-0" }
            }
            DropdownMenuContent { DropdownMenuGroup {
                for (href, title) in links {
                    DropdownMenuItem { DropdownMenuAction { href, "{title}" } }
                }
            } }
        }
    }
}

#[component]
fn IconAccordionSection(icon: &'static str, title: String, links: Vec<(String, String)>, open: bool) -> Element {
    rsx! {
        SidenavMenuItem {
            AccordionItem {
                AccordionTrigger { open, class: "p-2 peer-checked:bg-accent hover:bg-accent",
                    AccordionHeader {
                        match icon {
                            "book" => rsx! { BookOpen {} },
                            "bot" => rsx! { Bot {} },
                            "terminal" => rsx! { SquareTerminal {} },
                            _ => rsx! { Settings {} },
                        }
                        AccordionTitle { "{title}" }
                    }
                }
                AccordionContent { class: "p-0", if links.is_empty() {
                    for _ in 0..4 { SidenavLink { href: "#", "Link" } }
                } else { SidenavLinkList { links, sub: true } } }
            }
        }
    }
}

#[component]
fn ProjectItem() -> Element {
    rsx! {
        SidenavMenuItem {
            a { href: "#", class: "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm hover:bg-sidenav-accent",
                Frame { class: "size-4 shrink-0" }
                span { class: "truncate", "Design Engineering" }
                DropdownMenuTriggerEllipsis {
                    icons::Ellipsis {}
                    span { class: "hidden", "More" }
                }
            }
        }
    }
}

pub fn standard_sidebar(
    current_section: DocsRoutes,
    sidenav_route: SidenavRoutes,
    pattern: SidenavPattern,
    variant: SidenavVariant,
) -> Element {
    rsx! { Sidenav { variant, SidenavStandardContent { current_section, sidenav_route, pattern } } }
}

pub fn standard_mobile(
    current_section: DocsRoutes,
    sidenav_route: SidenavRoutes,
    pattern: SidenavPattern,
    direction: SheetDirection,
    rotate_icon: bool,
) -> Element {
    rsx! { SidenavMobileSheet { current_section, sidenav_route, direction, rotate_icon,
        SidenavStandardContent { current_section, sidenav_route, pattern }
    } }
}

#[component]
pub fn EmptyLegacyBlock() -> Element {
    rsx! {}
}

#[allow(dead_code)]
pub fn _keep_imports_used() {
    let _ = (Layers, ChevronRight, DemoDropdownMenuUserIcon);
}
