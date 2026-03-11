use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAvatarRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="flex gap-3 items-center max-w-fit">
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"م ع"</AvatarFallback>
            </Avatar>
            <span class="text-sm font-medium">"محمد علي"</span>
        </DirectionProvider>
    }
}
