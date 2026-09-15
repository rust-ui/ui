/// Joins the given classes into a single string.
mod tw_join;

/// Merges all the Tailwind classes, resolving conflicts.
pub mod merge;

/* ========================================================== */
/*                       🏹 TRAITS 🏹                         */
/* ========================================================== */

/// Used to extract a &str from a type
///
/// Implement this trait for your type to use it with the [`tw_join!`] and [`tw_merge!`] macros
pub trait AsTailwindClass {
    /// Extract a Tailwind class
    fn as_class(&self) -> &str;
}

impl AsTailwindClass for String {
    fn as_class(&self) -> &str {
        self.as_str()
    }
}

impl AsTailwindClass for &str {
    fn as_class(&self) -> &str {
        self
    }
}

impl<T> AsTailwindClass for &T
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        (*self).as_class()
    }
}

impl<T> AsTailwindClass for &mut T
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        (**self).as_class()
    }
}

impl<T> AsTailwindClass for std::rc::Rc<T>
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        self.as_ref().as_class()
    }
}

impl<T> AsTailwindClass for std::sync::Arc<T>
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        self.as_ref().as_class()
    }
}

impl AsTailwindClass for std::borrow::Cow<'_, str> {
    fn as_class(&self) -> &str {
        self.as_ref()
    }
}

impl<T> AsTailwindClass for Box<T>
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        (**self).as_class()
    }
}

impl<T> AsTailwindClass for Option<T>
where
    T: AsTailwindClass,
{
    fn as_class(&self) -> &str {
        match self {
            Some(t) => t.as_class(),
            None => "",
        }
    }
}
