use icons::{Check, Clock, FileText, FileWarning, RefreshCw, X};
use leptos::prelude::*;

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentState, AttachmentTitle,
};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoAttachmentStates() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 py-12 mx-auto w-full max-w-sm">
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Idle class="w-full">
                    <AttachmentMedia>
                        <Clock />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"selected-file.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Ready to upload"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove selected-file.pdf">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Uploading class="w-full">
                    <AttachmentMedia>
                        <Spinner />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"design-system.zip"</AttachmentTitle>
                        <AttachmentDescription>"Uploading · 64%"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Cancel upload">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Processing class="w-full">
                    <AttachmentMedia>
                        <FileText />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"market-research.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Processing document"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove market-research.pdf">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Error class="w-full">
                    <AttachmentMedia>
                        <FileWarning />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"financial-model.xlsx"</AttachmentTitle>
                        <AttachmentDescription>"Upload failed. Try again."</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Retry upload">
                            <RefreshCw />
                        </AttachmentAction>
                        <AttachmentAction attr:aria-label="Remove financial-model.xlsx">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Done class="w-full">
                    <AttachmentMedia>
                        <Check />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"uploaded-report.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Uploaded · 1.8 MB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove uploaded-report.pdf">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
        </div>
    }
}
