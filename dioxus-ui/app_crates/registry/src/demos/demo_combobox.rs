use dioxus::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::ui::command::{Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList};
use crate::ui::popover::{Popover, PopoverContent, PopoverTrigger};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
enum Language {
    Rust,
    JavaScript,
    Ruby,
    Python,
}

#[component]
pub fn DemoCombobox() -> Element {
    let mut value_signal = use_signal(|| None::<Language>);

    rsx! {
        Popover {
            PopoverTrigger { class: "justify-between w-[200px]",
                span { class: "truncate",
                    {value_signal().map(|l| l.to_string()).unwrap_or_else(|| "Select language...".into())}
                }
                icons::ChevronsUpDown { class: "ml-auto opacity-50 size-4" }
            }

            PopoverContent { class: "p-0 w-[200px]",
                Command {
                    div { class: "flex gap-2 items-center px-2 border-b",
                        icons::Search { class: "size-4 text-muted-foreground shrink-0" }
                        CommandInput { }
                    }
                    CommandList { class: "min-h-0",
                        CommandEmpty { "No language found." }
                        CommandGroup {
                            for language in Language::iter() {
                                CommandItem {
                                    value: language.to_string(),
                                    selected: value_signal() == Some(language),
                                    on_select: move |_| {
                                        value_signal.set(Some(language));
                                    },
                                    {language.to_string()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
