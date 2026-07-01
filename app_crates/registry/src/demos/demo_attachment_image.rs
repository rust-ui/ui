use icons::X;
use leptos::prelude::*;

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation, AttachmentTitle, AttachmentTrigger,
};

#[component]
pub fn DemoAttachmentImage() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-sm">
            <div class="rounded-xl bg-accent p-3">
                <AttachmentGroup class="w-full">
                    <Attachment orientation=AttachmentOrientation::Vertical>
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
                        <AttachmentActions>
                            <AttachmentAction attr:aria-label="Remove workspace.png">
                                <X />
                            </AttachmentAction>
                        </AttachmentActions>
                        <AttachmentTrigger
                            href="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"
                            attr:target="_blank"
                            attr:rel="noreferrer"
                            attr:aria-label="Open workspace.png"
                        />
                    </Attachment>
                    <Attachment orientation=AttachmentOrientation::Vertical>
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
                        <AttachmentActions>
                            <AttachmentAction attr:aria-label="Remove desk-reference.jpg">
                                <X />
                            </AttachmentAction>
                        </AttachmentActions>
                        <AttachmentTrigger
                            href="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80"
                            attr:target="_blank"
                            attr:rel="noreferrer"
                            attr:aria-label="Open desk-reference.jpg"
                        />
                    </Attachment>
                    <Attachment orientation=AttachmentOrientation::Vertical>
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
                        <AttachmentActions>
                            <AttachmentAction attr:aria-label="Remove office-reference.jpg">
                                <X />
                            </AttachmentAction>
                        </AttachmentActions>
                        <AttachmentTrigger
                            href="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80"
                            attr:target="_blank"
                            attr:rel="noreferrer"
                            attr:aria-label="Open office-reference.jpg"
                        />
                    </Attachment>
                </AttachmentGroup>
            </div>
        </div>
    }
}
