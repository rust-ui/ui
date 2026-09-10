use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AnimateVariant {
    #[default]
    Default,
    FadeUp,
    AnimateScrollFadeOut,
    AnimateScrollBigger,
}

impl AnimateVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::FadeUp => "opacity-0 animate-fade_up",
            Self::AnimateScrollFadeOut => {
                "animate-fade_out_down [animation-range:0px_300px] [animation-timeline:scroll()] supports-no-scroll-driven-animations:animate-none"
            }
            Self::AnimateScrollBigger => {
                "animate-make_it_bigger [animation-range:0%_60%] [animation-timeline:--quote] [view-timeline-name:--quote] supports-no-scroll-driven-animations:animate-none"
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, strum::Display)]
pub enum AnimateHoverVariant {
    #[default]
    Default,
    Blink,
    BlurredFadeIn,
    BounceFadeIn,
    BounceHorizontal,
    BounceVertical,
    BounceCustom,
    ContractHorizontally,
    ContractVertically,
    ExpandHorizontally,
    ExpandVertically,
    FadeIn,
    FadeInDown,
    FadeInLeft,
    FadeInRight,
    FadeInUp,
    FadeOut,
    FadeOutUp,
    FadeOutDownV2,
    FadeOutLeft,
    FadeOutRight,
    Flash,
    FlashV0,
    FlipHorizontal,
    FlipVertical,
    FlipX,
    FlipY,
    FlipInY,
    FlipInX,
    FlipOutY,
    FlipOutX,
    Float,
    Hang,
    Heartbeat,
    HorizontalVibration,
    Jiggle,
    JiggleV0,
    Jump,
    Pop,
    PulseCustom,
    PulseFadeIn,
    Rise,
    RollIn,
    RollOut,
    Rotate180,
    Rotate360,
    Rotate90,
    RotateIn,
    RotateOut,
    RotationalWave,
    RubberBand,
    RubberBandV0,
    Scale,
    Shake,
    ShakeV0,
    Sink,
    Skew,
    SlideDown,
    SlideDownAndFade,
    SlideInBottom,
    SlideInLeft,
    SlideInRight,
    SlideInTop,
    SlideLeft,
    SlideLeftAndFade,
    SlideOutBottom,
    SlideOutLeft,
    SlideOutTop,
    SlideRight,
    SlideRightAndFade,
    SlideRotateIn,
    SlideRotateOut,
    SlideUp,
    SlideUpAndFade,
    SlideUpFade,
    SpinClockwise,
    SpinCounterClockwise,
    Sway,
    Swing,
    SwingDropIn,
    SwingV0,
    Squeeze,
    Tada,
    TiltHorizontal,
    Vibrate,
    Wobble,
    ZoomIn,
    ZoomOut,
}

impl AnimateHoverVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Blink => "hover:animate-Blink",
            Self::BlurredFadeIn => "hover:animate-BlurredFadeIn",
            Self::BounceFadeIn => "hover:animate-BounceFadeIn",
            Self::BounceHorizontal => "hover:animate-BounceHorizontal",
            Self::BounceVertical => "hover:animate-BounceVertical",
            Self::BounceCustom => "hover:animate-BounceCustom",
            Self::ContractHorizontally => "hover:animate-ContractHorizontally",
            Self::ContractVertically => "hover:animate-ContractVertically",
            Self::ExpandHorizontally => "hover:animate-ExpandHorizontally",
            Self::ExpandVertically => "hover:animate-ExpandVertically",
            Self::FadeIn => "hover:animate-FadeIn",
            Self::FadeInDown => "hover:animate-FadeInDown",
            Self::FadeInLeft => "hover:animate-FadeInLeft",
            Self::FadeInRight => "hover:animate-FadeInRight",
            Self::FadeInUp => "hover:animate-FadeInUp",
            Self::FadeOut => "hover:animate-FadeOut",
            Self::FadeOutUp => "hover:animate-FadeOutUp",
            Self::FadeOutDownV2 => "hover:animate-FadeOutDownV2",
            Self::FadeOutLeft => "hover:animate-FadeOutLeft",
            Self::FadeOutRight => "hover:animate-FadeOutRight",
            Self::Flash => "hover:animate-Flash",
            Self::FlashV0 => "hover:animate-FlashV0",
            Self::FlipHorizontal => "hover:animate-FlipHorizontal",
            Self::FlipVertical => "hover:animate-FlipVertical",
            Self::FlipX => "hover:animate-FlipX",
            Self::FlipY => "hover:animate-FlipY",
            Self::FlipInY => "hover:animate-FlipInY",
            Self::FlipInX => "hover:animate-FlipInX",
            Self::FlipOutY => "hover:animate-FlipOutY",
            Self::FlipOutX => "hover:animate-FlipOutX",
            Self::Float => "hover:animate-Float",
            Self::Hang => "hover:animate-Hang",
            Self::Heartbeat => "hover:animate-Heartbeat",
            Self::HorizontalVibration => "hover:animate-HorizontalVibration",
            Self::Jiggle => "hover:animate-Jiggle",
            Self::JiggleV0 => "hover:animate-JiggleV0",
            Self::Jump => "hover:animate-Jump",
            Self::Pop => "hover:animate-Pop",
            Self::PulseCustom => "hover:animate-PulseCustom",
            Self::PulseFadeIn => "hover:animate-PulseFadeIn",
            Self::Rise => "hover:animate-Rise",
            Self::RollIn => "hover:animate-RollIn",
            Self::RollOut => "hover:animate-RollOut",
            Self::Rotate180 => "hover:animate-Rotate180",
            Self::Rotate360 => "hover:animate-Rotate360",
            Self::Rotate90 => "hover:animate-Rotate90",
            Self::RotateIn => "hover:animate-RotateIn",
            Self::RotateOut => "hover:animate-RotateOut",
            Self::RotationalWave => "hover:animate-RotationalWave",
            Self::RubberBand => "hover:animate-RubberBand",
            Self::RubberBandV0 => "hover:animate-RubberBandV0",
            Self::Scale => "hover:animate-Scale",
            Self::Shake => "hover:animate-Shake",
            Self::ShakeV0 => "hover:animate-ShakeV0",
            Self::Sink => "hover:animate-Sink",
            Self::Skew => "hover:animate-Skew",
            Self::SlideDown => "hover:animate-SlideDown",
            Self::SlideDownAndFade => "hover:animate-SlideDownAndFade",
            Self::SlideInBottom => "hover:animate-SlideInBottom",
            Self::SlideInLeft => "hover:animate-SlideInLeft",
            Self::SlideInRight => "hover:animate-SlideInRight",
            Self::SlideInTop => "hover:animate-SlideInTop",
            Self::SlideLeft => "hover:animate-SlideLeft",
            Self::SlideLeftAndFade => "hover:animate-SlideLeftAndFade",
            Self::SlideOutBottom => "hover:animate-SlideOutBottom",
            Self::SlideOutLeft => "hover:animate-SlideOutLeft",
            Self::SlideOutTop => "hover:animate-SlideOutTop",
            Self::SlideRight => "hover:animate-SlideRight",
            Self::SlideRightAndFade => "hover:animate-SlideRightAndFade",
            Self::SlideRotateIn => "hover:animate-SlideRotateIn",
            Self::SlideRotateOut => "hover:animate-SlideRotateOut",
            Self::SlideUp => "hover:animate-SlideUp",
            Self::SlideUpAndFade => "hover:animate-SlideUpAndFade",
            Self::SlideUpFade => "hover:animate-SlideUpFade",
            Self::SpinClockwise => "hover:animate-SpinClockwise",
            Self::SpinCounterClockwise => "hover:animate-SpinCounterClockwise",
            Self::Sway => "hover:animate-Sway",
            Self::Swing => "hover:animate-Swing",
            Self::SwingDropIn => "hover:animate-SwingDropIn",
            Self::SwingV0 => "hover:animate-SwingV0",
            Self::Squeeze => "hover:animate-Squeeze",
            Self::Tada => "hover:animate-Tada",
            Self::TiltHorizontal => "hover:animate-TiltHorizontal",
            Self::Vibrate => "hover:animate-Vibrate",
            Self::Wobble => "hover:animate-Wobble",
            Self::ZoomIn => "hover:animate-ZoomIn",
            Self::ZoomOut => "hover:animate-ZoomOut",
        }
    }
}

#[component]
pub fn Animate(
    #[props(default = AnimateVariant::Default)] variant: AnimateVariant,
    #[props(default = AnimateHoverVariant::Default)] hover_variant: AnimateHoverVariant,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] style: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "flex justify-center items-center w-full",
        variant.class(),
        hover_variant.class(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-name": "Animate", class: "{c}", style: style.as_deref().unwrap_or(""), {children} }
    }
}

#[component]
pub fn AnimateGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("w-full", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "AnimateGroup", class: "{c}", {children} } }
}

#[component]
pub fn AnimateGroupItem(
    #[props(default = AnimateVariant::Default)] variant: AnimateVariant,
    #[props(default = AnimateHoverVariant::Default)] hover_variant: AnimateHoverVariant,
    #[props(into, optional)] class: Option<String>,
    delay_ms: u32,
    #[props(default = "forwards")] fill_mode: &'static str,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "flex justify-center items-center w-full",
        variant.class(),
        hover_variant.class(),
        class.as_deref().unwrap_or("")
    );
    let style = format!("animation-delay: {delay_ms}ms; animation-fill-mode: {fill_mode};");
    rsx! {
        div { "data-name": "AnimateGroupItem", class: "{c}", style: "{style}", {children} }
    }
}
