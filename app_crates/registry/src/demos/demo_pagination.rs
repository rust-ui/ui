use dioxus::prelude::*;

use crate::ui::pagination::{
    PageDirection, Pagination, PaginationItem, PaginationLink, PaginationList, PaginationNavButton,
};

#[component]
pub fn DemoPagination() -> Element {
    rsx! {
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
