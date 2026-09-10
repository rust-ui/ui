use dioxus::prelude::*;

use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};
use crate::ui::input::{Input, InputType};
use crate::ui::label::Label;
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoDrawerFocus() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent {
                DrawerHandle {}

                DrawerBody {
                    DrawerHeader {
                        DrawerTitle { "Focus Drawer" }
                        DrawerDescription {
                            "Test keyboard navigation: Press Tab to cycle through elements, Shift+Tab to go back, and Escape to close."
                        }
                    }

                    form { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-2",
                            Label { html_for:"test-input", "Text Input" }
                            Input { id: "test-input", placeholder: "Type something..." }
                        }

                        div { class: "flex flex-col gap-2",
                            Label { html_for:"test-email", "Email" }
                            Input { id: "test-email", r#type: InputType::Email, placeholder: "email@example.com" }
                        }

                        div { class: "flex flex-col gap-2",
                            Label { html_for:"test-textarea", "Message" }
                            Textarea { id: "test-textarea", rows: 4, placeholder: "Write a message..." }
                        }
                    }

                    DrawerClose { "Close" }
                }
            }
        }
    }
}
