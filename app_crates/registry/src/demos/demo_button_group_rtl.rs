use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonGroupRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <ButtonGroup>
                <Button variant=ButtonVariant::Outline>"الأول"</Button>
                <Button variant=ButtonVariant::Outline>"الثاني"</Button>
                <Button variant=ButtonVariant::Outline>"الثالث"</Button>
            </ButtonGroup>
        </DirectionProvider>
    }
}
