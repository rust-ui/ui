use dioxus::prelude::*;

#[component]
pub fn DemoSheetExperimental() -> Element {
    // TODO: `anchor` attribute is still an experimental feature:
    // See: https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/anchor
    const TARGET_ID: &str = "sheet__target";
    const ANCHOR_ID: &str = "menu__anchor";

    rsx! {
        style {
            r#"nav.my__sheet[popover] {{
            transform: translateX(-100%);
            }}

            nav.my__sheet:popover-open {{
            transform: translateX(0);
            }}"#
        }

        div {
            button {
                class: "flex items-center text-2xl bg-none border-none cursor-pointer",
                id: ANCHOR_ID,
                popovertarget: TARGET_ID,
                popovertargetaction: "toggle",
                "aria-label": "Open settings my_sheet",
                "☰"
            }

            nav {
                id: "sheet__target",
                "anchor": "menu__anchor",
                class: "flex fixed inset-y-0 left-0 z-50 flex-col justify-between p-4 border-r transition-transform duration-300 ease-in-out my__sheet w-[320px] h-[100dvh] bg-neutral-200 border-r-gray-200",
                popover: "",

                div { class: "relative",
                    button {
                        class: "absolute top-2 right-2 p-2 rounded-lg border cursor-pointer text-neutral-500 border-neutral-300",
                        popovertarget: "sheet__target",
                        popovertargetaction: "hide",
                        "X"
                    }

                    h2 { class: "text-2xl font-bold lg:text-3xl text-pretty", "Workspace Settings" }

                    ul { class: "mt-6 space-y-2",
                        li {
                            a { href: "#", "Profile" }
                        }
                        li {
                            a { href: "#", "Billing" }
                        }
                        li {
                            a { href: "#", "Team" }
                        }
                        li {
                            a { href: "#", "Keyboard Shortcuts" }
                        }
                    }
                }
                footer { class: "text-right",
                    button {
                        class: "py-2 px-4 text-white rounded-md bg-sky-500",
                        popovertarget: "sheet__target",
                        popovertargetaction: "hide",
                        "Close"
                    }
                }
            }
        }
    }
}
