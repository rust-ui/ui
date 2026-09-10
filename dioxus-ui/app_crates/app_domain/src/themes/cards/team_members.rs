use dioxus::prelude::*;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const ROLES: [&str; 2] = ["Owner", "Member"];

#[component]
pub fn CardTeamMembers() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Team Members" }
                CardDescription { "Invite your team members to collaborate." }
            }
            CardContent { class: "grid gap-6",
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "OM" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Sofia Davis" }
                            p { class: "text-sm text-muted-foreground", "m@example.com" }
                        }
                    }
                    Select { default_value: ROLES[0],
                        SelectTrigger { class: "ml-auto w-[100px]",
                            SelectValue { placeholder: ROLES[0] }
                        }
                        SelectContent {
                            SelectGroup {
                                {ROLES.iter().map(|role| rsx! {
                                    SelectOption { key: "{role}", value: *role, "{role}" }
                                })}
                            }
                        }
                    }
                }
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "JL" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Jackson Lee" }
                            p { class: "text-sm text-muted-foreground", "p@example.com" }
                        }
                    }
                    Select { default_value: ROLES[1],
                        SelectTrigger { class: "ml-auto w-[100px]",
                            SelectValue { placeholder: ROLES[1] }
                        }
                        SelectContent {
                            SelectGroup {
                                {ROLES.iter().map(|role| rsx! {
                                    SelectOption { key: "{role}", value: *role, "{role}" }
                                })}
                            }
                        }
                    }
                }
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "IN" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Isabella Nguyen" }
                            p { class: "text-sm text-muted-foreground", "i@example.com" }
                        }
                    }
                    Select { default_value: ROLES[1],
                        SelectTrigger { class: "ml-auto w-[100px]",
                            SelectValue { placeholder: ROLES[1] }
                        }
                        SelectContent {
                            SelectGroup {
                                {ROLES.iter().map(|role| rsx! {
                                    SelectOption { key: "{role}", value: *role, "{role}" }
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}
