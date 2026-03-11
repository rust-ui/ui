use icons::{Minus, Plus};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::button_group::{ButtonGroup, ButtonGroupOrientation};

#[component]
pub fn DemoButtonGroupIcon() -> impl IntoView {
    view! {
        <ButtonGroup orientation=ButtonGroupOrientation::Vertical>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
                <Plus />
            </Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Icon>
                <Minus />
            </Button>
        </ButtonGroup>
    }
}
