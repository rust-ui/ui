use icons::{FileCode, X};
use leptos::prelude::*;

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentState, AttachmentTitle,
};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoAttachment() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 mx-auto w-full max-w-sm">
            <div class="rounded-xl bg-accent p-3">
                <AttachmentGroup>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"
                                alt="Workspace"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"workspace.png"</AttachmentTitle>
                            <AttachmentDescription>"PNG · 820 KB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80"
                                alt="Desk"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"desk-reference.jpg"</AttachmentTitle>
                            <AttachmentDescription>"JPG · 1.1 MB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80"
                                alt="Office"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"office-reference.jpg"</AttachmentTitle>
                            <AttachmentDescription>"JPG · 940 KB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                </AttachmentGroup>
            </div>
            <div class="rounded-xl bg-accent p-3">
                <Attachment state=AttachmentState::Uploading class="w-full">
                    <AttachmentMedia>
                        <Spinner />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"sales-dashboard.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Uploading · 64%"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Cancel upload">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="rounded-xl bg-accent p-3">
                <Attachment class="w-full">
                    <AttachmentMedia>
                        <FileCode />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"message-renderer.tsx"</AttachmentTitle>
                        <AttachmentDescription>"TypeScript · 12 KB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove message-renderer.tsx">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
        </div>
    }
}
