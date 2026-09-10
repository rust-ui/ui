use dioxus::prelude::*;
use icons::{ArrowDown, ArrowRight, ArrowUp, CircleDashed, CornerDownLeft, Search};
use strum::Display;

use crate::ui::command::{
    Command, CommandDescription, CommandFooter, CommandGroup, CommandGroupLabel, CommandHeader, CommandInput,
    CommandItemLink, CommandList, CommandTitle,
};
use crate::ui::input_group::{InputGroup, InputGroupAddon};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoCommand() -> Element {
    rsx! {
        div { "data-name": "__DemoCard", class: "my-6 mx-auto w-full rounded-md border max-w-[450px]",
            CommandHeader {
                CommandTitle { "Search documentation..." }
                CommandDescription { "Search for a command to run..." }
            }
            Command {
                InputGroup { class: "h-9 rounded-none border-b",
                    InputGroupAddon {
                        Search {}
                    }
                    CommandInput { class: "flex-1 py-0 h-9 rounded-none border-0 shadow-none" }
                }
                CommandList { id: "command_demo", tabindex: "-1",
                    {[(CommandCategory::Pages, PAGES_ITEMS), (CommandCategory::Components, COMPONENTS_ITEMS)]
                        .into_iter()
                        .map(|(category, items)| {
                            let cat_label = category.to_string();
                            rsx! {
                                CommandGroup { role: "presentation", class: "p-0",
                                    CommandGroupLabel { aria_hidden: "true", class: "p-3",
                                        {cat_label}
                                    }
                                    {items.iter().map(|item| {
                                        let icon = item.icon();
                                        rsx! {
                                            CommandItemLink {
                                                class: "px-3",
                                                href: item.href,
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                {icon}
                                                span { {item.label} }
                                            }
                                        }
                                    })}
                                }
                            }
                        })}
                }
            }
            CommandFooter {
                div { class: "flex gap-2 items-center",
                    Kbd { ArrowUp {} }
                    Kbd { ArrowDown {} }
                    span { "Navigate" }
                }
                div { class: "flex gap-2 items-center",
                    Kbd { CornerDownLeft {} }
                    span { "Go to Page" }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ DATA STRUCTURES ✨                  */
/* ========================================================== */

#[derive(Clone, Debug, Display)]
enum CommandCategory {
    Pages,
    Components,
}

#[derive(Clone, Debug)]
struct CommandItemData {
    label: &'static str,
    href: &'static str,
    category: CommandCategory,
}

impl CommandItemData {
    fn icon(&self) -> Element {
        match self.category {
            CommandCategory::Pages => rsx! { ArrowRight {} },
            CommandCategory::Components => rsx! { CircleDashed {} },
        }
    }
}

const PAGES_ITEMS: &[CommandItemData] = &[
    CommandItemData { label: "Docs", href: "/docs", category: CommandCategory::Pages },
    CommandItemData { label: "Components", href: "/components", category: CommandCategory::Pages },
    CommandItemData { label: "Blocks", href: "/blocks", category: CommandCategory::Pages },
];

const COMPONENTS_ITEMS: &[CommandItemData] = &[
    CommandItemData { label: "Accordion", href: "/components/accordion", category: CommandCategory::Components },
    CommandItemData { label: "Alert", href: "/components/alert", category: CommandCategory::Components },
    CommandItemData { label: "Alert Dialog", href: "/components/alert-dialog", category: CommandCategory::Components },
    CommandItemData { label: "Avatar", href: "/components/avatar", category: CommandCategory::Components },
    CommandItemData { label: "Badge", href: "/components/badge", category: CommandCategory::Components },
    CommandItemData { label: "Breadcrumb", href: "/components/breadcrumb", category: CommandCategory::Components },
];
