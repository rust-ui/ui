use app_routes::BlockRoutes;
use leptos::prelude::*;
use strum::{Display, EnumString};

use crate::domain::blocks::block_entry::{BlockEntry, BlockMeta};

// * This file was generated automatically with build_registry.

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum BlockIdKebab {
    Chat01,
    Chat02,
    Chat03,
    Chat04,
    Chat05,
    Chat06,
    Chat07,
    Faq01,
    Faq02,
    Faq03,
    Footer01,
    Footer02,
    Footer03,
    Footer04,
    Footer05,
    Header01,
    Integration01,
    Integration02,
    Integration03,
    Integration04,
    Integration05,
    Integration06,
    Integration07,
    Login01,
    Login02,
    Login03,
    Login04,
    Pricing01,
    Pricing02,
    Pricing03,
    Pricing04,
    Pricing05,
    Pricing06,
    Pricing07,
    Pricing08,
    Pricing09,
    Pricing10,
    Pricing11,
    Sidenav01,
    Sidenav02,
    Sidenav03,
    Sidenav04,
    Sidenav05,
    Sidenav06,
    Sidenav07,
    Sidenav08,
    Sidenav09,
    Sidenav10,
    Sidenav11,
}

impl BlockIdKebab {
    pub fn to_component(&self) -> AnyView {
        match self {
            BlockIdKebab::Chat01 => registry::blocks::chat01::Chat01().into_any(),
            BlockIdKebab::Chat02 => registry::blocks::chat02::Chat02().into_any(),
            BlockIdKebab::Chat03 => registry::blocks::chat03::Chat03().into_any(),
            BlockIdKebab::Chat04 => registry::blocks::chat04::Chat04().into_any(),
            BlockIdKebab::Chat05 => registry::blocks::chat05::Chat05().into_any(),
            BlockIdKebab::Chat06 => registry::blocks::chat06::Chat06().into_any(),
            BlockIdKebab::Chat07 => registry::blocks::chat07::Chat07().into_any(),
            BlockIdKebab::Faq01 => registry::blocks::faq01::Faq01().into_any(),
            BlockIdKebab::Faq02 => registry::blocks::faq02::Faq02().into_any(),
            BlockIdKebab::Faq03 => registry::blocks::faq03::Faq03().into_any(),
            BlockIdKebab::Footer01 => registry::blocks::footer01::Footer01().into_any(),
            BlockIdKebab::Footer02 => registry::blocks::footer02::Footer02().into_any(),
            BlockIdKebab::Footer03 => registry::blocks::footer03::Footer03().into_any(),
            BlockIdKebab::Footer04 => registry::blocks::footer04::Footer04().into_any(),
            BlockIdKebab::Footer05 => registry::blocks::footer05::Footer05().into_any(),
            BlockIdKebab::Header01 => registry::blocks::header01::Header01().into_any(),
            BlockIdKebab::Integration01 => registry::blocks::integration01::Integration01().into_any(),
            BlockIdKebab::Integration02 => registry::blocks::integration02::Integration02().into_any(),
            BlockIdKebab::Integration03 => registry::blocks::integration03::Integration03().into_any(),
            BlockIdKebab::Integration04 => registry::blocks::integration04::Integration04().into_any(),
            BlockIdKebab::Integration05 => registry::blocks::integration05::Integration05().into_any(),
            BlockIdKebab::Integration06 => registry::blocks::integration06::Integration06().into_any(),
            BlockIdKebab::Integration07 => registry::blocks::integration07::Integration07().into_any(),
            BlockIdKebab::Login01 => registry::blocks::login01::Login01().into_any(),
            BlockIdKebab::Login02 => registry::blocks::login02::Login02().into_any(),
            BlockIdKebab::Login03 => registry::blocks::login03::Login03().into_any(),
            BlockIdKebab::Login04 => registry::blocks::login04::Login04().into_any(),
            BlockIdKebab::Pricing01 => registry::blocks::pricing01::Pricing01().into_any(),
            BlockIdKebab::Pricing02 => registry::blocks::pricing02::Pricing02().into_any(),
            BlockIdKebab::Pricing03 => registry::blocks::pricing03::Pricing03().into_any(),
            BlockIdKebab::Pricing04 => registry::blocks::pricing04::Pricing04().into_any(),
            BlockIdKebab::Pricing05 => registry::blocks::pricing05::Pricing05().into_any(),
            BlockIdKebab::Pricing06 => registry::blocks::pricing06::Pricing06().into_any(),
            BlockIdKebab::Pricing07 => registry::blocks::pricing07::Pricing07().into_any(),
            BlockIdKebab::Pricing08 => registry::blocks::pricing08::Pricing08().into_any(),
            BlockIdKebab::Pricing09 => registry::blocks::pricing09::Pricing09().into_any(),
            BlockIdKebab::Pricing10 => registry::blocks::pricing10::Pricing10().into_any(),
            BlockIdKebab::Pricing11 => registry::blocks::pricing11::Pricing11().into_any(),
            BlockIdKebab::Sidenav01 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav02 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav03 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav04 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav05 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav06 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav07 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav08 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav09 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav10 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
            BlockIdKebab::Sidenav11 => crate::sidenav_shortfix_all_blocks::SidenavShortfixAllBlocks().into_any(),
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            BlockIdKebab::Chat01 => "Simple Chat Card",
            BlockIdKebab::Chat02 => "Team Group Chat",
            BlockIdKebab::Chat03 => "AI Chatbot Interface",
            BlockIdKebab::Chat04 => "Chat with File Sharing",
            BlockIdKebab::Chat05 => "Wide Chat Card",
            BlockIdKebab::Chat06 => "Chat App with Sidebar",
            BlockIdKebab::Chat07 => "Full Chat Application",
            BlockIdKebab::Faq01 => "FAQ with Numbered Grid",
            BlockIdKebab::Faq02 => "FAQ with Sticky Sidebar",
            BlockIdKebab::Faq03 => "FAQ with Sidebar Navigation",
            BlockIdKebab::Footer01 => "Footer with CTA Card",
            BlockIdKebab::Footer02 => "Footer Centered Simple",
            BlockIdKebab::Footer03 => "Footer Grid with Social Links",
            BlockIdKebab::Footer04 => "Footer with Newsletter and Language Selector",
            BlockIdKebab::Footer05 => "Footer with Newsletter and Status Indicator",
            BlockIdKebab::Header01 => "Header with animation on scroll",
            BlockIdKebab::Integration01 => "Icon Library Showcase",
            BlockIdKebab::Integration02 => "Icon Library with Centered Grid",
            BlockIdKebab::Integration03 => "Icon Library with Connected Lines",
            BlockIdKebab::Integration04 => "Icon Library with Orbital Rings",
            BlockIdKebab::Integration05 => "Icon Library with 3D Perspective Grid",
            BlockIdKebab::Integration06 => "Icon Library with Scrolling Rows",
            BlockIdKebab::Integration07 => "Icon Library with Scattered Layout",
            BlockIdKebab::Login01 => "Simple Login form",
            BlockIdKebab::Login02 => "Login with Social Auth",
            BlockIdKebab::Login03 => "Two-Factor Authentication",
            BlockIdKebab::Login04 => "Password Reset Form",
            BlockIdKebab::Pricing01 => "Simple Pricing with Features",
            BlockIdKebab::Pricing02 => "Multi-Tier Pricing with Feature Categories",
            BlockIdKebab::Pricing03 => "Four-Tier Pricing with Equal Height Cards",
            BlockIdKebab::Pricing04 => "Pricing Comparison Table with Feature Grid",
            BlockIdKebab::Pricing05 => "Pricing with Monthly/Yearly Toggle",
            BlockIdKebab::Pricing06 => "Responsive Pricing Table with Annual/Monthly Toggle",
            BlockIdKebab::Pricing07 => "Centered Single Plan with Feature Grid",
            BlockIdKebab::Pricing08 => "Pricing with Billing Cycle Toggle",
            BlockIdKebab::Pricing09 => "Two-Tier Comparison Pricing with Tooltips",
            BlockIdKebab::Pricing10 => "Four-Tier Pricing with Info Tooltips",
            BlockIdKebab::Pricing11 => "Three-Tier Pricing with Tabs and Tooltips",
            BlockIdKebab::Sidenav01 => "Simple Sidenav with grouped sections",
            BlockIdKebab::Sidenav02 => "Simple Sidenav with Collapsible menus",
            BlockIdKebab::Sidenav03 => "Simple Sidenav with submenus",
            BlockIdKebab::Sidenav04 => "A Floating Sidenav with submenus",
            BlockIdKebab::Sidenav05 => "Sidenav with Collapsible submenus",
            BlockIdKebab::Sidenav06 => "Sidenav with Dropdown Submenus",
            BlockIdKebab::Sidenav07 => "Collapsible Sidenav with Icons",
            BlockIdKebab::Sidenav08 => "Inset Sidenav with secondary navigation",
            BlockIdKebab::Sidenav09 => "Nested Sidenavs with Route-based Navigation",
            BlockIdKebab::Sidenav10 => "Sidenav with search functionality",
            BlockIdKebab::Sidenav11 => "Right-Side Sidenav",
        }
    }

    pub fn to_title(&self) -> &'static str {
        self.title()
    }

    pub fn to_md(self) -> String {
        format!("{}.md", self)
    }

    pub fn to_full_view_url(self) -> String {
        let base_view = format!("/view/{}", self);
        match self {
            BlockIdKebab::Chat01 => base_view,
            BlockIdKebab::Chat02 => base_view,
            BlockIdKebab::Chat03 => base_view,
            BlockIdKebab::Chat04 => base_view,
            BlockIdKebab::Chat05 => base_view,
            BlockIdKebab::Chat06 => base_view,
            BlockIdKebab::Chat07 => base_view,
            BlockIdKebab::Faq01 => base_view,
            BlockIdKebab::Faq02 => base_view,
            BlockIdKebab::Faq03 => base_view,
            BlockIdKebab::Footer01 => base_view,
            BlockIdKebab::Footer02 => base_view,
            BlockIdKebab::Footer03 => base_view,
            BlockIdKebab::Footer04 => base_view,
            BlockIdKebab::Footer05 => base_view,
            BlockIdKebab::Header01 => base_view,
            BlockIdKebab::Integration01 => base_view,
            BlockIdKebab::Integration02 => base_view,
            BlockIdKebab::Integration03 => base_view,
            BlockIdKebab::Integration04 => base_view,
            BlockIdKebab::Integration05 => base_view,
            BlockIdKebab::Integration06 => base_view,
            BlockIdKebab::Integration07 => base_view,
            BlockIdKebab::Login01 => base_view,
            BlockIdKebab::Login02 => base_view,
            BlockIdKebab::Login03 => base_view,
            BlockIdKebab::Login04 => base_view,
            BlockIdKebab::Pricing01 => base_view,
            BlockIdKebab::Pricing02 => base_view,
            BlockIdKebab::Pricing03 => base_view,
            BlockIdKebab::Pricing04 => base_view,
            BlockIdKebab::Pricing05 => base_view,
            BlockIdKebab::Pricing06 => base_view,
            BlockIdKebab::Pricing07 => base_view,
            BlockIdKebab::Pricing08 => base_view,
            BlockIdKebab::Pricing09 => base_view,
            BlockIdKebab::Pricing10 => base_view,
            BlockIdKebab::Pricing11 => base_view,
            BlockIdKebab::Sidenav01 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav02 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav03 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav04 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav05 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav06 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav07 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav08 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav09 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav10 => format!("{}/docs/components", base_view),
            BlockIdKebab::Sidenav11 => format!("{}/docs/components", base_view),
        }
    }

    pub fn files(&self) -> &'static [crate::domain::blocks::block_entry::BlockFile] {
        match self {
            BlockIdKebab::Chat01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat01.rs",
                    target: "blocks/chat01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat.rs",
                    target: "ui/chat.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/chat.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat02.rs",
                    target: "blocks/chat02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat03.rs",
                    target: "blocks/chat03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat04.rs",
                    target: "blocks/chat04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat05 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat05.rs",
                    target: "blocks/chat05.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat05.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat06 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat06.rs",
                    target: "blocks/chat06.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat06.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Chat07 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "chat07.rs",
                    target: "blocks/chat07.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/chat07.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Faq01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "faq01.rs",
                    target: "blocks/faq01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/faq01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "badge.rs",
                    target: "ui/badge.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/badge.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Faq02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "faq02.rs",
                    target: "blocks/faq02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/faq02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Faq03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "faq03.rs",
                    target: "blocks/faq03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/faq03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Footer01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer01.rs",
                    target: "blocks/footer01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/footer01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer.rs",
                    target: "ui/footer.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/footer.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Footer02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer02.rs",
                    target: "blocks/footer02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/footer02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer.rs",
                    target: "ui/footer.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/footer.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Footer03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer03.rs",
                    target: "blocks/footer03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/footer03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer.rs",
                    target: "ui/footer.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/footer.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Footer04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer04.rs",
                    target: "blocks/footer04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/footer04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer.rs",
                    target: "ui/footer.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/footer.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Footer05 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer05.rs",
                    target: "blocks/footer05.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/footer05.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "footer.rs",
                    target: "ui/footer.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/footer.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Header01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "header01.rs",
                    target: "blocks/header01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/header01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_accordion_icons.rs",
                    target: "demos/demo_accordion_icons.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_accordion_icons.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "header.rs",
                    target: "ui/header.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/header.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "theme_toggle.rs",
                    target: "ui/theme_toggle.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/theme_toggle.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration01.rs",
                    target: "blocks/integration01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "image.rs",
                    target: "ui/image.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/image.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration02.rs",
                    target: "blocks/integration02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration03.rs",
                    target: "blocks/integration03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration04.rs",
                    target: "blocks/integration04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration05 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration05.rs",
                    target: "blocks/integration05.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration05.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration06 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration06.rs",
                    target: "blocks/integration06.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration06.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Integration07 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "integration07.rs",
                    target: "blocks/integration07.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/integration07.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "badge.rs",
                    target: "ui/badge.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/badge.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Login01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "login01.rs",
                    target: "blocks/login01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/login01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Login02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "login02.rs",
                    target: "blocks/login02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/login02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Login03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "login03.rs",
                    target: "blocks/login03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/login03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Login04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "login04.rs",
                    target: "blocks/login04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/login04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "input.rs",
                    target: "ui/input.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/input.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing01.rs",
                    target: "blocks/pricing01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "separator.rs",
                    target: "ui/separator.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/separator.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing02.rs",
                    target: "blocks/pricing02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "avatar.rs",
                    target: "ui/avatar.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/avatar.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "badge.rs",
                    target: "ui/badge.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/badge.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing03.rs",
                    target: "blocks/pricing03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing04.rs",
                    target: "blocks/pricing04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "table.rs",
                    target: "ui/table.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/table.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing05 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing05.rs",
                    target: "blocks/pricing05.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing05.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "separator.rs",
                    target: "ui/separator.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/separator.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "switch.rs",
                    target: "ui/switch.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/switch.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing06 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing06.rs",
                    target: "blocks/pricing06.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing06.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "switch.rs",
                    target: "ui/switch.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/switch.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing07 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing07.rs",
                    target: "blocks/pricing07.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing07.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing08 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing08.rs",
                    target: "blocks/pricing08.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing08.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "badge.rs",
                    target: "ui/badge.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/badge.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing09 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing09.rs",
                    target: "blocks/pricing09.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing09.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "tooltip.rs",
                    target: "ui/tooltip.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/tooltip.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing10 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing10.rs",
                    target: "blocks/pricing10.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing10.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Pricing11 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "pricing11.rs",
                    target: "blocks/pricing11.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/pricing11.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "card.rs",
                    target: "ui/card.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/card.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav01 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav01.rs",
                    target: "blocks/sidenav01.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav01.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav02 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav02.rs",
                    target: "blocks/sidenav02.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav02.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav03 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav03.rs",
                    target: "blocks/sidenav03.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav03.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav04 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav04.rs",
                    target: "blocks/sidenav04.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav04.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav05 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav05.rs",
                    target: "blocks/sidenav05.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav05.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav06 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav06.rs",
                    target: "blocks/sidenav06.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav06.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "dropdown_menu.rs",
                    target: "ui/dropdown_menu.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/dropdown_menu.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav07 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav07.rs",
                    target: "blocks/sidenav07.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav07.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav08 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav08.rs",
                    target: "blocks/sidenav08.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav08.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav09 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav09.rs",
                    target: "blocks/sidenav09.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav09.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user_icon.rs",
                    target: "demos/demo_dropdown_menu_user_icon.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user_icon.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "accordion.rs",
                    target: "ui/accordion.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/accordion.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav10 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav10.rs",
                    target: "blocks/sidenav10.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav10.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "label.rs",
                    target: "ui/label.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/label.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
            BlockIdKebab::Sidenav11 => &[
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav11.rs",
                    target: "blocks/sidenav11.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav11.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes.rs",
                    target: "blocks/sidenav_routes.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_selector.rs",
                    target: "blocks/sidenav_routes_selector.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_selector.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav_routes_simplified.rs",
                    target: "blocks/sidenav_routes_simplified.rs",
                    content: include_str!("../../../app_crates/registry/src/blocks/sidenav_routes_simplified.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "demo_dropdown_menu_user.rs",
                    target: "demos/demo_dropdown_menu_user.rs",
                    content: include_str!("../../../app_crates/registry/src/demos/demo_dropdown_menu_user.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "button.rs",
                    target: "ui/button.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/button.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sheet.rs",
                    target: "ui/sheet.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sheet.rs"),
                    language: "rust",
                },
                crate::domain::blocks::block_entry::BlockFile {
                    name: "sidenav.rs",
                    target: "ui/sidenav.rs",
                    content: include_str!("../../../app_crates/registry/src/ui/sidenav.rs"),
                    language: "rust",
                },
            ],
        }
    }

    pub fn file_tree(&self) -> Vec<crate::domain::blocks::block_entry::BlockFileTreeItem> {
        match self {
            BlockIdKebab::Chat01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "chat.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Chat02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Chat03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Chat04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat04.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Chat05 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat05.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Chat06 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat06.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Chat07 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "chat07.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Faq01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "faq01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "badge.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Faq02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "faq02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "accordion.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Faq03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "faq03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Footer01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "footer.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Footer02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Footer03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Footer04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer04.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "footer.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Footer05 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "footer05.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "footer.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Header01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "header01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_accordion_icons.rs",
                        index: 1,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "header.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "theme_toggle.rs",
                            index: 4,
                        },
                    ],
                },
            ],
            BlockIdKebab::Integration01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "image.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration04.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration05 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration05.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration06 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration06.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Integration07 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "integration07.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "badge.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Login01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "login01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Login02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "login02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Login03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "login03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Login04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "login04.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "input.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Pricing01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing01.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "separator.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Pricing02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing02.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "avatar.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "badge.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Pricing03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing03.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Pricing04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing04.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "table.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Pricing05 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing05.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "separator.rs", index: 3 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "switch.rs", index: 4 },
                    ],
                },
            ],
            BlockIdKebab::Pricing06 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing06.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "switch.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Pricing07 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing07.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "button.rs",
                        index: 1,
                    }],
                },
            ],
            BlockIdKebab::Pricing08 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing08.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "badge.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Pricing09 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing09.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "tooltip.rs", index: 3 },
                    ],
                },
            ],
            BlockIdKebab::Pricing10 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing10.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Pricing11 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "pricing11.rs",
                        index: 0,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 1 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "card.rs", index: 2 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav01 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav01.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav02 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav02.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav03 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav03.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav04 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav04.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 7 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav05 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav05.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav06 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav06.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "dropdown_menu.rs",
                            index: 6,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav07 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav07.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav08 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav08.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav09 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav09.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "demo_dropdown_menu_user.rs",
                            index: 4,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "demo_dropdown_menu_user_icon.rs",
                            index: 5,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "accordion.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 8 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 9 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav10 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav10.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "label.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 7 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 8 },
                    ],
                },
            ],
            BlockIdKebab::Sidenav11 => vec![
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "blocks",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav11.rs", index: 0 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes.rs",
                            index: 1,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_selector.rs",
                            index: 2,
                        },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                            name: "sidenav_routes_simplified.rs",
                            index: 3,
                        },
                    ],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "demos",
                    items: vec![crate::domain::blocks::block_entry::BlockFileTreeItem::File {
                        name: "demo_dropdown_menu_user.rs",
                        index: 4,
                    }],
                },
                crate::domain::blocks::block_entry::BlockFileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "button.rs", index: 5 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sheet.rs", index: 6 },
                        crate::domain::blocks::block_entry::BlockFileTreeItem::File { name: "sidenav.rs", index: 7 },
                    ],
                },
            ],
        }
    }

    /// Meta used for `__ViewRouter`.
    pub fn meta(&self) -> BlockMeta {
        match self {
            BlockIdKebab::Footer01 => BlockMeta { iframe_height: "897px", ..BlockMeta::default() },
            BlockIdKebab::Footer02 => BlockMeta { container_class: "bg-muted", iframe_height: "448px" },
            BlockIdKebab::Footer03 => BlockMeta { iframe_height: "474px", ..BlockMeta::default() },
            BlockIdKebab::Footer04 => BlockMeta { iframe_height: "606px", ..BlockMeta::default() },
            BlockIdKebab::Footer05 => BlockMeta { container_class: "bg-muted", iframe_height: "536px" },
            BlockIdKebab::Header01 => BlockMeta { container_class: "min-h-[1200px] bg-muted", ..BlockMeta::default() },
            _ => BlockMeta::default(),
        }
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

// * Chat
pub const ALL_CHAT_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "chat01",
        block_title: "Simple Chat Card",
        block_id_kebab: BlockIdKebab::Chat01,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat02",
        block_title: "Team Group Chat",
        block_id_kebab: BlockIdKebab::Chat02,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat03",
        block_title: "AI Chatbot Interface",
        block_id_kebab: BlockIdKebab::Chat03,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat04",
        block_title: "Chat with File Sharing",
        block_id_kebab: BlockIdKebab::Chat04,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat05",
        block_title: "Wide Chat Card",
        block_id_kebab: BlockIdKebab::Chat05,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat06",
        block_title: "Chat App with Sidebar",
        block_id_kebab: BlockIdKebab::Chat06,
        block_route: BlockRoutes::Chat,
    },
    BlockEntry {
        block_id_str: "chat07",
        block_title: "Full Chat Application",
        block_id_kebab: BlockIdKebab::Chat07,
        block_route: BlockRoutes::Chat,
    },
];

// * Faq
pub const ALL_FAQ_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "faq01",
        block_title: "FAQ with Numbered Grid",
        block_id_kebab: BlockIdKebab::Faq01,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "faq02",
        block_title: "FAQ with Sticky Sidebar",
        block_id_kebab: BlockIdKebab::Faq02,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "faq03",
        block_title: "FAQ with Sidebar Navigation",
        block_id_kebab: BlockIdKebab::Faq03,
        block_route: BlockRoutes::Login,
    },
];

// * Footer
pub const ALL_FOOTER_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "footer01",
        block_title: "Footer with CTA Card",
        block_id_kebab: BlockIdKebab::Footer01,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "footer02",
        block_title: "Footer Centered Simple",
        block_id_kebab: BlockIdKebab::Footer02,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "footer03",
        block_title: "Footer Grid with Social Links",
        block_id_kebab: BlockIdKebab::Footer03,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "footer04",
        block_title: "Footer with Newsletter and Language Selector",
        block_id_kebab: BlockIdKebab::Footer04,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "footer05",
        block_title: "Footer with Newsletter and Status Indicator",
        block_id_kebab: BlockIdKebab::Footer05,
        block_route: BlockRoutes::Login,
    },
];

// * Header
pub const ALL_HEADER_BLOCKS: &[BlockEntry] = &[BlockEntry {
    block_id_str: "header01",
    block_title: "Header with animation on scroll",
    block_id_kebab: BlockIdKebab::Header01,
    block_route: BlockRoutes::Headers,
}];

// * Integration
pub const ALL_INTEGRATION_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "integration01",
        block_title: "Icon Library Showcase",
        block_id_kebab: BlockIdKebab::Integration01,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration02",
        block_title: "Icon Library with Centered Grid",
        block_id_kebab: BlockIdKebab::Integration02,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration03",
        block_title: "Icon Library with Connected Lines",
        block_id_kebab: BlockIdKebab::Integration03,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration04",
        block_title: "Icon Library with Orbital Rings",
        block_id_kebab: BlockIdKebab::Integration04,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration05",
        block_title: "Icon Library with 3D Perspective Grid",
        block_id_kebab: BlockIdKebab::Integration05,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration06",
        block_title: "Icon Library with Scrolling Rows",
        block_id_kebab: BlockIdKebab::Integration06,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "integration07",
        block_title: "Icon Library with Scattered Layout",
        block_id_kebab: BlockIdKebab::Integration07,
        block_route: BlockRoutes::Login,
    },
];

// * Login
pub const ALL_LOGIN_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "login01",
        block_title: "Simple Login form",
        block_id_kebab: BlockIdKebab::Login01,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "login02",
        block_title: "Login with Social Auth",
        block_id_kebab: BlockIdKebab::Login02,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "login03",
        block_title: "Two-Factor Authentication",
        block_id_kebab: BlockIdKebab::Login03,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "login04",
        block_title: "Password Reset Form",
        block_id_kebab: BlockIdKebab::Login04,
        block_route: BlockRoutes::Login,
    },
];

// * Pricing
pub const ALL_PRICING_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "pricing01",
        block_title: "Simple Pricing with Features",
        block_id_kebab: BlockIdKebab::Pricing01,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing02",
        block_title: "Multi-Tier Pricing with Feature Categories",
        block_id_kebab: BlockIdKebab::Pricing02,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing03",
        block_title: "Four-Tier Pricing with Equal Height Cards",
        block_id_kebab: BlockIdKebab::Pricing03,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing04",
        block_title: "Pricing Comparison Table with Feature Grid",
        block_id_kebab: BlockIdKebab::Pricing04,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing05",
        block_title: "Pricing with Monthly/Yearly Toggle",
        block_id_kebab: BlockIdKebab::Pricing05,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing06",
        block_title: "Responsive Pricing Table with Annual/Monthly Toggle",
        block_id_kebab: BlockIdKebab::Pricing06,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing07",
        block_title: "Centered Single Plan with Feature Grid",
        block_id_kebab: BlockIdKebab::Pricing07,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing08",
        block_title: "Pricing with Billing Cycle Toggle",
        block_id_kebab: BlockIdKebab::Pricing08,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing09",
        block_title: "Two-Tier Comparison Pricing with Tooltips",
        block_id_kebab: BlockIdKebab::Pricing09,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing10",
        block_title: "Four-Tier Pricing with Info Tooltips",
        block_id_kebab: BlockIdKebab::Pricing10,
        block_route: BlockRoutes::Login,
    },
    BlockEntry {
        block_id_str: "pricing11",
        block_title: "Three-Tier Pricing with Tabs and Tooltips",
        block_id_kebab: BlockIdKebab::Pricing11,
        block_route: BlockRoutes::Login,
    },
];

// * Sidenav
pub const ALL_SIDENAV_BLOCKS: &[BlockEntry] = &[
    BlockEntry {
        block_id_str: "sidenav01",
        block_title: "Simple Sidenav with grouped sections",
        block_id_kebab: BlockIdKebab::Sidenav01,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav02",
        block_title: "Simple Sidenav with Collapsible menus",
        block_id_kebab: BlockIdKebab::Sidenav02,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav03",
        block_title: "Simple Sidenav with submenus",
        block_id_kebab: BlockIdKebab::Sidenav03,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav04",
        block_title: "A Floating Sidenav with submenus",
        block_id_kebab: BlockIdKebab::Sidenav04,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav05",
        block_title: "Sidenav with Collapsible submenus",
        block_id_kebab: BlockIdKebab::Sidenav05,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav06",
        block_title: "Sidenav with Dropdown Submenus",
        block_id_kebab: BlockIdKebab::Sidenav06,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav07",
        block_title: "Collapsible Sidenav with Icons",
        block_id_kebab: BlockIdKebab::Sidenav07,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav08",
        block_title: "Inset Sidenav with secondary navigation",
        block_id_kebab: BlockIdKebab::Sidenav08,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav09",
        block_title: "Nested Sidenavs with Route-based Navigation",
        block_id_kebab: BlockIdKebab::Sidenav09,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav10",
        block_title: "Sidenav with search functionality",
        block_id_kebab: BlockIdKebab::Sidenav10,
        block_route: BlockRoutes::Sidenav,
    },
    BlockEntry {
        block_id_str: "sidenav11",
        block_title: "Right-Side Sidenav",
        block_id_kebab: BlockIdKebab::Sidenav11,
        block_route: BlockRoutes::Sidenav,
    },
];
