use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoDialogStickyFooter() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open Dialog"</DialogTrigger>

            <DialogContent class="sm:max-w-[480px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Edit Profile"</DialogTitle>
                        <DialogDescription>
                            "Update your account information. Scroll down to see all fields."
                        </DialogDescription>
                    </DialogHeader>

                    // Scrollable middle — bleeds to dialog edges with -mx-6 px-6
                    <div class="overflow-y-auto px-6 -mx-6 max-h-[50vh]">
                        <div class="flex flex-col gap-4 pb-2">
                            <div class="grid grid-cols-2 gap-4">
                                <div class="flex flex-col gap-2">
                                    <Label html_for="first-name">"First name"</Label>
                                    <Input attr:id="first-name" attr:placeholder="Max" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <Label html_for="last-name">"Last name"</Label>
                                    <Input attr:id="last-name" attr:placeholder="Wells" />
                                </div>
                            </div>

                            <div class="flex flex-col gap-2">
                                <Label html_for="username">"Username"</Label>
                                <Input attr:id="username" attr:placeholder="@maxwells" />
                            </div>

                            <div class="flex flex-col gap-2">
                                <Label html_for="email">"Email"</Label>
                                <Input attr:id="email" attr:r#type="email" attr:placeholder="max@example.com" />
                            </div>

                            <div class="flex flex-col gap-2">
                                <Label html_for="bio">"Bio"</Label>
                                <Textarea
                                    attr:id="bio"
                                    attr:placeholder="Tell us a little about yourself..."
                                    attr:rows="3"
                                />
                            </div>

                            <div class="flex flex-col gap-2">
                                <Label html_for="website">"Website"</Label>
                                <Input attr:id="website" attr:placeholder="https://example.com" />
                            </div>

                            <div class="flex flex-col gap-2">
                                <Label html_for="location">"Location"</Label>
                                <Input attr:id="location" attr:placeholder="San Francisco, CA" />
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div class="flex flex-col gap-2">
                                    <Label html_for="twitter">"Twitter"</Label>
                                    <Input attr:id="twitter" attr:placeholder="@handle" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <Label html_for="github">"GitHub"</Label>
                                    <Input attr:id="github" attr:placeholder="username" />
                                </div>
                            </div>
                        </div>
                    </div>

                    // Footer stays pinned below the scrollable area
                    <DialogFooter>
                        <DialogClose>"Cancel"</DialogClose>
                        <Button>"Save changes"</Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}
