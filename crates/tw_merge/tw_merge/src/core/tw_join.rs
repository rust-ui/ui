/// Joins the given classes into a single string.
///
/// Items can be anything that implements [`crate::AsTailwindClass`].
///
/// If you want to handle conflicts use [`crate::tw_merge!`].
///
/// If you want a custom type to be used with this macro, implement the [`crate::AsTailwindClass`] trait.
#[macro_export]
macro_rules! tw_join {
    ($item:expr) => {{
        $crate::AsTailwindClass::as_class(&$item).trim().to_string()
    }};
    ($($item:expr),+ $(,)?) => {{
        let mut result = String::with_capacity(
            0 $(+ $crate::AsTailwindClass::as_class(&$item).len())*
        );
        $(
            let class = $item;
            let class = $crate::AsTailwindClass::as_class(&class);
            let class = class.trim();
            if !class.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(class);
            }
        )*
        result
    }};
}

#[test]
fn join() {
    assert_eq!(tw_join!("a"), "a");
    assert_eq!(tw_join!("a", "b"), "a b");
    assert_eq!(tw_join!("a", "b", "c"), "a b c");
    assert_eq!(tw_join!("a", "b", "c", "d"), "a b c d");
    assert_eq!(tw_join!("a", "b", "c", "d", "e"), "a b c d e");
    assert_eq!(tw_join!("a", "b", "c", "d", "e", "f"), "a b c d e f");

    assert_eq!(tw_join!(" one", "two ", " three".to_string()), "one two three");
    assert_eq!(tw_join!("a", "    ", "b", "c", " "), "a b c");

    assert_eq!(tw_join!("a", (false).then_some("b"), (true).then_some("c")), "a c")
}
