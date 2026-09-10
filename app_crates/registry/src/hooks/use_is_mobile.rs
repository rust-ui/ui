use dioxus::prelude::*;

use super::use_media_query::use_media_query;

pub const MOBILE_BREAKPOINT: u32 = 768;

pub fn use_is_mobile() -> ReadSignal<bool> {
    use_media_query(&format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1))
}
