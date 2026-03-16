use app_routes::{BlockRoutes, ComponentsRoutes, HooksRoutes};
use icons::{
    AlignHorizontalSpaceAround, BlocksAnimate, CalendarDaysAnimate, CompassAnimate, FrameAnimate, LogIn, Menu,
    PanelLeftOpenAnimate, SearchAnimate, WindAnimate, X,
};
use leptos::prelude::*;
use registry::demos::demo_accordion_icons::DemoAccordionIcons;
use registry::ui::header::{
    Header, IconWrapper, InsetCard, NavMenu, NavMenuContent, NavMenuContentInset, NavMenuFixed, NavMenuHomeLink,
    NavMenuItem, NavMenuLink, NavMenuLinkDescription, NavMenuLinkGrid, NavMenuLinkTitle, NavMenuList, NavMenuMiddle,
    NavMenuTitle, NavMenuTrigger, NavMenuWrapper,
};
use registry::ui::theme_toggle::ThemeToggle;

use crate::components::command_search_docs::CommandSearchDocs;

#[component]
pub fn AppHeader() -> impl IntoView {
    let is_mobile_menu_open_signal = RwSignal::new(false);
    let data_state = move || if is_mobile_menu_open_signal.get() { "active" } else { "inactive" };

    view! {
        <Header attr:data-state=data_state>
            <NavMenuFixed>
                <NavMenuWrapper>
                    <div class="flex relative flex-wrap justify-between items-center lg:py-3">
                        <div class="flex gap-8 justify-between items-center max-md:in-data-[state=active]:border-b max-md:h-14 max-md:w-full">
                            <NavMenuHomeLink attr:aria-label="home" attr:href="/">
                                <span class="text-lg font-semibold">"Rust/UI"</span>
                            </NavMenuHomeLink>
                            // ------- MOBILE --------- //
                            <MobileMenuTrigger is_open=is_mobile_menu_open_signal />
                        </div>

                        <NavMenuMiddle>
                            <NavMenu>
                                <NavMenuList attr:data-orientation="horizontal" attr:dir="ltr">
                                    <FirstNavMenu />
                                    <SecondNavMenu />

                                    <NavMenuItem>
                                        <NavMenuLink attr:href="/icons">Icons</NavMenuLink>
                                    </NavMenuItem>
                                    <NavMenuItem>
                                        <NavMenuLink attr:href="/create">Create</NavMenuLink>
                                    </NavMenuItem>
                                </NavMenuList>
                            </NavMenu>
                        </NavMenuMiddle>

                        // Right
                        <NavMenuRight />
                    </div>

                    // ------- MOBILE MENU --------- //
                    <MobileMenu is_open=is_mobile_menu_open_signal />
                </NavMenuWrapper>
            </NavMenuFixed>
        </Header>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn FirstNavMenu() -> impl IntoView {
    view! {
        <NavMenuItem>
            <NavMenuTrigger attr:href=ComponentsRoutes::base_url()>"Registry"</NavMenuTrigger>
            <NavMenuContent class="min-w-lg">
                <NavMenuContentInset class="grid gap-2 grid-cols-[auto_1fr]">
                    <InsetCard>
                        <NavMenuTitle>Content</NavMenuTitle>
                        <ul class="mt-1 space-y-2">
                            <li>
                                <NavMenuLinkGrid attr:href=ComponentsRoutes::base_url()>
                                    <IconWrapper>
                                        <BlocksAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Components</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>Find any components</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid attr:href="/icons">
                                    <IconWrapper>
                                        <SearchAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Icons</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>More than 1600 icons</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid attr:href=HooksRoutes::base_url()>
                                    <IconWrapper>
                                        <CompassAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Hooks</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>Reusable logic functions</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                        </ul>
                    </InsetCard>
                    <div class="p-2">
                        <NavMenuTitle>Get Started</NavMenuTitle>
                        <ul class="mt-1">
                            <li>
                                <NavMenuLinkGrid class="items-center" attr:href="/docs/components/introduction">
                                    <IconWrapper>
                                        <FrameAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <NavMenuLinkTitle>Introduction</NavMenuLinkTitle>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid class="items-center" attr:href="/docs/components/installation">
                                    <IconWrapper>
                                        <WindAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <NavMenuLinkTitle>Installation</NavMenuLinkTitle>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid class="items-center" attr:href="docs/components/changelog">
                                    <IconWrapper>
                                        <CalendarDaysAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <NavMenuLinkTitle>Changelog</NavMenuLinkTitle>
                                </NavMenuLinkGrid>
                            </li>
                        </ul>
                    </div>
                </NavMenuContentInset>
            </NavMenuContent>
        </NavMenuItem>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn SecondNavMenu() -> impl IntoView {
    view! {
        <NavMenuItem>
            <NavMenuTrigger attr:href=BlockRoutes::Login.to_route()>"Blocks"</NavMenuTrigger>

            <NavMenuContent class="w-[300px]">
                <NavMenuContentInset>
                    <InsetCard>
                        <h3 class="px-2 mb-4 text-sm font-semibold text-foreground">Demonstrations</h3>
                        <ul class="space-y-2">
                            <li>
                                <NavMenuLinkGrid attr:href=BlockRoutes::Login.to_route()>
                                    <IconWrapper>
                                        <LogIn class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Login</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>Login Blocks</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid attr:href=BlockRoutes::Sidenav.to_route()>
                                    <IconWrapper>
                                        <PanelLeftOpenAnimate class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Sidenav</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>Sidenav Blocks</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                            <li>
                                <NavMenuLinkGrid attr:href=BlockRoutes::Footers.to_route()>
                                    <IconWrapper>
                                        <AlignHorizontalSpaceAround class="text-foreground" />
                                    </IconWrapper>
                                    <div class="space-y-0.5">
                                        <NavMenuLinkTitle>Footers</NavMenuLinkTitle>
                                        <NavMenuLinkDescription>Footers Blocks</NavMenuLinkDescription>
                                    </div>
                                </NavMenuLinkGrid>
                            </li>
                        </ul>
                    </InsetCard>
                    <div class="p-2 w-full">
                        <NavMenuLink
                            class="items-start font-normal underline text-primary"
                            attr:href=BlockRoutes::Login.to_route()
                        >
                            Browse all Blocks
                        </NavMenuLink>
                    </div>
                </NavMenuContentInset>
            </NavMenuContent>
        </NavMenuItem>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn NavMenuRight() -> impl IntoView {
    view! {
        <div class="hidden md:flex md:gap-6 md:p-0 md:m-0 md:bg-transparent md:border-transparent md:shadow-none dark:shadow-none md:w-fit dark:md:bg-transparent">
            <div class="flex flex-row gap-3">
                <CommandSearchDocs />
                <div class="flex justify-center items-center mr-1">
                    <ThemeToggle />
                </div>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MobileMenuTrigger(is_open: RwSignal<bool>) -> impl IntoView {
    let toggle_menu = move |_| {
        is_open.update(|open| *open = !*open);
    };

    view! {
        <button
            aria-label="Open Menu"
            class="block relative z-20 p-2.5 -m-2.5 -mr-3 cursor-pointer md:hidden"
            on:click=toggle_menu
        >
            <Menu class="m-auto duration-200 lucide lucide-menu in-data-[state=active]:rotate-180 in-data-[state=active]:scale-0 in-data-[state=active]:opacity-0 size-5" />
            <X class="absolute inset-0 m-auto opacity-0 duration-200 scale-0 -rotate-180 lucide lucide-x in-data-[state=active]:rotate-0 in-data-[state=active]:scale-100 in-data-[state=active]:opacity-100 size-5" />

        </button>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MobileMenu(is_open: RwSignal<bool>) -> impl IntoView {
    let display_class = move || {
        if is_open.get() { "flex" } else { "hidden" }
    };

    view! {
        <div class=move || format!("flex-col gap-4 mt-6 w-full md:hidden mb-4 {}", display_class())>
            <DemoAccordionIcons />
            <div class="flex flex-row gap-3 w-full">
                <CommandSearchDocs />
                <div class="flex justify-center items-center mr-1">
                    <ThemeToggle />
                </div>
            </div>
        </div>
    }
}
