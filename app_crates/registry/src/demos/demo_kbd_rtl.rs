use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::kbd::{Kbd, KbdGroup};

#[component]
pub fn DemoKbdRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex flex-col gap-4 items-center">
                <div class="flex gap-2 items-center text-sm text-muted-foreground">
                    <span>"بحث:"</span>
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"K"</Kbd>
                    </KbdGroup>
                </div>
                <div class="flex gap-2 items-center text-sm text-muted-foreground">
                    <span>"غامق:"</span>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <span>"+"</span>
                        <Kbd>"B"</Kbd>
                    </KbdGroup>
                </div>
            </div>
        </DirectionProvider>
    }
}
