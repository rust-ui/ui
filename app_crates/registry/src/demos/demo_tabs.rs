use dioxus::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabs() -> Element {
    rsx! {
        Tabs { default_value: "account",
            TabsList {
                TabsTrigger { value: "account", "Account" }
                TabsTrigger { value: "password", "Password" }
            }
            TabsContent { value: "account",
                p { class: "text-sm text-muted-foreground",
                    "Change your account settings here."
                }
            }
            TabsContent { value: "password",
                p { class: "text-sm text-muted-foreground",
                    "Update your password here."
                }
            }
        }
    }
}
