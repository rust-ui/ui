use leptos::prelude::*;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};
use registry::ui::textarea::Textarea;

const AREAS: [&str; 4] = ["Billing", "Team", "Account", "Support"];

const SECURITY_LEVELS: [&str; 4] = ["Level 1", "Level 2", "Level 3", "Level 4"];

#[component]
pub fn CardReportIssue() -> impl IntoView {
    let area_signal = RwSignal::new(None::<&str>);
    let security_level_signal = RwSignal::new(None::<&str>);

    view! {
        <Card>
            <CardHeader>
                <CardTitle>Report an issue</CardTitle>
                <CardDescription>What area are you having problems with?</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-6">
                <div class="grid gap-4 sm:grid-cols-2">
                    <div class="grid gap-2">
                        <Label>Area</Label>
                        <Select class="w-full">
                            <SelectTrigger>
                                <SelectValue placeholder="Select area" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    {AREAS
                                        .into_iter()
                                        .map(|area| {
                                            view! {
                                                <SelectOption
                                                    aria_selected=Signal::derive(move || {
                                                        area_signal.get() == Some(area)
                                                    })
                                                    value=area
                                                    on:click=move |_| area_signal.set(Some(area))
                                                >
                                                    {area}
                                                </SelectOption>
                                            }
                                        })
                                        .collect_view()}
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                    <div class="grid gap-2">
                        <Label>Security Level</Label>
                        <Select class="w-full">
                            <SelectTrigger>
                                <SelectValue placeholder="Select level" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    {SECURITY_LEVELS
                                        .into_iter()
                                        .map(|level| {
                                            view! {
                                                <SelectOption
                                                    aria_selected=Signal::derive(move || {
                                                        security_level_signal.get() == Some(level)
                                                    })
                                                    value=level
                                                    on:click=move |_| security_level_signal.set(Some(level))
                                                >
                                                    {level}
                                                </SelectOption>
                                            }
                                        })
                                        .collect_view()}
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                </div>
                <div class="grid gap-2">
                    <Label>Subject</Label>
                    <Input attr:placeholder="I need help with..." />
                </div>
                <div class="grid gap-2">
                    <Label>Description</Label>
                    <Textarea attr:placeholder="Please include all information relevant to your issue." />
                </div>
            </CardContent>
            <CardFooter class="gap-2 justify-end">
                <Button variant=ButtonVariant::Outline>Cancel</Button>
                <Button>Submit</Button>
            </CardFooter>
        </Card>
    }
}
