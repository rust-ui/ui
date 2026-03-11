use icons::ChevronDown;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

use crate::hooks::use_data_scrolled::use_data_scrolled;

mod components {
    use super::*;
    clx! {NavMenuFixed, div, "fixed inset-x-0 top-0 z-50 pt-[calc(0.5rem+env(safe-area-inset-top))] md:pt-[calc(0.75rem+env(safe-area-inset-top))] max-md:in-data-[state=active]:bg-background/75 max-md:in-data-[state=active]:h-screen max-md:in-data-[state=active]:backdrop-blur max-md:h-18 max-md:overflow-hidden max-md:px-2"}
    clx! {NavMenuWrapper, nav, "px-3 in-data-scrolled:ring-foreground/5 in-data-scrolled:bg-background in-data-scrolled:shadow-black/10 in-data-scrolled:max-w-4xl max-md:in-data-scrolled:px-5 in-data-scrolled:backdrop-blur mx-auto w-full max-w-6xl rounded-2xl   transition-[padding,background-color,box-shadow,max-width,backdrop-filter] duration-500 ease-in-out max-md:in-data-[state=active]:backdrop-blur max-md:in-data-[state=active]:ring-foreground/5 max-md:in-data-[state=active]:bg-background/75 max-md:in-data-[state=active]:px-5 max-md:in-data-[state=active]:shadow-black/10  in-data-scrolled:shadow-lg in-data-scrolled:border"}
    clx! {NavMenuLink, a, "inline-flex flex-col gap-1 w-full items-center justify-center p-2 py-1 px-4 h-8 text-sm font-medium rounded-md outline-none disabled:opacity-50 disabled:pointer-events-none data-[active=true]:focus:bg-muted data-[active=true]:hover:bg-accent data-[active=true]:bg-muted/50 data-[active=true]:text-foreground [>_svg:not([class*='text-'])]:text-muted-foreground [>_svg:not([class*='size-'])]:size-4 group text-muted-foreground data-[state=open]:hover:bg-foreground/5 data-[state=open]:text-foreground data-[state=open]:focus:bg-foreground/5 data-[state=open]:bg-foreground/5 transition-[color,box-shadow] hover:bg-foreground/5 hover:text-foreground focus:bg-foreground/5 focus:text-foreground focus-visible:ring-ring/50 focus-visible:ring-[3px] focus-visible:outline-1"}

    clx! {NavMenuHomeLink, a, "transition-all duration-500 h-fit md:in-data-scrolled:px-2 flex gap-2"}
    clx! {NavMenuMiddle, div, "absolute inset-0 m-auto size-fit"}
    clx! {NavMenuList, menu, "flex flex-1 gap-0 justify-center items-center list-none group"}
    clx! {NavMenuItem, li, "relative group/dropdown"}

    clx! {NavMenuTitle, span, "ml-2 text-xs text-muted-foreground"}
    clx! {NavMenuLinkGrid, a, "grid gap-3.5 p-2 text-sm rounded-md transition-all outline-none grid-cols-[auto_1fr] hover:bg-accent hover:text-foreground focus:bg-muted focus:text-foreground focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:ring-[3px]"}
    clx! {NavMenuLinkTitle, span, "text-sm font-medium text-foreground"}
    clx! {NavMenuLinkDescription, p, "text-xs text-muted-foreground line-clamp-1"}

    // TODO. We should import AnimatedIconWrapper from icons crate when ready.
    clx! {IconWrapper, div, "flex relative justify-center items-center rounded border border-transparent ring-1 shadow-sm bg-background ring-foreground/10 size-9 [&_svg:not([class*='size-'])]:size-4"}

    clx! {NavMenuContentInset, div, "p-0.5 rounded-xl border shadow-lg bg-popover backdrop-blur-md border-border/50"}
    clx! {InsetCard, div, "p-2 rounded-xl border shadow bg-background ring-foreground/5 border-border"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn NavMenuContent(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "absolute left-1/2 -translate-x-1/2 top-full z-[100] opacity-0 invisible pointer-events-none scale-95 origin-center transition-all duration-150 ease-out group-hover/dropdown:opacity-100 group-hover/dropdown:visible group-hover/dropdown:pointer-events-auto group-hover/dropdown:scale-100 group-hover/dropdown:delay-200 group-focus-within/dropdown:opacity-100 group-focus-within/dropdown:visible group-focus-within/dropdown:pointer-events-auto group-focus-within/dropdown:scale-100 group-focus-within/dropdown:delay-200 focus-within:opacity-100 focus-within:visible focus-within:pointer-events-auto focus-within:scale-100 focus-within:delay-200",
        class
    );

    view! {
        <div data-name="NavMenuContent" class=merged_class>
            <div data-name="NavMenuGap" class="h-2 in-data-scrolled:h-5" />

            {children()}
        </div>
    }
}

#[component]
pub fn Header(children: Children) -> impl IntoView {
    const SCROLL_THRESHOLD_PX: u32 = 20;
    let is_data_scrolled = use_data_scrolled(SCROLL_THRESHOLD_PX);

    view! {
        <style>
            r#"
            :root {
            --slide-offset: 120px;
            }
            
            /* Hide inactive dropdowns when switching */
            /* Slide the immediately next sibling to the right */
            [data-name="NavMenuItem"]:hover + [data-name="NavMenuItem"] > div[data-name="NavMenuContent"]:not(:hover) {
            transform: translateX(var(--slide-offset));
            opacity: 0;
            visibility: hidden;
            pointer-events: none;
            }
            
            /* Slide the immediately previous sibling to the left */
            [data-name="NavMenuItem"]:has(+ [data-name="NavMenuItem"]:hover) > div[data-name="NavMenuContent"]:not(:hover) {
            transform: translateX(calc(var(--slide-offset) * -1));
            opacity: 0;
            visibility: hidden;
            pointer-events: none;
            
            /* Hide dropdown immediately when any link inside is active or focused */
            [data-name="NavMenuItem"]:has(a:active) > div[data-name="NavMenuContent"],
            [data-name="NavMenuItem"]:has(a:focus:active) > div[data-name="NavMenuContent"] {
            opacity: 0 !important;
            visibility: hidden !important;
            transition: none !important;
            }
            "#
        </style>

        <header
            data-name="Header"
            class="[--color-popover:color-mix(in_oklch,var(--color-muted)_25%,var(--color-background))]"
            data-scrolled=move || if is_data_scrolled.get() { "true" } else { "false" }
        >
            {children()}
        </header>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn NavMenu(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex relative flex-1 justify-center items-center max-w-max group/navigation-menu **:data-[slot=navigation-menu-content]:top-12 max-md:hidden",
        class
    );

    view! {
        <div
            data-name="NavMenu"
            class=merged_class
            aria-label="Main"
            data-orientation="horizontal"
            dir="ltr"
            data-viewport="false"
            role="navigation"
        >
            <div class="relative">{children()}</div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn NavMenuTrigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let merged_class = tw_merge!(
        "nav__dropdown__trigger",
        "inline-flex justify-center items-center py-1 px-4 w-max h-8 text-sm font-medium rounded-md outline-none disabled:opacity-50 disabled:pointer-events-none text-muted-foreground data-[state=open]:hover:bg-foreground/5 data-[state=open]:text-foreground data-[state=open]:focus:bg-foreground/5 data-[state=open]:bg-foreground/5 transition-[color,box-shadow] hover:bg-foreground/5 hover:text-foreground focus:bg-foreground/5 focus:text-foreground focus-visible:ring-[3px] focus-visible:outline-1  ",
        class
    );

    view! {
        <a
            class=merged_class
            data-name="NavMenuTrigger"
            data-state="closed"
            aria-expanded="false"
            aria-controls="radix-_R_16inpfiv3b_-content-radix-_R_1d6inpfiv3b_"
        >
            <span>{children()}</span>
            <ChevronDown class="relative ml-1.5 opacity-75 transition duration-300 lucide lucide-chevron-down top-[1px] size-3 group-hover/dropdown:rotate-180 group-hover/dropdown:translate-y-px group-focus-within/dropdown:rotate-180 group-focus-within/dropdown:translate-y-px" />
        </a>
    }
}
