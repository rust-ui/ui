use dioxus::prelude::*;

use crate::common::static_svg_element::StaticSvgElement;

// * 🧪 cargo test --package icons --lib --features dioxus

impl StaticSvgElement {
    pub fn to_dioxus_element(&self) -> Element {
        match self {
            StaticSvgElement::Path { d } => rsx! {
                path { d: *d }
            },
            StaticSvgElement::Circle { cx, cy, r } => rsx! {
                circle { cx: *cx, cy: *cy, r: *r }
            },
            StaticSvgElement::Rect { x, y, width, height, rx, ry } => match (rx, ry) {
                (Some(rx), Some(ry)) => rsx! {
                    rect {
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                        rx: *rx,
                        ry: *ry,
                    }
                },
                (Some(rx), None) => rsx! {
                    rect {
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                        rx: *rx,
                    }
                },
                (None, Some(ry)) => rsx! {
                    rect {
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                        ry: *ry,
                    }
                },
                (None, None) => rsx! {
                    rect {
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                    }
                },
            },
            StaticSvgElement::Ellipse { cx, cy, rx, ry } => rsx! {
                ellipse {
                    cx: *cx,
                    cy: *cy,
                    rx: *rx,
                    ry: *ry,
                }
            },
            StaticSvgElement::Line { x1, y1, x2, y2 } => rsx! {
                line {
                    x1: *x1,
                    y1: *y1,
                    x2: *x2,
                    y2: *y2,
                }
            },
            StaticSvgElement::Polyline { points } => rsx! {
                polyline { points: *points }
            },
            StaticSvgElement::Polygon { points } => rsx! {
                polygon { points: *points }
            },
        }
    }
}
