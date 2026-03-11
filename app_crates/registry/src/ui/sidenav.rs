use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_ui::{clx, variants, void};

mod components {
    use super::*;
    // clx! {SidenavInset, div, "bg-background relative flex w-full flex-1 flex-col md:peer-data-[variant=Inset]:m-2 md:peer-data-[variant=Inset]:ml-0 md:peer-data-[variant=Inset]:rounded-xl md:peer-data-[variant=Inset]:shadow-sm md:peer-data-[variant=Inset]:peer-data-[state=Collapsed]:ml-2"}
    clx! {SidenavInset, div, "bg-background relative flex w-full flex-1 flex-col data-[variant=Inset]:rounded-lg data-[variant=Inset]:border data-[variant=Inset]:border-sidenav-border data-[variant=Inset]:shadow-sm data-[variant=Inset]:m-2"}
    // * data-[], not group-data-[]
    clx! {SidenavInner, div, "flex flex-col w-full h-full bg-sidenav data-[variant=Floating]:rounded-lg data-[variant=Floating]:border data-[variant=Floating]:border-sidenav-border data-[variant=Floating]:shadow-sm"}
    clx! {SidenavHeader, div, "flex flex-col gap-2 p-2"}
    clx! {SidenavMenu, ul, "flex flex-col gap-1 w-full min-w-0"}
    clx! {SidenavMenuSub, ul, "border-sidenav-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5 group-data-[collapsible=Icon]:hidden"}
    clx! {SidenavMenuItem, li, "relative group/menu-item"}
    clx! {SidenavMenuSubItem, li, "group/menu-sub-item"}
    clx! {SidenavContent, div, "scrollbar__on_hover", "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=Icon]:overflow-hidden"}
    clx! {SidenavGroup, div, "flex relative flex-col p-2 w-full min-w-0"}
    clx! {SidenavGroupContent, div, "w-full text-sm"}
    clx! {SidenavGroupLabel, div, "text-sidenav-foreground/70 ring-sidenav-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:-mt-8 group-data-[collapsible=Icon]:opacity-0"}
    clx! {SidenavFooter, footer, "flex flex-col gap-2 p-2"}
    // Button "More"
    clx! {DropdownMenuTriggerEllipsis, button, "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground peer-hover/menu-button:text-sidenav-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=Icon]:hidden peer-data-[active=true]/menu-button:text-sidenav-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0"}

    void! {SidenavInput, input,
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-ring focus-visible:ring-ring/50",
        "focus-visible:ring-2", // TODO. Port tw_merge to Tailwind V4.
     // "focus-visible:ring-[3px]", // TODO. Port tw_merge to Tailwind V4.
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        "read-only:bg-muted",
        // Specific to Sidenav
        "w-full h-8 shadow-none bg-background"
    }
}

pub use components::*;

/* ========================================================== */
/*                   ✨ SIDENAV CONTEXT ✨                    */
/* ========================================================== */

/// Reactive context for controlling the Sidenav open/closed state.
/// Provided automatically by `SidenavWrapper`.
#[derive(Clone, Copy)]
pub struct SidenavContext {
    pub open: RwSignal<bool>,
}

impl SidenavContext {
    pub fn toggle(self) {
        self.open.update(|v| *v = !*v);
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Root layout wrapper for the Sidenav system.
/// Provides `SidenavContext` to all descendants automatically.
/// Use `default_open` to set the initial state (default: `true` = expanded).
#[component]
pub fn SidenavWrapper(
    #[prop(into, optional)] class: String,
    #[prop(default = true)] default_open: bool,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    provide_context(SidenavContext { open });

    let merged_class = tw_merge!("group/sidenav-wrapper has-data-[variant=Inset]:bg-sidenav flex h-full w-full", class);

    view! {
        <div class=merged_class data-name="SidenavWrapper">
            {children()}
        </div>
    }
}

#[component]
pub fn SidenavLink(
    children: Children,
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left outline-hidden ring-sidenav-ring transition-[width,height,padding] focus-visible:ring-2 active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-semibold aria-[current=page]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent data-[state=open]:hover:text-sidenav-accent-foreground group-data-[collapsible=Icon]:size-8! group-data-[collapsible=Icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 hover:bg-sidenav-accent hover:text-sidenav-accent-foreground h-8 text-sm",
        class
    );

    let location = use_location();

    // Check if the link is active based on current path
    let href_clone = href.clone();
    let is_active = move || {
        let path = location.pathname.get();
        path == href_clone || path.starts_with(&format!("{}/", href_clone))
    };

    let aria_current = move || if is_active() { "page" } else { "false" };

    view! {
        <a data-name="SidenavLink" class=merged_class href=href aria-current=aria_current>
            {children()}
        </a>
    }
}

variants! {
    SidenavMenuButton {
        base: "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-hidden ring-sidenav-ring transition-[width,height,padding] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground focus-visible:ring-2 active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-medium aria-[current=page]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent data-[state=open]:hover:text-sidenav-accent-foreground [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0   group-data-[collapsible=Icon]:size-8! group-data-[collapsible=Icon]:p-0!   [&>svg]:stroke-2 aria-[current=page]:[&>svg]:stroke-[2.7]",
        variants: {
            variant: {
                Default: "hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", // Already in base
                Outline: "bg-background shadow-[0_0_0_1px_hsl(var(--sidenav-border))] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground hover:shadow-[0_0_0_1px_hsl(var(--sidenav-accent))]",
            },
            size: {
                Default: "h-8 text-sm",
                Sm: "h-7 text-xs",
                Lg: "h-12",
            }
        },
        component: {
            element: button,
            support_href: true,
            support_aria_current: true
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, strum::IntoStaticStr)]
pub enum SidenavVariant {
    #[default]
    Sidenav,
    Floating,
    Inset,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SidenavSide {
    #[default]
    Left,
    Right,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum SidenavCollapsible {
    #[default]
    Offcanvas,
    None,
    Icon,
}

#[component]
pub fn Sidenav(
    #[prop(into, optional)] class: String,
    #[prop(default = SidenavVariant::default())] variant: SidenavVariant,
    #[prop(default = SidenavState::default())] data_state: SidenavState,
    #[prop(default = SidenavSide::default())] data_side: SidenavSide,
    #[prop(default = SidenavCollapsible::default())] data_collapsible: SidenavCollapsible,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<SidenavContext>();
    // Context signal takes priority; falls back to static prop (JS can still mutate data-state directly)
    let is_open: Signal<bool> =
        ctx.map(|c| c.open.into()).unwrap_or_else(|| Signal::derive(move || data_state == SidenavState::Expanded));

    view! {
        {if data_collapsible == SidenavCollapsible::None {
            view! {
                <aside
                    data-name="Sidenav"
                    class=tw_merge!(
                        "flex flex-col h-full bg-sidenav text-sidenav-foreground w-(--sidenav-width)", class
                    )
                >
                    {children()}
                </aside>
            }
                .into_any()
        } else {
            view! {
                <aside
                    data-name="Sidenav"
                    data-sidenav=data_state.to_string()
                    data-state=move || if is_open.get() { "Expanded" } else { "Collapsed" }
                    data-side=data_side.to_string()
                    class="hidden md:block group peer text-sidenav-foreground data-[state=Collapsed]:hidden"
                >
                    // * SidenavGap: This is what handles the sidenav gap on desktop
                    <div
                        data-name="SidenavGap"
                        class=tw_merge!(
                            "relative w-(--sidenav-width) bg-transparent transition-[width] duration-200 ease-linear",
                            "group-data-[collapsible=Offcanvas]:w-0",
                            "group-data-[side=Right]:rotate-180",
                            match variant {
                                SidenavVariant::Sidenav => "group-data-[collapsible=Icon]:w-(--sidenav-width-icon)",
                                SidenavVariant::Floating | SidenavVariant::Inset =>
                                    "group-data-[collapsible=Icon]:w-[calc(var(--sidenav-width-icon)+(--spacing(4)))]",
                            }
                        )
                    />
                    <div
                        data-name="SidenavContainer"
                        class=tw_merge!(
                            "fixed inset-y-0 z-10 hidden h-svh w-(--sidenav-width) transition-[left,right,width] duration-200 ease-linear md:flex",
                            class,
                            match data_side {
                                SidenavSide::Left => "left-0 group-data-[collapsible=Offcanvas]:left-[calc(var(--sidenav-width)*-1)]",
                                SidenavSide::Right => "right-0 group-data-[collapsible=Offcanvas]:right-[calc(var(--sidenav-width)*-1)]"
                            },
                            match variant {
                                 SidenavVariant::Sidenav => "group-data-[collapsible=Icon]:w-(--sidenav-width-icon) group-data-[side=Left]:border-r group-data-[side=Right]:border-l",
                                SidenavVariant::Floating | SidenavVariant::Inset =>
                                    "p-2 group-data-[collapsible=Icon]:w-[calc(var(--sidenav-width-icon)+(--spacing(4))+2px)]",
                            },
                        )
                    >
                        // * Act as a Sidenav for the onclick trigger to work with nested Sidenavs.
                        <SidenavInner attr:data-sidenav="Sidenav" attr:data-variant=variant.to_string()>
                            {children()}
                            <SidenavToggleRail />
                        </SidenavInner>
                    </div>
                </aside>
            }
                .into_any()
        }}
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SidenavState {
    #[default]
    Expanded,
    Collapsed,
}

const ONCLICK_TRIGGER: &str = "document.querySelector('[data-name=\"Sidenav\"]').setAttribute('data-state', document.querySelector('[data-name=\"Sidenav\"]').getAttribute('data-state') === 'Collapsed' ? 'Expanded' : 'Collapsed')";

#[component]
pub fn SidenavTrigger(children: Children) -> impl IntoView {
    let ctx = use_context::<SidenavContext>();

    view! {
        <button
            onclick=if ctx.is_none() { ONCLICK_TRIGGER } else { "" }
            on:click=move |_| {
                if let Some(c) = ctx {
                    c.toggle();
                }
            }
            data-name="SidenavTrigger"
            class="inline-flex gap-2 justify-center items-center -ml-1 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive size-7 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
        >
            {children()}
        </button>
    }
}

#[component]
fn SidenavToggleRail() -> impl IntoView {
    let ctx = use_context::<SidenavContext>();

    view! {
        <button
            data-name="SidenavToggleRail"
            aria-label="Toggle Sidenav"
            tabindex="-1"
            onclick=if ctx.is_none() { ONCLICK_TRIGGER } else { "" }
            on:click=move |_| {
                if let Some(c) = ctx {
                    c.toggle();
                }
            }
            class="hidden absolute inset-y-0 z-20 w-4 transition-all ease-linear -translate-x-1/2 sm:flex group-data-[side=Left]:-right-4 group-data-[side=Right]:left-0 after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] in-data-[side=Left]:cursor-w-resize in-data-[side=Right]:cursor-e-resize [[data-side=Left][data-state=Collapsed]_&]:cursor-e-resize [[data-side=Right][data-state=Collapsed]_&]:cursor-w-resize group-data-[collapsible=Offcanvas]:translate-x-0 group-data-[collapsible=Offcanvas]:after:left-full [[data-side=Left][data-collapsible=Offcanvas]_&]:-right-2 [[data-side=Right][data-collapsible=Offcanvas]_&]:-left-0eft-2 hover:after:bg-sidenav-border hover:group-data-[collapsible=Offcanvas]:bg-sidenav"
        />
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Link button for use inside `SidenavMenuSub` / `SidenavMenuSubItem`.
/// Renders an `<a>` with active-state detection via `aria-current`.
#[component]
pub fn SidenavMenuSubButton(
    children: Children,
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground active:bg-sidenav-accent active:text-sidenav-accent-foreground flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 text-sm outline-hidden focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-medium aria-[current=page]:text-sidenav-accent-foreground [&>svg]:size-4 [&>svg]:shrink-0",
        class
    );

    let location = use_location();
    let href_clone = href.clone();
    let is_active = move || {
        let path = location.pathname.get();
        path == href_clone || path.starts_with(&format!("{}/", href_clone))
    };
    let aria_current = move || if is_active() { "page" } else { "false" };

    view! {
        <a data-name="SidenavMenuSubButton" class=merged_class href=href aria-current=aria_current>
            {children()}
        </a>
    }
}

/// Action button rendered inside a `SidenavMenuItem`, positioned absolutely to the right.
/// Set `show_on_hover=true` to hide it until the menu item is hovered/focused.
#[component]
pub fn SidenavMenuAction(
    children: Children,
    #[prop(optional)] show_on_hover: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground peer-hover/menu-button:text-sidenav-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=Sm]/menu-button:top-1 peer-data-[size=Default]/menu-button:top-1.5 peer-data-[size=Lg]/menu-button:top-2.5 group-data-[collapsible=Icon]:hidden",
        if show_on_hover {
            "group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 peer-data-[active=true]/menu-button:text-sidenav-accent-foreground md:opacity-0"
        } else {
            ""
        },
        class
    );

    view! {
        <button data-sidenav="menu-action" data-name="SidenavMenuAction" class=merged_class>
            {children()}
        </button>
    }
}
