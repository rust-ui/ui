use dioxus::prelude::*;
use icons::{ExternalLink, Mail};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::field::{Field, FieldGroup, FieldLabel};
use crate::ui::input::InputType;
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};

#[component]
pub fn DemoInputGroupInCard() -> Element {
    rsx! {
        Card { class: "w-full max-w-md",
            CardHeader {
                CardTitle { "Profile Settings" }
                CardDescription { "Update your contact information." }
            }
            CardContent {
                FieldGroup {
                    Field {
                        FieldLabel { r#for: "card-email", "Email Address" }
                        InputGroup {
                            InputGroupInput {
                                id: "card-email",
                                r#type: InputType::Email,
                                placeholder: "you@example.com"
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Mail {}
                            }
                        }
                    }

                    Field {
                        FieldLabel { r#for: "card-website", "Website" }
                        InputGroup {
                            InputGroupAddon {
                                InputGroupText { "https://" }
                            }
                            InputGroupInput { id: "card-website", placeholder: "example.com" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                ExternalLink {}
                            }
                        }
                    }

                    Field {
                        FieldLabel { r#for: "card-bio", "Bio" }
                        InputGroup {
                            textarea {
                                "data-slot": "input-group-control",
                                id: "card-bio",
                                rows: "3",
                                placeholder: "Tell us a little about yourself...",
                                class: "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none focus-visible:ring-0 dark:bg-transparent placeholder:text-muted-foreground w-full min-w-0 px-3 text-base outline-none transition-[color,box-shadow]",
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::BlockEnd, class: "border-t",
                                InputGroupText { "0 / 200 characters" }
                            }
                        }
                    }
                }
            }
            CardFooter { class: "gap-2 justify-end",
                Button { variant: ButtonVariant::Outline, "Cancel" }
                Button { "Save changes" }
            }
        }
    }
}
