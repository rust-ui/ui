use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};

#[component]
pub fn DemoAvatarSize() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 items-center">
            <Avatar size=AvatarSize::Sm>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
            <Avatar size=AvatarSize::Lg>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
        </div>
    }
}
