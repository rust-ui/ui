use dioxus::prelude::*;

use crate::ui::chips::{ChipItem, ChipsContainer};

#[component]
pub fn DemoChips() -> Element {
    rsx! {
        ChipsContainer {
            ChipItem { label: "sunny" }
            ChipItem { label: "cloudy" }
            ChipItem { label: "rainy" }
            ChipItem { label: "windy" }
            ChipItem { label: "stormy" }
            ChipItem { label: "foggy" }
            ChipItem { label: "snowy" }
            ChipItem { label: "icy" }
            ChipItem { label: "humid" }
            ChipItem { label: "dry" }
            ChipItem { label: "hot" }
            ChipItem { label: "cold" }
            ChipItem { label: "warm" }
            ChipItem { label: "chilly" }
            ChipItem { label: "breezy" }
            ChipItem { label: "hazy" }
        }
    }
}
