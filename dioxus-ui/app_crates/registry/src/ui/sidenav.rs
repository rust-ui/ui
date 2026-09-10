use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_core::has_context;
use tw_merge::tw_merge;

#[derive(Clone, Copy)]
pub struct SidenavContext {
    pub open: Signal<bool>,
}

impl SidenavContext {
    pub fn toggle(mut self) {
        self.open.set(!(self.open)());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavVariant {
    #[default]
    Sidenav,
    Floating,
    Inset,
}

impl SidenavVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Sidenav => "Sidenav",
            Self::Floating => "Floating",
            Self::Inset => "Inset",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavSide {
    #[default]
    Left,
    Right,
}

impl SidenavSide {
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavCollapsible {
    #[default]
    Offcanvas,
    None,
    Icon,
}

impl SidenavCollapsible {
    #[allow(dead_code)]
    fn as_str(self) -> &'static str {
        match self {
            Self::Offcanvas => "Offcanvas",
            Self::None => "None",
            Self::Icon => "Icon",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavState {
    #[default]
    Expanded,
    Collapsed,
}

impl SidenavState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Expanded => "Expanded",
            Self::Collapsed => "Collapsed",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SidenavMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

#[component]
pub fn SidenavInset(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_variant: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "bg-background relative flex w-full flex-1 flex-col data-[variant=Inset]:rounded-lg data-[variant=Inset]:border data-[variant=Inset]:border-sidenav-border data-[variant=Inset]:shadow-sm data-[variant=Inset]:m-2",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "SidenavInset", "data-variant": data_variant, class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavInner(
    #[props(into, optional)] variant: Option<String>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "flex flex-col w-full h-full bg-sidenav data-[variant=Floating]:rounded-lg data-[variant=Floating]:border data-[variant=Floating]:border-sidenav-border data-[variant=Floating]:shadow-sm",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "SidenavInner", "data-variant": variant.as_deref().unwrap_or(""), class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-col gap-2 p-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SidenavHeader", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavMenu(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-col gap-1 w-full min-w-0", class.as_deref().unwrap_or(""));
    rsx! { ul { "data-name": "SidenavMenu", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavMenuSub(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "border-sidenav-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5 group-data-[collapsible=Icon]:hidden",
        class.as_deref().unwrap_or("")
    );
    rsx! { ul { "data-name": "SidenavMenuSub", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavMenuItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("relative group/menu-item", class.as_deref().unwrap_or(""));
    rsx! { li { "data-name": "SidenavMenuItem", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavMenuSubItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("group/menu-sub-item", class.as_deref().unwrap_or(""));
    rsx! { li { "data-name": "SidenavMenuSubItem", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "scrollbar__on_hover flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=Icon]:overflow-hidden",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "SidenavContent", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavGroup(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_sidenav: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("flex relative flex-col p-2 w-full min-w-0", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SidenavGroup", "data-sidenav": data_sidenav, class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavGroupContent(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_sidenav: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("w-full text-sm", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "SidenavGroupContent", "data-sidenav": data_sidenav, class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavGroupLabel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "text-sidenav-foreground/70 ring-sidenav-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:-mt-8 group-data-[collapsible=Icon]:opacity-0",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "SidenavGroupLabel", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-col gap-2 p-2", class.as_deref().unwrap_or(""));
    rsx! { footer { "data-name": "SidenavFooter", class: "{merged_class}", {children} } }
}

#[component]
pub fn DropdownMenuTriggerEllipsis(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground peer-hover/menu-button:text-sidenav-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=Icon]:hidden peer-data-[active=true]/menu-button:text-sidenav-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0",
        class.as_deref().unwrap_or("")
    );
    rsx! { button { "data-name": "DropdownMenuTriggerEllipsis", class: "{merged_class}", {children} } }
}

#[component]
pub fn SidenavInput(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] name: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged_class = tw_merge!(
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-2 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive read-only:bg-muted w-full h-8 shadow-none bg-background",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        input {
            "data-name": "SidenavInput",
            class: "{merged_class}",
            placeholder: placeholder,
            id: id,
            name: name,
            value: value,
            oninput: move |event| {
                if let Some(handler) = &oninput {
                    handler.call(event);
                }
            },
        }
    }
}

#[component]
pub fn SidenavWrapper(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] style: Option<String>,
    #[props(default = true)] default_open: bool,
    #[props(default = 160)] min_width: u32,
    #[props(default = 480)] max_width: u32,
    children: Element,
) -> Element {
    let open = use_signal(|| default_open);
    provide_context(SidenavContext { open });
    let merged_class = tw_merge!(
        "group/sidenav-wrapper has-data-[variant=Inset]:bg-sidenav flex h-full w-full",
        class.as_deref().unwrap_or("")
    );
    let wrapper_style = format!("--sidenav-width: 256px;{}", style.as_deref().unwrap_or(""));
    rsx! {
        div {
            "data-name": "SidenavWrapper",
            "data-min-width": "{min_width}",
            "data-max-width": "{max_width}",
            style: "{wrapper_style}",
            class: "{merged_class}",
            {children}
        }
    }
}

/// Drag handle — place inside `Sidenav`, on the trailing edge.
/// Reads `data-min-width` / `data-max-width` from the nearest `SidenavWrapper`
/// for clamping; persists width to localStorage as `sidenav-width`.
#[component]
pub fn SidenavResizeHandle(#[props(into, optional)] class: Option<String>) -> Element {
    use_effect(move || {
        eval(
            r#"(function() {
            const h = document.querySelector('[data-sidenav-resize-handle]');
            if (!h || h.dataset.initialized) return;
            h.dataset.initialized = '1';

            const wrapper = h.closest('[data-name="SidenavWrapper"]');
            if (!wrapper) return;

            const saved = localStorage.getItem('sidenav-width');
            if (saved) wrapper.style.setProperty('--sidenav-width', saved + 'px');

            h.addEventListener('mousedown', function(e) {
                e.preventDefault();
                document.body.style.userSelect = 'none';
                const aside = wrapper.querySelector('aside');
                const x0 = e.clientX;
                const w0 = aside ? aside.getBoundingClientRect().width : 256;
                const mn = parseInt(wrapper.dataset.minWidth) || 160;
                const mx = parseInt(wrapper.dataset.maxWidth) || 480;

                function move(e) {
                    const w = Math.max(mn, Math.min(mx, w0 + e.clientX - x0));
                    wrapper.style.setProperty('--sidenav-width', w + 'px');
                }
                function up() {
                    document.body.style.userSelect = '';
                    const aside = wrapper.querySelector('aside');
                    if (aside) localStorage.setItem('sidenav-width', aside.getBoundingClientRect().width);
                    document.removeEventListener('mousemove', move);
                    document.removeEventListener('mouseup', up);
                }
                document.addEventListener('mousemove', move);
                document.addEventListener('mouseup', up);
            });
        })();"#,
        );
    });

    let merged_class = tw_merge!(
        "absolute top-0 right-0 w-1 h-full z-50 cursor-col-resize hover:bg-sidenav-border transition-colors",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-sidenav-resize-handle": true,
            class: "{merged_class}",
        }
    }
}

#[component]
pub fn SidenavLink(
    #[props(into)] href: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left outline-hidden ring-sidenav-ring transition-[width,height,padding] focus-visible:ring-2 active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-semibold aria-[current=page]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent data-[state=open]:hover:text-sidenav-accent-foreground group-data-[collapsible=Icon]:size-8! group-data-[collapsible=Icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 hover:bg-sidenav-accent hover:text-sidenav-accent-foreground h-8 text-sm",
        class.as_deref().unwrap_or("")
    );
    #[cfg(target_arch = "wasm32")]
    let path = web_sys::window().and_then(|w| w.location().pathname().ok()).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    let path = String::new();
    let is_active = path == href || path.starts_with(&format!("{}/", href));

    rsx! {
        a {
            "data-name": "SidenavLink",
            class: "{merged_class}",
            href: "{href}",
            "aria-current": if is_active { "page" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn SidenavMenuButton(
    #[props(into, optional)] href: Option<String>,
    #[props(into, optional)] class: Option<String>,
    #[props(default = SidenavMenuButtonVariant::default())] variant: SidenavMenuButtonVariant,
    #[props(default = SidenavMenuButtonSize::default())] size: SidenavMenuButtonSize,
    #[props(into, optional)] aria_current: Option<String>,
    children: Element,
) -> Element {
    let base = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-hidden ring-sidenav-ring transition-[width,height,padding] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground focus-visible:ring-2 active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-medium aria-[current=page]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent data-[state=open]:hover:text-sidenav-accent-foreground [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:size-8! group-data-[collapsible=Icon]:p-0! [&>svg]:stroke-2 aria-[current=page]:[&>svg]:stroke-[2.7]";
    let variant_class = match variant {
        SidenavMenuButtonVariant::Default => "hover:bg-sidenav-accent hover:text-sidenav-accent-foreground",
        SidenavMenuButtonVariant::Outline => {
            "bg-background shadow-[0_0_0_1px_hsl(var(--sidenav-border))] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground hover:shadow-[0_0_0_1px_hsl(var(--sidenav-accent))]"
        }
    };
    let size_class = match size {
        SidenavMenuButtonSize::Default => "h-8 text-sm",
        SidenavMenuButtonSize::Sm => "h-7 text-xs",
        SidenavMenuButtonSize::Lg => "h-12",
    };
    let merged_class = tw_merge!(base, variant_class, size_class, class.as_deref().unwrap_or(""));

    if let Some(href) = href {
        rsx! {
            a {
                "data-name": "SidenavMenuButton",
                "data-variant": match variant {
                    SidenavMenuButtonVariant::Default => "Default",
                    SidenavMenuButtonVariant::Outline => "Outline",
                },
                "data-size": match size {
                    SidenavMenuButtonSize::Default => "default",
                    SidenavMenuButtonSize::Sm => "sm",
                    SidenavMenuButtonSize::Lg => "lg",
                },
                class: "{merged_class}",
                href: "{href}",
                "aria-current": aria_current.unwrap_or_else(|| "false".to_string()),
                {children}
            }
        }
    } else {
        rsx! {
            button {
                "data-name": "SidenavMenuButton",
                "data-variant": match variant {
                    SidenavMenuButtonVariant::Default => "Default",
                    SidenavMenuButtonVariant::Outline => "Outline",
                },
                "data-size": match size {
                    SidenavMenuButtonSize::Default => "default",
                    SidenavMenuButtonSize::Sm => "sm",
                    SidenavMenuButtonSize::Lg => "lg",
                },
                class: "{merged_class}",
                {children}
            }
        }
    }
}

#[component]
pub fn Sidenav(
    #[props(into, optional)] class: Option<String>,
    #[props(default = SidenavVariant::default())] variant: SidenavVariant,
    #[props(default = SidenavState::default())] data_state: SidenavState,
    #[props(default = SidenavSide::default())] data_side: SidenavSide,
    #[props(default = SidenavCollapsible::default())] data_collapsible: SidenavCollapsible,
    children: Element,
) -> Element {
    let ctx = has_context::<SidenavContext>();
    let is_open = ctx.map(|c| c.open).unwrap_or_else(|| use_signal(|| data_state == SidenavState::Expanded));
    let class_value = class.as_deref().unwrap_or("").to_string();

    if data_collapsible == SidenavCollapsible::None {
        let merged =
            tw_merge!("flex flex-col h-full bg-sidenav text-sidenav-foreground w-(--sidenav-width)", class_value);
        rsx! {
            aside {
                "data-name": "Sidenav",
                class: "{merged}",
                {children}
            }
        }
    } else {
        let gap_class = tw_merge!(
            "relative w-(--sidenav-width) bg-transparent transition-[width] duration-200 ease-linear group-data-[collapsible=Offcanvas]:w-0 group-data-[side=Right]:rotate-180",
            match variant {
                SidenavVariant::Sidenav => "group-data-[collapsible=Icon]:w-(--sidenav-width-icon)",
                SidenavVariant::Floating | SidenavVariant::Inset => {
                    "group-data-[collapsible=Icon]:w-[calc(var(--sidenav-width-icon)+(--spacing(4)))]"
                }
            }
        );
        let container_class = tw_merge!(
            "fixed inset-y-0 z-10 hidden h-svh w-(--sidenav-width) transition-[left,right,width] duration-200 ease-linear md:flex",
            class_value,
            match data_side {
                SidenavSide::Left => "left-0 group-data-[collapsible=Offcanvas]:left-[calc(var(--sidenav-width)*-1)]",
                SidenavSide::Right =>
                    "right-0 group-data-[collapsible=Offcanvas]:right-[calc(var(--sidenav-width)*-1)]",
            },
            match variant {
                SidenavVariant::Sidenav =>
                    "group-data-[collapsible=Icon]:w-(--sidenav-width-icon) group-data-[side=Left]:border-r group-data-[side=Right]:border-l",
                SidenavVariant::Floating | SidenavVariant::Inset => {
                    "p-2 group-data-[collapsible=Icon]:w-[calc(var(--sidenav-width-icon)+(--spacing(4))+2px)]"
                }
            },
        );

        rsx! {
            aside {
                "data-name": "Sidenav",
                "data-sidenav": data_state.as_str(),
                "data-state": if is_open() { "Expanded" } else { "Collapsed" },
                "data-side": data_side.as_str(),
                class: "hidden md:block group peer text-sidenav-foreground data-[state=Collapsed]:hidden",
                div { "data-name": "SidenavGap", class: "{gap_class}" }
                div {
                    "data-name": "SidenavContainer",
                    class: "{container_class}",
                    SidenavInner { variant: variant.as_str(), {children} }
                    SidenavToggleRail {}
                }
            }
        }
    }
}

#[component]
pub fn SidenavTrigger(children: Element) -> Element {
    let ctx = has_context::<SidenavContext>();
    rsx! {
        button {
            "data-name": "SidenavTrigger",
            class: "inline-flex gap-2 justify-center items-center -ml-1 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive size-7 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
            onclick: move |_| {
                if let Some(c) = ctx {
                    c.toggle();
                }
            },
            {children}
        }
    }
}

#[component]
fn SidenavToggleRail() -> Element {
    let ctx = has_context::<SidenavContext>();
    rsx! {
        button {
            "data-name": "SidenavToggleRail",
            "aria-label": "Toggle Sidenav",
            tabindex: "-1",
            class: "hidden absolute inset-y-0 z-20 w-4 transition-all ease-linear -translate-x-1/2 sm:flex group-data-[side=Left]:-right-4 group-data-[side=Right]:left-0 after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] in-data-[side=Left]:cursor-w-resize in-data-[side=Right]:cursor-e-resize [[data-side=Left][data-state=Collapsed]_&]:cursor-e-resize [[data-side=Right][data-state=Collapsed]_&]:cursor-w-resize group-data-[collapsible=Offcanvas]:translate-x-0 group-data-[collapsible=Offcanvas]:after:left-full [[data-side=Left][data-collapsible=Offcanvas]_&]:-right-2 [[data-side=Right][data-collapsible=Offcanvas]_&]:-left-2 hover:after:bg-sidenav-border hover:group-data-[collapsible=Offcanvas]:bg-sidenav",
            onclick: move |_| {
                if let Some(c) = ctx {
                    c.toggle();
                }
            },
        }
    }
}

#[component]
pub fn SidenavMenuSubButton(
    #[props(into)] href: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground active:bg-sidenav-accent active:text-sidenav-accent-foreground flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 text-sm outline-hidden focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-medium aria-[current=page]:text-sidenav-accent-foreground [&>svg]:size-4 [&>svg]:shrink-0",
        class.as_deref().unwrap_or("")
    );
    #[cfg(target_arch = "wasm32")]
    let path = web_sys::window().and_then(|w| w.location().pathname().ok()).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    let path = String::new();
    let is_active = path == href || path.starts_with(&format!("{}/", href));

    rsx! {
        a {
            "data-name": "SidenavMenuSubButton",
            class: "{merged_class}",
            href: "{href}",
            "aria-current": if is_active { "page" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn SidenavMenuAction(
    #[props(optional)] show_on_hover: bool,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground peer-hover/menu-button:text-sidenav-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=Icon]:hidden",
        if show_on_hover {
            "group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 peer-data-[active=true]/menu-button:text-sidenav-accent-foreground md:opacity-0"
        } else {
            ""
        },
        class.as_deref().unwrap_or("")
    );

    rsx! { button { "data-sidenav": "menu-action", "data-name": "SidenavMenuAction", class: "{merged_class}", {children} } }
}
