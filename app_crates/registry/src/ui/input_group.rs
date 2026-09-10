use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::ui::input::InputType;

#[component]
pub fn InputGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "group/input-group border-input dark:bg-input/30 relative flex w-full items-center rounded-md border shadow-xs transition-[color,box-shadow] outline-none h-9 min-w-0 has-[>textarea]:h-auto has-[>[data-align=inline-start]]:[&>input]:pl-2 has-[>[data-align=inline-end]]:[&>input]:pr-2 has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-start]]:[&>input]:pb-3 has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-end]]:[&>input]:pt-3 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot=input-group-control]:focus-visible]:ring-[3px] has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-name": "InputGroup", "data-slot": "input-group", role: "group", class: "{merged}",
            {children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAddonAlign {
    fn as_str(&self) -> &'static str {
        match self {
            InputGroupAddonAlign::InlineStart => "inline-start",
            InputGroupAddonAlign::InlineEnd => "inline-end",
            InputGroupAddonAlign::BlockStart => "block-start",
            InputGroupAddonAlign::BlockEnd => "block-end",
        }
    }

    fn class(&self) -> &'static str {
        match self {
            InputGroupAddonAlign::InlineStart => {
                "order-first pl-3 has-[>button]:ml-[-0.45rem] has-[>kbd]:ml-[-0.35rem]"
            }
            InputGroupAddonAlign::InlineEnd => "order-last pr-3 has-[>button]:mr-[-0.45rem] has-[>kbd]:mr-[-0.35rem]",
            InputGroupAddonAlign::BlockStart => {
                "order-first w-full justify-start px-3 pt-3 [.border-b]:pb-3 group-has-[>input]/input-group:pt-2.5"
            }
            InputGroupAddonAlign::BlockEnd => {
                "order-last w-full justify-start px-3 pb-3 [.border-t]:pt-3 group-has-[>input]/input-group:pb-2.5"
            }
        }
    }
}

#[component]
pub fn InputGroupAddon(
    #[props(default = InputGroupAddonAlign::default())] align: InputGroupAddonAlign,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "text-muted-foreground flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium select-none [&>svg:not([class*='size-'])]:size-4 [&>kbd]:rounded-[calc(var(--radius)-5px)] group-data-[disabled=true]/input-group:opacity-50",
        align.class(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "InputGroupAddon",
            "data-slot": "input-group-addon",
            "data-align": align.as_str(),
            role: "group",
            class: "{merged}",
            {children}
        }
    }
}

#[component]
pub fn InputGroupText(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "text-muted-foreground flex items-center gap-2 text-sm [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { class: "{merged}", {children} } }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupButtonVariant {
    #[default]
    Ghost,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupButtonSize {
    Xs,
    #[default]
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    fn class(&self) -> &'static str {
        match self {
            InputGroupButtonSize::Xs => {
                "h-6 gap-1 px-2 rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-3.5 has-[>svg]:px-2"
            }
            InputGroupButtonSize::Sm => "h-8 px-2.5 gap-1.5 rounded-md has-[>svg]:px-2.5",
            InputGroupButtonSize::IconXs => "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
            InputGroupButtonSize::IconSm => "size-8 p-0 has-[>svg]:p-0",
        }
    }
}

#[component]
pub fn InputGroupButton(
    #[props(default = InputGroupButtonVariant::default())] variant: InputGroupButtonVariant,
    #[props(default = InputGroupButtonSize::default())] size: InputGroupButtonSize,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let _ = variant;
    let merged = tw_merge!("text-sm shadow-none flex gap-2 items-center", size.class(), class.as_deref().unwrap_or(""));
    rsx! {
        button {
            r#type: "button",
            class: "{merged}",
            aria_label: aria_label,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn InputGroupInput(
    #[props(into, optional)] class: Option<String>,
    #[props(default = InputType::default())] r#type: InputType,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] name: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(into, optional)] autocomplete: Option<String>,
    #[props(optional)] disabled: bool,
    #[props(optional)] readonly: bool,
    #[props(optional)] required: bool,
    #[props(into, optional)] step: Option<String>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged = tw_merge!(
        "flex-1 rounded-none border-0 bg-transparent shadow-none focus-visible:ring-0 dark:bg-transparent",
        "placeholder:text-muted-foreground h-9 w-full min-w-0 px-3 py-1 text-base outline-none transition-[color,box-shadow]",
        "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        input {
            "data-slot": "input-group-control",
            r#type: r#type.as_str(),
            class: "{merged}",
            placeholder: placeholder,
            name: name,
            id: id,
            value: value,
            autocomplete: autocomplete,
            step: step,
            disabled: disabled,
            readonly: readonly,
            required: required,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}

#[component]
pub fn InputGroupTextarea(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none focus-visible:ring-0 dark:bg-transparent",
        "placeholder:text-muted-foreground w-full min-w-0 px-3 text-base outline-none transition-[color,box-shadow]",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        textarea { "data-slot": "input-group-control", class: "{merged}" }
    }
}
