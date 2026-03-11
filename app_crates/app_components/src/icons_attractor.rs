use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {IconsAttractorList, div, "absolute left-0 top-11 w-full z-6"}
    clx! {AttractorItem, div, "w-[48px] h-[48px]", "absolute left-1/2 -translate-x-1/2 -translate-y-1/2"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */

#[component]
pub fn IconsAttractor(children: Children) -> impl IntoView {
    view! {
        <style>
            {r#"
            @property --attractor-item-scale {
            syntax: "<number>";
            initial-value: 1;
            inherits: true;
            }
            
            [data-name="AttractorItem"] {
            offset-path: path("M 24 24 A 600 375.5 0 0 1 624 399.5");
            offset-distance: 0
            }
            
            [data-name="AttractorItem"][data-direction=right] {
            offset-path: path("M 24 24 A 600 375.5 0 0 1 624 399.5");
            --start-distance: 67%;
            --end-distance: 0%;
            animation: attractorMove 15s linear infinite
            }
            
            [data-name="AttractorItem"][data-direction=left] {
            offset-path: path("M -576 399.5 A 600 375.5 0 0 1 24 24");
            --start-distance: 33%;
            --end-distance: 100%;
            animation: attractorMove 15s linear infinite
            }
            
            [data-name="AttractorItem"] div {
            scale: var(--attractor-item-scale);
            }
            
            @keyframes attractorMove {
            0% {
            opacity: 0;
            filter: blur(4px);
            --attractor-item-scale: 0.5;
            offset-distance: var(--start-distance)
            }
            
            25% {
            filter: blur(0);
            opacity: 1
            }
            
            65% {
            --attractor-item-scale: 1
            }
            
            to {
            offset-distance: var(--end-distance)
            }
            }
            
            "#}
        </style>
        <div data-name="ItemsAttractor" class="container relative pb-24 md:pb-36 overflow-x-clip pt-38 md:pt-42">
            {children()}
        </div>
    }
}

#[component]
pub fn IconsAttractorList(children: Children) -> impl IntoView {
    view! {
        <div class="absolute left-0 top-11 w-full z-6">
            <BackgroundEllipseSvg />
            {children()}
        </div>
    }
}

#[component]
fn BackgroundEllipseSvg() -> impl IntoView {
    view! {
        <svg class="absolute left-1/2 max-w-none -translate-x-1/2" width="1008" height="380" fill="none">
            <ellipse
                class="fill-[url(#:S3-light:)] dark:fill-[url(#:S3-dark:)]"
                cx="503"
                cy="375.5"
                rx="600"
                ry="375.5"
            ></ellipse>
            <path
                class="stroke-[url(#:S4-light:)] dark:stroke-[url(#:S4-dark:)]"
                stroke-opacity=".3"
                stroke-width="1.5"
                d="M1102.25 375.5c0 103.34-66.93 197.01-175.384 264.883C818.425 708.249 668.567 750.25 503 750.25S187.575 708.249 79.134 640.383C-29.318 572.51-96.25 478.84-96.25 375.5c0-103.34 66.932-197.01 175.384-264.883C187.575 42.751 337.433.75 503 .75s315.425 42 423.866 109.867C1035.32 178.49 1102.25 272.16 1102.25 375.5Z"
            ></path>
            <defs>
                // Light mode gradients
                <radialGradient
                    id=":S3-light:"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientTransform="matrix(0 751 -799 0 503 0)"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop offset=".108" stop-color="#EFEFF2"></stop>
                    <stop offset=".33" stop-color="#fff" stop-opacity="0"></stop>
                </radialGradient>
                <radialGradient
                    id=":S4-light:"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientTransform="matrix(0 315 -503.329 0 503 0)"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#BBBBBC"></stop>
                    <stop offset="1" stop-color="#565656" stop-opacity="0"></stop>
                </radialGradient>

                // Dark mode gradients
                <radialGradient
                    id=":S3-dark:"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientTransform="matrix(0 751 -799 0 503 0)"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop offset=".108" stop-color="#1a1a1a"></stop>
                    <stop offset=".33" stop-color="#000" stop-opacity="0"></stop>
                </radialGradient>
                <radialGradient
                    id=":S4-dark:"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientTransform="matrix(0 315 -503.329 0 503 0)"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#555555"></stop>
                    <stop offset="1" stop-color="#aaaaaa" stop-opacity="0"></stop>
                </radialGradient>
            </defs>
        </svg>
    }
}
