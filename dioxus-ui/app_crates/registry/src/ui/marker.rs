use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
}

impl MarkerVariant {
    fn as_str(&self) -> &'static str {
        match self {
            MarkerVariant::Default => "Default",
            MarkerVariant::Separator => "Separator",
            MarkerVariant::Border => "Border",
        }
    }
}

#[component]
pub fn MarkerContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "min-w-0 wrap-break-word group-data-[variant=Separator]/marker:flex-none group-data-[variant=Separator]/marker:text-center *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "MarkerContent", class: "{merged}", {children} } }
}

#[component]
pub fn MarkerIcon(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("size-4 shrink-0 [&_svg:not([class*='size-'])]:size-4", class.as_deref().unwrap_or(""));
    rsx! {
        span { "data-name": "MarkerIcon", "aria-hidden": "true", class: "{merged}", {children} }
    }
}

#[component]
pub fn Marker(
    #[props(default = MarkerVariant::default())] variant: MarkerVariant,
    #[props(into, optional)] class: Option<String>,
    /// Renders the marker as an <a> element
    #[props(into, optional)]
    href: Option<String>,
    /// Renders the marker as a <button> element
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// role attribute (e.g. "status" for streaming markers)
    #[props(into, optional)]
    role: Option<String>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        MarkerVariant::Default => "",
        MarkerVariant::Separator => {
            "before:mr-1 before:h-px before:min-w-0 before:flex-1 before:bg-border after:ml-1 after:h-px after:min-w-0 after:flex-1 after:bg-border"
        }
        MarkerVariant::Border => "border-b border-border pb-2",
    };
    let merged = tw_merge!(
        "group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm text-muted-foreground [&_svg:not([class*='size-'])]:size-4 [a]:underline [a]:underline-offset-3 [a]:hover:text-foreground",
        variant_class,
        class.as_deref().unwrap_or("")
    );

    if let Some(url) = href {
        return rsx! {
            a {
                "data-name": "Marker",
                "data-variant": variant.as_str(),
                href: "{url}",
                role: role.as_deref(),
                class: "{merged}",
                {children}
            }
        };
    }

    if onclick.is_some() {
        return rsx! {
            button {
                r#type: "button",
                "data-name": "Marker",
                "data-variant": variant.as_str(),
                role: role.as_deref(),
                class: "{merged}",
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                },
                {children}
            }
        };
    }

    rsx! {
        div {
            "data-name": "Marker",
            "data-variant": variant.as_str(),
            role: role.as_deref(),
            class: "{merged}",
            {children}
        }
    }
}
