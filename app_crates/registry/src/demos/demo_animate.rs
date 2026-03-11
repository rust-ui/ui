use leptos::prelude::*;

use crate::ui::animate::{Animate, AnimateHoverVariant};
use crate::ui::card::{Card, CardDescription};

#[component]
pub fn DemoAnimate() -> impl IntoView {
    view! {
        <div class="my-4 space-y-6">

            <h3 class="text-2xl text-center">{format!("Hover animations ({})", HOVER_ANIMATIONS.len())}</h3>

            <div data-name="Grid3" class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {HOVER_ANIMATIONS
                    .iter()
                    .map(|variant| {
                        view! {
                            <Card class="flex flex-col gap-4 items-center">
                                <Animate hover_variant=*variant>
                                    <AnimatedChildren />
                                </Animate>
                                <CardDescription>{variant.to_string()}</CardDescription>
                            </Card>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

// * 💁 Could be literally anything you want to animate :)
#[component]
fn AnimatedChildren() -> impl IntoView {
    view! { <div class="p-4 rounded-lg bg-primary/30 size-20" style="animation-iteration-count: infinite;" /> }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const HOVER_ANIMATIONS: &[AnimateHoverVariant] = &[
    AnimateHoverVariant::Blink,
    AnimateHoverVariant::BlurredFadeIn,
    AnimateHoverVariant::BounceFadeIn,
    AnimateHoverVariant::BounceHorizontal,
    AnimateHoverVariant::BounceVertical,
    AnimateHoverVariant::BounceCustom,
    AnimateHoverVariant::ContractHorizontally,
    AnimateHoverVariant::ContractVertically,
    AnimateHoverVariant::ExpandHorizontally,
    AnimateHoverVariant::ExpandVertically,
    AnimateHoverVariant::FadeIn,
    AnimateHoverVariant::FadeInDown,
    AnimateHoverVariant::FadeInLeft,
    AnimateHoverVariant::FadeInRight,
    AnimateHoverVariant::FadeInUp,
    AnimateHoverVariant::FadeOut,
    AnimateHoverVariant::FadeOutUp,
    AnimateHoverVariant::FadeOutDownV2,
    AnimateHoverVariant::FadeOutLeft,
    AnimateHoverVariant::FadeOutRight,
    AnimateHoverVariant::Flash,
    AnimateHoverVariant::FlashV0,
    AnimateHoverVariant::FlipHorizontal,
    AnimateHoverVariant::FlipVertical,
    AnimateHoverVariant::FlipX,
    AnimateHoverVariant::FlipY,
    AnimateHoverVariant::FlipInY,
    AnimateHoverVariant::FlipInX,
    AnimateHoverVariant::FlipOutY,
    AnimateHoverVariant::FlipOutX,
    AnimateHoverVariant::Float,
    AnimateHoverVariant::Hang,
    AnimateHoverVariant::Heartbeat,
    AnimateHoverVariant::HorizontalVibration,
    AnimateHoverVariant::Jiggle,
    AnimateHoverVariant::JiggleV0,
    AnimateHoverVariant::Jump,
    AnimateHoverVariant::Pop,
    AnimateHoverVariant::PulseCustom,
    AnimateHoverVariant::PulseFadeIn,
    AnimateHoverVariant::Rise,
    AnimateHoverVariant::RollIn,
    AnimateHoverVariant::RollOut,
    AnimateHoverVariant::Rotate180,
    AnimateHoverVariant::Rotate360,
    AnimateHoverVariant::Rotate90,
    AnimateHoverVariant::RotateIn,
    AnimateHoverVariant::RotateOut,
    AnimateHoverVariant::RotationalWave,
    AnimateHoverVariant::RubberBand,
    AnimateHoverVariant::RubberBandV0,
    AnimateHoverVariant::Scale,
    AnimateHoverVariant::Shake,
    AnimateHoverVariant::ShakeV0,
    AnimateHoverVariant::Sink,
    AnimateHoverVariant::Skew,
    AnimateHoverVariant::SlideDown,
    AnimateHoverVariant::SlideDownAndFade,
    AnimateHoverVariant::SlideInBottom,
    AnimateHoverVariant::SlideInLeft,
    AnimateHoverVariant::SlideInRight,
    AnimateHoverVariant::SlideInTop,
    AnimateHoverVariant::SlideLeft,
    AnimateHoverVariant::SlideLeftAndFade,
    AnimateHoverVariant::SlideOutBottom,
    AnimateHoverVariant::SlideOutLeft,
    AnimateHoverVariant::SlideOutTop,
    AnimateHoverVariant::SlideRight,
    AnimateHoverVariant::SlideRightAndFade,
    AnimateHoverVariant::SlideRotateIn,
    AnimateHoverVariant::SlideRotateOut,
    AnimateHoverVariant::SlideUp,
    AnimateHoverVariant::SlideUpAndFade,
    AnimateHoverVariant::SlideUpFade,
    AnimateHoverVariant::SpinClockwise,
    AnimateHoverVariant::SpinCounterClockwise,
    AnimateHoverVariant::Sway,
    AnimateHoverVariant::Swing,
    AnimateHoverVariant::SwingDropIn,
    AnimateHoverVariant::SwingV0,
    AnimateHoverVariant::Squeeze,
    AnimateHoverVariant::Tada,
    AnimateHoverVariant::TiltHorizontal,
    AnimateHoverVariant::Vibrate,
    AnimateHoverVariant::Wobble,
    AnimateHoverVariant::ZoomIn,
    AnimateHoverVariant::ZoomOut,
];
