use icons::{ArrowUp, AtSign, ChevronDown, Globe, Paperclip, WandSparkles};
use leptos::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupTextarea,
};

#[component]
pub fn DemoInputGroupBlock() -> impl IntoView {
    view! {
        <div class="w-full max-w-lg">
            <InputGroup>
                <InputGroupAddon align=InputGroupAddonAlign::BlockStart>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <AtSign />
                        "Add context"
                    </InputGroupButton>
                    <Badge variant=BadgeVariant::Secondary>"Vision"</Badge>
                    <Badge variant=BadgeVariant::Secondary>"Research"</Badge>
                </InputGroupAddon>

                <InputGroupTextarea attr:placeholder="Ask anything..." attr:rows="3" />

                <InputGroupAddon align=InputGroupAddonAlign::BlockEnd>
                    <InputGroupButton size=InputGroupButtonSize::IconXs>
                        <Paperclip />
                    </InputGroupButton>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <WandSparkles />
                        "Auto"
                        <ChevronDown />
                    </InputGroupButton>
                    <InputGroupButton size=InputGroupButtonSize::Xs>
                        <Globe />
                        "Sources"
                    </InputGroupButton>
                    <div class="ml-auto">
                        <InputGroupButton
                            size=InputGroupButtonSize::IconXs
                            class="bg-primary text-primary-foreground hover:bg-primary/90"
                        >
                            <ArrowUp />
                        </InputGroupButton>
                    </div>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
