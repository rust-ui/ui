use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::pagination::{
    PageDirection, Pagination, PaginationItem, PaginationLink, PaginationList, PaginationNavButton,
};

#[component]
pub fn DemoPaginationRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Pagination {
                PaginationList {
                    PaginationItem { PaginationNavButton { direction: PageDirection::Previous } }
                    for p in 1..=5u32 {
                        PaginationItem { PaginationLink { page: p } }
                    }
                    PaginationItem { PaginationNavButton { direction: PageDirection::Next } }
                }
            }
        }
    }
}
