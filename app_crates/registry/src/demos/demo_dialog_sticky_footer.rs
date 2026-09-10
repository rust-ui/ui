use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoDialogStickyFooter() -> Element {
    rsx! {
        Dialog {
            DialogTrigger { "Open Dialog" }
            DialogContent { class: "sm:max-w-[480px]",
                DialogBody {
                    DialogHeader {
                        DialogTitle { "Edit Profile" }
                        DialogDescription {
                            "Update your account information. Scroll down to see all fields."
                        }
                    }
                    div { class: "overflow-y-auto px-6 -mx-6 max-h-[50vh]",
                        div { class: "flex flex-col gap-4 pb-2",
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "flex flex-col gap-2",
                                    Label { html_for: "first-name", "First name" }
                                    Input { id: "first-name", placeholder: "Max" }
                                }
                                div { class: "flex flex-col gap-2",
                                    Label { html_for: "last-name", "Last name" }
                                    Input { id: "last-name", placeholder: "Wells" }
                                }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "username", "Username" }
                                Input { id: "username", placeholder: "@maxwells" }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "email", "Email" }
                                Input { id: "email", r#type: crate::ui::input::InputType::Email, placeholder: "max@example.com" }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { "Bio" }
                                Textarea { placeholder: "Tell us a little about yourself...", rows: 3 }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "website", "Website" }
                                Input { id: "website", placeholder: "https://example.com" }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "location", "Location" }
                                Input { id: "location", placeholder: "San Francisco, CA" }
                            }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "flex flex-col gap-2",
                                    Label { html_for: "twitter", "Twitter" }
                                    Input { id: "twitter", placeholder: "@handle" }
                                }
                                div { class: "flex flex-col gap-2",
                                    Label { html_for: "github", "GitHub" }
                                    Input { id: "github", placeholder: "username" }
                                }
                            }
                        }
                    }
                    DialogFooter {
                        DialogClose { "Cancel" }
                        Button { "Save changes" }
                    }
                }
            }
        }
    }
}
