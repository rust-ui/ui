use dioxus::prelude::*;

#[component]
pub fn DemoSliderControlled() -> Element {
    let mut value = use_signal(|| 50_i32);

    rsx! {
        div { class: "flex flex-col gap-4 w-72",
            div { class: "flex justify-between items-center",
                span { class: "text-sm font-medium", "Volume" }
                span { class: "text-sm tabular-nums text-muted-foreground", "{value}" }
            }
            input {
                r#type: "range",
                min: "0",
                max: "100",
                step: "1",
                value: "{value}",
                class: "overflow-hidden relative bg-transparent transition-all duration-100 ease-in-out appearance-none text-[1.5rem] w-[12.5em] text-primary active:cursor-grabbing",
                oninput: move |ev| {
                    if let Ok(v) = ev.value().parse::<i32>() {
                        value.set(v);
                    }
                },
            }
        }
    }
}
