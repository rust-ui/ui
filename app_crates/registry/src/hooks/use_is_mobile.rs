use leptos::prelude::*;

use super::use_media_query::use_media_query;

/// Mobile breakpoint in pixels (matches Tailwind's `md` breakpoint).
pub const MOBILE_BREAKPOINT: u32 = 768;

/// Reactive hook that returns `true` when the viewport is below the mobile breakpoint.
///
/// Equivalent to `use_media_query("(max-width: 767px)")`.
///
/// # Example
/// ```ignore
/// let is_mobile = use_is_mobile();
///
/// view! {
///     {move || if is_mobile.get() {
///         view! { <Drawer>...</Drawer> }.into_any()
///     } else {
///         view! { <Dialog>...</Dialog> }.into_any()
///     }}
/// }
/// ```
pub fn use_is_mobile() -> Signal<bool> {
    use_media_query(&format!("(max-width: {}px)", MOBILE_BREAKPOINT - 1))
}
