use icons::{Mail, Search};
use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroupRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="grid gap-6 w-full max-w-sm">
                <InputGroup>
                    <InputGroupInput placeholder="بحث..." />
                    <InputGroupAddon>
                        <Search />
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput placeholder="أدخل بريدك الإلكتروني" />
                    <InputGroupAddon>
                        <Mail />
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput placeholder="أدخل النص هنا" />
                    <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                        <Search />
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </DirectionProvider>
    }
}
