use icons::Search;
use leptos::prelude::*;

use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoEmptyInputGroup() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyTitle>"404 — Not Found"</EmptyTitle>
                <EmptyDescription>
                    "The page you're looking for doesn't exist. Try searching for what you need below."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <InputGroup class="sm:w-3/4">
                    <InputGroupInput placeholder="Try searching for pages..." />
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                    <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                        <Kbd>"/"</Kbd>
                    </InputGroupAddon>
                </InputGroup>
                <EmptyDescription>
                    "Need help? "
                    <a href="#" class="underline transition-colors underline-offset-4 hover:text-foreground">
                        "Contact support"
                    </a>
                </EmptyDescription>
            </EmptyContent>
        </Empty>
    }
}
