use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;
use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

#[component]
pub fn DemoButtonGroupSelect() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Select default_value="https">
                <SelectTrigger>
                    <SelectValue placeholder="Protocol" />
                </SelectTrigger>
                <SelectContent>
                    <SelectGroup>
                        <SelectOption value="https">"https://"</SelectOption>
                        <SelectOption value="http">"http://"</SelectOption>
                    </SelectGroup>
                </SelectContent>
            </Select>
            <Button variant=ButtonVariant::Outline>"Go"</Button>
        </ButtonGroup>
    }
}
