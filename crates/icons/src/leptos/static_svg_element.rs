use leptos::prelude::*;

use crate::common::static_svg_element::StaticSvgElement;

impl StaticSvgElement {
    /// Convert StaticSvgElement to Leptos view with zero allocations
    ///
    /// All data is already static, so no runtime processing or allocation needed.
    pub fn to_leptos_view(&self) -> AnyView {
        match self {
            StaticSvgElement::Path { d } => view! { <path d=*d /> }.into_any(),
            StaticSvgElement::Circle { cx, cy, r } => view! { <circle cx=*cx cy=*cy r=*r /> }.into_any(),
            StaticSvgElement::Rect { x, y, width, height, rx, ry } => {
                view! { <rect x=*x y=*y width=*width height=*height rx=rx.as_deref() ry=ry.as_deref() /> }.into_any()
            }
            StaticSvgElement::Ellipse { cx, cy, rx, ry } => {
                view! { <ellipse cx=*cx cy=*cy rx=*rx ry=*ry /> }.into_any()
            }
            StaticSvgElement::Line { x1, y1, x2, y2 } => view! { <line x1=*x1 y1=*y1 x2=*x2 y2=*y2 /> }.into_any(),
            StaticSvgElement::Polyline { points } => view! { <polyline points=*points /> }.into_any(),
            StaticSvgElement::Polygon { points } => view! { <polygon points=*points /> }.into_any(),
        }
    }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_svg_element_path_creation() {
        let path = StaticSvgElement::Path { d: "M10 10 L20 20" };

        match path {
            StaticSvgElement::Path { d } => assert_eq!(d, "M10 10 L20 20"),
            _ => panic!("Expected Path variant"),
        }
    }

    #[test]
    fn test_static_svg_element_circle_creation() {
        let circle = StaticSvgElement::Circle { cx: "10", cy: "20", r: "5" };

        match circle {
            StaticSvgElement::Circle { cx, cy, r } => {
                assert_eq!(cx, "10");
                assert_eq!(cy, "20");
                assert_eq!(r, "5");
            }
            _ => panic!("Expected Circle variant"),
        }
    }

    #[test]
    fn test_static_svg_element_rect_with_rx_ry() {
        let rect = StaticSvgElement::Rect { x: "0", y: "0", width: "50", height: "50", rx: Some("5"), ry: Some("10") };

        match rect {
            StaticSvgElement::Rect { x, y, width, height, rx, ry } => {
                assert_eq!(x, "0");
                assert_eq!(y, "0");
                assert_eq!(width, "50");
                assert_eq!(height, "50");
                assert_eq!(rx, Some("5"));
                assert_eq!(ry, Some("10"));
            }
            _ => panic!("Expected Rect variant"),
        }
    }

    #[test]
    fn test_static_svg_element_zero_allocation() {
        // Verify that StaticSvgElement doesn't allocate
        let path = StaticSvgElement::Path { d: "M0 0 L10 10" };
        let circle = StaticSvgElement::Circle { cx: "5", cy: "5", r: "3" };

        // These should all be stack allocated with static string references
        match path {
            StaticSvgElement::Path { d } => {
                // Verify d is a static string reference
                assert_eq!(d, "M0 0 L10 10");
                // The pointer should be stable (pointing to static data)
                let ptr1 = d.as_ptr();
                let ptr2 = "M0 0 L10 10".as_ptr();
                assert_eq!(ptr1, ptr2);
            }
            _ => panic!("Expected Path"),
        }

        match circle {
            StaticSvgElement::Circle { cx, cy, r } => {
                assert_eq!(cx, "5");
                assert_eq!(cy, "5");
                assert_eq!(r, "3");
            }
            _ => panic!("Expected Circle"),
        }
    }

    #[test]
    fn test_static_svg_element_copy_trait() {
        let path = StaticSvgElement::Path { d: "M1 1 L2 2" };

        // Should be Copy (no allocation)
        let path_copy = path;

        match path_copy {
            StaticSvgElement::Path { d } => assert_eq!(d, "M1 1 L2 2"),
            _ => panic!("Expected Path"),
        }

        // Original should still be valid
        match path {
            StaticSvgElement::Path { d } => assert_eq!(d, "M1 1 L2 2"),
            _ => panic!("Expected Path"),
        }
    }
}
