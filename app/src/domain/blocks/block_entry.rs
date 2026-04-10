use app_routes::BlockRoutes;

use crate::__registry__::all_blocks::{
    ALL_FAQ_BLOCKS, ALL_FOOTER_BLOCKS, ALL_HEADER_BLOCKS, ALL_INTEGRATION_BLOCKS, ALL_LOGIN_BLOCKS, ALL_SIDENAV_BLOCKS,
    BlockIdKebab,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn titles(entries: &[BlockEntry]) -> Vec<&'static str> {
        entries.iter().map(|entry| entry.block_title).collect()
    }

    #[test]
    fn login_block_titles_are_accurate_and_consistent() {
        let blocks = BlockEntry::get_login_blocks();
        let block_titles = titles(&blocks);

        assert_eq!(
            block_titles,
            vec![
                "Login Form Card",
                "Split Login with GitHub",
                "Social Login with Email Fallback",
                "Split Login with Social Buttons",
            ]
        );

        for block in blocks {
            assert_eq!(block.block_title, block.block_id_kebab.to_title());
        }
    }

    #[test]
    fn sidenav_block_titles_are_accurate_and_consistent() {
        let blocks = BlockEntry::get_sidenav_blocks();
        let block_titles = titles(&blocks);

        assert_eq!(
            block_titles,
            vec![
                "Sidenav with Grouped Sections",
                "Sidenav with Collapsible Menus",
                "Sidenav with Submenus",
                "Floating Sidenav with Submenus",
                "Sidenav with Collapsible Submenus",
                "Sidenav with Dropdown Submenus",
                "Collapsible Sidenav with Icons",
                "Inset Sidenav with Secondary Navigation",
                "Nested Sidenav with Route-Based Navigation",
                "Sidenav with Search",
                "Right-Side Sidenav",
            ]
        );

        for block in blocks {
            assert_eq!(block.block_title, block.block_id_kebab.to_title());
        }
    }

    #[test]
    fn login_blocks_include_password_field_security_and_usability_features() {
        let login_sources = [
            include_str!("../../../../app_crates/registry/src/blocks/login01.rs"),
            include_str!("../../../../app_crates/registry/src/blocks/login02.rs"),
            include_str!("../../../../app_crates/registry/src/blocks/login03.rs"),
            include_str!("../../../../app_crates/registry/src/blocks/login04.rs"),
        ];

        for source in login_sources {
            assert!(source.contains("autocomplete=\"current-password\""));
            assert!(source.contains("minlength=8"));
            assert!(source.contains("Show password"));
            assert!(source.contains("Hide password"));
        }
    }
}
