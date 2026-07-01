use icons::{Copy, FileSearch, X};
use leptos::prelude::*;

use crate::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentTitle, AttachmentTrigger,
};
use crate::ui::dialog::{Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogHeader, DialogTitle};

#[component]
pub fn DemoAttachmentTrigger() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-sm">
            // AttachmentTrigger auto-detects the Dialog context and sets data-dialog-trigger.
            <Dialog>
                <Attachment class="w-full">
                    <AttachmentMedia>
                        <FileSearch />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"research-summary.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Open preview dialog"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Copy link">
                            <Copy />
                        </AttachmentAction>
                        <AttachmentAction attr:aria-label="Remove research-summary.pdf">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                    <AttachmentTrigger attr:aria-label="Preview research-summary.pdf" />
                </Attachment>

                <DialogContent class="sm:max-w-[425px]">
                    <DialogBody>
                        <DialogHeader>
                            <DialogTitle>"research-summary.pdf"</DialogTitle>
                            <DialogDescription>
                                "The attachment trigger fills the card and opens this dialog, while the actions stay independently clickable above it."
                            </DialogDescription>
                        </DialogHeader>
                        <DialogClose>"Close"</DialogClose>
                    </DialogBody>
                </DialogContent>
            </Dialog>
        </div>
    }
}
