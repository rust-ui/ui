use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHandle, DrawerHeader,
    DrawerTitle, DrawerTrigger,
};
use crate::ui::input::{Input, InputType};

#[component]
pub fn DemoDrawerDialog() -> Element {
    rsx! {
        // Mobile — hidden on md+
        div { class: "md:hidden",
            Drawer {
                DrawerTrigger { "Subscribe" }
                DrawerContent {
                    DrawerHandle {}
                    DrawerBody {
                        DrawerHeader {
                            DrawerTitle { "Subscribe" }
                            DrawerDescription { "Get the latest updates delivered to your inbox." }
                        }
                        Input { r#type: InputType::Email, placeholder: "you@example.com" }
                        DrawerFooter {
                            DrawerClose { class: "w-full sm:w-fit", "Cancel" }
                            Button { class: "w-full sm:w-fit", "Subscribe" }
                        }
                    }
                }
            }
        }

        // Desktop — hidden below md
        div { class: "hidden md:block",
            Dialog {
                DialogTrigger { "Subscribe" }
                DialogContent { class: "sm:max-w-[400px]",
                    DialogBody {
                        DialogHeader {
                            DialogTitle { "Subscribe" }
                            DialogDescription { "Get the latest updates delivered to your inbox." }
                        }
                        Input { r#type: InputType::Email, placeholder: "you@example.com" }
                        DialogFooter {
                            DialogClose { class: "w-full sm:w-fit", "Cancel" }
                            Button { class: "w-full sm:w-fit", "Subscribe" }
                        }
                    }
                }
            }
        }
    }
}
