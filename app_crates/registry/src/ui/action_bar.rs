use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn LiquidPointerIndicator(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "block overflow-hidden absolute w-12 h-20 bg-transparent border border-white pointer-events-none mt-[calc(anchor-size(height)*-0.5)] rounded-[2rem]",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}" } }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ActionBar(children: Element) -> Element {
    rsx! {
        style {
            "
            /* CSS-only selected state using radio buttons */
            input[type=\"radio\"]:checked + [data-name=\"ActionBarButton\"] {{
            background-color: #fcebeb;
            color: red;
            anchor-name: --action-bar-selected;
            }}
            "
        }

        div {
            "data-name": "ActionBar",
            class: "flex items-center p-2 rounded-2xl border shadow-lg border-input bg-[#fcfcfc]",
            {children}
            SvgFilter {}
        }

        script { r#type: "module", src: "/app_components/action_bar.js" }
    }
}

// TODO: Not working yet
#[component]
pub fn ActionBarButton(children: Element, target: &'static str) -> Element {
    const CLASS_LABEL: &str = "flex relative justify-center items-center mx-1 bg-transparent border-0 duration-300 cursor-pointer outline-none action__bar__button px-[15px] py-[10px] before:inset-[-0.4rem] rounded-[50px] transition-[background-color,color] ease-[ease] has-[:checked]:hover:bg-[#fcebeb] has-[:checked]:focus:bg-[#fcebeb] before:content-[''] before:absolute hover:bg-[#f5f5f5] focus:bg-[#f5f5f5]";

    rsx! {
        // sr-only: hidden
        input { r#type: "radio", id: target, name: "action", class: "hidden" }
        label { "data-name": "ActionBarButton", r#for: target, class: "{CLASS_LABEL}",
            {children}
        }
    }
}

#[component]
fn SvgFilter() -> Element {
    rsx! {
        svg {
            "data-name": "DisplacementFilterSVG",
            width: "0",
            height: "0",
            filter {
                id: "filter",
                "color-interpolation-filters": "linearRGB",
                "filterUnits": "objectBoundingBox",
                "primitiveUnits": "userSpaceOnUse",
                feDisplacementMap {
                    _in: "SourceGraphic",
                    in2: "SourceGraphic",
                    scale: "5",
                    "xChannelSelector": "A",
                    "yChannelSelector": "A",
                    x: "5",
                    y: "-5",
                    width: "100%",
                    height: "100%",
                    result: "displacementMap",
                }
            }
        }
    }
}
