use dioxus::prelude::*;
use icons::{ScrollText, Trash2};

use crate::app::Route;
use crate::components::layout::back_button::BackButton;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::skeleton::Skeleton;
use crate::domain::item::item_services::{delete_item, get_item};
use crate::utils::param::parse_uuid;

#[component]
pub fn ItemDetailsPage(id: String) -> Element {
    let uuid = parse_uuid(&id);
    let navigator = use_navigator();

    let item = use_server_future(move || {
        let uuid = uuid;
        async move {
            match uuid {
                Some(id) => get_item(id).await,
                None => Ok(None),
            }
        }
    })?;

    rsx! {
        div { class: "flex flex-col mx-auto w-full max-w-md h-full bg-background",
            header { class: "flex items-center px-4 pt-[calc(env(safe-area-inset-top)+4rem)] pb-3 sm:pt-4",
                BackButton {}
                h1 { class: "flex-1 text-lg font-semibold text-center", "Item Details" }
                div { class: "size-8" }
            }

            div { class: "overflow-y-auto flex-1 px-4 pb-4",
                match &*item.read() {
                    Some(Ok(Some(item))) => {
                        let item_id = item.id;
                        let title = item.title.clone();
                        let description = item.description.clone();
                        rsx! {
                            div { class: "flex flex-col gap-6",
                                div { class: "flex justify-center py-6",
                                    div { class: "flex justify-center items-center rounded-full size-20 bg-primary/10",
                                        ScrollText { class: "size-10 text-primary" }
                                    }
                                }

                                h2 { class: "text-2xl font-bold text-center", "{title}" }

                                Card {
                                    CardHeader {
                                        CardTitle { class: "text-sm text-muted-foreground", "Description" }
                                    }
                                    CardContent {
                                        if let Some(desc) = &description {
                                            p { class: "leading-relaxed", "{desc}" }
                                        } else {
                                            p { class: "text-muted-foreground italic", "No description." }
                                        }
                                    }
                                }

                                Card {
                                    CardHeader {
                                        CardTitle { class: "text-sm text-muted-foreground", "Details" }
                                    }
                                    CardContent {
                                        dl { class: "grid gap-y-2 gap-x-4 text-sm grid-cols-[auto_1fr]",
                                            dt { class: "text-muted-foreground", "ID" }
                                            dd { class: "font-mono text-xs truncate", "{item_id}" }
                                        }
                                    }
                                }

                                Button {
                                    variant: ButtonVariant::Destructive,
                                    class: "w-full",
                                    onclick: move |_| {
                                        let nav = navigator.clone();
                                        spawn(async move {
                                            if delete_item(item_id).await.is_ok() {
                                                nav.push(Route::ItemList {});
                                            }
                                        });
                                    },
                                    Trash2 { class: "size-4" }
                                    "Delete Item"
                                }
                            }
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        p { class: "text-muted-foreground", "Item not found." }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "text-destructive text-sm", "Error: {e}" }
                    },
                    None => rsx! {
                        div { class: "flex flex-col gap-6",
                            div { class: "flex justify-center py-6",
                                Skeleton { class: "rounded-full size-20" }
                            }
                            Skeleton { class: "mx-auto w-48 h-8" }
                            Card {
                                CardHeader { Skeleton { class: "w-24 h-4" } }
                                CardContent { Skeleton { class: "w-full h-4" } }
                            }
                        }
                    },
                }
            }
        }
    }
}
