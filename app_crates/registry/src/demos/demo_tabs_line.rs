use dioxus::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[component]
pub fn DemoTabsLine() -> Element {
    rsx! {
        Tabs { default_value: "preview", class: "w-full max-w-sm",
            TabsList { variant: TabsVariant::Line,
                TabsTrigger { value: "preview", "Preview" }
                TabsTrigger { value: "code", "Code" }
                TabsTrigger { value: "output", "Output" }
            }
            TabsContent { value: "preview",
                p { class: "text-muted-foreground", "Live preview of your component." }
            }
            TabsContent { value: "code",
                p { class: "text-muted-foreground", "View and edit the source code." }
            }
            TabsContent { value: "output",
                p { class: "text-muted-foreground", "Compiled output and build logs." }
            }
        }
    }
}
