pub(crate) mod config;
pub(crate) mod get_collision_id;
pub(crate) mod get_collisions;
pub(crate) mod tw_merge_override;
pub(crate) mod validators;

pub use config::*;
pub use tw_merge_override::tw_merge_override;

/// Merges all the Tailwind classes, resolving conflicts.
///
/// Items can be anything that implements [`crate::AsTailwindClass`].
///
/// If you DON'T want to handle conflicts use [`crate::tw_join!`].
///
/// If you want to set global options use [`crate::merge::set_merge_options`].
///
/// If you want a custom type to be used with this macro, implement the [`crate::MaybeIntoTailwindClass`] trait.
#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        $crate::merge::merge_classes(joined.as_str())
    }};
}

/// Internal function triggered by the `tw_merge!` macro.
/// Merges all the Tailwind classes in the string, resolving conflicts.
///
/// If you need custom options use [`tw_merge_options`].
#[inline]
pub fn merge_classes(class: impl AsRef<str>) -> String {
    tw_merge_slice_options(&[class.as_ref()], Default::default())
}

/// Merges all the Tailwind classes in the provided strings, resolving conflicts.
/// Useful to avoid collecting all the strings into a single string.
///
/// If you need custom options use [`tw_merge_slice_options`].
#[inline]
pub fn tw_merge_slice(class: &[&str]) -> String {
    tw_merge_slice_options(class, Default::default())
}

/// Merges all the Tailwind classes, resolving conflicts, with the provided options.
///
/// ## Example: With Tailwind Prefix
///
/// ```
/// # use tw_merge::merge::*;
/// const OPTIONS: MergeOptions = MergeOptions {
///   prefix: "tw-",
///   separator: ":",
/// };
///
/// pub fn my_custom_tw_merge(class: impl AsRef<str>) -> String {
///    tw_merge_options(class, OPTIONS)
/// }
/// ```
#[inline]
pub fn tw_merge_options(class: impl AsRef<str>, options: MergeOptions) -> String {
    tw_merge_override::tw_merge_override(&[class.as_ref()], options, |_: &[&str], _: Option<&str>| None, |_: &str| None)
}

/// Merges all the Tailwind classes in the provided strings, resolving conflicts.
/// Useful to avoid collecting all the strings into a single string.
///
/// If you don't need custom options use [`tw_merge_slice`].
///
/// ## Example: With Tailwind Prefix
///
/// ```
/// # use tw_merge::merge::*;
/// const OPTIONS: MergeOptions = MergeOptions {
///   prefix: "tw-",
///   separator: ":",
/// };
///
/// pub fn my_custom_tw_merge(class: &[&str]) -> String {
///    tw_merge_slice_options(class, OPTIONS)
/// }
/// ```
#[inline]
pub fn tw_merge_slice_options(class: &[&str], options: MergeOptions) -> String {
    tw_merge_override::tw_merge_override(class, options, |_: &[&str], _: Option<&str>| None, |_: &str| None)
}

/// Return a ConflictId for a given Tailwind Class.
pub trait CollisionIdFn {
    /// elements: parts of the Tailwind class separated by `-`.
    ///
    /// (e.g. `bg-red-500` would be `["bg", "red", "500"]`)
    ///
    /// arbitrary: the arbitrary value at the end of the Tailwind class
    ///
    /// <https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values>
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str>;
}

impl<F> CollisionIdFn for F
where
    F: Fn(&[&str], Option<&str>) -> Option<&'static str> + 'static,
{
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str> {
        self(elements, arbitrary)
    }
}

/// Return list of CollisionIds that collide with the given CollisionId.
///
/// The list does not need to contain the given CollisionId.
///
/// e.g. "flex-row" should probably collide with "flex-col"
pub trait GetCollisionsFn {
    /// Return list of CollisionIds that collide with the given CollisionId.
    fn apply(&self, collision_id: &str) -> Option<Vec<&'static str>>;
}

impl<F> GetCollisionsFn for F
where
    F: Fn(&str) -> Option<Vec<&'static str>>,
{
    fn apply(&self, collision_id: &str) -> Option<Vec<&'static str>> {
        self(collision_id)
    }
}
