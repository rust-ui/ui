use icons::Plus;
use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage, AvatarSize};
use crate::ui::button::{Button, ButtonSize};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};

#[component]
pub fn DemoEmptyAvatarGroup() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia>
                    <AvatarGroup>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=alice"
                                attr:alt="@alice"
                            />
                            <AvatarFallback>"AL"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=bob"
                                attr:alt="@bob"
                            />
                            <AvatarFallback>"BO"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=carol"
                                attr:alt="@carol"
                            />
                            <AvatarFallback>"CA"</AvatarFallback>
                        </Avatar>
                    </AvatarGroup>
                </EmptyMedia>
                <EmptyTitle>"No Team Members"</EmptyTitle>
                <EmptyDescription>"Invite your team to collaborate on this project."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button size=ButtonSize::Sm>
                    <Plus />
                    "Invite Members"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
