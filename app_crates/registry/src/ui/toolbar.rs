use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::ui::separator::{Separator, SeparatorOrientation};

// ─── Context ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
struct ToolbarToggleGroupCtx {
    #[allow(dead_code)]
    multiple: bool,
}

// ─── Toolbar ─────────────────────────────────────────────────────────────────

#[component]
pub fn Toolbar(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-1 p-1 rounded-md border bg-background shadow-sm w-fit",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "Toolbar",
            role: "toolbar",
            aria_label: aria_label.as_deref().unwrap_or("Toolbar"),
            class: "{merged}",
            {children}
        }
    }
}

// ─── ToolbarList / ToolbarItem ───────────────────────────────────────────────

#[component]
pub fn ToolbarList(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center gap-1 list-none m-0 p-0", class.as_deref().unwrap_or(""));

    rsx! {
        ul { "data-name": "ToolbarList", class: "{merged}", {children} }
    }
}

#[component]
pub fn ToolbarItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center", class.as_deref().unwrap_or(""));

    rsx! {
        li { "data-name": "ToolbarItem", class: "{merged}", {children} }
    }
}

// ─── ToolbarButton ───────────────────────────────────────────────────────────

#[derive(Default, Clone, PartialEq)]
pub enum ToolbarButtonVariant {
    #[default]
    Default,
    Ghost,
    Outline,
}

impl ToolbarButtonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ToolbarButtonVariant::Default => "bg-transparent hover:bg-accent hover:text-accent-foreground",
            ToolbarButtonVariant::Ghost => "bg-transparent hover:bg-muted hover:text-muted-foreground",
            ToolbarButtonVariant::Outline => {
                "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground"
            }
        }
    }
}

#[component]
pub fn ToolbarButton(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ToolbarButtonVariant::default())] variant: ToolbarButtonVariant,
    #[props(optional)] disabled: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center gap-2 rounded-sm px-2 h-8 text-sm font-medium whitespace-nowrap transition-colors disabled:pointer-events-none disabled:opacity-50 cursor-pointer outline-none focus-visible:ring-2 focus-visible:ring-ring [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0",
        variant.as_str(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "ToolbarButton",
            r#type: "button",
            class: "{merged}",
            disabled,
            onclick: move |e| {
                if let Some(h) = &onclick {
                    h.call(e);
                }
            },
            {children}
        }
    }
}

// ─── ToolbarLink ─────────────────────────────────────────────────────────────

#[component]
pub fn ToolbarLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] href: String,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center gap-2 rounded-sm px-2 h-8 text-sm font-medium whitespace-nowrap transition-colors cursor-pointer outline-none focus-visible:ring-2 focus-visible:ring-ring hover:bg-accent hover:text-accent-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        a { "data-name": "ToolbarLink", class: "{merged}", href: "{href}", {children} }
    }
}

// ─── ToolbarToggleGroup ───────────────────────────────────────────────────────

#[component]
pub fn ToolbarToggleGroup(
    #[props(into, optional)] class: Option<String>,
    /// Allow multiple items pressed at once
    #[props(default = false)]
    multiple: bool,
    children: Element,
) -> Element {
    provide_context(ToolbarToggleGroupCtx { multiple });

    let merged = tw_merge!("flex items-center gap-px", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            "data-name": "ToolbarToggleGroup",
            role: "group",
            class: "{merged}",
            {children}
        }
    }
}

// ─── ToolbarToggleItem ────────────────────────────────────────────────────────

#[component]
pub fn ToolbarToggleItem(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] title: Option<String>,
    #[props(default = false)] pressed: bool,
    #[props(optional)] disabled: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let state = if pressed { "on" } else { "off" };

    let merged = tw_merge!(
        "inline-flex items-center justify-center rounded-sm size-8 text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 cursor-pointer outline-none focus-visible:ring-2 focus-visible:ring-ring bg-transparent hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "ToolbarToggleItem",
            r#type: "button",
            class: "{merged}",
            "data-state": "{state}",
            "aria-pressed": "{pressed}",
            title: title.as_deref().unwrap_or(""),
            disabled,
            onclick: move |e| {
                if let Some(h) = &onclick {
                    h.call(e);
                }
            },
            {children}
        }
    }
}

// ─── ToolbarSeparator ─────────────────────────────────────────────────────────

#[component]
pub fn ToolbarSeparator(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!("mx-1 self-stretch", class.as_deref().unwrap_or(""));

    rsx! {
        Separator { class: "{merged}", orientation: SeparatorOrientation::Vertical }
    }
}
