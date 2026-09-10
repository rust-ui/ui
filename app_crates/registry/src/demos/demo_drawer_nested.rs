use dioxus::prelude::*;

use crate::ui::drawer::DrawerHandle;

#[component]
pub fn DemoDrawerNested() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/drawer_nested.css" }

        div {
            div { class: "text-center",
                h1 { class: "mb-4 text-4xl font-bold text-foreground", "Nested Drawer Demo" }
                p { class: "mb-8 text-muted-foreground", "Click the button below to open the drawer" }
                button {
                    id: "drawer-trigger",
                    class: "py-3 px-6 font-medium rounded-lg transition-opacity hover:opacity-90 bg-primary text-primary-foreground",
                    "Open Drawer"
                }
            }

            div {
                id: "drawer-overlay",
                class: "hidden drawer-overlay",
                "data-vaul-overlay": "",
                "data-vaul-snap-points": "false",
                "data-vaul-animate": "true",
                "data-state": "closed",
            }

            div {
                id: "drawer-content",
                class: "hidden bg-white drawer-content rounded-t-[10px]",
                "data-vaul-drawer": "",
                "data-vaul-drawer-direction": "bottom",
                "data-vaul-snap-points": "false",
                "data-vaul-animate": "true",
                "data-state": "closed",
                style: "--initial-transform: 100%;",

                div { class: "flex overflow-y-auto flex-col flex-1 gap-4 p-6",
                    DrawerHandle {}

                    h2 { class: "text-2xl font-bold text-foreground", "Parent Drawer" }
                    p { class: "text-muted-foreground",
                        "Click the button below to open a nested drawer and see the parent drawer scale down."
                    }

                    button {
                        id: "open-nested-drawer",
                        class: "py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-secondary text-secondary-foreground focus:ring-secondary",
                        "Open Nested Drawer"
                    }

                    button {
                        id: "drawer-close",
                        class: "py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground focus:ring-primary",
                        "Close"
                    }
                }
            }

            div {
                id: "nested-drawer-overlay",
                class: "hidden drawer-overlay",
                "data-vaul-overlay": "",
                "data-vaul-snap-points": "false",
                "data-vaul-animate": "true",
                "data-state": "closed",
            }

            div {
                id: "nested-drawer-content",
                class: "hidden bg-white drawer-content rounded-t-[10px]",
                "data-vaul-drawer": "",
                "data-vaul-drawer-direction": "bottom",
                "data-vaul-snap-points": "false",
                "data-vaul-animate": "true",
                "data-state": "closed",
                style: "--initial-transform: 100%;",

                div { class: "flex overflow-y-auto flex-col flex-1 gap-4 p-6",
                    h2 { class: "text-2xl font-bold text-foreground", "Nested Drawer" }
                    p { class: "text-muted-foreground",
                        "Notice how the parent drawer scales down when this nested drawer opens."
                    }

                    button {
                        id: "nested-drawer-close",
                        class: "py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground focus:ring-primary",
                        "Close Nested Drawer"
                    }
                }
            }
        }

        script { r#type: "module", src: "/drawer_v3.js" }
    }
}
