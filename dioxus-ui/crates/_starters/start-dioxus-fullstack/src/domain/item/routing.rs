use dioxus::prelude::*;

use crate::domain::item::page_item_details::ItemDetailsPage;
use crate::domain::item::page_item_list::ItemListPage;

pub struct ItemRoutes;

impl ItemRoutes {
    pub const LABEL: &'static str = "Items";
}

#[component]
pub fn ItemList() -> Element {
    rsx! { ItemListPage {} }
}

#[component]
pub fn ItemDetails(id: String) -> Element {
    rsx! { ItemDetailsPage { id } }
}
