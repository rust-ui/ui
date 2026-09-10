use dioxus::prelude::*;
use icons::{CircleCheckBig, Plus, Trash2};

use crate::app::Route;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardDescription};
use crate::components::ui::empty::{Empty, EmptyDescription, EmptyTitle};
use crate::components::ui::input::Input;
use crate::components::ui::skeleton::Skeleton;
use crate::domain::item::item_services::{create_item, delete_item, get_items};

#[component]
pub fn ItemListPage() -> Element {
    let mut items = use_server_future(get_items)?;
    let navigator = use_navigator();

    let mut new_title = use_signal(String::new);
    let mut creating = use_signal(|| false);

    rsx! {
        div { class: "flex relative flex-col mx-auto w-full max-w-md h-full",
            header { class: "px-4 pt-[calc(env(safe-area-inset-top)+4rem)] pb-3 sm:pt-4",
                h1 { class: "text-2xl font-bold tracking-tight", "Items" }
                p { class: "text-sm text-muted-foreground", "Manage your items" }
            }

            div { class: "overflow-y-auto flex-1 px-4",
                match &*items.read() {
                    Some(Ok(list)) if list.is_empty() => rsx! {
                        Empty {
                            EmptyTitle { "No items yet" }
                            EmptyDescription { "Add your first item below." }
                        }
                    },
                    Some(Ok(list)) => rsx! {
                        div { class: "flex flex-col gap-2",
                            for item in list.iter() {
                                {
                                    let id = item.id.to_string();
                                    let id_nav = id.clone();
                                    let item_id = item.id;
                                    rsx! {
                                        div {
                                            key: "{id}",
                                            class: "cursor-pointer",
                                            onclick: move |_| { navigator.push(Route::ItemDetails { id: id_nav.clone() }); },
                                            Card { class: "flex-row gap-3 items-center p-4 hover:bg-accent/50 transition-colors",
                                                CircleCheckBig { class: "shrink-0 size-5 text-primary/40" }
                                                div { class: "flex flex-col flex-1 gap-1 min-w-0",
                                                    span { class: "font-medium truncate", "{item.title}" }
                                                    if let Some(desc) = &item.description {
                                                        CardDescription { class: "line-clamp-1", "{desc}" }
                                                    }
                                                }
                                                div {
                                                    onclick: move |e| { e.stop_propagation(); },
                                                    Button {
                                                        variant: ButtonVariant::Ghost,
                                                        size: ButtonSize::Icon,
                                                        class: "size-8 text-muted-foreground hover:text-destructive",
                                                        onclick: move |_| {
                                                            spawn(async move {
                                                                if delete_item(item_id).await.is_ok() {
                                                                    items.restart();
                                                                }
                                                            });
                                                        },
                                                        Trash2 { class: "size-4" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "text-destructive text-sm", "Error: {e}" }
                    },
                    None => rsx! {
                        div { class: "flex flex-col gap-2",
                            Skeleton { class: "h-16 w-full" }
                            Skeleton { class: "h-16 w-full" }
                            Skeleton { class: "h-16 w-full" }
                        }
                    },
                }
            }

            footer { class: "border-t bg-background",
                form {
                    class: "flex gap-2 items-center p-4",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let title = new_title().trim().to_string();
                        if title.is_empty() { return; }
                        creating.set(true);
                        spawn(async move {
                            if create_item(title, None).await.is_ok() {
                                new_title.set(String::new());
                                items.restart();
                            }
                            creating.set(false);
                        });
                    },
                    Input {
                        placeholder: "New item title...",
                        value: new_title(),
                        oninput: move |e: FormEvent| new_title.set(e.value()),
                    }
                    Button {
                        size: ButtonSize::Icon,
                        disabled: creating(),
                        Plus { class: "size-5" }
                    }
                }
            }
        }
    }
}
