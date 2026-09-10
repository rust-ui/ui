use dioxus::prelude::*;

use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubItem, ContextMenuSubTrigger, ContextMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoContextMenu() -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                class: "flex justify-center items-center text-sm rounded-md border border-dashed h-[150px] w-[300px]",
                "Right click here"
            }

            ContextMenuContent {
                ContextMenuLabel { "Actions" }

                ContextMenuGroup {
                    ContextMenuItem {
                        ContextMenuAction { "Back" }
                    }
                    ContextMenuItem {
                        ContextMenuAction { "Forward" }
                    }
                    ContextMenuItem {
                        ContextMenuAction { "Reload" }
                    }

                    ContextMenuSub {
                        ContextMenuSubTrigger { "More Tools" }
                        ContextMenuSubContent {
                            ContextMenuSubItem { "Save Page As..." }
                            ContextMenuSubItem { "Create Shortcut..." }
                            ContextMenuSubItem { "Name Window..." }
                            Separator { class: "my-1" }
                            ContextMenuSubItem { "Developer Tools" }
                        }
                    }
                }

                Separator { class: "my-1" }

                ContextMenuGroup {
                    ContextMenuItem {
                        ContextMenuAction { "Show Bookmarks Bar" }
                    }
                    ContextMenuItem {
                        ContextMenuAction { "Show Full URLs" }
                    }
                }
            }
        }
    }
}
