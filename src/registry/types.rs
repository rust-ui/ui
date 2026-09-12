use crate::markdown::parse_md;

pub struct RegistryEntry {
    pub slug: &'static str,
    pub raw: &'static str,
    pub tags: &'static [&'static str],
}

impl PartialEq for RegistryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

impl RegistryEntry {
    #[must_use]
    pub fn title(&self) -> String {
        parse_md(self.raw).0.title
    }

    #[must_use]
    pub fn description(&self) -> String {
        parse_md(self.raw).0.description
    }

    #[must_use]
    pub fn body_md(&self) -> &str {
        parse_md(self.raw).1
    }
}
