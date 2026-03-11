use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;
use leptos_ui::clx::IntoTailwindClass;
use strum::Display;
use tw_merge::*;

use crate::hooks::use_pagination::{PaginationContext, use_pagination};
use crate::ui::button::{ButtonClass, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {PaginationNav, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:opacity-0 [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:pointer-events-none"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}
}

pub use components::*;

/* ========================================================== */
/*                    ✨ COMPONENTS ✨                        */
/* ========================================================== */

#[component]
pub fn Pagination(children: Children, #[prop(default = false.into(), into)] scroll: Signal<bool>) -> impl IntoView {
    let ctx = use_pagination();
    provide_context(ctx);
    provide_context(scroll);

    view! { <PaginationNav>{children()}</PaginationNav> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum PageDirection {
    Previous,
    Next,
}

#[component]
pub fn PaginationNavButton(direction: PageDirection) -> impl IntoView {
    use leptos_router::NavigateOptions;
    use leptos_router::hooks::use_navigate;

    let ctx = expect_context::<PaginationContext>();
    let scroll = use_context::<Signal<bool>>().unwrap_or(Signal::from(false));
    let navigate = use_navigate();

    let (href, is_disabled, icon) = match direction {
        PageDirection::Previous => (ctx.prev_href, ctx.is_first_page, view! { <ChevronLeft /> }.into_any()),
        PageDirection::Next => (ctx.next_href, Signal::derive(|| false), view! { <ChevronRight /> }.into_any()),
    };

    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let url = href.get();
        if url != "#" {
            navigate(&url, NavigateOptions { scroll: scroll.get(), ..Default::default() });
        }
    };

    let button_class = ButtonClass { variant: ButtonVariant::Ghost, size: ButtonSize::Default };

    view! {
        <a
            href=href
            on:click=on_click
            class=button_class.with_class("")
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            aria-label=format!("Go to {} page", direction)
        >
            {icon}
        </a>
    }
}

#[component]
pub fn PaginationLink(page: u32, #[prop(into, optional)] class: String) -> impl IntoView {
    use leptos_router::NavigateOptions;
    use leptos_router::hooks::use_navigate;

    let ctx = expect_context::<PaginationContext>();
    let scroll = use_context::<Signal<bool>>().unwrap_or(Signal::from(false));
    let navigate = use_navigate();

    let href = Signal::derive(move || ctx.page_href.run(page));
    let aria_current = move || ctx.aria_current.run(page);

    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let url = href.get();
        navigate(&url, NavigateOptions { scroll: scroll.get(), ..Default::default() });
    };

    let button_class = ButtonClass { variant: ButtonVariant::Ghost, size: ButtonSize::Icon };

    let merged_class = button_class.with_class(tw_merge!(
        "aria-[current=page]:bg-primary aria-[current=page]:text-primary-foreground aria-[current=page]:hover:bg-primary/90",
        class
    ));

    view! {
        <a href=href on:click=on_click aria-current=aria_current class=merged_class>
            {page}
        </a>
    }
}

#[component]
pub fn PaginationEllipsis() -> impl IntoView {
    view! {
        <EllipsisRoot attr:aria-hidden="true">
            <Ellipsis class="size-4" />
            <span class="hidden">More pages</span>
        </EllipsisRoot>
    }
}
