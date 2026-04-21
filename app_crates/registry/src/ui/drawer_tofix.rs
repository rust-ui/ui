use std::time::Duration;

use leptos::{ev, html, leptos_dom::helpers::window_event_listener, prelude::*};
use leptos_ui::clx;
use tw_merge::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Element, HtmlElement, KeyboardEvent, PointerEvent};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};

const VELOCITY_THRESHOLD: f64 = 0.4;
const CLOSE_THRESHOLD: f64 = 0.25;
const TRANSITION_DURATION_MS: u64 = 500;
const TRANSITION_DURATION_MS_F64: f64 = 500.0;
const TRANSITION_EASING: &str = "cubic-bezier(0.32, 0.72, 0, 1)";
const WINDOW_TOP_OFFSET: f64 = 26.0;
const BORDER_RADIUS: f64 = 8.0;
const WRAPPER_TRANSLATE_Y: f64 = 14.0;
const SCROLL_LOCK_TIMEOUT_MS: u32 = 500;
const FOCUS_DELAY_MS: u64 = 100;

mod components {
    use super::*;
    clx! {DrawerBody, div, "flex flex-col gap-4 mx-auto max-w-[500px]"}
    clx! {DrawerTitle, h2, "text-lg leading-none font-semibold"}
    clx! {DrawerDescription, p, "text-sm text-muted-foreground"}
    clx! {DrawerHeader, div, "flex flex-col gap-2"}
    clx! {DrawerFooter, footer, "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"}
}

pub use components::*;

#[derive(Clone)]
pub struct DrawerContext {
    open: RwSignal<bool>,
    overlay_ref: NodeRef<html::Div>,
    content_ref: NodeRef<html::Div>,
    hidden: RwSignal<bool>,
    state: RwSignal<&'static str>,
    content_animate: RwSignal<bool>,
    overlay_animate: RwSignal<bool>,
    dismissible: RwSignal<bool>,
    lock_body_scroll: bool,
    previous_active_element: RwSignal<Option<Element>>,
    animation_epoch: RwSignal<u64>,
}

#[derive(Clone, Copy)]
struct DragState {
    is_dragging: RwSignal<bool>,
    current_pos: RwSignal<f64>,
    start_pos: RwSignal<f64>,
    drag_start_time: RwSignal<f64>,
    drawer_size: RwSignal<f64>,
}

impl DrawerContext {
    pub fn open(&self) {
        if !self.open.get_untracked() {
            self.open.set(true);
        }
    }

    pub fn close(&self) {
        if self.open.get_untracked() {
            self.open.set(false);
        }
    }
}

#[component]
pub fn DrawerTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = use_context::<DrawerContext>();

    view! {
        <Button
            data_name="DrawerTrigger"
            class=class
            variant=variant
            size=size
            on:click=move |_| {
                if let Some(ctx) = &ctx {
                    ctx.open();
                }
            }
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn DrawerClose(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = use_context::<DrawerContext>();

    view! {
        <Button
            data_name="DrawerClose"
            class=class
            variant=variant
            size=size
            on:click=move |_| {
                if let Some(ctx) = &ctx {
                    ctx.close();
                }
            }
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn Drawer(
    children: Children,
    #[prop(optional, default = true)] show_overlay: bool,
    #[prop(optional, default = true)] lock_body_scroll: bool,
) -> impl IntoView {
    let overlay_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<html::Div>::new();
    let hidden = RwSignal::new(true);
    let state = RwSignal::new("closed");
    let content_animate = RwSignal::new(true);
    let overlay_animate = RwSignal::new(true);
    let dismissible = RwSignal::new(true);
    let previous_active_element = RwSignal::new(None::<Element>);
    let animation_epoch = RwSignal::new(0_u64);
    let open = RwSignal::new(false);

    let ctx = DrawerContext {
        open,
        overlay_ref,
        content_ref,
        hidden,
        state,
        content_animate,
        overlay_animate,
        dismissible,
        lock_body_scroll,
        previous_active_element,
        animation_epoch,
    };
    provide_context(ctx.clone());

    let overlay_class = move || {
        let mut class = "fixed inset-0 z-200 bg-black/50".to_string();
        if hidden.get() || !show_overlay {
            class.push_str(" hidden");
        }
        class
    };

    view! {
        <div
            node_ref=overlay_ref
            data-name="DrawerOverlay"
            class=overlay_class
            data-vaul-overlay=""
            data-vaul-snap-points="false"
            data-vaul-animate=move || bool_attr(overlay_animate.get())
            data-state=move || state.get()
            data-lock-body-scroll=if lock_body_scroll { "true" } else { "false" }
            on:click=move |_| {
                if show_overlay && dismissible.get() {
                    ctx.close();
                }
            }
        ></div>

        {children()}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerPosition {
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum DrawerVariant {
    #[default]
    Inset,
    Floating,
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = DrawerPosition::default())] position: DrawerPosition,
    #[prop(optional, default = DrawerVariant::default())] variant: DrawerVariant,
    #[prop(into, default = "--initial-transform: 100%;".to_string())] style: String,
    #[prop(optional, default = true)] dismissible: bool,
) -> impl IntoView {
    let ctx = expect_context::<DrawerContext>();
    let is_dragging = RwSignal::new(false);
    let is_allowed_to_drag = RwSignal::new(false);
    let start_pos = RwSignal::new(0.0);
    let current_pos = RwSignal::new(0.0);
    let drawer_size = RwSignal::new(0.0);
    let drag_start_time = RwSignal::new(0.0);
    let open_time = RwSignal::new(None::<f64>);
    let last_time_drag_prevented = RwSignal::new(None::<f64>);
    let drag_state = DragState { is_dragging, current_pos, start_pos, drag_start_time, drawer_size };

    ctx.dismissible.set(dismissible);

    let watch_ctx = ctx.clone();
    Effect::watch(
        move || ctx.open.get(),
        move |is_open, previous, _| {
            if previous.is_none() {
                return;
            }

            if *is_open {
                open_drawer_dom(&watch_ctx, position, variant, open_time);
            } else {
                close_drawer_dom(&watch_ctx, position, variant);
            }
        },
        false,
    );

    let keydown_ctx = ctx.clone();
    Effect::new(move |_| {
        let keydown_ctx = keydown_ctx.clone();
        let handle = window_event_listener(ev::keydown, move |event: KeyboardEvent| {
            if !keydown_ctx.open.get_untracked() {
                return;
            }

            if event.key() == "Escape" {
                if keydown_ctx.dismissible.get_untracked() {
                    event.prevent_default();
                    keydown_ctx.close();
                }
                return;
            }

            if event.key() != "Tab" {
                return;
            }

            let Some(drawer) = content_element(&keydown_ctx) else { return };
            let focusable = focusable_elements(&drawer);
            if focusable.is_empty() {
                return;
            }

            let Some(document) = window().document() else { return };
            let first = focusable.first().cloned();
            let last = focusable.last().cloned();
            let active = document.active_element();

            if event.shift_key() {
                if active.as_ref() == first.as_ref().map(|el| el.as_ref()) {
                    event.prevent_default();
                    if let Some(last) = last {
                        let _ = last.focus();
                    }
                }
            } else if active.as_ref() == last.as_ref().map(|el| el.as_ref()) {
                event.prevent_default();
                if let Some(first) = first {
                    let _ = first.focus();
                }
            }
        });

        on_cleanup(move || handle.remove());
    });

    let handle_content_click = {
        let ctx = ctx.clone();
        move |event: web_sys::MouseEvent| {
            if !event_target_matches(event.target(), "[data-name=\"DrawerClose\"]") {
                return;
            }

            ctx.close();
        }
    };

    let handle_select_start = move |event: web_sys::Event| {
        if is_dragging.get() && is_allowed_to_drag.get() {
            event.prevent_default();
        }
    };

    let handle_pointer_down = {
        let ctx = ctx.clone();
        move |event: PointerEvent| {
            if !ctx.open.get() || !ctx.dismissible.get() {
                return;
            }

            if event_target_matches(event.target(), "[data-name=\"DrawerClose\"]") {
                return;
            }

            let Some(drawer) = content_element(&ctx) else { return };
            is_dragging.set(true);
            is_allowed_to_drag.set(false);

            let pointer_position = pointer_axis_value(&event, position);
            start_pos.set(pointer_position);
            current_pos.set(pointer_position);
            drag_start_time.set(now_ms());
            drawer_size.set(measure_drawer_size(&drawer, position));

            let _ = drawer.style().set_property("transition", "none");
            let _ = drawer.set_pointer_capture(event.pointer_id());
        }
    };

    let handle_pointer_move = {
        let ctx = ctx.clone();
        move |event: PointerEvent| {
            if !is_dragging.get() || !ctx.dismissible.get() {
                return;
            }

            let Some(drawer) = content_element(&ctx) else { return };
            let Some(overlay) = overlay_element(&ctx) else { return };

            current_pos.set(pointer_axis_value(&event, position));
            let delta = current_pos.get_untracked() - start_pos.get_untracked();
            let closing_direction = dragging_in_closing_direction(delta, position);

            if !is_allowed_to_drag.get()
                && !should_drag(event.target(), &drawer, open_time, last_time_drag_prevented)
            {
                return;
            }

            is_allowed_to_drag.set(true);

            if closing_direction {
                let abs_delta = delta.abs();
                let _ = drawer
                    .style()
                    .set_property("transform", &drag_transform(delta, closing_direction, position));

                if let Some(wrapper) = query_drawer_wrapper() {
                    let percentage_dragged = percentage_dragged(abs_delta, drawer_size.get_untracked());
                    let scale = get_scale() + percentage_dragged * (1.0 - get_scale());
                    let border_radius = (BORDER_RADIUS - percentage_dragged * BORDER_RADIUS).max(0.0);
                    let translate = (WRAPPER_TRANSLATE_Y - percentage_dragged * WRAPPER_TRANSLATE_Y).max(0.0);

                    let _ = wrapper.style().set_property("transition", "none");
                    let _ = wrapper.style().set_property("border-radius", &format!("{border_radius}px"));
                    let _ = wrapper
                        .style()
                        .set_property("transform", &format!("scale({scale}) translate3d(0, {translate}px, 0)"));
                }

                let opacity = (1.0 - percentage_dragged(abs_delta, drawer_size.get_untracked())).max(0.0);
                let _ = overlay.style().set_property("transition", "none");
                let _ = overlay.style().set_property("opacity", &opacity.to_string());
            } else {
                let _ = drawer
                    .style()
                    .set_property("transform", &drag_transform(delta, closing_direction, position));
            }
        }
    };

    let pointer_up_ctx = ctx.clone();
    let pointer_up_cancel_ctx = ctx.clone();

    let position_class = match position {
        DrawerPosition::Bottom => "right-0 bottom-0 left-0 max-h-[96vh] rounded-t-[10px]",
        DrawerPosition::Left => "left-0 top-0 bottom-0 max-w-[96vw] rounded-r-[10px]",
        DrawerPosition::Right => "right-0 top-0 bottom-0 max-w-[96vw] rounded-l-[10px]",
    };

    let merged_class = tw_merge!(
        "flex flex-col pt-3 pb-6 px-6 fixed z-210 bg-background hidden outline-none",
        position_class,
        class
    );

    let class_ctx = ctx.clone();
    let content_animate_ctx = ctx.clone();
    let dismissible_ctx = ctx.clone();
    let state_ctx = ctx.clone();

    view! {
        <div
            node_ref=ctx.content_ref
            data-name="DrawerContent"
            class=move || {
                let mut class = merged_class.clone();
                if !class_ctx.hidden.get() {
                    class = class.replace(" hidden", "");
                }
                class
            }
            tabindex="-1"
            data-vaul-drawer=""
            data-vaul-drawer-position=position.to_string()
            data-vaul-variant=variant.to_string()
            data-vaul-snap-points="false"
            data-vaul-animate=move || bool_attr(content_animate_ctx.content_animate.get())
            data-vaul-dismissible=move || bool_attr(dismissible_ctx.dismissible.get())
            data-state=move || state_ctx.state.get()
            style=style
            on:click=handle_content_click
            on:selectstart=handle_select_start
            on:pointerdown=handle_pointer_down
            on:pointermove=handle_pointer_move
            on:pointerup=move |event| {
                finish_pointer_drag(&pointer_up_ctx, event, position, drag_state);
            }
            on:pointercancel=move |event| {
                finish_pointer_drag(&pointer_up_cancel_ctx, event, position, drag_state);
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerHandle() -> impl IntoView {
    view! {
        <div
            class="block relative mx-auto mb-8 w-8 rounded-2xl opacity-70 hover:opacity-100 active:opacity-100 shrink-0 bg-[#e2e2e4] h-[5px]"
            data-vaul-handle=""
        >
            <span data-vaul-handle-hitarea=""></span>
        </div>
    }
}

fn bool_attr(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn now_ms() -> f64 {
    js_sys::Date::now()
}

fn content_element(ctx: &DrawerContext) -> Option<HtmlElement> {
    ctx.content_ref.get_untracked().map(|element| element.unchecked_into::<HtmlElement>())
}

fn overlay_element(ctx: &DrawerContext) -> Option<HtmlElement> {
    ctx.overlay_ref.get_untracked().map(|element| element.unchecked_into::<HtmlElement>())
}

fn query_drawer_wrapper() -> Option<HtmlElement> {
    let document = window().document()?;
    let element = document.query_selector("[data-vaul-drawer-wrapper]").ok()??;
    element.dyn_into::<HtmlElement>().ok()
}

fn measure_drawer_size(drawer: &HtmlElement, position: DrawerPosition) -> f64 {
    let rect = drawer.get_bounding_client_rect();
    if is_horizontal(position) { rect.width() } else { rect.height() }
}

fn is_horizontal(position: DrawerPosition) -> bool {
    matches!(position, DrawerPosition::Left | DrawerPosition::Right)
}

fn pointer_axis_value(event: &PointerEvent, position: DrawerPosition) -> f64 {
    if is_horizontal(position) { f64::from(event.page_x()) } else { f64::from(event.page_y()) }
}

fn dragging_in_closing_direction(delta: f64, position: DrawerPosition) -> bool {
    match position {
        DrawerPosition::Bottom | DrawerPosition::Right => delta > 0.0,
        DrawerPosition::Left => delta < 0.0,
    }
}

fn percentage_dragged(delta: f64, drawer_size: f64) -> f64 {
    if drawer_size <= 0.0 { 0.0 } else { (delta / drawer_size).clamp(0.0, 1.0) }
}

fn get_scale() -> f64 {
    let Some(inner_width) = window().inner_width().ok().and_then(|value| value.as_f64()) else {
        return 1.0;
    };
    (inner_width - WINDOW_TOP_OFFSET) / inner_width
}

fn dampen_value(value: f64) -> f64 {
    8.0 * ((value + 1.0).ln() - 2.0)
}

fn drag_transform(delta: f64, closing_direction: bool, position: DrawerPosition) -> String {
    if closing_direction {
        return if is_horizontal(position) {
            format!("translate3d({delta}px, 0, 0)")
        } else {
            format!("translate3d(0, {delta}px, 0)")
        };
    }

    let damped = dampen_value(delta.abs());
    match position {
        DrawerPosition::Bottom => format!("translate3d(0, {}px, 0)", -damped),
        DrawerPosition::Left | DrawerPosition::Right => {
            let signed = if delta > 0.0 { damped } else { -damped };
            format!("translate3d({signed}px, 0, 0)")
        }
    }
}

fn should_close_from_drag(delta: f64, velocity: f64, drawer_size: f64, position: DrawerPosition) -> bool {
    match position {
        DrawerPosition::Bottom | DrawerPosition::Right => {
            (velocity > VELOCITY_THRESHOLD || delta / drawer_size >= CLOSE_THRESHOLD) && delta > 0.0
        }
        DrawerPosition::Left => {
            (velocity > VELOCITY_THRESHOLD || delta.abs() / drawer_size >= CLOSE_THRESHOLD) && delta < 0.0
        }
    }
}

fn finish_pointer_drag(
    ctx: &DrawerContext,
    event: PointerEvent,
    position: DrawerPosition,
    drag_state: DragState,
) {
    if !drag_state.is_dragging.get() || !ctx.dismissible.get() {
        return;
    }

    drag_state.is_dragging.set(false);

    let Some(drawer) = content_element(ctx) else { return };
    let overlay = overlay_element(ctx);
    let delta = drag_state.current_pos.get_untracked() - drag_state.start_pos.get_untracked();
    let elapsed = (now_ms() - drag_state.drag_start_time.get_untracked()).max(1.0);
    let velocity = delta.abs() / elapsed;

    let _ = drawer
        .style()
        .set_property("transition", &format!("transform 0.5s {TRANSITION_EASING}"));

    if let Some(wrapper) = query_drawer_wrapper() {
        let _ = wrapper.style().set_property(
            "transition",
            &format!("transform 0.5s {TRANSITION_EASING}, border-radius 0.5s {TRANSITION_EASING}"),
        );
    }

    if let Some(overlay) = &overlay {
        let _ = overlay
            .style()
            .set_property("transition", &format!("opacity 0.5s {TRANSITION_EASING}"));
    }

    let size = drag_state.drawer_size.get_untracked().max(1.0);
    if should_close_from_drag(delta, velocity, size, position) {
        ctx.close();
    } else {
        let _ = drawer.style().set_property("transform", "translate3d(0, 0, 0)");

        if let Some(wrapper) = query_drawer_wrapper() {
            let scale = get_scale();
            let _ = wrapper.style().set_property("border-radius", &format!("{BORDER_RADIUS}px"));
            let _ = wrapper
                .style()
                .set_property("transform", &format!("scale({scale}) translate3d(0, {WRAPPER_TRANSLATE_Y}px, 0)"));
        }

        if let Some(overlay) = overlay {
            let _ = overlay.style().set_property("opacity", "1");
        }
    }

    if drawer.has_pointer_capture(event.pointer_id()) {
        let _ = drawer.release_pointer_capture(event.pointer_id());
    }
}

fn event_target_matches(target: Option<web_sys::EventTarget>, selector: &str) -> bool {
    target
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|element| element.closest(selector).ok().flatten())
        .is_some()
}

fn should_drag(
    target: Option<web_sys::EventTarget>,
    drawer: &HtmlElement,
    open_time: RwSignal<Option<f64>>,
    last_time_drag_prevented: RwSignal<Option<f64>>,
) -> bool {
    let current_time = now_ms();

    if let Some(opened_at) = open_time.get_untracked()
        && current_time - opened_at < TRANSITION_DURATION_MS_F64
    {
        return false;
    }

    if let Some(last_prevented) = last_time_drag_prevented.get_untracked()
        && current_time - last_prevented < f64::from(SCROLL_LOCK_TIMEOUT_MS)
    {
        last_time_drag_prevented.set(Some(current_time));
        return false;
    }

    let mut current = target.and_then(|target| target.dyn_into::<Element>().ok());
    while let Some(element) = current {
        if element.is_same_node(Some(drawer.as_ref())) {
            break;
        }

        if let Ok(html_element) = element.clone().dyn_into::<HtmlElement>()
            && html_element.scroll_height() > html_element.client_height()
        {
            if html_element.scroll_top() != 0 {
                last_time_drag_prevented.set(Some(current_time));
                return false;
            }

            if html_element.get_attribute("role").as_deref() == Some("dialog") {
                return true;
            }
        }

        current = element.parent_element();
    }

    true
}

fn focusable_elements(drawer: &HtmlElement) -> Vec<HtmlElement> {
    const FOCUSABLE_SELECTOR: &str =
        "a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex=\"-1\"])";

    let Ok(elements) = drawer.query_selector_all(FOCUSABLE_SELECTOR) else {
        return Vec::new();
    };

    let mut focusable = Vec::new();
    for index in 0..elements.length() {
        let Some(element) = elements.item(index) else { continue };
        let Ok(html_element) = element.dyn_into::<HtmlElement>() else { continue };
        if html_element.offset_parent().is_some() {
            focusable.push(html_element);
        }
    }

    focusable
}

fn focus_first_drawer_element(ctx: &DrawerContext) {
    let ctx = ctx.clone();
    set_timeout(
        move || {
            if !ctx.open.get_untracked() {
                return;
            }

            let Some(drawer) = content_element(&ctx) else { return };
            let focusable = focusable_elements(&drawer);
            if focusable.is_empty() {
                let _ = drawer.focus();
                return;
            }

            let is_only_close_button = focusable.len() == 1
                && focusable
                    .first()
                    .and_then(|element| element.get_attribute("data-name"))
                    .as_deref()
                    == Some("DrawerClose");

            if is_only_close_button {
                let _ = drawer.focus();
            } else if let Some(first) = focusable.first() {
                let _ = first.focus();
            }
        },
        Duration::from_millis(FOCUS_DELAY_MS),
    );
}

fn fix_drawer_position(drawer: &HtmlElement) {
    let Some(document) = window().document() else { return };
    let viewport_height = window().inner_height().ok().and_then(|value| value.as_f64()).unwrap_or_default();
    let scroll_height = document.document_element().map(|element| element.scroll_height()).unwrap_or_default();
    if f64::from(scroll_height) <= viewport_height {
        return;
    }

    let rect = drawer.get_bounding_client_rect();
    let offset = viewport_height - rect.bottom();
    if offset.abs() <= f64::EPSILON {
        return;
    }

    let _ = drawer.style().set_property("top", &format!("{}px", rect.top() + offset));
}

fn lock_body_scroll() {
    let Some(document) = window().document() else { return };
    let Some(body) = document.body() else { return };
    let scrollbar_width =
        window().inner_width().ok().and_then(|width| width.as_f64()).unwrap_or_default() - f64::from(body.client_width());

    let _ = body.set_attribute("data-state", "open");
    if scrollbar_width > 0.0 {
        let _ = body.style().set_property("padding-right", &format!("{scrollbar_width}px"));
    }
}

fn unlock_body_scroll() {
    let Some(document) = window().document() else { return };
    let Some(body) = document.body() else { return };
    let _ = body.remove_attribute("data-state");
    let _ = body.style().remove_property("padding-right");
}

fn apply_open_wrapper_styles() {
    let Some(wrapper) = query_drawer_wrapper() else { return };
    let scale = get_scale();

    let _ = wrapper.style().set_property("transform-origin", "top");
    let _ = wrapper.style().set_property("transition-property", "transform, border-radius");
    let _ = wrapper.style().set_property("transition-duration", "0.5s");
    let _ = wrapper.style().set_property("transition-timing-function", TRANSITION_EASING);
    let _ = wrapper.style().set_property("border-radius", &format!("{BORDER_RADIUS}px"));
    let _ = wrapper.style().set_property("overflow", "hidden");
    let _ = wrapper.style().set_property(
        "transform",
        &format!("scale({scale}) translate3d(0, calc(env(safe-area-inset-top) + {WRAPPER_TRANSLATE_Y}px), 0)"),
    );

    if let Some(body) = window().document().and_then(|document| document.body()) {
        let _ = body.style().set_property("background", "black");
    }
}

fn reset_wrapper_styles() {
    let Some(wrapper) = query_drawer_wrapper() else { return };

    let _ = wrapper.style().set_property(
        "transition",
        &format!("transform 0.5s {TRANSITION_EASING}, border-radius 0.5s {TRANSITION_EASING}"),
    );
    let _ = wrapper.style().set_property("transform", "scale(1) translate3d(0, 0, 0)");
    let _ = wrapper.style().set_property("border-radius", "0px");
}

fn clear_wrapper_styles() {
    let Some(wrapper) = query_drawer_wrapper() else { return };
    for property in [
        "transform-origin",
        "transition-property",
        "transition-duration",
        "transition-timing-function",
        "transition",
        "border-radius",
        "overflow",
        "transform",
    ] {
        let _ = wrapper.style().remove_property(property);
    }

    if let Some(body) = window().document().and_then(|document| document.body()) {
        let _ = body.style().remove_property("background");
    }
}

fn close_transform(drawer_size: f64, position: DrawerPosition, variant: DrawerVariant) -> String {
    match position {
        DrawerPosition::Bottom => format!("translate3d(0, {drawer_size}px, 0)"),
        DrawerPosition::Left => {
            let distance = if variant == DrawerVariant::Floating { drawer_size + 8.0 } else { drawer_size };
            format!("translate3d(-{distance}px, 0, 0)")
        }
        DrawerPosition::Right => {
            let distance = if variant == DrawerVariant::Floating { drawer_size + 8.0 } else { drawer_size };
            format!("translate3d({distance}px, 0, 0)")
        }
    }
}

fn next_epoch(ctx: &DrawerContext) -> u64 {
    let next = ctx.animation_epoch.get_untracked().saturating_add(1);
    ctx.animation_epoch.set(next);
    next
}

fn open_drawer_dom(
    ctx: &DrawerContext,
    _position: DrawerPosition,
    variant: DrawerVariant,
    open_time: RwSignal<Option<f64>>,
) {
    let epoch = next_epoch(ctx);
    let Some(document) = window().document() else { return };
    let Some(drawer) = content_element(ctx) else { return };
    let Some(overlay) = overlay_element(ctx) else { return };

    ctx.hidden.set(false);
    ctx.state.set("closed");
    ctx.content_animate.set(true);
    ctx.overlay_animate.set(true);
    ctx.previous_active_element.set(document.active_element());

    if variant == DrawerVariant::Floating {
        let _ = overlay.style().set_property("opacity", "1");
    }

    if ctx.lock_body_scroll {
        lock_body_scroll();
        fix_drawer_position(&drawer);
    }

    apply_open_wrapper_styles();

    let ctx = ctx.clone();
    let callback = Closure::once(move || {
        if ctx.animation_epoch.get_untracked() != epoch || !ctx.open.get_untracked() {
            return;
        }

        if variant == DrawerVariant::Floating {
            ctx.overlay_animate.set(false);
        }

        ctx.state.set("open");
        open_time.set(Some(now_ms()));
        focus_first_drawer_element(&ctx);
    });

    let _ = window().request_animation_frame(callback.as_ref().unchecked_ref());
    callback.forget();
}

fn close_drawer_dom(ctx: &DrawerContext, position: DrawerPosition, variant: DrawerVariant) {
    let epoch = next_epoch(ctx);
    let Some(drawer) = content_element(ctx) else { return };
    let Some(overlay) = overlay_element(ctx) else { return };
    let size = measure_drawer_size(&drawer, position);

    ctx.content_animate.set(false);
    ctx.overlay_animate.set(false);

    let _ = drawer
        .style()
        .set_property("transition", &format!("transform 0.5s {TRANSITION_EASING}"));
    let _ = drawer.style().set_property("transform", &close_transform(size, position, variant));

    let _ = overlay
        .style()
        .set_property("transition", &format!("opacity 0.5s {TRANSITION_EASING}"));
    let _ = overlay.style().set_property("opacity", "0");

    reset_wrapper_styles();
    ctx.state.set("closed");

    if ctx.lock_body_scroll {
        unlock_body_scroll();
    }

    let ctx = ctx.clone();
    set_timeout(
        move || {
            if ctx.animation_epoch.get_untracked() != epoch || ctx.open.get_untracked() {
                return;
            }

            let Some(drawer) = content_element(&ctx) else { return };
            let Some(overlay) = overlay_element(&ctx) else { return };

            ctx.hidden.set(true);
            let _ = drawer.style().remove_property("transform");
            let _ = drawer.style().remove_property("transition");
            let _ = drawer.style().remove_property("top");

            let _ = overlay.style().remove_property("opacity");
            let _ = overlay.style().remove_property("transition");

            ctx.content_animate.set(true);
            ctx.overlay_animate.set(true);
            clear_wrapper_styles();

            if let Some(previous) = ctx.previous_active_element.get_untracked()
                && let Ok(html_element) = previous.dyn_into::<HtmlElement>()
            {
                let _ = html_element.focus();
            }
            ctx.previous_active_element.set(None);
        },
        Duration::from_millis(TRANSITION_DURATION_MS),
    );
}
