use leptos::prelude::*;

use crate::ui::pagination::{
    PageDirection, Pagination, PaginationItem, PaginationLink, PaginationList, PaginationNavButton,
};

#[component]
pub fn DemoPagination() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationList>
                <PaginationItem>
                    <PaginationNavButton direction=PageDirection::Previous />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=1 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=2 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=3 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=4 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=5 />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNavButton direction=PageDirection::Next />
                </PaginationItem>
            </PaginationList>
        </Pagination>
    }
}
