---
title: "Demo Alert Dialog Rtl"
name: "demo_alert_dialog_rtl"
cargo_dependencies: []
registry_dependencies: ["alert_dialog", "button", "direction_provider"]
type: "components:demos"
path: "demos/demo_alert_dialog_rtl.rs"
---

# Demo Alert Dialog Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_dialog_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::components::ui::button::Button;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertDialogRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <AlertDialog>
                <AlertDialogTrigger>"فتح التنبيه"</AlertDialogTrigger>

                <AlertDialogContent class="w-[425px]">
                    <AlertDialogBody>
                        <AlertDialogHeader>
                            <AlertDialogTitle>"هل أنت متأكد تماماً؟"</AlertDialogTitle>

                            <AlertDialogDescription>
                                "لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائياً وإزالة بياناتك من خوادمنا."
                            </AlertDialogDescription>
                        </AlertDialogHeader>

                        <AlertDialogFooter>
                            <AlertDialogClose class="w-full sm:w-fit">"إلغاء"</AlertDialogClose>
                            <Button attr:r#type="submit" class="w-full sm:w-fit">
                                "متابعة"
                            </Button>
                        </AlertDialogFooter>
                    </AlertDialogBody>
                </AlertDialogContent>
            </AlertDialog>
        </DirectionProvider>
    }
}
```
