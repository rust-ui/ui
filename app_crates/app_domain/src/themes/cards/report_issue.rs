use dioxus::prelude::*;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};
use registry::ui::textarea::Textarea;

const AREAS: [&str; 4] = ["Billing", "Team", "Account", "Support"];
const SECURITY_LEVELS: [&str; 4] = ["Level 1", "Level 2", "Level 3", "Level 4"];

#[component]
pub fn CardReportIssue() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Report an issue" }
                CardDescription { "What area are you having problems with?" }
            }
            CardContent { class: "grid gap-6",
                div { class: "grid gap-4 sm:grid-cols-2",
                    div { class: "grid gap-2",
                        Label { "Area" }
                        Select {
                            SelectTrigger {
                                SelectValue { placeholder: "Select area" }
                            }
                            SelectContent {
                                SelectGroup {
                                    {AREAS.iter().map(|area| rsx! {
                                        SelectOption { key: "{area}", value: *area, "{area}" }
                                    })}
                                }
                            }
                        }
                    }
                    div { class: "grid gap-2",
                        Label { "Security Level" }
                        Select {
                            SelectTrigger {
                                SelectValue { placeholder: "Select level" }
                            }
                            SelectContent {
                                SelectGroup {
                                    {SECURITY_LEVELS.iter().map(|level| rsx! {
                                        SelectOption { key: "{level}", value: *level, "{level}" }
                                    })}
                                }
                            }
                        }
                    }
                }
                div { class: "grid gap-2",
                    Label { "Subject" }
                    Input { placeholder: "I need help with..." }
                }
                div { class: "grid gap-2",
                    Label { "Description" }
                    Textarea { placeholder: "Please include all information relevant to your issue." }
                }
            }
            CardFooter { class: "gap-2 justify-end",
                Button { variant: ButtonVariant::Outline, "Cancel" }
                Button { "Submit" }
            }
        }
    }
}
