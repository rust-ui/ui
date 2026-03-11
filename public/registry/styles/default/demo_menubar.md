---
title: "Demo Menubar"
name: "demo_menubar"
cargo_dependencies: []
registry_dependencies: ["menubar"]
type: "components:demos"
path: "demos/demo_menubar.rs"
---

# Demo Menubar

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_menubar
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::menubar::{
    Menubar, MenubarCheckboxItem, MenubarContent, MenubarGroup, MenubarItem, MenubarLabel, MenubarMenu,
    MenubarRadioGroup, MenubarRadioItem, MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent,
    MenubarSubItem, MenubarSubTrigger, MenubarTrigger,
};

#[component]
pub fn DemoMenubar() -> impl IntoView {
    let show_bookmarks = RwSignal::new(true);
    let show_full_urls = RwSignal::new(false);
    let zoom_level = RwSignal::new("100%".to_string());

    view! {
        <Menubar class="w-fit">
            // ── File ──
            <MenubarMenu>
                <MenubarTrigger>"File"</MenubarTrigger>
                <MenubarContent>
                    <MenubarGroup>
                        <MenubarItem>"New Tab" <MenubarShortcut>"⌘T"</MenubarShortcut></MenubarItem>
                        <MenubarItem>"New Window" <MenubarShortcut>"⌘N"</MenubarShortcut></MenubarItem>
                        <MenubarItem attr:data-disabled="" class="opacity-50 pointer-events-none">
                            "New Incognito Window"
                        </MenubarItem>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarSub>
                            <MenubarSubTrigger>"Share"</MenubarSubTrigger>
                            <MenubarSubContent>
                                <MenubarSubItem>"Email link"</MenubarSubItem>
                                <MenubarSubItem>"Messages"</MenubarSubItem>
                                <MenubarSubItem>"Notes"</MenubarSubItem>
                            </MenubarSubContent>
                        </MenubarSub>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarItem>"Print..." <MenubarShortcut>"⌘P"</MenubarShortcut></MenubarItem>
                    </MenubarGroup>
                </MenubarContent>
            </MenubarMenu>

            // ── Edit ──
            <MenubarMenu>
                <MenubarTrigger>"Edit"</MenubarTrigger>
                <MenubarContent>
                    <MenubarGroup>
                        <MenubarItem>"Undo" <MenubarShortcut>"⌘Z"</MenubarShortcut></MenubarItem>
                        <MenubarItem>"Redo" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut></MenubarItem>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarSub>
                            <MenubarSubTrigger>"Find"</MenubarSubTrigger>
                            <MenubarSubContent>
                                <MenubarSubItem>"Search the web"</MenubarSubItem>
                                <MenubarSeparator />
                                <MenubarSubItem>"Find..."</MenubarSubItem>
                                <MenubarSubItem>"Find Next"</MenubarSubItem>
                                <MenubarSubItem>"Find Previous"</MenubarSubItem>
                            </MenubarSubContent>
                        </MenubarSub>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarItem>"Cut"</MenubarItem>
                        <MenubarItem>"Copy"</MenubarItem>
                        <MenubarItem>"Paste"</MenubarItem>
                    </MenubarGroup>
                </MenubarContent>
            </MenubarMenu>

            // ── View ──
            <MenubarMenu>
                <MenubarTrigger>"View"</MenubarTrigger>
                <MenubarContent>
                    <MenubarGroup>
                        <MenubarCheckboxItem checked=show_bookmarks>"Always Show Bookmarks Bar"</MenubarCheckboxItem>
                        <MenubarCheckboxItem checked=show_full_urls>"Always Show Full URLs"</MenubarCheckboxItem>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarLabel>"Appearance"</MenubarLabel>
                    <MenubarGroup>
                        <MenubarItem>"Reload" <MenubarShortcut>"⌘R"</MenubarShortcut></MenubarItem>
                        <MenubarItem>"Force Reload" <MenubarShortcut>"⇧⌘R"</MenubarShortcut></MenubarItem>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarRadioGroup value=zoom_level>
                            <MenubarRadioItem value="75%".to_string()>"75%"</MenubarRadioItem>
                            <MenubarRadioItem value="100%".to_string()>"100%"</MenubarRadioItem>
                            <MenubarRadioItem value="125%".to_string()>"125%"</MenubarRadioItem>
                        </MenubarRadioGroup>
                    </MenubarGroup>
                </MenubarContent>
            </MenubarMenu>

            // ── Profiles ──
            <MenubarMenu>
                <MenubarTrigger>"Profiles"</MenubarTrigger>
                <MenubarContent>
                    <MenubarGroup>
                        <MenubarItem>"Edit..."</MenubarItem>
                    </MenubarGroup>
                    <MenubarSeparator />
                    <MenubarGroup>
                        <MenubarItem>"Add Profile..."</MenubarItem>
                    </MenubarGroup>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    }
}
```
