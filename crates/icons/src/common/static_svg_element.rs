/// Zero-allocation SVG element using &'static str for all data
#[derive(Debug, Clone, Copy)]
pub enum StaticSvgElement {
    Path {
        d: &'static str,
    },
    Circle {
        cx: &'static str,
        cy: &'static str,
        r: &'static str,
    },
    Rect {
        x: &'static str,
        y: &'static str,
        width: &'static str,
        height: &'static str,
        rx: Option<&'static str>,
        ry: Option<&'static str>,
    },
    Ellipse {
        cx: &'static str,
        cy: &'static str,
        rx: &'static str,
        ry: &'static str,
    },
    Line {
        x1: &'static str,
        y1: &'static str,
        x2: &'static str,
        y2: &'static str,
    },
    Polyline {
        points: &'static str,
    },
    Polygon {
        points: &'static str,
    },
}
