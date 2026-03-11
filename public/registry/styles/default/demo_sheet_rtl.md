---
title: "Demo Sheet Rtl"
name: "demo_sheet_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "direction_provider", "sheet"]
type: "components:demos"
path: "demos/demo_sheet_rtl.rs"
---

# Demo Sheet Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sheet_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Sheet>
                <SheetTrigger>"فتح اللوحة الجانبية"</SheetTrigger>

                <SheetContent direction=SheetDirection::Right>
                    <SheetHeader>
                        <SheetTitle>"تعديل الملف الشخصي"</SheetTitle>
                        <SheetDescription>
                            "قم بإجراء تغييرات على ملفك الشخصي هنا."
                        </SheetDescription>
                    </SheetHeader>

                    <div class="p-4 text-sm text-muted-foreground">
                        "قم بتحديث اسمك المعروض وصورتك الرمزية وتفاصيل ملفك الشخصي الأخرى."
                    </div>

                    <SheetFooter>
                        <SheetClose variant=ButtonVariant::Outline>"إلغاء"</SheetClose>
                        <Button>"حفظ التغييرات"</Button>
                    </SheetFooter>
                </SheetContent>
            </Sheet>
        </DirectionProvider>
    }
}
```
