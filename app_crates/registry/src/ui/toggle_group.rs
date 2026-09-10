use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Default)]
struct ToggleGroupCtx {
    variant: ToggleGroupVariant,
    orientation: ToggleGroupOrientation,
    spacing: i32,
}

#[component]
pub fn ToggleGroup(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ToggleGroupVariant::Default)] variant: ToggleGroupVariant,
    #[props(default = ToggleGroupOrientation::Horizontal)] orientation: ToggleGroupOrientation,
    #[props(default = 1)] spacing: i32,
    children: Element,
) -> Element {
    provide_context(ToggleGroupCtx { variant, orientation, spacing });

    let is_vertical = orientation == ToggleGroupOrientation::Vertical;
    let gap_style = if spacing == 0 { "gap: 0px".to_string() } else { format!("gap: {}rem", spacing as f64 * 0.25) };

    let class = tw_merge!(
        "flex items-center rounded-md group/toggle-group w-fit",
        if is_vertical { "flex-col" } else { "" },
        if variant == ToggleGroupVariant::Outline { "shadow-xs" } else { "" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{class}",
            style: "{gap_style}",
            "data-variant": if variant == ToggleGroupVariant::Outline { "Outline" } else { "Default" },
            "data-orientation": if is_vertical { "Vertical" } else { "Horizontal" },
            "data-spacing": "{spacing}",
            {children}
        }
    }
}

#[component]
pub fn ToggleGroupAction(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] href: String,
    #[props(into, optional)] target: Option<String>,
    #[props(into, optional)] title: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex gap-2 justify-center items-center p-0 text-sm font-medium whitespace-nowrap rounded-sm transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive size-6 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        a {
            class: "{merged}",
            href: "{href}",
            target: target.as_deref(),
            title: title.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn ToggleGroupItem(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] title: String,
    #[props(default = false)] pressed: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let ctx = use_context::<ToggleGroupCtx>();
    let is_vertical = ctx.orientation == ToggleGroupOrientation::Vertical;
    let is_grouped = ctx.spacing == 0;
    let is_outline = ctx.variant == ToggleGroupVariant::Outline;

    let rounded = match (is_grouped, is_vertical) {
        (true, true) => "rounded-none first:rounded-t-md last:rounded-b-md",
        (true, false) => "rounded-none first:rounded-l-md last:rounded-r-md",
        (false, _) => "rounded-md",
    };

    let border = if is_outline && is_grouped {
        if is_vertical { "border border-t-0 first:border-t" } else { "border border-l-0 first:border-l" }
    } else if is_outline {
        "border"
    } else {
        ""
    };

    let width = if is_vertical { "w-full" } else { "" };
    let state = if pressed { "on" } else { "off" };

    let merged = tw_merge!(
        "inline-flex flex-1 gap-2 justify-center items-center px-2 min-w-0 h-9 text-sm font-medium whitespace-nowrap bg-transparent shadow-none outline-none focus:z-10 focus-visible:z-10 disabled:opacity-50 disabled:pointer-events-none data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 transition-[color,box-shadow] aria-invalid:ring-destructive/20 aria-invalid:border-destructive shrink-0 dark:aria-invalid:ring-destructive/40 hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
        rounded,
        border,
        width,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            r#type: "button",
            "data-name": "ToggleGroupItem",
            class: "{merged}",
            role: "radio",
            tabindex: "-1",
            title: "{title}",
            "data-state": "{state}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
