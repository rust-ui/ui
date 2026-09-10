use dioxus::prelude::*;
use icons::{Bold, Italic, Strikethrough, Underline};

use crate::ui::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem};

#[component]
pub fn DemoToolbar() -> Element {
    let mut bold = use_signal(|| false);
    let mut italic = use_signal(|| false);
    let mut underline = use_signal(|| false);
    let mut strikethrough = use_signal(|| false);

    rsx! {
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
            ToolbarButton { "Edit" }
        }
    }
}
