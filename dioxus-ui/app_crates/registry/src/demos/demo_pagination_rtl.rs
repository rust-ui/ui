use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::pagination::{
    Pagination, PaginationItem, PaginationLink, PaginationList, PaginationNext, PaginationPrev,
};

#[component]
pub fn DemoPaginationRtl() -> Element {
    let mut page = use_signal(|| 1u32);
    let total = 5u32;

    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Pagination {
                PaginationList {
                    PaginationItem {
                        PaginationPrev {
                            disabled: page() == 1,
                            onclick: move |_| { if page() > 1 { page.set(page() - 1); } },
                        }
                    }
                    for p in 1..=total {
                        PaginationItem {
                            PaginationLink {
                                page: p,
                                is_active: page() == p,
                                onclick: move |_| page.set(p),
                            }
                        }
                    }
                    PaginationItem {
                        PaginationNext {
                            disabled: page() == total,
                            onclick: move |_| { if page() < total { page.set(page() + 1); } },
                        }
                    }
                }
            }
        }
    }
}
