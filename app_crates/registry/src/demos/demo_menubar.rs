use dioxus::prelude::*;

use crate::ui::menubar::{
    Menubar, MenubarCheckboxItem, MenubarContent, MenubarGroup, MenubarItem, MenubarLabel, MenubarMenu,
    MenubarRadioGroup, MenubarRadioItem, MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent,
    MenubarSubItem, MenubarSubTrigger, MenubarTrigger,
};

#[component]
pub fn DemoMenubar() -> Element {
    let show_bookmarks = use_signal(|| true);
    let show_full_urls = use_signal(|| false);
    let zoom_level = use_signal(|| "100%".to_string());

    rsx! {
        Menubar { class: "w-fit",
            // ── File ──
            MenubarMenu {
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarGroup {
                        MenubarItem { "New Tab" MenubarShortcut { "⌘T" } }
                        MenubarItem { "New Window" MenubarShortcut { "⌘N" } }
                        MenubarItem { class: "opacity-50 pointer-events-none",
                            "New Incognito Window"
                        }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarSub {
                            MenubarSubTrigger { "Share" }
                            MenubarSubContent {
                                MenubarSubItem { "Email link" }
                                MenubarSubItem { "Messages" }
                                MenubarSubItem { "Notes" }
                            }
                        }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarItem { "Print..." MenubarShortcut { "⌘P" } }
                    }
                }
            }

            // ── Edit ──
            MenubarMenu {
                MenubarTrigger { "Edit" }
                MenubarContent {
                    MenubarGroup {
                        MenubarItem { "Undo" MenubarShortcut { "⌘Z" } }
                        MenubarItem { "Redo" MenubarShortcut { "⇧⌘Z" } }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarSub {
                            MenubarSubTrigger { "Find" }
                            MenubarSubContent {
                                MenubarSubItem { "Search the web" }
                                MenubarSeparator {}
                                MenubarSubItem { "Find..." }
                                MenubarSubItem { "Find Next" }
                                MenubarSubItem { "Find Previous" }
                            }
                        }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarItem { "Cut" }
                        MenubarItem { "Copy" }
                        MenubarItem { "Paste" }
                    }
                }
            }

            // ── View ──
            MenubarMenu {
                MenubarTrigger { "View" }
                MenubarContent {
                    MenubarGroup {
                        MenubarCheckboxItem { checked: show_bookmarks, "Always Show Bookmarks Bar" }
                        MenubarCheckboxItem { checked: show_full_urls, "Always Show Full URLs" }
                    }
                    MenubarSeparator {}
                    MenubarLabel { "Appearance" }
                    MenubarGroup {
                        MenubarItem { "Reload" MenubarShortcut { "⌘R" } }
                        MenubarItem { "Force Reload" MenubarShortcut { "⇧⌘R" } }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarRadioGroup { value: zoom_level,
                            MenubarRadioItem { value: "75%".to_string(), "75%" }
                            MenubarRadioItem { value: "100%".to_string(), "100%" }
                            MenubarRadioItem { value: "125%".to_string(), "125%" }
                        }
                    }
                }
            }

            // ── Profiles ──
            MenubarMenu {
                MenubarTrigger { "Profiles" }
                MenubarContent {
                    MenubarGroup {
                        MenubarItem { "Edit..." }
                    }
                    MenubarSeparator {}
                    MenubarGroup {
                        MenubarItem { "Add Profile..." }
                    }
                }
            }
        }
    }
}
