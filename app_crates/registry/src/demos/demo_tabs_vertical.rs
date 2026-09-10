use dioxus::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger};

#[component]
pub fn DemoTabsVertical() -> Element {
    rsx! {
        Tabs { default_value: "general", orientation: TabsOrientation::Vertical, class: "w-full max-w-md",
            TabsList {
                TabsTrigger { value: "general", "General" }
                TabsTrigger { value: "security", "Security" }
                TabsTrigger { value: "notifications", "Notifications" }
            }
            TabsContent { value: "general",
                p { class: "text-muted-foreground", "Configure general application settings." }
            }
            TabsContent { value: "security",
                p { class: "text-muted-foreground", "Manage your security preferences and two-factor authentication." }
            }
            TabsContent { value: "notifications",
                p { class: "text-muted-foreground", "Control how and when you receive notifications." }
            }
        }
    }
}
