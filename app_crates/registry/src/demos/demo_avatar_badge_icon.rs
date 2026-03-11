use icons::Plus;
use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatarBadgeIcon() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
            <AvatarFallback>"RS"</AvatarFallback>
            <AvatarBadge>
                <Plus />
            </AvatarBadge>
        </Avatar>
    }
}
