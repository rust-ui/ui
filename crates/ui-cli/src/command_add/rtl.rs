/// Direct physical-to-logical Tailwind class replacements.
/// Order matters: longer / more-specific patterns must come before shorter ones.
const DIRECT_REPLACEMENTS: &[(&str, &str)] = &[
    // negative margin (longer before shorter)
    ("-ml-", "-ms-"),
    ("-mr-", "-me-"),
    // positive margin
    ("ml-", "ms-"),
    ("mr-", "me-"),
    // padding
    ("pl-", "ps-"),
    ("pr-", "pe-"),
    // side utilities (with dash, longer first)
    ("-left-", "-start-"),
    ("-right-", "-end-"),
    ("left-", "start-"),
    ("right-", "end-"),
    // inset
    ("inset-l-", "inset-inline-start-"),
    ("inset-r-", "inset-inline-end-"),
    // rounded (corner-specific before side-specific)
    ("rounded-tl-", "rounded-ss-"),
    ("rounded-tr-", "rounded-se-"),
    ("rounded-bl-", "rounded-es-"),
    ("rounded-br-", "rounded-ee-"),
    ("rounded-l-", "rounded-s-"),
    ("rounded-r-", "rounded-e-"),
    // border (with dash before bare)
    ("border-l-", "border-s-"),
    ("border-r-", "border-e-"),
    ("border-l", "border-s"),
    ("border-r", "border-e"),
    // text alignment
    ("text-left", "text-start"),
    ("text-right", "text-end"),
    // scroll
    ("scroll-ml-", "scroll-ms-"),
    ("scroll-mr-", "scroll-me-"),
    ("scroll-pl-", "scroll-ps-"),
    ("scroll-pr-", "scroll-pe-"),
    // float / clear
    ("float-left", "float-start"),
    ("float-right", "float-end"),
    ("clear-left", "clear-start"),
    ("clear-right", "clear-end"),
    // origin (corner-specific before side-specific)
    ("origin-top-left", "origin-top-start"),
    ("origin-top-right", "origin-top-end"),
    ("origin-bottom-left", "origin-bottom-start"),
    ("origin-bottom-right", "origin-bottom-end"),
    ("origin-left", "origin-start"),
    ("origin-right", "origin-end"),
];

pub fn apply_rtl_transforms(content: &str) -> String {
    let mut result = content.to_string();

    for &(from, to) in DIRECT_REPLACEMENTS {
        result = result.replace(from, to);
    }

    result = add_suffix_after_prefix(result, "space-x-", "rtl:space-x-reverse", false);
    result = add_suffix_after_prefix(result, "divide-x-", "rtl:divide-x-reverse", false);

    // Positive translate-x-{n}: exclude preceding `-` (that's the negative variant)
    result = add_translate_rtl(result, false);
    // Negative -translate-x-{n}
    result = add_translate_rtl(result, true);

    result = swap_cursor_resize(result);

    result
}

/* ========================================================== */
/*                     ✨ HELPERS ✨                           */
/* ========================================================== */

/// After each `prefix{value}` class token inserts ` {rtl_class}`.
///
/// `exclude_dash_before`: when true, also fails the boundary check if the char
/// before the match is `-` (used to distinguish `translate-x-` from `-translate-x-`).
///
/// Idempotent: skips entirely if `rtl_class` is already present in the content.
fn add_suffix_after_prefix(
    content: String,
    prefix: &str,
    rtl_class: &str,
    exclude_dash_before: bool,
) -> String {
    if content.contains(rtl_class) || !content.contains(prefix) {
        return content;
    }

    let mut result = String::with_capacity(content.len() + rtl_class.len() + 8);
    let bytes = content.as_bytes();
    let plen = prefix.len();
    let mut cursor = 0;

    while cursor < bytes.len() {
        // Find next occurrence of the prefix from cursor
        let Some(rel) = find_bytes(&bytes[cursor..], prefix.as_bytes()) else {
            break;
        };
        let pos = cursor + rel;

        // Boundary check: char before prefix must not be alphanumeric
        // (and optionally not `-`)
        let prev = if pos > 0 { bytes[pos - 1] } else { 0 };
        let boundary_ok = !prev.is_ascii_alphanumeric() && !(exclude_dash_before && prev == b'-');

        if !boundary_ok {
            // Not a class boundary — advance past this char and retry
            result.push(content.as_bytes()[cursor] as char);
            cursor += 1;
            continue;
        }

        // Push everything up to and including the prefix
        result.push_str(&content[cursor..pos + plen]);
        cursor = pos + plen;

        // Collect the value (non-whitespace chars that follow the prefix)
        let value_end = bytes[cursor..]
            .iter()
            .position(|&b| b.is_ascii_whitespace() || matches!(b, b'"' | b'\'' | b')' | b']'))
            .map(|i| cursor + i)
            .unwrap_or(bytes.len());

        result.push_str(&content[cursor..value_end]);
        result.push(' ');
        result.push_str(rtl_class);
        cursor = value_end;
    }
    result.push_str(&content[cursor..]);
    result
}

/// Adds `rtl:-translate-x-{n}` after each `translate-x-{n}` (positive),
/// or `rtl:translate-x-{n}` after each `-translate-x-{n}` (negative).
fn add_translate_rtl(content: String, negative: bool) -> String {
    let (prefix, rtl_prefix, rtl_marker) = if negative {
        ("-translate-x-", "rtl:translate-x-", "rtl:translate-x-")
    } else {
        ("translate-x-", "rtl:-translate-x-", "rtl:-translate-x-")
    };

    if content.contains(rtl_marker) || !content.contains(prefix) {
        return content;
    }

    let mut result = String::with_capacity(content.len() + 24);
    let bytes = content.as_bytes();
    let plen = prefix.len();
    let mut cursor = 0;

    while cursor < bytes.len() {
        let Some(rel) = find_bytes(&bytes[cursor..], prefix.as_bytes()) else {
            break;
        };
        let pos = cursor + rel;

        // For positive: not preceded by `-` or alphanumeric
        // For negative: not preceded by alphanumeric
        let prev = if pos > 0 { bytes[pos - 1] } else { 0 };
        let boundary_ok = if negative {
            !prev.is_ascii_alphanumeric()
        } else {
            !prev.is_ascii_alphanumeric() && prev != b'-'
        };

        if !boundary_ok {
            result.push(content.as_bytes()[cursor] as char);
            cursor += 1;
            continue;
        }

        result.push_str(&content[cursor..pos + plen]);
        cursor = pos + plen;

        let value_end = bytes[cursor..]
            .iter()
            .position(|&b| b.is_ascii_whitespace() || matches!(b, b'"' | b'\'' | b')' | b']'))
            .map(|i| cursor + i)
            .unwrap_or(bytes.len());

        let value = &content[cursor..value_end];
        result.push_str(value);
        result.push(' ');
        result.push_str(rtl_prefix);
        result.push_str(value);
        cursor = value_end;
    }
    result.push_str(&content[cursor..]);
    result
}

fn swap_cursor_resize(content: String) -> String {
    const PLACEHOLDER: &str = "\x00RTL_CW\x00";
    content
        .replace("cursor-w-resize", PLACEHOLDER)
        .replace("cursor-e-resize", "cursor-w-resize")
        .replace(PLACEHOLDER, "cursor-e-resize")
}

/// Finds the first occurrence of `needle` in `haystack`, returning the byte offset.
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_ml_to_ms() {
        assert_eq!(apply_rtl_transforms("ml-4"), "ms-4");
    }

    #[test]
    fn transforms_negative_ml_to_negative_ms() {
        assert_eq!(apply_rtl_transforms("-ml-4"), "-ms-4");
    }

    #[test]
    fn transforms_rounded_tl_before_rounded_l() {
        assert_eq!(apply_rtl_transforms("rounded-tl-lg"), "rounded-ss-lg");
        assert_eq!(apply_rtl_transforms("rounded-l-lg"), "rounded-s-lg");
    }

    #[test]
    fn transforms_border_l_bare_and_with_value() {
        assert_eq!(apply_rtl_transforms("border-l-2"), "border-s-2");
        assert_eq!(apply_rtl_transforms("border-l"), "border-s");
    }

    #[test]
    fn transforms_origin_top_left_before_origin_left() {
        assert_eq!(apply_rtl_transforms("origin-top-left"), "origin-top-start");
        assert_eq!(apply_rtl_transforms("origin-left"), "origin-start");
    }

    #[test]
    fn transforms_text_left_to_text_start() {
        assert_eq!(apply_rtl_transforms("text-left"), "text-start");
        assert_eq!(apply_rtl_transforms("text-right"), "text-end");
    }

    #[test]
    fn preserves_sm_responsive_prefix() {
        assert_eq!(apply_rtl_transforms("sm:ml-4"), "sm:ms-4");
    }

    #[test]
    fn preserves_hover_variant_prefix() {
        assert_eq!(apply_rtl_transforms("hover:ml-4"), "hover:ms-4");
    }

    #[test]
    fn handles_arbitrary_value_brackets() {
        assert_eq!(apply_rtl_transforms("ml-[24px]"), "ms-[24px]");
    }

    #[test]
    fn adds_rtl_space_x_reverse() {
        let result = apply_rtl_transforms("space-x-4");
        assert!(result.contains("space-x-4"), "original class preserved");
        assert!(result.contains("rtl:space-x-reverse"), "rtl class added");
    }

    #[test]
    fn does_not_duplicate_rtl_space_x_reverse() {
        let once = apply_rtl_transforms("space-x-4");
        let twice = apply_rtl_transforms(&once);
        assert_eq!(once.matches("rtl:space-x-reverse").count(), 1);
        assert_eq!(twice.matches("rtl:space-x-reverse").count(), 1);
    }

    #[test]
    fn adds_rtl_divide_x_reverse() {
        let result = apply_rtl_transforms("divide-x-2");
        assert!(result.contains("divide-x-2"));
        assert!(result.contains("rtl:divide-x-reverse"));
    }

    #[test]
    fn adds_rtl_negative_translate_x() {
        let result = apply_rtl_transforms("translate-x-4");
        assert!(result.contains("translate-x-4"));
        assert!(result.contains("rtl:-translate-x-4"));
    }

    #[test]
    fn adds_rtl_positive_for_negative_translate_x() {
        let result = apply_rtl_transforms("-translate-x-4");
        assert!(result.contains("-translate-x-4"));
        assert!(result.contains("rtl:translate-x-4"));
    }

    #[test]
    fn swaps_cursor_w_and_e_resize() {
        assert_eq!(apply_rtl_transforms("cursor-w-resize"), "cursor-e-resize");
        assert_eq!(apply_rtl_transforms("cursor-e-resize"), "cursor-w-resize");
        let both = apply_rtl_transforms("cursor-w-resize cursor-e-resize");
        let w_count = both.matches("cursor-w-resize").count();
        let e_count = both.matches("cursor-e-resize").count();
        assert_eq!(w_count, 1);
        assert_eq!(e_count, 1);
    }

    #[test]
    fn idempotent_on_already_transformed_content() {
        let once = apply_rtl_transforms("ml-4 pl-2 text-left");
        let twice = apply_rtl_transforms(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn empty_input_returns_empty() {
        assert_eq!(apply_rtl_transforms(""), "");
    }

    #[test]
    fn multiple_classes_in_one_string() {
        let result = apply_rtl_transforms("flex ml-4 hover:pl-2 rounded-tl-lg border-l");
        assert!(result.contains("ms-4"));
        assert!(result.contains("hover:ps-2"));
        assert!(result.contains("rounded-ss-lg"));
        assert!(result.contains("border-s"));
    }
}
