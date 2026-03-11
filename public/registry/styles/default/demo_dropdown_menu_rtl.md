---
title: "Demo Dropdown Menu Rtl"
name: "demo_dropdown_menu_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "dropdown_menu", "separator"]
type: "components:demos"
path: "demos/demo_dropdown_menu_rtl.rs"
---

# Demo Dropdown Menu Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dropdown_menu_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem, DropdownMenuSubTrigger,
    DropdownMenuTrigger,
};
use crate::components::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <DropdownMenu>
                <DropdownMenuTrigger>"فتح القائمة"</DropdownMenuTrigger>

                <DropdownMenuContent>
                    <DropdownMenuLabel>"القائمة الرئيسية"</DropdownMenuLabel>

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"عنصر بسيط"</DropdownMenuAction>
                        </DropdownMenuItem>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"الإعدادات"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"إعدادات الحساب"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"إعدادات الخصوصية"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"إعدادات الإشعارات"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"الأدوات"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"تصدير البيانات"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"استيراد البيانات"</DropdownMenuSubItem>
                                <Separator class="my-1" />
                                <DropdownMenuSubItem>"أدوات المطور"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                    </DropdownMenuGroup>

                    <Separator class="my-1" />

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuLink attr:href="/">"الرئيسية"</DropdownMenuLink>
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"تسجيل الخروج"</DropdownMenuAction>
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </DirectionProvider>
    }
}
```
