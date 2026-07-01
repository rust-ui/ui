// TODO PORT: This demo requires Component 4 (Attachment). The full shadcn demo shows:
//   - Message align=end with a vertical image Attachment + Bubble asking to add to PDF
//   - Message with a muted Bubble reply + PDF file Attachment with Download action
//   - Message align=end with a plain "Thanks. Looks good." Bubble
// Using simplified placeholders below until attachment.rs is implemented.
// Shadcn source: examples/radix/message-attachment.tsx
use icons::{Download, FileText};
use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageContent};

#[component]
pub fn DemoMessageAttachment() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Message align=MessageAlign::End>
                <MessageContent>
                    // TODO PORT: Replace with <Attachment orientation="vertical"> + <AttachmentMedia variant="image"><img/></AttachmentMedia>
                    <div class="flex overflow-hidden flex-col w-24 rounded-xl border bg-muted">
                        <div class="flex justify-center items-center w-full aspect-square text-muted-foreground">
                            "📷"
                        </div>
                    </div>
                    <Bubble>
                        <BubbleContent>
                            "Here's the image. Can you add it to the PDF? Use it for the cover page."
                        </BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"Done. Here's the PDF with the image added as the cover page."</BubbleContent>
                    </Bubble>
                    // TODO PORT: Replace with <Attachment> + <AttachmentMedia> + <AttachmentContent> + <AttachmentActions>
                    <div class="flex gap-2 items-center py-2 px-2.5 text-sm rounded-xl border bg-card">
                        <FileText />
                        <div class="flex flex-col flex-1 min-w-0 leading-tight">
                            <span class="font-medium truncate">"sales-dashboard.pdf"</span>
                            <span class="text-xs truncate text-muted-foreground">"PDF · 2.4 MB"</span>
                        </div>
                        <Download class="size-4 text-muted-foreground" />
                    </div>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Thanks. Looks good."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
        </div>
    }
}
