use dioxus::prelude::*;
use icons::{CircleUser, House, SlidersHorizontal, Wallet};

use crate::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum NavPage {
    Home,
    #[default]
    Wallet,
    Settings,
    Profile,
}

impl NavPage {
    fn label(self) -> &'static str {
        match self {
            NavPage::Home => "Home",
            NavPage::Wallet => "Wallet",
            NavPage::Settings => "Settings",
            NavPage::Profile => "Profile",
        }
    }

    fn icon(self) -> Element {
        match self {
            NavPage::Home => rsx! { House { class: "size-5" } },
            NavPage::Wallet => rsx! { Wallet { class: "size-5" } },
            NavPage::Settings => rsx! { SlidersHorizontal { class: "size-5" } },
            NavPage::Profile => rsx! { CircleUser { class: "size-5" } },
        }
    }
}

const PAGES: &[NavPage] = &[NavPage::Home, NavPage::Wallet, NavPage::Settings, NavPage::Profile];

#[component]
pub fn DemoBottomNav() -> Element {
    let mut active = use_signal(|| NavPage::Wallet);

    rsx! {
        div { class: "flex flex-col my-10 rounded-t-2xl border h-[300px] w-[400px]",
            div { class: "flex-1 bg-gray-200 rounded-t-2xl" }
            BottomNav {
                BottomNavGrid {
                    for page in PAGES {
                        {
                            let page = *page;
                            rsx! {
                                BottomNavButton {
                                    onclick: move |_| active.set(page),
                                    aria_current: if active() == page { "page" } else { "" },
                                    {page.icon()}
                                    BottomNavLabel { {page.label()} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
