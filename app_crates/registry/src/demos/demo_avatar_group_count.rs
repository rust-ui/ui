use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};

#[component]
pub fn DemoAvatarGroupCount() -> impl IntoView {
    view! {
        <AvatarGroup>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=alice" attr:alt="@alice" />
                <AvatarFallback>"AL"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=bob" attr:alt="@bob" />
                <AvatarFallback>"BO"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=carol" attr:alt="@carol" />
                <AvatarFallback>"CA"</AvatarFallback>
            </Avatar>
            <AvatarGroupCount>"+3"</AvatarGroupCount>
        </AvatarGroup>
    }
}
