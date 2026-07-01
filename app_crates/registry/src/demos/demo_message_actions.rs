// TODO PORT: shadcn uses size="icon-xs" for the Retry button and size="icon" for
// the main action buttons. Our Button component has ButtonSize::Icon (size-9) but
// no ButtonSize::IconXs or ButtonSize::IconSm. Using ButtonSize::Icon as fallback
// for icon and ButtonSize::Sm for icon-xs until those sizes are added.
use icons::{Copy, RefreshCcw, ThumbsDown, ThumbsUp};
use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::message::{Message, MessageAlign, MessageContent, MessageFooter};

#[component]
pub fn DemoMessageActions() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Message>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"The install failure is coming from the workspace package."</BubbleContent>
                    </Bubble>
                    <MessageFooter>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Copy"
                            attr:title="Copy"
                        >
                            <Copy />
                        </Button>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Like"
                            attr:title="Like"
                        >
                            <ThumbsUp />
                        </Button>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Dislike"
                            attr:title="Dislike"
                        >
                            <ThumbsDown />
                        </Button>
                    </MessageFooter>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Okay drop me a link. Taking a look..."</BubbleContent>
                    </Bubble>
                    <MessageFooter class="gap-2">
                        <span class="font-normal text-destructive">"Failed to send"</span>
                        // TODO PORT: size="icon-xs" not in ButtonSize — using ButtonSize::Sm
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Sm
                            attr:title="Retry"
                            attr:aria-label="Retry"
                        >
                            <RefreshCcw />
                        </Button>
                    </MessageFooter>
                </MessageContent>
            </Message>
        </div>
    }
}
