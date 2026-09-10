use dioxus::prelude::*;

#[component]
pub fn DemoResizable() -> Element {
    rsx! {
        style {
            r#".direction__rtl {{
            direction: rtl;
            }}

            #behind-scene-toggle:checked ~ .wrapper .resizer {{
            opacity: 1;
            }}"#
        }

        div {
            input { r#type: "checkbox", id: "behind-scene-toggle", class: "hidden" }
            div { class: "flex h-screen",
                div { class: "overflow-y-scroll flex-1 p-2.5 h-screen",
                    h1 { class: "text-3xl font-bold lg:text-4xl text-pretty",
                        "CSS-only resizable panels"
                    }
                    p {
                        "This is an example of a CSS-only resizeable panels solution. It uses a combination of the flexbox model, CSS "
                        code { "resize" }
                        "property, and "
                        code { "position: absolute" }
                        "for the resized panel contents."
                    }
                    h2 { class: "text-2xl font-bold lg:text-3xl text-pretty", "No JavaScript!" }
                    p { "This is a pure CSS solution!" }
                    h2 { class: "text-2xl font-bold lg:text-3xl text-pretty", "Browser support" }
                    p {
                        a { href: "https://caniuse.com/css-resize",
                            "All browsers supporting "
                            code { "resize" }
                        }
                    }
                }
                div { class: "relative bg-red-200 aside min-w-[300px]",
                    div { class: "flex overflow-y-scroll absolute top-0 bottom-0 right-2.5 left-2.5 flex-col m-0 aside-inner z-2",
                        h1 { class: "text-3xl font-bold lg:text-4xl text-pretty", "Side Panel" }
                        p {
                            "Drag left/right the "
                            span { class: "resize-icon", "⇆" }
                            " handle in order to resize this panel!"
                        }
                        label { class: "cursor-pointer", r#for: "behind-scene-toggle",
                            "Click here to see behind the scene!"
                        }
                    }
                    div { class: "inline-block absolute top-1/2 p-0.5 m-0 h-3 leading-3 text-white bg-black rounded-full resize-icon margin-left-[-7px]",
                        "⇆"
                    }
                    div { class: "overflow-hidden relative top-1/2 w-full h-3 bg-red-500 opacity-0 resize-x margin-left-[-7px] direction__rtl resizer cursor-ew-resize" }
                }
            }
        }
    }
}
