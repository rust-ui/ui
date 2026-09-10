use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SliderVariant {
    #[default]
    Round,
    Flat,
}

impl SliderVariant {
    fn as_str(&self) -> &'static str {
        match self {
            SliderVariant::Round => "Round",
            SliderVariant::Flat => "Flat",
        }
    }
}

#[component]
pub fn Slider(
    #[props(into, optional)] class: Option<String>,
    #[props(default)] variant: SliderVariant,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = 50.0)] value: f64,
    #[props(optional)] disabled: bool,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged = tw_merge!(
        "overflow-hidden relative bg-transparent transition-all duration-100 ease-in-out appearance-none disabled:opacity-30 disabled:cursor-not-allowed text-[1.5rem] w-[12.5em] text-primary active:cursor-grabbing disabled:grayscale",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        input {
            "data-name": "Slider",
            "data-variant": "{variant.as_str()}",
            r#type: "range",
            class: "{merged}",
            min: "{min}",
            max: "{max}",
            step: "{step}",
            value: "{value}",
            disabled: disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}
