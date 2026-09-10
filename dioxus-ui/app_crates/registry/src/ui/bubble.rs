use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BubbleVariant::Default => "Default",
            BubbleVariant::Secondary => "Secondary",
            BubbleVariant::Muted => "Muted",
            BubbleVariant::Tinted => "Tinted",
            BubbleVariant::Outline => "Outline",
            BubbleVariant::Ghost => "Ghost",
            BubbleVariant::Destructive => "Destructive",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

impl BubbleAlign {
    fn as_str(&self) -> &'static str {
        match self {
            BubbleAlign::Start => "Start",
            BubbleAlign::End => "End",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum BubbleReactionsSide {
    #[default]
    Bottom,
    Top,
}

impl BubbleReactionsSide {
    fn as_str(&self) -> &'static str {
        match self {
            BubbleReactionsSide::Bottom => "Bottom",
            BubbleReactionsSide::Top => "Top",
        }
    }
}

#[component]
pub fn BubbleGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex min-w-0 flex-col gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "BubbleGroup", class: "{merged}", {children} } }
}

#[component]
pub fn Bubble(
    #[props(default = BubbleVariant::default())] variant: BubbleVariant,
    #[props(default = BubbleAlign::default())] align: BubbleAlign,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        BubbleVariant::Default => {
            "*:data-[name=BubbleContent]:bg-primary *:data-[name=BubbleContent]:text-primary-foreground [&>[data-name=BubbleContent]:is(button,a):hover]:bg-primary/80"
        }
        BubbleVariant::Secondary => {
            "*:data-[name=BubbleContent]:bg-secondary *:data-[name=BubbleContent]:text-secondary-foreground [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
        }
        BubbleVariant::Muted => {
            "*:data-[name=BubbleContent]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
        }
        BubbleVariant::Tinted => {
            "*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] *:data-[name=BubbleContent]:text-foreground dark:*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
        }
        BubbleVariant::Outline => {
            "*:data-[name=BubbleContent]:border-border *:data-[name=BubbleContent]:bg-background [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-input/30"
        }
        BubbleVariant::Ghost => {
            "border-none *:data-[name=BubbleContent]:rounded-none *:data-[name=BubbleContent]:bg-transparent *:data-[name=BubbleContent]:p-0 [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted/50"
        }
        BubbleVariant::Destructive => {
            "*:data-[name=BubbleContent]:bg-destructive/10 *:data-[name=BubbleContent]:text-destructive dark:*:data-[name=BubbleContent]:bg-destructive/20 [&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/20 dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/30"
        }
    };

    let merged = tw_merge!(
        "group/bubble relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1 group-data-[align=End]/message:self-end data-[align=End]:self-end data-[variant=Ghost]:max-w-full",
        variant_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "Bubble",
            "data-variant": variant.as_str(),
            "data-align": align.as_str(),
            class: "{merged}",
            {children}
        }
    }
}

#[component]
pub fn BubbleContent(
    #[props(into, optional)] href: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "w-fit max-w-full min-w-0 overflow-hidden rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed wrap-break-word group-data-[align=End]/bubble:self-end [button]:text-left [button,a]:transition-colors [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50",
        class.as_deref().unwrap_or("")
    );

    if let Some(url) = href {
        rsx! {
            a { "data-name": "BubbleContent", class: "{merged}", href: "{url}", {children} }
        }
    } else if onclick.is_some() {
        rsx! {
            button {
                "data-name": "BubbleContent",
                r#type: "button",
                class: "{merged}",
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                },
                {children}
            }
        }
    } else {
        rsx! {
            div { "data-name": "BubbleContent", class: "{merged}", {children} }
        }
    }
}

#[component]
pub fn BubbleReactions(
    #[props(default = BubbleReactionsSide::default())] side: BubbleReactionsSide,
    #[props(default = BubbleAlign::default())] align: BubbleAlign,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] role: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let side_class = match side {
        BubbleReactionsSide::Top => "top-0 -translate-y-3/4",
        BubbleReactionsSide::Bottom => "bottom-0 translate-y-3/4",
    };
    let align_class = match align {
        BubbleAlign::Start => "left-3",
        BubbleAlign::End => "right-3",
    };

    let merged = tw_merge!(
        "absolute z-10 flex w-fit shrink-0 items-center justify-center gap-1 rounded-full bg-muted px-1.5 py-0.5 text-sm ring-3 ring-card has-[button]:p-0",
        side_class,
        align_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-name": "BubbleReactions",
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            role: role.as_deref(),
            "aria-label": aria_label.as_deref(),
            class: "{merged}",
            {children}
        }
    }
}
