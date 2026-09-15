/// Filter items based on search query (prefix matching)
pub fn filter_items<'a>(items: &[&'a str], search_query: &str) -> Vec<&'a str> {
    if search_query.is_empty() {
        items.to_vec()
    } else {
        items
            .iter()
            .filter(|item| item.to_lowercase().starts_with(&search_query.to_lowercase()))
            .copied()
            .collect()
    }
}

/// Get the currently selected item based on scroll position and search query
pub fn get_selected_item<'a>(items: &[&'a str], scroll: usize, search_query: &str) -> Option<&'a str> {
    let filtered_items = filter_items(items, search_query);
    filtered_items.get(scroll).copied()
}

/// Get item at a specific visual index in the filtered list
pub fn get_item_at_visual_index<'a>(
    items: &[&'a str],
    visual_index: usize,
    search_query: &str,
) -> Option<&'a str> {
    let filtered_items = filter_items(items, search_query);
    filtered_items.get(visual_index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITEMS: &[&str] = &["button", "badge", "card", "checkbox", "dialog", "dropdown"];

    #[test]
    fn filter_items_empty_query_returns_all() {
        let result = filter_items(TEST_ITEMS, "");
        assert_eq!(result, TEST_ITEMS);
    }

    #[test]
    fn filter_items_matches_prefix() {
        let result = filter_items(TEST_ITEMS, "b");
        assert_eq!(result, vec!["button", "badge"]);
    }

    #[test]
    fn filter_items_case_insensitive() {
        let result = filter_items(TEST_ITEMS, "B");
        assert_eq!(result, vec!["button", "badge"]);
    }

    #[test]
    fn filter_items_no_match_returns_empty() {
        let result = filter_items(TEST_ITEMS, "xyz");
        assert!(result.is_empty());
    }

    #[test]
    fn filter_items_exact_match() {
        let result = filter_items(TEST_ITEMS, "button");
        assert_eq!(result, vec!["button"]);
    }

    #[test]
    fn get_selected_item_valid_index() {
        let result = get_selected_item(TEST_ITEMS, 2, "");
        assert_eq!(result, Some("card"));
    }

    #[test]
    fn get_selected_item_with_filter() {
        let result = get_selected_item(TEST_ITEMS, 1, "b");
        assert_eq!(result, Some("badge"));
    }

    #[test]
    fn get_selected_item_out_of_bounds() {
        let result = get_selected_item(TEST_ITEMS, 100, "");
        assert_eq!(result, None);
    }

    #[test]
    fn get_item_at_visual_index_valid() {
        let result = get_item_at_visual_index(TEST_ITEMS, 0, "c");
        assert_eq!(result, Some("card"));
    }

    #[test]
    fn get_item_at_visual_index_out_of_bounds() {
        let result = get_item_at_visual_index(TEST_ITEMS, 10, "c");
        assert_eq!(result, None);
    }
}
