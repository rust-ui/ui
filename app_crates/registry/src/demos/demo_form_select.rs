use leptos::prelude::*;

use crate::ui::form::{FormContent, FormDescription, FormLabel};
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const DEPARTMENTS: [&str; 8] =
    ["Engineering", "Design", "Marketing", "Sales", "Customer Support", "Human Resources", "Finance", "Operations"];

#[component]
pub fn DemoFormSelect() -> impl IntoView {
    let department_signal = RwSignal::new(None::<&str>);

    view! {
        <div class="w-full max-w-md">
            <FormContent>
                <FormLabel>Department</FormLabel>
                <Select>
                    <SelectTrigger>
                        <SelectValue placeholder="Choose department" />
                    </SelectTrigger>

                    <SelectContent>
                        <SelectGroup>
                            {DEPARTMENTS
                                .into_iter()
                                .map(|dept| {
                                    view! {
                                        <SelectOption
                                            aria_selected=Signal::derive(move || {
                                                department_signal.get() == Some(dept)
                                            })
                                            value=dept
                                            on:click=move |_| department_signal.set(Some(dept))
                                        >
                                            {dept}
                                        </SelectOption>
                                    }
                                })
                                .collect_view()}
                        </SelectGroup>
                    </SelectContent>
                </Select>
                <FormDescription>Select your department or area of work.</FormDescription>
            </FormContent>
        </div>
    }
}
