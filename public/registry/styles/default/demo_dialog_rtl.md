---
title: "Demo Dialog Rtl"
name: "demo_dialog_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "dialog", "direction_provider", "input", "label"]
type: "components:demos"
path: "demos/demo_dialog_rtl.rs"
---

# Demo Dialog Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_dialog_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn DemoDialogRtl() -> impl IntoView {
    let name_signal = RwSignal::new("محمد علي".to_string());
    let username_signal = RwSignal::new("@محمد_علي".to_string());

    view! {
        <DirectionProvider dir=Direction::Rtl>
            <Dialog>
                <DialogTrigger>"فتح الحوار"</DialogTrigger>

                <DialogContent class="sm:max-w-[425px]">
                    <DialogBody>
                        <DialogHeader>
                            <DialogTitle>"تعديل الملف الشخصي"</DialogTitle>

                            <DialogDescription>
                                "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر على حفظ عند الانتهاء."
                            </DialogDescription>
                        </DialogHeader>

                        <div class="flex flex-col gap-4 justify-center">
                            <div class="flex flex-col gap-2">
                                <Label html_for="rtl-name-1">"الاسم"</Label>
                                <Input
                                    attr:autofocus=true
                                    attr:id="rtl-name-1"
                                    attr:name="name"
                                    prop:value=name_signal
                                />
                            </div>
                            <div class="flex flex-col gap-2">
                                <Label html_for="rtl-username-1">"اسم المستخدم"</Label>
                                <Input attr:id="rtl-username-1" attr:name="username" prop:value=username_signal />
                            </div>
                        </div>

                        <DialogFooter>
                            <DialogClose class="w-full sm:w-fit">"إلغاء"</DialogClose>
                            <Button attr:r#type="submit" class="w-full sm:w-fit">
                                "حفظ التغييرات"
                            </Button>
                        </DialogFooter>
                    </DialogBody>
                </DialogContent>
            </Dialog>
        </DirectionProvider>
    }
}
```
