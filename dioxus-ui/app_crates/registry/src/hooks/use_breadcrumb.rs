const LAST_SEGMENT_INDEX: usize = 1;

fn to_title_case(s: &str) -> String {
    s.split(['-', '_', ' '])
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// Custom hook for breadcrumb navigation starting from a specific segment (inclusive)
pub fn use_breadcrumb_from_segment(start_segment: &str) -> Vec<(String, String, bool)> {
    build_breadcrumb_items(start_segment, true)
}

// Custom hook for breadcrumb navigation starting after a specific segment (exclusive)
pub fn use_breadcrumb_after_segment(start_segment: &str) -> Vec<(String, String, bool)> {
    build_breadcrumb_items(start_segment, false)
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn build_breadcrumb_items(start_segment: &str, inclusive: bool) -> Vec<(String, String, bool)> {
    #[cfg(target_arch = "wasm32")]
    let path = web_sys::window().and_then(|w| w.location().pathname().ok()).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    let path = String::new();

    let segments: Vec<String> = path.split('/').filter(|segment| !segment.is_empty()).map(String::from).collect();

    segments
        .iter()
        .position(|segment| segment == start_segment)
        .map(|start_idx| {
            let actual_start_idx = if inclusive { start_idx } else { start_idx + 1 };

            if actual_start_idx >= segments.len() {
                return Vec::new();
            }

            segments
                .iter()
                .enumerate()
                .skip(actual_start_idx)
                .map(|(i, segment)| {
                    let path = segments.get(..=i).map(|s| s.join("/")).unwrap_or_default();
                    (to_title_case(segment), format!("/{path}"), i == segments.len() - LAST_SEGMENT_INDEX)
                })
                .collect()
        })
        .unwrap_or_default()
}
