use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
use leptos::prelude::*;
use tw_merge::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

const MAX_TOASTS: usize = 5;
const VISIBLE_TOASTS_AMOUNT: usize = 3;
const ENTER_DURATION_MS: u32 = 300;
const EXIT_DURATION_MS: u32 = 300;
const SWIPE_THRESHOLD: f64 = 45.0;
const SWIPE_VELOCITY_THRESHOLD: f64 = 0.11;
const DEFAULT_DURATION_MS: u32 = 5000;

const SONNER_STYLE: &str = r#"
[data-name='SonnerList'] {
  position: fixed;
  z-index: 9999;
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--gap);
  opacity: 1;
  height: 200px;
  width: 400px;
}

[data-name='SonnerList'][data-direction='BottomUp'] {
  --fold-multiplier: -1;
}

[data-name='SonnerList'][data-direction='TopDown'] {
  --fold-multiplier: 1;
}

[data-name='SonnerList'][data-position='TopLeft'] {
  top: 0.75rem;
  left: 0.75rem;
}

[data-name='SonnerList'][data-position='TopCenter'] {
  top: 0.75rem;
  left: 50%;
  transform: translateX(-50%);
}

[data-name='SonnerList'][data-position='TopRight'] {
  top: 0.75rem;
  right: 0.75rem;
}

[data-name='SonnerList'][data-position='BottomLeft'] {
  bottom: 0.75rem;
  left: 0.75rem;
}

[data-name='SonnerList'][data-position='BottomCenter'] {
  bottom: 0.75rem;
  left: 50%;
  transform: translateX(-50%);
}

[data-name='SonnerList'][data-position='BottomRight'] {
  bottom: 0.75rem;
  right: 0.75rem;
}

[data-name='SonnerItem'] {
  --y: translateY(0);
  transform: var(--y);
  transition: transform var(--stack-duration) var(--transition-easing), opacity var(--exit-duration) var(--transition-easing);
}

[data-name='SonnerList'][data-direction='BottomUp'] [data-name='SonnerItem'][data-mounted='false'] {
  --y: translateY(100%);
  opacity: 0;
}

[data-name='SonnerList'][data-direction='TopDown'] [data-name='SonnerItem'][data-mounted='false'] {
  --y: translateY(-100%);
  opacity: 0;
}

[data-name='SonnerItem'][data-mounted='true'] {
  --y: translateY(0);
  opacity: 1;
}

[data-name='SonnerList'] [data-name='SonnerItem'][data-mounted='true'][data-entering='true'] {
  --y: translateY(0);
  transform: var(--y);
  opacity: 1;
}

[data-name='SonnerItem'][data-visible='false'] {
  opacity: 0;
  pointer-events: none;
}

[data-name='SonnerItem'][data-hidden='true'] {
  display: none;
}

[data-name='SonnerItem'][data-mounted='true'][data-expanded='false'] {
  --y: translateY(calc(var(--fold-multiplier) * var(--index) * var(--stack-spacing)));
  transform: var(--y) scale(calc(1 - var(--index) * var(--scale-factor)));
  z-index: var(--z-index);
}

[data-name='SonnerList'][data-expanded='true'] [data-name='SonnerItem'][data-mounted='true'] {
  --y: translateY(calc(var(--fold-multiplier) * var(--index) * var(--expand-spacing)));
  transform: var(--y) scale(1);
  z-index: var(--z-index);
}

[data-name='SonnerItem'][data-expanded='true']::after {
  content: '';
  position: absolute;
  left: 0;
  height: calc(var(--gap) + 1px);
  bottom: 100%;
  width: 100%;
}

[data-name='SonnerItem'][data-removed='true'][data-front='true'][data-swipe-out='false'] {
  --y: translateY(calc(var(--fold-multiplier) * -100%));
  opacity: 0;
}

[data-name='SonnerList'][data-expanded='true'] [data-name='SonnerItem'][data-removed='true'][data-front='false'][data-swipe-out='false'] {
  --y: translateY(calc(var(--fold-multiplier) * var(--index) * var(--expand-spacing) + var(--fold-multiplier) * -100%));
  opacity: 0;
}

[data-name='SonnerList'][data-expanded='false'] [data-name='SonnerItem'][data-removed='true'][data-front='false'][data-swipe-out='false'] {
  --y: translateY(40%);
  opacity: 0;
  transition: transform 500ms var(--transition-easing), opacity 200ms;
}

[data-name='SonnerItem'][data-swiping='true'] {
  transition: none !important;
  cursor: grabbing;
  user-select: none;
}

[data-name='SonnerItem'][data-swiping='true'][data-mounted='true'] {
  transform: var(--y)
             scale(calc(1 - var(--index) * var(--scale-factor)))
             translateX(var(--swipe-amount-x, 0px))
             translateY(var(--swipe-amount-y, 0px));
}

[data-name='SonnerItem'][data-swipe-out='true'][data-swipe-direction='Right'] {
  animation: swipe-out-right var(--exit-duration) ease-out forwards;
}

[data-name='SonnerItem'][data-swipe-out='true'][data-swipe-direction='Left'] {
  animation: swipe-out-left var(--exit-duration) ease-out forwards;
}

[data-name='SonnerItem'][data-swipe-out='true'][data-swipe-direction='Up'] {
  animation: swipe-out-up var(--exit-duration) ease-out forwards;
}

[data-name='SonnerItem'][data-swipe-out='true'][data-swipe-direction='Down'] {
  animation: swipe-out-down var(--exit-duration) ease-out forwards;
}

[data-duration-progress] {
  transition: transform linear;
}

[data-name='SonnerList'][data-expanded='true'] [data-name='SonnerItem'] [data-duration-progress] {
  animation-play-state: paused !important;
}

@keyframes sonner-progress {
  from { transform: scaleX(1); }
  to { transform: scaleX(0); }
}

[data-name='SonnerItem'][data-variant='Loading'] [data-duration-track] {
  display: none;
}

[data-icon] svg {
  width: 1rem;
  height: 1rem;
}

[data-close-button] svg {
  width: 0.75rem;
  height: 0.75rem;
}

@keyframes swipe-out-right {
  from {
    transform: var(--y) translateX(var(--swipe-amount-x, 0px));
    opacity: 1;
  }
  to {
    transform: var(--y) translateX(calc(100% + 100px));
    opacity: 0;
  }
}

@keyframes swipe-out-left {
  from {
    transform: var(--y) translateX(var(--swipe-amount-x, 0px));
    opacity: 1;
  }
  to {
    transform: var(--y) translateX(calc(-100% - 100px));
    opacity: 0;
  }
}

@keyframes swipe-out-up {
  from {
    transform: var(--y) translateY(var(--swipe-amount-y, 0px));
    opacity: 1;
  }
  to {
    transform: var(--y) translateY(calc(-100% - 100px));
    opacity: 0;
  }
}

@keyframes swipe-out-down {
  from {
    transform: var(--y) translateY(var(--swipe-amount-y, 0px));
    opacity: 1;
  }
  to {
    transform: var(--y) translateY(calc(100% + 100px));
    opacity: 0;
  }
}
"#;

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, Hash)]
pub enum ToastType {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, Hash)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SonnerDirection {
    TopDown,
    #[default]
    BottomUp,
}

#[derive(Clone)]
struct SonnerToast {
    id: u64,
    variant: ToastType,
    title: String,
    description: Option<String>,
    duration_ms: u32,
    position: SonnerPosition,
    mounted: bool,
    entering: bool,
    removed: bool,
    swipe_out: bool,
    swiping: bool,
    swipe_direction: Option<&'static str>,
}

#[derive(Clone)]
struct SonnerContext {
    toasts: RwSignal<Vec<SonnerToast>>,
    next_id: RwSignal<u64>,
    expanded_position: RwSignal<Option<SonnerPosition>>,
    timers: Arc<Mutex<HashMap<u64, SonnerTimer>>>,
}

struct SonnerTimer {
    timeout: Option<TimeoutHandle>,
    remaining_ms: u32,
    started_at_ms: f64,
}

#[derive(Clone)]
pub struct ToastApi {
    ctx: SonnerContext,
}

pub struct ToastBuilder {
    ctx: SonnerContext,
    variant: ToastType,
    title: String,
    description: Option<String>,
    duration_ms: u32,
    position: SonnerPosition,
}

#[derive(Clone, Copy)]
struct RenderMeta {
    index: usize,
    z_index: usize,
    front: bool,
    visible: bool,
    hidden: bool,
}

impl ToastApi {
    pub fn success(self, title: impl Into<String>) {
        self.message(title).variant(ToastType::Success).push();
    }

    pub fn error(self, title: impl Into<String>) {
        self.message(title).variant(ToastType::Error).push();
    }

    pub fn warning(self, title: impl Into<String>) {
        self.message(title).variant(ToastType::Warning).push();
    }

    pub fn info(self, title: impl Into<String>) {
        self.message(title).variant(ToastType::Info).push();
    }

    pub fn loading(self, title: impl Into<String>) -> u64 {
        self.message(title).variant(ToastType::Loading).duration(60_000).push()
    }

    pub fn with_description(self, title: impl Into<String>, description: impl Into<String>) {
        self.message(title).description(description).push();
    }

    pub fn message(self, title: impl Into<String>) -> ToastBuilder {
        ToastBuilder {
            ctx: self.ctx,
            variant: ToastType::Default,
            title: title.into(),
            description: None,
            duration_ms: DEFAULT_DURATION_MS,
            position: SonnerPosition::BottomRight,
        }
    }

    pub fn dismiss(self, toast_id: u64) {
        dismiss_toast(&self.ctx, toast_id, false, None);
    }

    pub fn update_to_success(self, toast_id: u64, title: impl Into<String>, description: Option<String>) {
        update_toast(&self.ctx, toast_id, ToastType::Success, title.into(), description, Some(DEFAULT_DURATION_MS));
    }
}

impl ToastBuilder {
    pub fn variant(mut self, variant: ToastType) -> Self {
        self.variant = variant;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn duration(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn position(mut self, position: SonnerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn push(self) -> u64 {
        push_toast(&self.ctx, self.variant, self.title, self.description, self.duration_ms, self.position)
    }
}

pub fn provide_sonner() {
    if use_context::<SonnerContext>().is_none() {
        provide_context(new_sonner_context());
    }
}

pub fn show_toast() -> ToastApi {
    ToastApi { ctx: expect_context::<SonnerContext>() }
}

#[component]
pub fn SonnerTrigger(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = ToastType::default())] variant: ToastType,
    #[prop(into)] title: String,
    #[prop(into, optional)] description: String,
    #[prop(into, optional)] position: String,
) -> impl IntoView {
    let variant_classes = match variant {
        ToastType::Default => "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
        ToastType::Success => "bg-success text-success-foreground hover:bg-success/90",
        ToastType::Error => "bg-destructive text-white shadow-xs hover:bg-destructive/90 dark:bg-destructive/60",
        ToastType::Warning => "bg-warning text-warning-foreground hover:bg-warning/90",
        ToastType::Info => "bg-info text-info-foreground shadow-xs hover:bg-info/90",
        ToastType::Loading => "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
    };

    let merged_class = tw_merge!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] w-fit cursor-pointer h-9 px-4 py-2",
        variant_classes,
        class
    );

    let click_title = title.clone();
    let click_description = if description.is_empty() { None } else { Some(description.clone()) };
    let click_position = parse_position(position.as_str()).unwrap_or_default();

    view! {
        <button
            class=merged_class
            data-name="SonnerTrigger"
            data-variant=variant.to_string()
            data-toast-title=title
            data-toast-description=description
            data-toast-position=if position.is_empty() { None } else { Some(position) }
            on:click=move |_| {
                let mut builder = show_toast().message(click_title.clone()).variant(variant).position(click_position);
                if let Some(text) = &click_description {
                    builder = builder.description(text.clone());
                }
                if variant == ToastType::Loading {
                    let id = builder.duration(1_200).push();
                    set_timeout(
                        move || {
                            show_toast().update_to_success(id, "Completed", Some("Background task finished".to_string()));
                        },
                        Duration::from_millis(1_300),
                    );
                } else {
                    builder.push();
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SonnerContainer(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional, default = SonnerPosition::default())] position: SonnerPosition,
) -> impl IntoView {
    let merged_class = tw_merge!("fixed z-[9999]", class);
    view! {
        <div class=merged_class data-position=position.to_string()>{children()}</div>
    }
}

#[component]
pub fn SonnerList(
    #[prop(optional, default = SonnerPosition::default())] position: SonnerPosition,
    #[prop(optional, default = SonnerDirection::default())] direction: SonnerDirection,
) -> impl IntoView {
    let ctx = expect_context::<SonnerContext>();
    let ctx_mouseenter = ctx.clone();
    let ctx_mousemove = ctx.clone();
    let ctx_mouseleave = ctx.clone();
    let ctx_focusin = ctx.clone();
    let ctx_focusout = ctx.clone();
    let position_toasts =
        move || ctx.toasts.get().into_iter().filter(|toast| toast.position == position).collect::<Vec<_>>();
    let position_toast_ids = move || position_toasts().into_iter().map(|toast| toast.id).collect::<Vec<u64>>();

    let expanded = move || ctx.expanded_position.get() == Some(position);

    view! {
        <ol
            data-name="SonnerList"
            data-sonner-toaster="true"
            data-sonner-theme="light"
            data-position=position.to_string()
            data-expanded=move || bool_attr(expanded())
            data-direction=direction.to_string()
            style="--max-toasts: 5; --dismiss-delay: 5000ms; --enter-duration: 300ms; --exit-duration: 300ms; --stack-duration: 300ms; --stack-spacing: 20px; --expand-spacing: 110px; --gap: 15px; --scale-factor: 0.05; --transition-easing: ease-out; --stack-easing: ease-in-out;"
            on:mouseenter=move |_| {
                ctx_mouseenter.expanded_position.set(Some(position));
                pause_position_timers(&ctx_mouseenter, position);
            }
            on:mousemove=move |_| {
                ctx_mousemove.expanded_position.set(Some(position));
            }
            on:mouseleave=move |_| {
                if ctx_mouseleave.expanded_position.get() == Some(position) {
                    ctx_mouseleave.expanded_position.set(None);
                }
                resume_position_timers(&ctx_mouseleave, position);
            }
            on:focusin=move |_| {
                ctx_focusin.expanded_position.set(Some(position));
                pause_position_timers(&ctx_focusin, position);
            }
            on:focusout=move |_| {
                if ctx_focusout.expanded_position.get() == Some(position) {
                    ctx_focusout.expanded_position.set(None);
                }
                resume_position_timers(&ctx_focusout, position);
            }
        >
            <For
                each=position_toast_ids
                key=|toast_id| *toast_id
                let:toast_id
            >
                <SonnerItem toast_id=toast_id position=position expanded=Signal::derive(expanded) />
            </For>
        </ol>
    }
}

#[component]
fn SonnerItem(toast_id: u64, position: SonnerPosition, expanded: Signal<bool>) -> impl IntoView {
    let ctx = expect_context::<SonnerContext>();
    let swipe_amount_x = RwSignal::new(0.0_f64);
    let swipe_amount_y = RwSignal::new(0.0_f64);
    let pointer_start = RwSignal::new(None::<(f64, f64)>);
    let drag_start = RwSignal::new(0.0_f64);
    let swipe_axis = RwSignal::new(None::<char>);

    let ctx_pointer_down = ctx.clone();
    let on_pointer_down = move |event: PointerEvent| {
        if is_toast_removed(&ctx_pointer_down, toast_id) {
            return;
        }

        if let Some(target) = event.target()
            && let Ok(element) = target.dyn_into::<HtmlElement>()
        {
            let _ = element.set_pointer_capture(event.pointer_id());
        }

        pointer_start.set(Some((f64::from(event.client_x()), f64::from(event.client_y()))));
        drag_start.set(js_sys::Date::now());
        swipe_axis.set(None);
        set_swiping(&ctx_pointer_down, toast_id, true);
    };

    let on_pointer_move = move |event: PointerEvent| {
        let Some((start_x, start_y)) = pointer_start.get() else { return };
        let delta_x = f64::from(event.client_x()) - start_x;
        let delta_y = f64::from(event.client_y()) - start_y;

        if swipe_axis.get().is_none() && (delta_x.abs() > 1.0 || delta_y.abs() > 1.0) {
            let axis = if delta_x.abs() > delta_y.abs() { 'x' } else { 'y' };
            swipe_axis.set(Some(axis));
        }

        let mut x = 0.0_f64;
        let mut y = 0.0_f64;

        match swipe_axis.get() {
            Some('x') => {
                x = delta_x;
            }
            Some('y') => {
                if allowed_vertical_delta(position, delta_y) {
                    y = delta_y;
                } else {
                    y = delta_y * dampening(delta_y);
                }
            }
            _ => {}
        }

        swipe_amount_x.set(x);
        swipe_amount_y.set(y);
    };

    let ctx_pointer_up = ctx.clone();
    let on_pointer_up = move |_| {
        let Some(axis) = swipe_axis.get() else {
            pointer_start.set(None);
            set_swiping(&ctx_pointer_up, toast_id, false);
            return;
        };

        let elapsed = (js_sys::Date::now() - drag_start.get()).max(1.0);
        let amount = if axis == 'x' { swipe_amount_x.get().abs() } else { swipe_amount_y.get().abs() };
        let velocity = amount / elapsed;

        if amount >= SWIPE_THRESHOLD || velocity > SWIPE_VELOCITY_THRESHOLD {
            let direction = if axis == 'x' {
                if swipe_amount_x.get() >= 0.0 { "Right" } else { "Left" }
            } else if swipe_amount_y.get() >= 0.0 {
                "Down"
            } else {
                "Up"
            };

            dismiss_toast(&ctx_pointer_up, toast_id, true, Some(direction));
        } else {
            set_swiping(&ctx_pointer_up, toast_id, false);
        }

        pointer_start.set(None);
        swipe_axis.set(None);
        swipe_amount_x.set(0.0);
        swipe_amount_y.set(0.0);
    };

    let ctx_pointer_cancel = ctx.clone();
    let on_pointer_cancel = move |_| {
        let Some(axis) = swipe_axis.get() else {
            pointer_start.set(None);
            set_swiping(&ctx_pointer_cancel, toast_id, false);
            return;
        };

        let elapsed = (js_sys::Date::now() - drag_start.get()).max(1.0);
        let amount = if axis == 'x' { swipe_amount_x.get().abs() } else { swipe_amount_y.get().abs() };
        let velocity = amount / elapsed;

        if amount >= SWIPE_THRESHOLD || velocity > SWIPE_VELOCITY_THRESHOLD {
            let direction = if axis == 'x' {
                if swipe_amount_x.get() >= 0.0 { "Right" } else { "Left" }
            } else if swipe_amount_y.get() >= 0.0 {
                "Down"
            } else {
                "Up"
            };

            dismiss_toast(&ctx_pointer_cancel, toast_id, true, Some(direction));
        } else {
            set_swiping(&ctx_pointer_cancel, toast_id, false);
        }

        pointer_start.set(None);
        swipe_axis.set(None);
        swipe_amount_x.set(0.0);
        swipe_amount_y.set(0.0);
    };

    let y_position = if is_top_position(position) { "Top" } else { "Bottom" };
    let x_position = if matches!(position, SonnerPosition::TopLeft | SonnerPosition::BottomLeft) {
        "Left"
    } else if matches!(position, SonnerPosition::TopCenter | SonnerPosition::BottomCenter) {
        "Center"
    } else {
        "Right"
    };

    let toast_state = Signal::derive({
        let ctx = ctx.clone();
        move || toast_snapshot(&ctx, toast_id)
    });
    let render_meta_ctx = ctx.clone();
    let render_meta = Signal::derive({
        move || {
            let active_ids = render_meta_ctx
                .toasts
                .get()
                .into_iter()
                .filter(|toast| toast.position == position && !toast.removed)
                .map(|toast| toast.id)
                .collect::<Vec<_>>();
            render_meta(&active_ids, toast_id)
        }
    });
    let merged_class = move || {
        let variant_class = match toast_state.get().map(|toast| toast.variant).unwrap_or(ToastType::Default) {
            ToastType::Default => "bg-background text-foreground border-border",
            ToastType::Success => "bg-success-light text-success-dark border-success",
            ToastType::Error => "bg-destructive-light text-destructive-dark border-destructive",
            ToastType::Warning => "bg-warning-light text-warning-dark border-warning",
            ToastType::Info => "bg-info-light text-info-dark border-info",
            ToastType::Loading => "bg-background text-foreground border-border",
        };

        tw_merge!(
            "p-5 shadow-lg border rounded-lg cursor-grab absolute w-full max-w-96 touch-none flex items-start gap-3",
            variant_class,
            if is_top_position(position) { "top-0" } else { "bottom-0" }
        )
    };

    view! {
        <li
            data-name="SonnerItem"
            data-sonner-toast="true"
            data-variant=move || {
                toast_state
                    .get()
                    .map(|toast| toast.variant.to_string())
                    .unwrap_or_else(|| ToastType::Default.to_string())
            }
            data-mounted=move || bool_attr(toast_state.get().map(|toast| toast.mounted).unwrap_or(false))
            data-entering=move || bool_attr(toast_state.get().map(|toast| toast.entering).unwrap_or(false))
            data-expanded=move || bool_attr(expanded.get())
            data-visible=move || bool_attr(render_meta.get().visible)
            data-hidden=move || bool_attr(render_meta.get().hidden)
            data-front=move || bool_attr(render_meta.get().front)
            data-removed=move || bool_attr(toast_state.get().map(|toast| toast.removed).unwrap_or(false))
            data-swiping=move || bool_attr(toast_state.get().map(|toast| toast.swiping).unwrap_or(false))
            data-swipe-out=move || bool_attr(toast_state.get().map(|toast| toast.swipe_out).unwrap_or(false))
            data-swipe-direction=move || toast_state.get().and_then(|toast| toast.swipe_direction)
            data-y-position=y_position
            data-x-position=x_position
            class=merged_class
            style=("--index", move || render_meta.get().index.to_string())
            style=("--toasts-before", move || render_meta.get().index.to_string())
            style=("--z-index", move || render_meta.get().z_index.to_string())
            style=("--lift", if is_top_position(position) { "1" } else { "-1" })
            style=("--swipe-amount-x", move || format!("{}px", swipe_amount_x.get()))
            style=("--swipe-amount-y", move || format!("{}px", swipe_amount_y.get()))
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointercancel=on_pointer_cancel
        >
            <div data-icon class=move || if toast_state.get().map(|toast| toast.variant) == Some(ToastType::Loading) {
                "flex items-center justify-center w-5 h-5 shrink-0 mr-3 [&>svg]:animate-spin"
            } else {
                "flex items-center justify-center w-5 h-5 shrink-0 mr-3"
            }>
                {move || toast_icon(toast_state.get().map(|toast| toast.variant).unwrap_or(ToastType::Default))}
            </div>

            <div class="flex-1">
                <div class="flex items-center gap-2">
                    <h3 class="font-semibold text-base leading-[1.4] flex-1">
                        {move || toast_state.get().map(|toast| toast.title).unwrap_or_default()}
                    </h3>
                    <Show when=move || toast_state.get().map(|toast| toast.variant != ToastType::Loading).unwrap_or(false)>
                        <button
                            data-close-button=""
                            aria-label="Close toast"
                            r#type="button"
                            class="w-5 h-5 flex items-center justify-center border-none bg-transparent text-muted-foreground cursor-pointer transition-colors duration-150 shrink-0 p-0 ml-auto hover:text-foreground"
                            on:click={
                                let ctx = ctx.clone();
                                move |_| dismiss_toast(&ctx, toast_id, false, None)
                            }
                        >
                            <CloseIcon />
                        </button>
                    </Show>
                </div>

                <Show when=move || toast_state.get().and_then(|toast| toast.description).is_some()>
                    <p class="text-sm leading-[1.5] opacity-90 mt-1">
                        {move || toast_state.get().and_then(|toast| toast.description).unwrap_or_default()}
                    </p>
                </Show>
            </div>

            <div
                data-duration-track
                class="absolute bottom-0 inset-x-0 h-[3px] bg-black/10 overflow-hidden rounded-b-lg"
                style:display=move || {
                    if toast_state.get().map(|toast| toast.variant) == Some(ToastType::Loading) {
                        "none"
                    } else {
                        "block"
                    }
                }
            >
                <div
                    data-duration-progress
                    class="h-full w-full bg-current opacity-30 origin-left"
                    style=("animation-name", "sonner-progress")
                    style=("animation-timing-function", "linear")
                    style=("animation-fill-mode", "forwards")
                    style=("animation-duration", move || {
                        format!(
                            "{}ms",
                            toast_state
                                .get()
                                .map(|toast| toast.duration_ms)
                                .unwrap_or(DEFAULT_DURATION_MS)
                        )
                    })
                />
            </div>
        </li>
    }
}

#[component]
fn CloseIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
    }
}

#[component]
fn SuccessIcon() -> impl IntoView {
    view! {
        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
        </svg>
    }
}

#[component]
fn ErrorIcon() -> impl IntoView {
    view! {
        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
    }
}

#[component]
fn WarningIcon() -> impl IntoView {
    view! {
        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
        </svg>
    }
}

#[component]
fn InfoIcon() -> impl IntoView {
    view! {
        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
        </svg>
    }
}

#[component]
fn LoadingIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"
            />
        </svg>
    }
}

fn toast_icon(variant: ToastType) -> AnyView {
    match variant {
        ToastType::Success => view! { <SuccessIcon /> }.into_any(),
        ToastType::Error => view! { <ErrorIcon /> }.into_any(),
        ToastType::Warning => view! { <WarningIcon /> }.into_any(),
        ToastType::Info => view! { <InfoIcon /> }.into_any(),
        ToastType::Loading => view! { <LoadingIcon /> }.into_any(),
        ToastType::Default => view! { <InfoIcon /> }.into_any(),
    }
}

#[component]
pub fn SonnerToaster(#[prop(default = SonnerPosition::default())] position: SonnerPosition) -> impl IntoView {
    if use_context::<SonnerContext>().is_none() {
        provide_context(new_sonner_context());
    }
    let direction = direction_from_position(position);
    let container_class = match position {
        SonnerPosition::TopLeft => "left-6 top-6",
        SonnerPosition::TopRight => "right-6 top-6",
        SonnerPosition::TopCenter => "left-1/2 -translate-x-1/2 top-6",
        SonnerPosition::BottomCenter => "left-1/2 -translate-x-1/2 bottom-6",
        SonnerPosition::BottomLeft => "left-6 bottom-6",
        SonnerPosition::BottomRight => "right-6 bottom-6",
    };

    view! {
        <style>{SONNER_STYLE}</style>
        <SonnerContainer class=container_class position=position>
            <SonnerList position=position direction=direction />
        </SonnerContainer>
    }
}

fn new_sonner_context() -> SonnerContext {
    SonnerContext {
        toasts: RwSignal::new(Vec::new()),
        next_id: RwSignal::new(1),
        expanded_position: RwSignal::new(None),
        timers: Arc::new(Mutex::new(HashMap::new())),
    }
}

fn push_toast(
    ctx: &SonnerContext,
    variant: ToastType,
    title: String,
    description: Option<String>,
    duration_ms: u32,
    position: SonnerPosition,
) -> u64 {
    let id = ctx.next_id.get_untracked();
    ctx.next_id.set(id.saturating_add(1));

    let oldest = {
        let toasts = ctx.toasts.get_untracked();
        let mut active = toasts
            .iter()
            .filter(|toast| toast.position == position && !toast.removed)
            .map(|toast| toast.id)
            .collect::<Vec<_>>();
        if active.len() >= MAX_TOASTS { active.pop() } else { None }
    };

    if let Some(oldest_id) = oldest {
        dismiss_toast(ctx, oldest_id, false, None);
    }

    let toast = SonnerToast {
        id,
        variant,
        title,
        description,
        duration_ms,
        position,
        mounted: false,
        entering: true,
        removed: false,
        swipe_out: false,
        swiping: false,
        swipe_direction: None,
    };

    ctx.toasts.update(|toasts| {
        toasts.insert(0, toast);
    });

    let mount_ctx = ctx.clone();
    set_timeout(
        move || {
            mount_ctx.toasts.update(|toasts| {
                if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id) {
                    toast.mounted = true;
                }
            });
        },
        Duration::from_millis(16),
    );

    let entering_ctx = ctx.clone();
    set_timeout(
        move || {
            entering_ctx.toasts.update(|toasts| {
                if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id) {
                    toast.entering = false;
                }
            });
        },
        Duration::from_millis(u64::from(ENTER_DURATION_MS)),
    );

    if variant != ToastType::Loading {
        schedule_timer(ctx, id, duration_ms);
    }

    id
}

fn update_toast(
    ctx: &SonnerContext,
    id: u64,
    variant: ToastType,
    title: String,
    description: Option<String>,
    duration_ms: Option<u32>,
) {
    ctx.toasts.update(|toasts| {
        if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id) {
            toast.variant = variant;
            toast.title = title;
            toast.description = description;
            if let Some(duration) = duration_ms {
                toast.duration_ms = duration;
            }
        }
    });

    if variant == ToastType::Loading {
        cancel_timer(ctx, id);
    } else if let Some(duration) = duration_ms {
        schedule_timer(ctx, id, duration);
    }
}

fn dismiss_toast(ctx: &SonnerContext, id: u64, swipe_out: bool, swipe_direction: Option<&'static str>) {
    cancel_timer(ctx, id);

    ctx.toasts.update(|toasts| {
        if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id) {
            if toast.removed {
                return;
            }
            toast.removed = true;
            toast.swipe_out = swipe_out;
            toast.swipe_direction = swipe_direction;
            toast.swiping = false;
        }
    });

    let ctx = ctx.clone();
    set_timeout(
        move || {
            ctx.toasts.update(|toasts| {
                if let Some(index) = toasts.iter().position(|toast| toast.id == id) {
                    toasts.remove(index);
                }
            });
            cancel_timer(&ctx, id);
        },
        Duration::from_millis(u64::from(EXIT_DURATION_MS)),
    );
}

fn render_meta(active_ids: &[u64], toast_id: u64) -> RenderMeta {
    if let Some(index) = active_ids.iter().position(|id| *id == toast_id) {
        let z_index = active_ids.len().saturating_sub(index);
        let from_end = index;
        return RenderMeta {
            index,
            z_index,
            front: index == 0,
            visible: from_end < VISIBLE_TOASTS_AMOUNT,
            hidden: from_end >= MAX_TOASTS,
        };
    }

    RenderMeta { index: active_ids.len(), z_index: 1, front: false, visible: false, hidden: true }
}

fn toast_snapshot(ctx: &SonnerContext, toast_id: u64) -> Option<SonnerToast> {
    ctx.toasts.get().into_iter().find(|toast| toast.id == toast_id)
}

fn bool_attr(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn parse_position(value: &str) -> Option<SonnerPosition> {
    match value {
        "TopLeft" => Some(SonnerPosition::TopLeft),
        "TopCenter" => Some(SonnerPosition::TopCenter),
        "TopRight" => Some(SonnerPosition::TopRight),
        "BottomLeft" => Some(SonnerPosition::BottomLeft),
        "BottomCenter" => Some(SonnerPosition::BottomCenter),
        "BottomRight" => Some(SonnerPosition::BottomRight),
        _ => None,
    }
}

fn direction_from_position(position: SonnerPosition) -> SonnerDirection {
    match position {
        SonnerPosition::TopLeft | SonnerPosition::TopCenter | SonnerPosition::TopRight => SonnerDirection::TopDown,
        _ => SonnerDirection::BottomUp,
    }
}

fn is_top_position(position: SonnerPosition) -> bool {
    matches!(position, SonnerPosition::TopLeft | SonnerPosition::TopCenter | SonnerPosition::TopRight)
}

fn allowed_vertical_delta(position: SonnerPosition, delta_y: f64) -> bool {
    if is_top_position(position) { delta_y < 0.0 } else { delta_y > 0.0 }
}

fn dampening(delta: f64) -> f64 {
    1.0 / (1.5 + delta.abs() / 20.0)
}

fn set_swiping(ctx: &SonnerContext, id: u64, swiping: bool) {
    ctx.toasts.update(|toasts| {
        if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id) {
            toast.swiping = swiping;
            if !swiping {
                toast.swipe_direction = None;
            }
        }
    });
}

fn is_toast_removed(ctx: &SonnerContext, id: u64) -> bool {
    ctx.toasts.get_untracked().iter().any(|toast| toast.id == id && toast.removed)
}

fn pause_position_timers(ctx: &SonnerContext, position: SonnerPosition) {
    let toast_ids = ctx
        .toasts
        .get_untracked()
        .into_iter()
        .filter(|toast| toast.position == position && !toast.removed && toast.variant != ToastType::Loading)
        .map(|toast| toast.id)
        .collect::<Vec<_>>();

    for id in toast_ids {
        pause_timer(ctx, id);
    }
}

fn resume_position_timers(ctx: &SonnerContext, position: SonnerPosition) {
    let toast_ids = ctx
        .toasts
        .get_untracked()
        .into_iter()
        .filter(|toast| toast.position == position && !toast.removed && toast.variant != ToastType::Loading)
        .map(|toast| toast.id)
        .collect::<Vec<_>>();

    for id in toast_ids {
        resume_timer(ctx, id);
    }
}

fn schedule_timer(ctx: &SonnerContext, id: u64, duration_ms: u32) {
    cancel_timer(ctx, id);

    let dismiss_ctx = ctx.clone();
    if let Ok(timeout) = set_timeout_with_handle(
        move || dismiss_toast(&dismiss_ctx, id, false, None),
        Duration::from_millis(u64::from(duration_ms)),
    ) && let Ok(mut timers) = ctx.timers.lock()
    {
        timers.insert(
            id,
            SonnerTimer { timeout: Some(timeout), remaining_ms: duration_ms, started_at_ms: js_sys::Date::now() },
        );
    }
}

fn pause_timer(ctx: &SonnerContext, id: u64) {
    let now = js_sys::Date::now();
    let Ok(mut timers) = ctx.timers.lock() else { return };
    let Some(timer) = timers.get_mut(&id) else { return };

    if let Some(timeout) = timer.timeout.take() {
        timeout.clear();
    }

    let elapsed = (now - timer.started_at_ms).max(0.0) as u32;
    timer.remaining_ms = timer.remaining_ms.saturating_sub(elapsed);
}

fn resume_timer(ctx: &SonnerContext, id: u64) {
    let remaining = {
        let Ok(timers) = ctx.timers.lock() else { return };
        let Some(timer) = timers.get(&id) else { return };
        if timer.timeout.is_some() {
            return;
        }
        timer.remaining_ms
    };
    if remaining == 0 {
        dismiss_toast(ctx, id, false, None);
        return;
    }
    schedule_timer(ctx, id, remaining);
}

fn cancel_timer(ctx: &SonnerContext, id: u64) {
    if let Ok(mut timers) = ctx.timers.lock()
        && let Some(mut timer) = timers.remove(&id)
        && let Some(timeout) = timer.timeout.take()
    {
        timeout.clear();
    }
}
