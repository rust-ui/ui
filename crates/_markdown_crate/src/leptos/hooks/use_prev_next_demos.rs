use std::sync::Arc;

/// Trait for types that can be used with prev/next navigation
pub trait NavigableEntry {
    fn title(&self) -> &str;
    fn path_url(&self) -> &str;
}

pub fn use_prev_next_demos<T: NavigableEntry>(
    all_demos: Arc<Vec<T>>,
    current_demo_name: String,
    base_path: Arc<String>,
) -> (String, String, String, String) {
    let current_index = all_demos.iter().position(|demo| demo.title() == current_demo_name);

    let (prev_demo, next_demo) = if let Some(index) = current_index {
        let prev_index = if index == 0 { all_demos.len() - 1 } else { index - 1 };
        let next_index = if index == all_demos.len() - 1 { 0 } else { index + 1 };
        (all_demos.get(prev_index), all_demos.get(next_index))
    } else {
        (None, None)
    };

    let (prev_demo_name, prev_demo_href) = if let Some(demo) = prev_demo {
        (demo.title().to_string(), format!("{base_path}/{}", demo.path_url()))
    } else {
        ("".to_string(), "".to_string())
    };

    let (next_demo_name, next_demo_href) = if let Some(demo) = next_demo {
        (demo.title().to_string(), format!("{base_path}/{}", demo.path_url()))
    } else {
        ("".to_string(), "".to_string())
    };

    (prev_demo_name, next_demo_name, prev_demo_href, next_demo_href)
}
