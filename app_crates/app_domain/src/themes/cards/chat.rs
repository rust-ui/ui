use icons::{Plus, Send};
use leptos::prelude::*;
use leptos_ui::clx;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardFooter, CardHeader};
use registry::ui::input::Input;

#[component]
pub fn CardChat() -> impl IntoView {
    clx! {ChatBubble, div, "flex flex-col gap-2 py-2 px-3 w-max text-sm rounded-lg max-w-[75%] bg-muted data-[variant=primary]:bg-primary data-[variant=primary]:text-primary-foreground data-[variant=primary]:ml-auto"}

    view! {
        <Card>
            <CardHeader class="flex flex-row items-center">
                <div class="flex items-center space-x-4">
                    <Avatar class="size-10">
                        <AvatarFallback>SD</AvatarFallback>
                    </Avatar>
                    <div>
                        <p class="text-sm font-medium leading-none">Sofia Davis</p>
                        <p class="text-sm text-muted-foreground">m@example.com</p>
                    </div>
                </div>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Icon class="ml-auto rounded-full">
                    <Plus />
                    <span class="hidden">New message</span>
                </Button>
            </CardHeader>
            <CardContent>
                <div class="space-y-4">
                    <ChatBubble>Hi, how can I help you today?</ChatBubble>
                    <ChatBubble attr:data-variant="primary">"Hey, I'm having trouble with my account."</ChatBubble>
                    <ChatBubble>What seems to be the problem?</ChatBubble>
                    <ChatBubble attr:data-variant="primary">I can not log in.</ChatBubble>
                </div>
            </CardContent>
            <CardFooter>
                <form class="flex items-center space-x-2 w-full">
                    <Input
                        class="flex-1"
                        attr:id="message"
                        attr:placeholder="Type your message..."
                        attr:autocomplete="off"
                        attr:value=""
                    />
                    <Button size=ButtonSize::Icon attr:r#type="submit" attr:disabled>
                        <Send />
                        <span class="hidden">Send</span>
                    </Button>
                </form>
            </CardFooter>
        </Card>
    }
}
