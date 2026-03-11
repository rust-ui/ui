use icons::ChevronDown;
use leptos::prelude::*;

use crate::ui::card::{Card, CardContent};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn DemoCollapsibleBasic() -> impl IntoView {
    view! {
        <Card class="mx-auto w-full max-w-sm">
            <CardContent>
                <Collapsible class="rounded-md data-[state=open]:bg-muted">
                    <CollapsibleTrigger class="flex gap-2 items-center py-2 px-3 w-full text-sm font-medium rounded-md transition-colors group hover:bg-accent hover:text-accent-foreground">
                        "Product details"
                        <ChevronDown class="ml-auto transition-transform duration-300 size-4 group-data-[state=open]:rotate-180" />
                    </CollapsibleTrigger>
                    <CollapsibleContent class="flex flex-col gap-2 items-start p-2.5 pt-0 text-sm">
                        <p>"This panel can be expanded or collapsed to reveal additional content."</p>
                    </CollapsibleContent>
                </Collapsible>
            </CardContent>
        </Card>
    }
}
