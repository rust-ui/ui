use leptos::prelude::*;
use strum::AsRefStr;
use tw_merge::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, AsRefStr, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

#[component]
pub fn DirectionProvider(
    children: Children,
    #[prop(optional)] dir: Direction,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let class = tw_merge!(class);

    view! {
        <div data-slot="direction-provider" dir=dir.to_string() class=class>
            {children()}
        </div>
    }
}
