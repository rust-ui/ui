use dioxus::prelude::*;
use icons::{ArrowUpRight, Folder};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyCard() -> Element {
    rsx! {
        Card { class: "w-full max-w-md",
            CardContent { class: "p-0",
                Empty {
                    EmptyHeader {
                        EmptyMedia { variant: EmptyMediaVariant::Icon,
                            Folder {}
                        }
                        EmptyTitle { "No Projects Yet" }
                        EmptyDescription {
                            "You haven't created any projects yet. Get started by creating your first project."
                        }
                    }
                    EmptyContent {
                        div { class: "flex gap-2",
                            Button { "Create Project" }
                            Button { variant: ButtonVariant::Outline, "Import Project" }
                        }
                        Button { variant: ButtonVariant::Link, class: "text-muted-foreground",
                            a { href: "#", class: "flex gap-1 items-center",
                                span { "Learn More" }
                                ArrowUpRight {}
                            }
                        }
                    }
                }
            }
        }
    }
}
