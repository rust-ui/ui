use serde::{Deserialize, Deserializer, Serialize};

use crate::tag::Tag;
use crate::NavigableEntry;

#[derive(Clone, Debug, PartialEq, Copy, Default)]
pub struct RegistryEntry {
    pub title: &'static str,
    pub path_url: &'static str,
    pub path_md: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub image: &'static str,
    pub image_dark: &'static str,
    pub is_new: bool,
    pub order: Option<u32>,
}

// TODO. 🤖 Find a way to simplify the serde.
// TODO. NOTE: I still want the Tag struct for the tags. Typesafety.
impl<'de> Deserialize<'de> for RegistryEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempRegistryEntry {
            title: String,
            #[serde(default)]
            path_url: String,
            #[serde(default)]
            path_md: String,
            description: String,
            #[serde(default)]
            tags: Vec<Tag>,
            #[serde(default)]
            image: String,
            #[serde(default)]
            image_dark: String,
            #[serde(default)]
            is_new: bool,
            #[serde(default)]
            order: Option<u32>,
        }

        let temp = TempRegistryEntry::deserialize(deserializer)?;

        // Convert Vec<Tag> to &'static [&'static str]
        let leaked_tags: Vec<&'static str> =
            temp.tags.into_iter().map(|tag| &*Box::leak(tag.to_string().into_boxed_str())).collect();
        let static_tags = Box::leak(leaked_tags.into_boxed_slice());

        Ok(RegistryEntry {
            title: &*Box::leak(temp.title.into_boxed_str()),
            path_url: &*Box::leak(temp.path_url.into_boxed_str()),
            path_md: &*Box::leak(temp.path_md.into_boxed_str()),
            description: &*Box::leak(temp.description.into_boxed_str()),
            tags: static_tags,
            image: &*Box::leak(temp.image.into_boxed_str()),
            image_dark: &*Box::leak(temp.image_dark.into_boxed_str()),
            is_new: temp.is_new,
            order: temp.order,
        })
    }
}

impl Serialize for RegistryEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RegistryEntry", 9)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("path_url", &self.path_url)?;
        state.serialize_field("path_md", &self.path_md)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("image", &self.image)?;
        state.serialize_field("image_dark", &self.image_dark)?;
        state.serialize_field("is_new", &self.is_new)?;
        state.serialize_field("order", &self.order)?;
        state.end()
    }
}

impl NavigableEntry for RegistryEntry {
    fn title(&self) -> &str {
        self.title
    }

    fn path_url(&self) -> &str {
        self.path_url
    }
}
