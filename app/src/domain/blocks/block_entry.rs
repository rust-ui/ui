use app_routes::BlockRoutes;

use crate::__registry__::all_blocks::{
    ALL_FAQ_BLOCKS, ALL_FOOTER_BLOCKS, ALL_HEADER_BLOCKS, ALL_INTEGRATION_BLOCKS, ALL_LOGIN_BLOCKS,
    ALL_SIDENAV_BLOCKS, BlockIdKebab,
};

#[derive(Clone, Debug, PartialEq)]
pub enum BlockFileTreeItem {
    File { name: &'static str, index: usize },
    Folder { name: &'static str, items: Vec<BlockFileTreeItem> },
}

impl BlockFileTreeItem {
    pub fn name(&self) -> &'static str {
        match self {
            Self::File { name, .. } => name,
            Self::Folder { name, .. } => name,
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
    pub block_route: BlockRoutes,
}

impl BlockEntry {
    pub fn meta(&self) -> BlockMeta {
        self.block_id_kebab.meta()
    }
}

impl BlockEntry {
    /// Get login blocks
    pub fn get_login_blocks() -> Vec<BlockEntry> {
        ALL_LOGIN_BLOCKS.to_vec()
    }

    /// Get sidenav blocks
    pub fn get_sidenav_blocks() -> Vec<BlockEntry> {
        ALL_SIDENAV_BLOCKS.to_vec()
    }

    /// Get header blocks
    pub fn get_header_blocks() -> Vec<BlockEntry> {
        ALL_HEADER_BLOCKS.to_vec()
    }

    /// Get footer blocks
    pub fn get_footer_blocks() -> Vec<BlockEntry> {
        ALL_FOOTER_BLOCKS.to_vec()
    }

    /// Get footer blocks
    pub fn get_faq_blocks() -> Vec<BlockEntry> {
        ALL_FAQ_BLOCKS.to_vec()
    }

    /// Get integration blocks
    pub fn get_integration_blocks() -> Vec<BlockEntry> {
        ALL_INTEGRATION_BLOCKS.to_vec()
    }


}
