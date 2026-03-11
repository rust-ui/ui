use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroupSizes() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <ButtonGroup>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Cut"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Copy"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "Paste"
                </Button>
            </ButtonGroup>

            <ButtonGroup>
                <Button variant=ButtonVariant::Outline>"Cut"</Button>
                <Button variant=ButtonVariant::Outline>"Copy"</Button>
                <Button variant=ButtonVariant::Outline>"Paste"</Button>
            </ButtonGroup>

            <ButtonGroup>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Cut"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Copy"
                </Button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Paste"
                </Button>
            </ButtonGroup>
        </div>
    }
}
