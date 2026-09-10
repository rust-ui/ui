use dioxus::prelude::*;
use icons::{Bold, Image, Italic, Plus, Strikethrough, TextCursorInput, Type, Underline};
use registry::ui::toolbar::{
    Toolbar, ToolbarButton, ToolbarItem, ToolbarLink, ToolbarList, ToolbarSeparator, ToolbarToggleGroup,
    ToolbarToggleItem,
};

#[component]
pub fn DemoToolbar() -> Element {
    let mut bold = use_signal(|| false);
    let mut italic = use_signal(|| false);
    let mut underline = use_signal(|| false);
    let mut strikethrough = use_signal(|| false);

    let mut text_active = use_signal(|| false);
    let mut image_active = use_signal(|| false);
    let mut input_active = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-8",

            div { class: "flex flex-col gap-2",
                p { class: "text-xs text-muted-foreground font-medium uppercase tracking-wider",
                    "Text Formatting"
                }
                Toolbar { aria_label: "Text formatting",
                    ToolbarToggleGroup {
                        ToolbarToggleItem {
                            title: "Bold",
                            pressed: bold(),
                            onclick: move |_| bold.set(!bold()),
                            Bold {}
                        }
                        ToolbarToggleItem {
                            title: "Italic",
                            pressed: italic(),
                            onclick: move |_| italic.set(!italic()),
                            Italic {}
                        }
                        ToolbarToggleItem {
                            title: "Underline",
                            pressed: underline(),
                            onclick: move |_| underline.set(!underline()),
                            Underline {}
                        }
                        ToolbarToggleItem {
                            title: "Strikethrough",
                            pressed: strikethrough(),
                            onclick: move |_| strikethrough.set(!strikethrough()),
                            Strikethrough {}
                        }
                    }
                    ToolbarSeparator {}
                    ToolbarButton {
                        onclick: move |_| {
                            bold.set(false);
                            italic.set(false);
                            underline.set(false);
                            strikethrough.set(false);
                        },
                        "Clear"
                    }
                }
            }

            div { class: "flex flex-col gap-2",
                p { class: "text-xs text-muted-foreground font-medium uppercase tracking-wider",
                    "Insert (ToolbarList)"
                }
                Toolbar { aria_label: "Insert elements",
                    ToolbarList {
                        ToolbarItem {
                            ToolbarToggleItem {
                                title: "Text",
                                pressed: text_active(),
                                onclick: move |_| text_active.set(!text_active()),
                                Type {}
                            }
                        }
                        ToolbarItem {
                            ToolbarToggleItem {
                                title: "Image",
                                pressed: image_active(),
                                onclick: move |_| image_active.set(!image_active()),
                                Image {}
                            }
                        }
                        ToolbarItem {
                            ToolbarToggleItem {
                                title: "Input",
                                pressed: input_active(),
                                onclick: move |_| input_active.set(!input_active()),
                                TextCursorInput {}
                            }
                        }
                    }
                    ToolbarSeparator {}
                    ToolbarList {
                        ToolbarItem {
                            ToolbarButton {
                                Plus {}
                                "Button"
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-2",
                p { class: "text-xs text-muted-foreground font-medium uppercase tracking-wider",
                    "With Link"
                }
                Toolbar { aria_label: "Mixed toolbar",
                    ToolbarButton { "Action" }
                    ToolbarSeparator {}
                    ToolbarLink { href: "#", "Docs" }
                    ToolbarLink { href: "#", "Changelog" }
                }
            }
        }
    }
}
