use leptos::prelude::*;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const PERMISSIONS: [&str; 2] = ["Can Edit", "Can View"];

#[component]
pub fn CardShareThisDocument() -> impl IntoView {
    let olivia_permission = RwSignal::new(Some(PERMISSIONS[0]));
    let isabella_permission = RwSignal::new(Some(PERMISSIONS[0]));
    let sofia_permission = RwSignal::new(Some(PERMISSIONS[1]));

    view! {
        <Card>
            <CardHeader>
                <CardTitle>Share this document</CardTitle>
                <CardDescription>Anyone with the link can view this document.</CardDescription>
            </CardHeader>
            <CardContent>
                <div class="flex space-x-2">
                    // sr-only: hidden
                    <Label class="hidden" html_for="link">
                        Link
                    </Label>
                    <Input attr:id="link" attr:readonly attr:value="http://example.com/link/to/document" />
                    <Button variant=ButtonVariant::Secondary class="shrink-0">
                        Copy Link
                    </Button>
                </div>
                <div data-orientation="horizontal" role="none" class="my-4 w-full shrink-0 bg-border h-[1px]"></div>
                <div class="space-y-4">
                    <h3 class="text-sm font-medium">People with access</h3>
                    <div class="grid gap-6">
                        <div class="flex justify-between items-center space-x-4">
                            <div class="flex items-center space-x-4">
                                <Avatar class="size-10">
                                    <AvatarFallback>OM</AvatarFallback>
                                </Avatar>
                                <div>
                                    <p class="text-sm font-medium leading-none">Olivia Martin</p>
                                    <p class="text-sm text-muted-foreground">m@example.com</p>
                                </div>
                            </div>
                            <Select>
                                <SelectTrigger class="ml-auto w-[110px]">
                                    <SelectValue placeholder=PERMISSIONS[0] />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        {PERMISSIONS
                                            .into_iter()
                                            .map(|permission| {
                                                view! {
                                                    <SelectOption
                                                        aria_selected=Signal::derive(move || {
                                                            olivia_permission.get() == Some(permission)
                                                        })
                                                        value=permission
                                                        on:click=move |_| olivia_permission.set(Some(permission))
                                                    >
                                                        {permission}
                                                    </SelectOption>
                                                }
                                            })
                                            .collect_view()}
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                        <div class="flex justify-between items-center space-x-4">
                            <div class="flex items-center space-x-4">
                                <Avatar class="size-10">
                                    <AvatarFallback>IN</AvatarFallback>
                                </Avatar>
                                <div>
                                    <p class="text-sm font-medium leading-none">Isabella Nguyen</p>
                                    <p class="text-sm text-muted-foreground">b@example.com</p>
                                </div>
                            </div>
                            <Select>
                                <SelectTrigger class="ml-auto w-[110px]">
                                    <SelectValue placeholder=PERMISSIONS[1] />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        {PERMISSIONS
                                            .into_iter()
                                            .map(|permission| {
                                                view! {
                                                    <SelectOption
                                                        aria_selected=Signal::derive(move || {
                                                            isabella_permission.get() == Some(permission)
                                                        })
                                                        value=permission
                                                        on:click=move |_| isabella_permission.set(Some(permission))
                                                    >
                                                        {permission}
                                                    </SelectOption>
                                                }
                                            })
                                            .collect_view()}
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                        <div class="flex justify-between items-center space-x-4">
                            <div class="flex items-center space-x-4">
                                <Avatar class="size-10">
                                    <AvatarFallback>SD</AvatarFallback>
                                </Avatar>
                                <div>
                                    <p class="text-sm font-medium leading-none">Sofia Davis</p>
                                    <p class="text-sm text-muted-foreground">p@example.com</p>
                                </div>
                            </div>
                            <Select>
                                <SelectTrigger class="ml-auto w-[110px]">
                                    <SelectValue placeholder=PERMISSIONS[1] />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        {PERMISSIONS
                                            .into_iter()
                                            .map(|permission| {
                                                view! {
                                                    <SelectOption
                                                        aria_selected=Signal::derive(move || {
                                                            sofia_permission.get() == Some(permission)
                                                        })
                                                        value=permission
                                                        on:click=move |_| sofia_permission.set(Some(permission))
                                                    >
                                                        {permission}
                                                    </SelectOption>
                                                }
                                            })
                                            .collect_view()}
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>
                </div>
            </CardContent>
        </Card>
    }
}
