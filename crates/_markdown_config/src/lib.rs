pub mod registry_entry;
pub mod tag;

pub use registry_entry::RegistryEntry;
pub use tag::Tag;

/// Trait for entries that can be navigated to (prev/next).
pub trait NavigableEntry {
    fn title(&self) -> &str;
    fn path_url(&self) -> &str;
}
