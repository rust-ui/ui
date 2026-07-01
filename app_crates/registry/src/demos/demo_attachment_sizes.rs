use icons::FileText;
use leptos::prelude::*;

use crate::ui::attachment::{
    Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentSize, AttachmentTitle,
};

#[component]
pub fn DemoAttachmentSizes() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 mx-auto w-full max-w-sm">
            <Attachment size=AttachmentSize::Default class="w-full">
                <AttachmentMedia>
                    <FileText />
                </AttachmentMedia>
                <AttachmentContent>
                    <AttachmentTitle>"Default attachment"</AttachmentTitle>
                    <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
                </AttachmentContent>
            </Attachment>
            <Attachment size=AttachmentSize::Sm class="w-full">
                <AttachmentMedia>
                    <FileText />
                </AttachmentMedia>
                <AttachmentContent>
                    <AttachmentTitle>"Small attachment"</AttachmentTitle>
                    <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
                </AttachmentContent>
            </Attachment>
            <Attachment size=AttachmentSize::Xs class="w-full">
                <AttachmentMedia>
                    <FileText />
                </AttachmentMedia>
                <AttachmentContent>
                    <AttachmentTitle>"Extra small attachment"</AttachmentTitle>
                </AttachmentContent>
            </Attachment>
        </div>
    }
}
