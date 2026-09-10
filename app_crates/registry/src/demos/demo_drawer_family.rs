use dioxus::prelude::*;
use icons::{FileText, Lock, TriangleAlert, X};

use crate::ui::drawer::{Drawer, DrawerContent, DrawerTrigger};

#[component]
pub fn DemoDrawerFamily() -> Element {
    rsx! {
        Drawer {
            DrawerTrigger { "Open Drawer" }

            DrawerContent { class: "overflow-hidden right-4 left-4 pb-6 mx-auto mb-4 max-w-[361px] rounded-[36px]",
                header { class: "flex justify-between items-center mb-4 border-b h-[72px] border-neutral-100",
                    h2 { class: "text-lg font-semibold text-foreground", "Options" }
                    button {
                        "data-name": "DrawerClose",
                        class: "flex justify-center items-center rounded-full transition-colors size-8 bg-neutral-100 text-neutral-700 hover:bg-neutral-200",
                        X { class: "size-3" }
                    }
                }

                div { class: "space-y-3",
                    button { class: "flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-neutral-900 hover:bg-neutral-200",
                        Lock { class: "size-[18px]" }
                        span { "View Private Key" }
                    }

                    button { class: "flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-neutral-900 hover:bg-neutral-200",
                        FileText { class: "size-[18px]" }
                        span { "View Recovery Phrase" }
                    }

                    button { class: "flex gap-4 items-center px-4 w-full h-12 text-base font-medium text-red-600 bg-red-50 rounded-2xl transition-colors hover:bg-red-100",
                        TriangleAlert { class: "size-[18px]" }
                        span { "Remove Wallet" }
                    }
                }
            }
        }
    }
}
