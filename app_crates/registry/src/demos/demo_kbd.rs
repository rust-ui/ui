use leptos::prelude::*;

use crate::ui::kbd::{Kbd, KbdGroup};

#[component]
pub fn DemoKbd() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center">
            <KbdGroup>
                <Kbd>"⌘"</Kbd>
                <Kbd>"⇧"</Kbd>
                <Kbd>"⌥"</Kbd>
                <Kbd>"⌃"</Kbd>
            </KbdGroup>
            <KbdGroup>
                <Kbd>"Ctrl"</Kbd>
                <span>"+"</span>
                <Kbd>"B"</Kbd>
            </KbdGroup>
        </div>
    }
}
