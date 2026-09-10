use dioxus::prelude::*;

const CHIPS_CONTAINER_STYLE: &str = r#"
[data-name="ChipsContainer"] label::after {
    content: "";
    background:
        linear-gradient(230deg, var(--warning) 0 0.96vmin, #fff0 0 100%),
        linear-gradient(142deg, var(--warning) 0 1.12vmin, #fff0 0 100%),
        conic-gradient(from 43deg at 43% calc(64% + 0.24vmin), #fff0 0 0%, var(--warning) 1% 76%, #fff0 77% 100%),
        conic-gradient(from -45deg at 43% 64%, #fff0 0 0%, var(--warning) 2% 25%, #fff0 26% 100%);
}
"#;

#[component]
pub fn ChipsContainer(children: Element) -> Element {
    rsx! {
        style { dangerous_inner_html: CHIPS_CONTAINER_STYLE }
        div {
            "data-name": "ChipsContainer",
            class: "flex relative flex-wrap justify-start content-center w-full transition-all chips__main__container max-w-[66vmin] z-[1] bg-card/5 p-[2.8vmin] rounded-[2vmin] shadow-[inset_0_0_1px_1px_theme(colors.border)] duration-[350ms] ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:transition-all *:duration-[350ms] *:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:before:transition-all *:before:duration-[350ms] *:before:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:after:transition-all *:after:duration-[350ms] *:after:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)]",
            {children}
        }
    }
}

#[component]
pub fn ChipItem(label: &'static str) -> Element {
    rsx! {
        label {
            "data-name": "ChipItem",
            class: "flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-muted rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-muted-foreground m-[1vmin] shadow-[inset_0_0_0_1px_theme(colors.border)] has-[:checked]:shadow-[inset_0_0_0_2px_var(--warning)] has-[:checked]:text-[var(--warning)] has-[:checked]:bg-warning after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]",
            span { "{label}" }
            input { r#type: "checkbox", id: label, class: "hidden" }
        }
    }
}
