use dioxus::prelude::*;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const PERMISSIONS: [&str; 2] = ["Can Edit", "Can View"];

#[component]
pub fn CardShareThisDocument() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Share this document" }
                CardDescription { "Anyone with the link can view this document." }
            }
            CardContent {
                div { class: "flex space-x-2",
                    Label { class: "hidden", html_for: "link", "Link" }
                    Input { id: "link", readonly: true, value: "http://example.com/link/to/document" }
                    Button { variant: ButtonVariant::Secondary, class: "shrink-0", "Copy Link" }
                }
                div {
                    "data-orientation": "horizontal",
                    role: "none",
                    class: "my-4 w-full shrink-0 bg-border h-[1px]",
                }
                div { class: "space-y-4",
                    h3 { class: "text-sm font-medium", "People with access" }
                    div { class: "grid gap-6",
                        div { class: "flex justify-between items-center space-x-4",
                            div { class: "flex items-center space-x-4",
                                Avatar { class: "size-10",
                                    AvatarFallback { "OM" }
                                }
                                div {
                                    p { class: "text-sm font-medium leading-none", "Olivia Martin" }
                                    p { class: "text-sm text-muted-foreground", "m@example.com" }
                                }
                            }
                            Select { default_value: PERMISSIONS[0],
                                SelectTrigger { class: "ml-auto w-[110px]",
                                    SelectValue { placeholder: PERMISSIONS[0] }
                                }
                                SelectContent {
                                    SelectGroup {
                                        {PERMISSIONS.iter().map(|p| rsx! {
                                            SelectOption { key: "{p}", value: *p, "{p}" }
                                        })}
                                    }
                                }
                            }
                        }
                        div { class: "flex justify-between items-center space-x-4",
                            div { class: "flex items-center space-x-4",
                                Avatar { class: "size-10",
                                    AvatarFallback { "IN" }
                                }
                                div {
                                    p { class: "text-sm font-medium leading-none", "Isabella Nguyen" }
                                    p { class: "text-sm text-muted-foreground", "b@example.com" }
                                }
                            }
                            Select { default_value: PERMISSIONS[0],
                                SelectTrigger { class: "ml-auto w-[110px]",
                                    SelectValue { placeholder: PERMISSIONS[0] }
                                }
                                SelectContent {
                                    SelectGroup {
                                        {PERMISSIONS.iter().map(|p| rsx! {
                                            SelectOption { key: "{p}", value: *p, "{p}" }
                                        })}
                                    }
                                }
                            }
                        }
                        div { class: "flex justify-between items-center space-x-4",
                            div { class: "flex items-center space-x-4",
                                Avatar { class: "size-10",
                                    AvatarFallback { "SD" }
                                }
                                div {
                                    p { class: "text-sm font-medium leading-none", "Sofia Davis" }
                                    p { class: "text-sm text-muted-foreground", "p@example.com" }
                                }
                            }
                            Select { default_value: PERMISSIONS[1],
                                SelectTrigger { class: "ml-auto w-[110px]",
                                    SelectValue { placeholder: PERMISSIONS[1] }
                                }
                                SelectContent {
                                    SelectGroup {
                                        {PERMISSIONS.iter().map(|p| rsx! {
                                            SelectOption { key: "{p}", value: *p, "{p}" }
                                        })}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
