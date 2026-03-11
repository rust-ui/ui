//! Pagination and data grid configuration constants.

/// Pagination and data grid configuration
pub struct PAGINATION;

impl PAGINATION {
    /// Default number of rows per page
    pub const DEFAULT_PAGE_SIZE: u32 = 1000;

    /// Available page size options for the dropdown
    pub const PAGE_SIZE_OPTIONS: [u32; 3] = [500, 700, 1000];

    /// Row height in pixels for virtual scrolling (must match CSS)
    pub const ROW_HEIGHT: usize = 36;
}
