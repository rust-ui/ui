use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::dialog::use_dialog_trigger_id;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AttachmentSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl AttachmentSize {
    fn as_str(&self) -> &'static str {
        match self {
            AttachmentSize::Default => "Default",
            AttachmentSize::Sm => "Sm",
            AttachmentSize::Xs => "Xs",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl AttachmentOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            AttachmentOrientation::Horizontal => "Horizontal",
            AttachmentOrientation::Vertical => "Vertical",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AttachmentState {
    #[default]
    Done,
    Idle,
    Uploading,
    Processing,
    Error,
}

impl AttachmentState {
    fn as_str(&self) -> &'static str {
        match self {
            AttachmentState::Done => "Done",
            AttachmentState::Idle => "Idle",
            AttachmentState::Uploading => "Uploading",
            AttachmentState::Processing => "Processing",
            AttachmentState::Error => "Error",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AttachmentMediaVariant {
    #[default]
    Icon,
    Image,
}

impl AttachmentMediaVariant {
    fn as_str(&self) -> &'static str {
        match self {
            AttachmentMediaVariant::Icon => "Icon",
            AttachmentMediaVariant::Image => "Image",
        }
    }
}

#[component]
pub fn AttachmentContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "max-w-full min-w-0 flex-1 leading-tight group-data-[orientation=Vertical]/attachment:px-1",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "AttachmentContent", class: "{merged}", {children} } }
}

#[component]
pub fn AttachmentTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "block max-w-full min-w-0 truncate font-medium group-data-[state=Processing]/attachment:shimmer group-data-[state=Uploading]/attachment:shimmer",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "AttachmentTitle", class: "{merged}", {children} } }
}

#[component]
pub fn AttachmentDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "mt-0.5 block min-w-0 truncate text-xs text-muted-foreground group-data-[state=Error]/attachment:text-destructive/80 max-w-full",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "AttachmentDescription", class: "{merged}", {children} } }
}

#[component]
pub fn AttachmentGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex min-w-0 scroll-fade-x snap-x snap-mandatory scroll-px-1 scrollbar-none gap-3 overflow-x-auto overscroll-x-contain py-1 *:data-[name=Attachment]:flex-none *:data-[name=Attachment]:snap-start",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "AttachmentGroup", class: "{merged}", {children} } }
}

#[component]
pub fn AttachmentActions(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "relative z-20 flex shrink-0 items-center group-data-[orientation=Vertical]/attachment:absolute group-data-[orientation=Vertical]/attachment:top-3 group-data-[orientation=Vertical]/attachment:right-3 group-data-[orientation=Vertical]/attachment:gap-1",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "AttachmentActions", class: "{merged}", {children} } }
}

#[component]
pub fn Attachment(
    #[props(default = AttachmentSize::default())] size: AttachmentSize,
    #[props(default = AttachmentOrientation::default())] orientation: AttachmentOrientation,
    #[props(default = AttachmentState::default())] state: AttachmentState,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let size_class = match size {
        AttachmentSize::Default => {
            "gap-2 text-sm has-data-[name=AttachmentContent]:px-2.5 has-data-[name=AttachmentContent]:py-2 has-data-[name=AttachmentMedia]:p-2"
        }
        AttachmentSize::Sm => {
            "gap-2.5 text-xs has-data-[name=AttachmentContent]:px-2 has-data-[name=AttachmentContent]:py-1.5 has-data-[name=AttachmentMedia]:p-1.5"
        }
        AttachmentSize::Xs => {
            "gap-1.5 rounded-lg text-xs has-data-[name=AttachmentContent]:px-1.5 has-data-[name=AttachmentContent]:py-1 has-data-[name=AttachmentMedia]:p-1"
        }
    };
    let orientation_class = match orientation {
        AttachmentOrientation::Horizontal => "min-w-40 items-center",
        AttachmentOrientation::Vertical => "w-24 flex-col has-data-[name=AttachmentContent]:w-30",
    };

    let merged = tw_merge!(
        "group/attachment relative flex w-fit max-w-full min-w-0 shrink-0 flex-wrap rounded-xl border bg-card text-card-foreground transition-colors focus-within:ring-1 focus-within:ring-ring/50 has-[>a,>button]:hover:bg-muted/50 data-[state=Error]:border-destructive/30 data-[state=Idle]:border-dashed",
        size_class,
        orientation_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "Attachment",
            "data-size": size.as_str(),
            "data-orientation": orientation.as_str(),
            "data-state": state.as_str(),
            class: "{merged}",
            {children}
        }
    }
}

#[component]
pub fn AttachmentMedia(
    #[props(default = AttachmentMediaVariant::default())] variant: AttachmentMediaVariant,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        AttachmentMediaVariant::Icon => "",
        AttachmentMediaVariant::Image => {
            "opacity-60 group-data-[state=Done]/attachment:opacity-100 group-data-[state=Idle]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
        }
    };

    let merged = tw_merge!(
        "relative flex aspect-square w-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-muted text-foreground group-data-[orientation=Vertical]/attachment:w-full group-data-[size=Sm]/attachment:w-8 group-data-[size=Xs]/attachment:w-7 group-data-[size=Xs]/attachment:rounded-md group-data-[state=Error]/attachment:bg-destructive/10 group-data-[state=Error]/attachment:text-destructive [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 group-data-[orientation=Vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[size=Xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5",
        variant_class,
        class.as_deref().unwrap_or("")
    );

    rsx! { div { "data-name": "AttachmentMedia", "data-variant": variant.as_str(), class: "{merged}", {children} } }
}

#[component]
pub fn AttachmentAction(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] variant: Option<ButtonVariant>,
    #[props(optional)] size: Option<ButtonSize>,
    #[props(into, optional)] aria_label: Option<String>,
    #[props(into, optional)] title: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        Button {
            variant: variant.unwrap_or(ButtonVariant::Ghost),
            size: size.unwrap_or(ButtonSize::IconXs),
            class: class,
            aria_label: aria_label,
            title: title,
            onclick,
            {children}
        }
    }
}

#[component]
pub fn AttachmentTrigger(
    #[props(into, optional)] href: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
) -> Element {
    let merged = tw_merge!("absolute inset-0 z-10 outline-none", class.as_deref().unwrap_or(""));
    let dialog_id = use_dialog_trigger_id();

    if let Some(url) = href {
        rsx! {
            a {
                "data-name": "AttachmentTrigger",
                class: "{merged}",
                href: "{url}",
                target: "_blank",
                rel: "noreferrer",
                "aria-label": aria_label.as_deref(),
            }
        }
    } else if onclick.is_some() {
        rsx! {
            button {
                "data-name": "AttachmentTrigger",
                r#type: "button",
                class: "{merged}",
                "aria-label": aria_label.as_deref(),
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                },
            }
        }
    } else {
        rsx! {
            button {
                "data-name": "AttachmentTrigger",
                r#type: "button",
                class: "{merged}",
                "aria-label": aria_label.as_deref(),
                "data-dialog-trigger": dialog_id.as_deref().unwrap_or(""),
            }
        }
    }
}
