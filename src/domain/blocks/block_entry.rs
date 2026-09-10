use crate::__registry__::all_blocks::BlockIdKebab;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockFileTreeItem {
    File { name: &'static str, index: usize },
    Folder { name: &'static str, items: Vec<BlockFileTreeItem> },
}

impl BlockFileTreeItem {
    pub fn name(&self) -> &'static str {
        match self {
            Self::File { name, .. } | Self::Folder { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct BlockFile {
    pub name: &'static str,
    pub target: &'static str,
    pub content: &'static str,
    pub language: &'static str,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct BlockMeta {
    pub iframe_height: &'static str,
    pub container_class: &'static str,
}

impl BlockMeta {
    pub const fn default() -> Self {
        Self { iframe_height: "930px", container_class: "w-full bg-background" }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct BlockEntry {
    pub block_id_str: &'static str,
    pub block_title: &'static str,
    pub block_id_kebab: BlockIdKebab,
    pub category: &'static str,
}
