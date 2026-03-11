use leptos::prelude::*;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const ROLES: [&str; 2] = ["Owner", "Member"];

#[component]
pub fn CardTeamMembers() -> impl IntoView {
    let sofia_role = RwSignal::new(Some(ROLES[0]));
    let jackson_role = RwSignal::new(Some(ROLES[1]));
    let isabella_role = RwSignal::new(Some(ROLES[1]));

    view! {
        <Card>
            <CardHeader>
                <CardTitle>Team Members</CardTitle>
                <CardDescription>Invite your team members to collaborate.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-6">
                <div class="flex justify-between items-center space-x-4">
                    <div class="flex items-center space-x-4">
                        <Avatar>
                            <AvatarFallback>OM</AvatarFallback>
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">Sofia Davis</p>
                            <p class="text-sm text-muted-foreground">m@example.com</p>
                        </div>
                    </div>
                    <Select>
                        <SelectTrigger class="ml-auto w-[100px]">
                            <SelectValue placeholder=ROLES[0] />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                {ROLES
                                    .into_iter()
                                    .map(|role| {
                                        view! {
                                            <SelectOption
                                                aria_selected=Signal::derive(move || sofia_role.get() == Some(role))
                                                value=role
                                                on:click=move |_| sofia_role.set(Some(role))
                                            >
                                                {role}
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
                        <Avatar>
                            <AvatarFallback>JL</AvatarFallback>
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">Jackson Lee</p>
                            <p class="text-sm text-muted-foreground">p@example.com</p>
                        </div>
                    </div>
                    <Select>
                        <SelectTrigger class="ml-auto w-[100px]">
                            <SelectValue placeholder=ROLES[1] />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                {ROLES
                                    .into_iter()
                                    .map(|role| {
                                        view! {
                                            <SelectOption
                                                aria_selected=Signal::derive(move || {
                                                    jackson_role.get() == Some(role)
                                                })
                                                value=role
                                                on:click=move |_| jackson_role.set(Some(role))
                                            >
                                                {role}
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
                        <Avatar>
                            <AvatarFallback>IN</AvatarFallback>
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">Isabella Nguyen</p>
                            <p class="text-sm text-muted-foreground">i@example.com</p>
                        </div>
                    </div>
                    <Select>
                        <SelectTrigger class="ml-auto w-[100px]">
                            <SelectValue placeholder=ROLES[1] />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                {ROLES
                                    .into_iter()
                                    .map(|role| {
                                        view! {
                                            <SelectOption
                                                aria_selected=Signal::derive(move || {
                                                    isabella_role.get() == Some(role)
                                                })
                                                value=role
                                                on:click=move |_| isabella_role.set(Some(role))
                                            >
                                                {role}
                                            </SelectOption>
                                        }
                                    })
                                    .collect_view()}
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </div>
            </CardContent>
        </Card>
    }
}
