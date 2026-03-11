use leptos::prelude::*;
use strum::Display;
use tw_merge::*;

#[derive(TwClass, Clone, Copy)]
#[tw(class = "flex justify-center items-center w-full")]
pub struct Animate {
    pub variant: AnimateVariant,
    pub hover_variant: AnimateHoverVariant,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Animate(
    #[prop(into, optional)] variant: Signal<AnimateVariant>,
    #[prop(into, optional)] hover_variant: Signal<AnimateHoverVariant>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] style: Signal<String>,

    children: Children,
) -> impl IntoView {
    let merged_class = move || {
        let animate = Animate { variant: variant.get(), hover_variant: hover_variant.get() };
        animate.with_class(class.clone())
    };

    view! {
        <div class=merged_class style=style>
            {children()}
        </div>
    }
}

#[component]
pub fn AnimateGroup(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("w-full", class);

    view! { <div class=merged_class>{children()}</div> }
}

#[component]
pub fn AnimateGroupItem(
    #[prop(into, optional)] variant: Signal<AnimateVariant>,
    #[prop(into, optional)] hover_variant: Signal<AnimateHoverVariant>,
    #[prop(into, optional)] class: String,
    #[prop(into)] delay_ms: Signal<u32>,
    #[prop(default = "forwards")] fill_mode: &'static str,

    children: Children,
) -> impl IntoView {
    let merged_class = move || {
        let animate = Animate { variant: variant.get(), hover_variant: hover_variant.get() };
        animate.with_class(class.clone())
    };

    let style = move || {
        let delay = delay_ms.get();
        format!("animation-delay: {delay}ms; animation-fill-mode: {fill_mode};")
    };

    view! {
        <div class=merged_class style=style>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// TODO later.

#[derive(TwVariant)]
pub enum AnimateVariant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "opacity-0 animate-fade_up")]
    FadeUp,
    #[tw(
        class = "animate-fade_out_down   [animation-range:0px_300px] [animation-timeline:scroll()] supports-no-scroll-driven-animations:animate-none"
    )]
    AnimateScrollFadeOut,
    #[tw(
        class = "animate-make_it_bigger   [animation-range:0%_60%] [animation-timeline:--quote] [view-timeline-name:--quote] supports-no-scroll-driven-animations:animate-none"
    )]
    AnimateScrollBigger,
}

#[derive(TwVariant, Display)]
pub enum AnimateHoverVariant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "hover:animate-Blink")]
    Blink,
    #[tw(class = "hover:animate-BlurredFadeIn")]
    BlurredFadeIn,
    #[tw(class = "hover:animate-BounceFadeIn")]
    BounceFadeIn,
    #[tw(class = "hover:animate-BounceHorizontal")]
    BounceHorizontal,
    #[tw(class = "hover:animate-BounceVertical")]
    BounceVertical,
    #[tw(class = "hover:animate-BounceCustom")] // TODO: check
    BounceCustom,
    #[tw(class = "hover:animate-ContractHorizontally")]
    ContractHorizontally,
    #[tw(class = "hover:animate-ContractVertically")]
    ContractVertically,
    #[tw(class = "hover:animate-ExpandHorizontally")]
    ExpandHorizontally,
    #[tw(class = "hover:animate-ExpandVertically")]
    ExpandVertically,
    #[tw(class = "hover:animate-FadeIn")]
    FadeIn,
    #[tw(class = "hover:animate-FadeInDown")]
    FadeInDown,
    #[tw(class = "hover:animate-FadeInLeft")]
    FadeInLeft,
    #[tw(class = "hover:animate-FadeInRight")]
    FadeInRight,
    #[tw(class = "hover:animate-FadeInUp")]
    FadeInUp,
    #[tw(class = "hover:animate-FadeOut")]
    FadeOut,
    #[tw(class = "hover:animate-FadeOutUp")]
    FadeOutUp,
    #[tw(class = "hover:animate-FadeOutDownV2")] // TODO: V2
    FadeOutDownV2,
    #[tw(class = "hover:animate-FadeOutLeft")]
    FadeOutLeft,
    #[tw(class = "hover:animate-FadeOutRight")]
    FadeOutRight,
    #[tw(class = "hover:animate-Flash")]
    Flash,
    #[tw(class = "hover:animate-FlashV0")] // TODO
    FlashV0,
    #[tw(class = "hover:animate-FlipHorizontal")]
    FlipHorizontal,
    #[tw(class = "hover:animate-FlipVertical")]
    FlipVertical,
    #[tw(class = "hover:animate-FlipX")]
    FlipX,
    #[tw(class = "hover:animate-FlipY")]
    FlipY,
    #[tw(class = "hover:animate-FlipInY")]
    FlipInY,
    #[tw(class = "hover:animate-FlipInX")]
    FlipInX,
    #[tw(class = "hover:animate-FlipOutY")]
    FlipOutY,
    #[tw(class = "hover:animate-FlipOutX")]
    FlipOutX,
    #[tw(class = "hover:animate-Float")]
    Float,
    #[tw(class = "hover:animate-Hang")]
    Hang,
    #[tw(class = "hover:animate-Heartbeat")]
    Heartbeat,
    #[tw(class = "hover:animate-HorizontalVibration")]
    HorizontalVibration,
    #[tw(class = "hover:animate-Jiggle")]
    Jiggle,
    #[tw(class = "hover:animate-JiggleV0")] // TODO
    JiggleV0,
    #[tw(class = "hover:animate-Jump")]
    Jump,
    #[tw(class = "hover:animate-Pop")]
    Pop,
    #[tw(class = "hover:animate-PulseCustom")] // TODO: custom
    PulseCustom,
    #[tw(class = "hover:animate-PulseFadeIn")]
    PulseFadeIn,
    #[tw(class = "hover:animate-Rise")]
    Rise,
    #[tw(class = "hover:animate-RollIn")]
    RollIn,
    #[tw(class = "hover:animate-RollOut")]
    RollOut,
    #[tw(class = "hover:animate-Rotate180")]
    Rotate180,
    #[tw(class = "hover:animate-Rotate360")]
    Rotate360,
    #[tw(class = "hover:animate-Rotate90")]
    Rotate90,
    #[tw(class = "hover:animate-RotateIn")]
    RotateIn,
    #[tw(class = "hover:animate-RotateOut")]
    RotateOut,
    #[tw(class = "hover:animate-RotationalWave")]
    RotationalWave,
    #[tw(class = "hover:animate-RubberBand")]
    RubberBand,
    #[tw(class = "hover:animate-RubberBandV0")] // TODO
    RubberBandV0,
    #[tw(class = "hover:animate-Scale")] // TODO
    Scale,
    #[tw(class = "hover:animate-Shake")]
    Shake,
    #[tw(class = "hover:animate-ShakeV0")] // TODO
    ShakeV0,
    #[tw(class = "hover:animate-Sink")]
    Sink,
    #[tw(class = "hover:animate-Skew")]
    Skew,
    #[tw(class = "hover:animate-SlideDown")]
    SlideDown,
    #[tw(class = "hover:animate-SlideDownAndFade")]
    SlideDownAndFade,
    #[tw(class = "hover:animate-SlideInBottom")]
    SlideInBottom,
    #[tw(class = "hover:animate-SlideInLeft")]
    SlideInLeft,
    #[tw(class = "hover:animate-SlideInRight")]
    SlideInRight,
    #[tw(class = "hover:animate-SlideInTop")]
    SlideInTop,
    #[tw(class = "hover:animate-SlideLeft")]
    SlideLeft,
    #[tw(class = "hover:animate-SlideLeftAndFade")]
    SlideLeftAndFade,
    #[tw(class = "hover:animate-SlideOutBottom")]
    SlideOutBottom,
    #[tw(class = "hover:animate-SlideOutLeft")]
    SlideOutLeft,
    #[tw(class = "hover:animate-SlideOutTop")]
    SlideOutTop,
    #[tw(class = "hover:animate-SlideRight")]
    SlideRight,
    #[tw(class = "hover:animate-SlideRightAndFade")]
    SlideRightAndFade,
    #[tw(class = "hover:animate-SlideRotateIn")]
    SlideRotateIn,
    #[tw(class = "hover:animate-SlideRotateOut")]
    SlideRotateOut,
    #[tw(class = "hover:animate-SlideUp")]
    SlideUp,
    #[tw(class = "hover:animate-SlideUpAndFade")]
    SlideUpAndFade,
    #[tw(class = "hover:animate-SlideUpFade")]
    SlideUpFade,
    #[tw(class = "hover:animate-SpinClockwise")]
    SpinClockwise,
    #[tw(class = "hover:animate-SpinCounterClockwise")]
    SpinCounterClockwise,
    #[tw(class = "hover:animate-Sway")]
    Sway,
    #[tw(class = "hover:animate-Swing")]
    Swing,
    #[tw(class = "hover:animate-SwingDropIn")]
    SwingDropIn,
    #[tw(class = "hover:animate-SwingV0")] // TODO
    SwingV0,
    #[tw(class = "hover:animate-Squeeze")]
    Squeeze,
    #[tw(class = "hover:animate-Tada")]
    Tada,
    #[tw(class = "hover:animate-TiltHorizontal")]
    TiltHorizontal,
    #[tw(class = "hover:animate-Vibrate")]
    Vibrate,
    #[tw(class = "hover:animate-Wobble")]
    Wobble,
    #[tw(class = "hover:animate-ZoomIn")]
    ZoomIn,
    #[tw(class = "hover:animate-ZoomOut")]
    ZoomOut,
}
