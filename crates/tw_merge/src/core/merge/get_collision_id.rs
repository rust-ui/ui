use super::validators;

pub type Result<T> = std::result::Result<T, &'static str>;

pub fn get_collision_id(classes: &[&str], arbitrary: &str) -> Result<&'static str> {
    match classes {
        // https://tailwindcss.com/docs/aspect-ratio
        ["aspect", "auto" | "square" | "video"] => Ok("aspect"),
        ["aspect", n] if parse_fraction(n).is_some() => Ok("aspect"),
        ["aspect"] if parse_decimal_fraction(arbitrary) => Ok("aspect"), 
        // https://tailwindcss.com/docs/container
        ["container"] => Ok("container"),

        // v4: https://tailwindcss.com/docs/container-queries
        // @container, @container/name (named containers)
        ["@container"] => Ok("@container"),
        [first, ..] if first.starts_with("@container/") || first.starts_with("@container") => Ok("@container"),

        // https://tailwindcss.com/docs/columns
        ["columns", "auto"] => Ok("columns"),
        ["columns", rest] if is_t_shirt_size(rest) || rest.parse::<usize>().is_ok() => {
            Ok("columns")
        }
        ["columns"] if is_arbitrary_len(arbitrary) => {
            Ok("columns")
        }
        // https://tailwindcss.com/docs/break-after
        ["break", "after", rest] if valid_break_after(rest) => Ok("break-after"),

        // https://tailwindcss.com/docs/break-before
        ["break", "before", rest] if valid_break_after(rest) => {
            Ok("break-before")
        }
        // https://tailwindcss.com/docs/break-inside
        ["break", "inside", rest] => {
            if valid_break_after(rest) {
                Ok("break-inside")
            } else {
                Err("Invalid break-inside")
            }
        }
        // https://tailwindcss.com/docs/box-decoration-break
        ["box", "decoration", "clone" | "slice"] => {
            Ok("box-decoration-break")
        }
        // https://tailwindcss.com/docs/box-sizing
        ["box", "border" | "content"] => Ok("box-sizing"),

        // https://tailwindcss.com/docs/display
        ["block"]
        | ["inline", "block"]
        | ["inline"]
        | ["flex"]
        | ["inline","flex"]
        | ["table"]
        | ["inline","table"]
        | ["table","caption"]
        | ["table","cell"]
        | ["table","column"]
        | ["table","column","group"]
        | ["table","footer","group"]
        | ["table","header","group"]
        | ["table","row","group"]
        | ["table","row"]
        | ["flow","root"]
        | ["grid"]
        | ["inline","grid"]
        | ["contents"]
        | ["hidden"]
            if arbitrary.is_empty() =>
        {
            Ok("display")
        }

        // https://tailwindcss.com/docs/float
        ["float", "start" | "end" | "right" | "none"] => {
            Ok("float")
        }

        // https://tailwindcss.com/docs/clear
        ["clear", "start" | "end" | "right" | "both" | "none"] => Ok("clear"),

        // https://tailwindcss.com/docs/isolation
        ["isolation"] | ["isolation", "auto"] => Ok("isolation"),

        // https://tailwindcss.com/docs/object-fit
        ["object", "contain"]
        | ["object", "cover"]
        | ["object", "fill"]
        | ["object", "none"]
        | ["object", "scale", "down"] => Ok("object-fit"),

        // https://tailwindcss.com/docs/object-position
        ["object", "bottom"]
        | ["object", "center"]
        | ["object", "left"]
        | ["object", "left", "bottom"]
        | ["object", "left", "top"]
        | ["object", "right"]
        | ["object", "right", "bottom"]
        | ["object", "right", "top"]
        | ["object", "top"] => Ok("object-position"),

        // https://tailwindcss.com/docs/overflow
        ["overflow", "auto"]
        | ["overflow", "hidden"]
        | ["overflow", "clip"]
        | ["overflow", "visible"]
        | ["overflow", "scroll"]
        | ["overflow", "x", "auto"]
        | ["overflow", "y", "auto"]
        | ["overflow","x", "hidden"]
        | ["overflow","y", "hidden"]
        | ["overflow", "x", "clip"]
        | ["overflow", "y","clip"]
        | ["overflow","x", "visible"]
        | ["overflow", "y", "visible"]
        | ["overflow","x", "scroll"]
        | ["overflow","y", "scroll"] => Ok("overflow"),

        // https://tailwindcss.com/docs/overscroll-behavior
        ["overscroll", "auto"]
        | ["overscroll", "contain"]
        | ["overscroll", "none"]
        | ["overscroll", "y", "auto"]
        | ["overscroll", "y", "contain"]
        | ["overscroll", "y", "none"]
        | ["overscroll", "x", "auto"]
        | ["overscroll", "x", "contain"]
        | ["overscroll", "x", "none"] => Ok("overscroll-behavior"),

        // https://tailwindcss.com/docs/position
        ["static"] | ["fixed"] | ["absolute"] | ["relative"] | ["sticky"] => Ok("position"),

        // v4: https://tailwindcss.com/docs/box-shadow#adding-an-inset-shadow
        ["inset", "shadow", "none"] => Ok("inset-shadow"),
        ["inset", "shadow", size] if is_t_shirt_size(size) => Ok("inset-shadow"),
        ["inset", "shadow"] if arbitrary.is_empty() => Ok("inset-shadow"),
        ["inset", "shadow", ..] => Ok("inset-shadow-color"),

        // v4: https://tailwindcss.com/docs/ring-width#adding-an-inset-ring
        ["inset", "ring", rest] if rest.parse::<usize>().is_ok() => Ok("inset-ring"),
        ["inset", "ring"] if arbitrary.is_empty() || arbitrary.parse::<usize>().is_ok() => Ok("inset-ring"),
        ["inset", "ring", ..] => Ok("inset-ring-color"),

        // https://tailwindcss.com/docs/top-right-bottom-left
        ["inset", "x", rest @ ..] => valid_trbl(rest, arbitrary, "inset-x", "Invalid inset-x"),
        ["inset", "y", rest @ ..] => valid_trbl(rest, arbitrary, "inset-y", "Invalid inset-y"),
        // https://tailwindcss.com/docs/top-right-bottom-left#using-logical-properties
        ["inset", "inline", "start", rest @ ..] => valid_trbl(rest, arbitrary, "start", "Invalid inset-inline-start"),
        ["inset", "inline", "end", rest @ ..] => valid_trbl(rest, arbitrary, "end", "Invalid inset-inline-end"),
        ["inset", "inline", rest @ ..] => valid_trbl(rest, arbitrary, "inset-x", "Invalid inset-inline"),
        ["inset", "block", "start", rest @ ..] => valid_trbl(rest, arbitrary, "top", "Invalid inset-block-start"),
        ["inset", "block", "end", rest @ ..] => valid_trbl(rest, arbitrary, "bottom", "Invalid inset-block-end"),
        ["inset", "block", rest @ ..] => valid_trbl(rest, arbitrary, "inset-y", "Invalid inset-block"),
        ["inset", rest @ ..] => valid_trbl(rest, arbitrary, "inset", "Invalid inset"),
        ["top", rest @ ..] => valid_trbl(rest, arbitrary, "top", "Invalid top"),
        ["right", rest @ ..] => valid_trbl(rest, arbitrary, "right", "Invalid right"),
        ["bottom", rest @ ..] => valid_trbl(rest, arbitrary, "bottom", "Invalid bottom"),
        ["left", rest @ ..] => valid_trbl(rest, arbitrary, "left", "Invalid left"),
        ["start", rest @ ..] => valid_trbl(rest, arbitrary, "start", "Invalid start"),
        ["end", rest @ ..] => valid_trbl(rest, arbitrary, "end", "Invalid end"),

        // https://tailwindcss.com/docs/visibility
        ["visible" | "invisible" | "collapse"] => Ok("visibility"),

        // https://tailwindcss.com/docs/z-index
        ["z", "auto"] => Ok("z-index"),
        ["z", index] => match index.parse::<usize>().ok() {
            Some(_) => Ok("z-index"),
            None => Err("Invalid z index"),
        },
        ["z"] => match arbitrary.parse::<usize>().ok() {
            Some(_) => Ok("z-index"),
            None => Err("Invalid z index"),
        },

        // https://tailwindcss.com/docs/flex-basis
        ["basis", "full" | "auto" | "px" ] => Ok("flex-basis"),
        ["basis", rest] => {
            if parse_fraction_or_usize(rest) {
                Ok("flex-basis")
            } else {
                Err("Invalid flex-basis")
            }
        }
        ["basis"] => {
            if parse_fraction_or_usize(arbitrary) {
                Ok("flex-basis")
            } else {
                Err("Invalid flex-basis")
            }
        }

        // https://tailwindcss.com/docs/flex-direction
        ["flex", "row"]
        | ["flex", "row", "reverse"]
        | ["flex", "col"]
        | ["flex", "col", "reverse"] => Ok("flex-direction"),

        // https://tailwindcss.com/docs/flex-wrap
        ["flex", "wrap"] | ["flex", "wrap", "reverse"] | ["flex", "nowrap"] => Ok("flex-wrap"),

        // https://tailwindcss.com/docs/flex
        ["flex", "1"] | ["flex", "auto"] | ["flex", "initial"] | ["flex", "none"] => Ok("flex"),
        // TODO: check this?
        ["flex", _] => Ok("flex"),
        ["flex"] if !arbitrary.is_empty() => Ok("flex"),

        // https://tailwindcss.com/docs/flex-grow
        ["grow", ..] => Ok("flex-grow"),

        // https://tailwindcss.com/docs/flex-shrink
        ["shrink", ..] => Ok("flex-shrink"),

        // https://tailwindcss.com/docs/order
        ["order", "first" | "last" | "none"] => Ok("order"),
        ["order", rest] if rest.parse::<isize>().is_ok() => Ok("order"),
        ["order"] if arbitrary.parse::<isize>().is_ok() => Ok("order"),

        // https://tailwindcss.com/docs/grid-template-columns
        ["grid", "cols", ..] => Ok("grid-template-columns"),

        // https://tailwindcss.com/docs/grid-column
        ["col", "auto"] | ["col", "span", ..] => Ok("col-start-end"),
        ["col", "start", ..] => Ok("col-start"),
        ["col", "end", ..] => Ok("col-end"),

        // https://tailwindcss.com/docs/grid-template-rows
        ["grid", "rows", ..] => Ok("grid-template-rows"),

        // https://tailwindcss.com/docs/grid-row
        ["row", "auto"] | ["row", "span", ..] => Ok("row-start-end"),
        ["row", "start", ..] => Ok("row-start"),
        ["row", "end", ..] => Ok("row-end"),

        // https://tailwindcss.com/docs/grid-auto-flow
        ["grid", "flow", "row"]
        | ["grid", "flow", "col"]
        | ["grid", "flow", "dense"]
        | ["grid", "flow", "row", "dense"]
        | ["grid", "flow", "col", "dense"] => Ok("grid-auto-flow"),

        // https://tailwindcss.com/docs/grid-auto-columns
        ["auto", "cols", ..] => Ok("auto-cols"),

        // https://tailwindcss.com/docs/grid-auto-rows
        ["auto", "rows", ..] => Ok("auto-rows"),

        // https://tailwindcss.com/docs/gap
        ["gap", "x", ..] => Ok("gap-x"),
        ["gap", "y", ..] => Ok("gap-y"),
        ["gap", ..] => Ok("gap"),

        // https://tailwindcss.com/docs/justify-content
        // v4.1: safe alignment variants (justify-center-safe, etc.)
        ["justify", "normal" | "start" | "end" | "center" | "between" | "around" | "evenly" | "stretch"] => Ok("justify-content"),
        ["justify", "center" | "start" | "end", "safe"] => Ok("justify-content"),
        // https://tailwindcss.com/docs/justify-items
        ["justify", "items", "start" | "end" | "center" | "stretch"] => Ok("justify-items"),
        ["justify", "items", "center" | "start" | "end", "safe"] => Ok("justify-items"),

        // https://tailwindcss.com/docs/justify-self
        ["justify", "self", "start" | "end" | "center" | "stretch"] => Ok("justify-self"),
        ["justify", "self", "center" | "start" | "end", "safe"] => Ok("justify-self"),

        // https://tailwindcss.com/docs/align-content
        ["content", "normal" | "center" | "start" | "end" | "between" | "around" | "evenly" | "baseline" | "stretch"] => Ok("align-content"),
        ["content", "center" | "start" | "end", "safe"] => Ok("align-content"),

        // https://tailwindcss.com/docs/align-items
        // v4.1: items-baseline-last
        ["items", "start" | "end" | "center" | "baseline" | "stretch"] => Ok("align-items"),
        ["items", "center" | "start" | "end", "safe"] => Ok("align-items"),
        ["items", "baseline", "last"] => Ok("align-items"),

        // https://tailwindcss.com/docs/align-self
        // v4.1: self-baseline-last
        ["self", "auto" | "start" | "end" | "center" | "stretch" | "baseline"] => Ok("align-self"),
        ["self", "center" | "start" | "end", "safe"] => Ok("align-self"),
        ["self", "baseline", "last"] => Ok("align-self"),

        // https://tailwindcss.com/docs/place-content
        ["place", "content", "center" | "start" | "end" | "between" | "around" | "evenly" | "baseline" | "stretch"] => Ok("place-content"),
        ["place", "content", "center" | "start" | "end", "safe"] => Ok("place-content"),

        // https://tailwindcss.com/docs/place-items
        ["place", "items", "start" | "end" | "center" | "baseline" | "stretch"] => Ok("place-items"),
        ["place", "items", "center" | "start" | "end", "safe"] => Ok("place-items"),

        // https://tailwindcss.com/docs/place-self
        ["place", "self", "start" | "end" | "center" | "baseline" | "stretch"] => Ok("place-self"),
        ["place", "self", "center" | "start" | "end", "safe"] => Ok("place-self"),

        // https://tailwindcss.com/docs/padding
        ["p", ..] => Ok("padding"),
        ["pl", ..] => Ok("padding-left"),
        ["pr", ..] => Ok("padding-right"),
        ["pt", ..] => Ok("padding-top"),
        ["pb", ..] => Ok("padding-bottom"),
        ["px", ..] => Ok("padding-x"),
        ["py", ..] => Ok("padding-y"),

        // https:: //tailwindcss.com/docs/margin
        ["m", ..] => Ok("margin"),
        ["ml", ..] => Ok("margin-left"),
        ["mr", ..] => Ok("margin-right"),
        ["mt", ..] => Ok("margin-top"),
        ["mb", ..] => Ok("margin-bottom"),
        ["mx", ..] => Ok("margin-x"),
        ["my", ..] => Ok("margin-y"),
        ["ms", ..] => Ok("margin-start"),
        ["me", ..] => Ok("margin-end"),

        // https://tailwindcss.com/docs/space
        ["space", "x", "reverse"] => Ok("space-x-reverse"),
        ["space", "y", "reverse"] => Ok("space-y-reverse"),
        ["space", "x", ..] => Ok("space-x"),
        ["space", "y", ..] => Ok("space-y"),

        // https://tailwindcss.com/docs/width
        // TODO: Add validation?
        ["w", ..] => Ok("width"),

        // https://tailwindcss.com/docs/min-width
        // TODO: Add validation?
        ["min", "w", ..] => Ok("min-width"),

        // https://tailwindcss.com/docs/max-width
        ["max", "w", ..] => Ok("max-width"),

        // https://tailwindcss.com/docs/height
        ["h", ..] => Ok("height"),

        // https://tailwindcss.com/docs/min-height
        ["min", "h", ..] => Ok("min-height"),

        // https://tailwindcss.com/docs/max-height
        ["max", "h", ..] => Ok("max-height"),

        // https://tailwindcss.com/docs/size
        ["size", ..] => Ok("size"),

        // https://tailwindcss.com/docs/font-family
        // TODO: This clash is bad
        ["font", "sans"] | ["font", "serif"] | ["font", "mono"] => Ok("font-family"),

        // https://tailwindcss.com/docs/text-align
        ["text", "left" | "center" | "right" | "justify" | "start" | "end"] => Ok("text-align"),

        // https://tailwindcss.com/docs/text-overflow
        ["text", "ellipsis" | "clip"] => Ok("text-overflow"),

        // https://tailwindcss.com/docs/text-wrap
        ["text", "wrap" | "nowrap" | "balance" | "pretty"] => Ok("text-wrap"),

        // v4.1: https://tailwindcss.com/docs/overflow-wrap
        ["wrap", "break", "word"] => Ok("overflow-wrap"),
        ["wrap", "anywhere"] => Ok("overflow-wrap"),
        ["wrap", "normal"] => Ok("overflow-wrap"),

        // https://tailwindcss.com/docs/font-size
        ["text", rest] if valid_text_size(rest) => Ok("font-size"),
        ["text"] if is_arbitrary_len(arbitrary) => Ok("font-size"),

        // v4: https://tailwindcss.com/docs/text-shadow
        ["text", "shadow", "none"] => Ok("text-shadow"),
        ["text", "shadow", size] if is_t_shirt_size(size) => Ok("text-shadow"),
        ["text", "shadow"] => Ok("text-shadow"),
        ["text", "shadow", ..] => Ok("text-shadow-color"),

        // https://tailwindcss.com/docs/text-color
        ["text", ..] => Ok("text-color"),

        // https://tailwindcss.com/docs/font-smoothing
        ["antialiased"] | ["subpixel", "antialiased"] => Ok("font-smoothing"),

        // https://tailwindcss.com/docs/font-style
        ["italic"] | ["not","italic"] => Ok("font-style"),

        // v4: https://tailwindcss.com/docs/font-stretch
        ["font", "stretch", ..] => Ok("font-stretch"),

        // https://tailwindcss.com/docs/font-weight
        ["font", ..] => Ok("font-weight"),

        // https://tailwindcss.com/docs/font-variant-numeric
        ["normal", "nums"] => Ok("fvn-normal"),
        ["ordinal"] => Ok("fvn-ordinal"),
        ["slashed", "zero"] => Ok("fvn-slashed-zero"),
        ["lining", "nums"] | ["oldstyle", "nums"] => Ok("fvn-figure"),
        ["proportional", "nums"] | ["tabular","nums"] => Ok("fvn-spacing"),
        ["diagonal", "fractions"] | ["stacked","fractions"] => Ok("fvn-fraction"),

        // https://tailwindcss.com/docs/letter-spacing
        ["tracking", ..] => Ok("letter-spacing"),

        // https://tailwindcss.com/docs/line-clamp
        ["line", "clamp", ..] => Ok("line-clamp"),

        // https://tailwindcss.com/docs/line-height
        ["leading", ..] => Ok("line-height"),

        // https://tailwindcss.com/docs/list-style-image
        ["list", "image", ..] => Ok("list-style-image"),

        // https://tailwindcss.com/docs/list-style-position
        ["list", "inside"] | ["list", "outside"] => Ok("list-style-position"),

        // https://tailwindcss.com/docs/list-style-type
        ["list", ..] => Ok("list-style-type"),

        // https://tailwindcss.com/docs/text-decoration
        ["underline"] | ["overline"] | ["line", "through"] | ["no", "underline"] => Ok("text-decoration"),

        // https://tailwindcss.com/docs/text-decoration-style
        ["decoration", "solid" | "double" | "dotted" | "dashed" | "wavy"] => Ok("text-decoration-style"),

        // https://tailwindcss.com/docs/text-decoration-thickness
        ["decoration", "auto"] | ["decoration", "from-font"] => Ok("text-decoration-thickness"),
        ["decoration", rest] => {
            if rest.parse::<usize>().is_ok() {
                Ok("text-decoration-thickness")
            } else {
                Err("Invalid text-decoration-thickness")
            }
        }
        ["decoration"] if arbitrary.parse::<usize>().is_ok() => Ok("text-decoration-thickness"),

        // https://tailwindcss.com/docs/text-decoration-color
        ["decoration", ..] => Ok("text-decoration-color"),

        // https://tailwindcss.com/docs/text-underline-offset
        ["underline", "offset", ..] => Ok("text-underline-offset"),

        // https://tailwindcss.com/docs/text-transform
        ["uppercase" | "lowercase" | "capitalize" | "normal-case"] => Ok("text-transform"),

        // https://tailwindcss.com/docs/text-overflow
        ["truncate"] => Ok("text-overflow"),

        // https://tailwindcss.com/docs/text-indent
        ["indent", ..] => Ok("text-indent"),

        // https://tailwindcss.com/docs/vertical-align
        ["align", ..] => Ok("vertical-align"),

        // https://tailwindcss.com/docs/whitespace
        ["whitespace", "normal"]
        | ["whitespace", "nowrap"]
        | ["whitespace", "pre"]
        | ["whitespace", "pre", "line"]
        | ["whitespace", "pre", "wrap"]
        | ["whitespace", "break", "spaces"] => Ok("whitespace"),

        // https://tailwindcss.com/docs/word-break
        ["break", "normal" | "words" | "all" | "keep"] => {
            Ok("word-break")
        }

        // https://tailwindcss.com/docs/hyphens
        ["hyphens", "none" | "manual" | "auto"] => Ok("hyphens"),

        // https://tailwindcss.com/docs/content
        ["content", "none"] => Ok("content"),
        ["content"] if is_arbitrary_value(arbitrary) => Ok("content"),

        // https://tailwindcss.com/docs/background-attachment
        ["bg", "fixed" | "local" | "scroll"] => Ok("background-attachment"),

        // https://tailwindcss.com/docs/background-clip
        ["bg", "clip", "border" | "padding" | "content" | "text"] => Ok("background-clip"),

        // https://tailwindcss.com/docs/background-origin
        ["bg", "origin", "border" | "padding" | "content"] => {
            Ok("background-origin")
        }

        // https://tailwindcss.com/docs/background-repeat
        ["bg", "repeat"]
        | ["bg", "no","repeat"]
        | ["bg", "repeat", "x" | "y" | "round" | "space"] => Ok("background-repeat"),

        // https://tailwindcss.com/docs/background-position
        // TODO: Integrate arbitrary value? (e.g. bg-[center_top_1rem])
        ["bg", "bottom"]
        | ["bg", "center"]
        | ["bg", "left"]
        | ["bg", "left", "bottom"]
        | ["bg", "left", "top"]
        | ["bg", "right"]
        | ["bg", "right", "bottom"]
        | ["bg", "right", "top"]
        | ["bg", "top"] => Ok("background-position"),

        // https://tailwindcss.com/docs/background-size
        ["bg", "auto" | "cover" | "contain"] => Ok("background-size"),
        ["bg"] if is_arbitrary_size(arbitrary) => Ok("background-size"),

        // https://tailwindcss.com/docs/background-image
        // v4: bg-linear-*, bg-conic-*, bg-radial-* (new gradient utilities)
        ["bg", "none"]
        | ["bg", "gradient", "to", ..]
        | ["bg", "linear", ..]
        | ["bg", "conic", ..]
        | ["bg", "radial", ..] => Ok("background-image"),
        ["bg"] if is_arbitrary_bg_image(arbitrary) => Ok("background-image"),

        // https://tailwindcss.com/docs/background-blend-mode
        // TODO: plus-lighter not valid
        ["bg", "blend", mode @ ..] if valid_blend(mode) => Ok("background-blend-mode"),

        // https://tailwindcss.com/docs/background-color
        ["bg", ..] => Ok("background-color"),

        // https://tailwindcss.com/docs/gradient-color-stops
        // TODO: Review this?
        ["from", ..] => Ok("from"),
        ["via", ..] => Ok("via"),
        ["to", ..] => Ok("to"),

        // https://tailwindcss.com/docs/border-radius
        // TODO: Review
        ["rounded", "t", ..] => Ok("rounded-t"),
        ["rounded", "r", ..] => Ok("rounded-r"),
        ["rounded", "b", ..] => Ok("rounded-b"),
        ["rounded", "l", ..] => Ok("rounded-l"),
        ["rounded", "tl", ..] => Ok("rounded-tl"),
        ["rounded", "tr", ..] => Ok("rounded-tr"),
        ["rounded", "bl", ..] => Ok("rounded-bl"),
        ["rounded", "br", ..] => Ok("rounded-br"),

        ["rounded", "s", ..] => Ok("rounded-s"),
        ["rounded", "e", ..] => Ok("rounded-e"),

        ["rounded", "ss", ..] => Ok("rounded-ss"),
        ["rounded", "se", ..] => Ok("rounded-se"),
        ["rounded", "ee", ..] => Ok("rounded-ee"),
        ["rounded", "es", ..] => Ok("rounded-es"),

        ["rounded", ..] => Ok("rounded"),

        // https://tailwindcss.com/docs/border-width
        ["border", "x", rest] if is_valid_length(rest) => Ok("border-w-x"),
        ["border", "x"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-x"),
        ["border", "y", rest] if is_valid_length(rest) => Ok("border-w-y"),
        ["border", "y"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-y"),
        ["border", "t", rest] if is_valid_length(rest) => Ok("border-w-t"),
        ["border", "t"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-t"),
        ["border", "r", rest] if arbitrary.is_empty() ||is_valid_length(rest) => Ok("border-w-r"),
        ["border", "r"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-r"),
        ["border", "b", rest] if arbitrary.is_empty() ||is_valid_length(rest) => Ok("border-w-b"),
        ["border", "b"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-b"),
        ["border", "l", rest] if is_valid_length(rest) => Ok("border-w-l"),
        ["border", "l"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-l"),
        ["border", "s", rest] if is_valid_length(rest) => Ok("border-w-s"),
        ["border", "s"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w-s"),
        ["border", rest] if is_valid_length(rest) => Ok("border-w"),
        ["border"] if arbitrary.is_empty() || is_arbitrary_len(arbitrary) => Ok("border-w"),

        // https://tailwindcss.com/docs/border-style
        ["border", "solid" | "dashed" | "dotted" | "double" | "hidden" | "none"] => {
            Ok("border-style")
        }
        // https://tailwindcss.com/docs/border-collapse
        ["border", "collapse" | "separate"] => Ok("border-collapse"),

        // https://tailwindcss.com/docs/border-spacing
        ["border", "spacing", "x", ..] => Ok("border-spacing-x"),
        ["border", "spacing", "y", ..] => Ok("border-spacing-y"),
        ["border", "spacing", ..] => Ok("border-spacing"),

        // https://tailwindcss.com/docs/border-color
        ["border", "t", ..] => Ok("border-color-t"),
        ["border", "r", ..] => Ok("border-color-r"),
        ["border", "b", ..] => Ok("border-color-b"),
        ["border", "l", ..] => Ok("border-color-l"),
        ["border", "s", ..] => Ok("border-color-s"),
        ["border", "e", ..] => Ok("border-color-e"),
        ["border", "x", ..] => Ok("border-color-x"),
        ["border", "y", ..] => Ok("border-color-y"),
        ["border", ..] => Ok("border-color"),

        // https://tailwindcss.com/docs/divide-style
        ["divide", "solid" | "dashed" | "dotted" | "double" | "none"] => Ok("divide-style"),

        // https://tailwindcss.com/docs/divide-width
        ["divide", "x", "reverse"] => Ok("divide-x-reverse"),
        ["divide", "y", "reverse"] => Ok("divide-y-reverse"),
        ["divide", "x", ..] => Ok("divide-x"),
        ["divide", "y", ..] => Ok("divide-y"),

        // https://tailwindcss.com/docs/divide-color
        ["divide", ..] => Ok("divide-color"),

        // https://tailwindcss.com/docs/outline-style
        | ["outline"]
        // v4: outline-hidden uses outline-style: none (different from outline-none which uses outline: 2px solid transparent)
        | ["outline", "none" | "hidden" | "solid" | "dashed" | "dotted" | "double"]
        // necessary for "outline"
            if arbitrary.is_empty() =>
        {
            Ok("outline-style")
        }

        // https://tailwindcss.com/docs/outline-width
        ["outline"] if arbitrary.parse::<usize>().is_ok() => Ok("outline-width"),
        ["outline", rest] if rest.parse::<usize>().is_ok() => Ok("outline-width"),

        // https://tailwindcss.com/docs/outline-offset
        ["outline", "offset", ..] => Ok("outline-offset"),

        // https://tailwindcss.com/docs/outline-color
        ["outline", ..] => Ok("outline-color"),

        // https://tailwindcss.com/docs/outline-offset
        ["ring", "inset"] => Ok("ring-width"),
        ["ring", rest] if rest.parse::<usize>().is_ok() => Ok("ring-width"),
        ["ring"] if arbitrary.parse::<usize>().is_ok() => Ok("ring-width"),

        // https://tailwindcss.com/docs/ring-offset-width
        ["ring", "offset", rest] if rest.parse::<usize>().is_ok() => Ok("ring-offset-width"),
        ["ring", "offset"] if arbitrary.parse::<usize>().is_ok() => Ok("ring-offset-width"),

        // https://tailwindcss.com/docs/ring-offset-color
        ["ring", "offset", ..] => Ok("ring-offset-color"),
        ["ring", ..] => Ok("ring-color"),


        // https://tailwindcss.com/docs/box-shadow
        // TODO: handle conflict with color + arbitrary
        ["shadow"] | ["shadow", "inner" | "none"] if arbitrary.is_empty() => Ok("box-shadow"),
        ["shadow", size] if is_t_shirt_size(size) => Ok("box-shadow"),

        // https://tailwindcss.com/docs/box-shadow-color
        ["shadow", ..] => Ok("box-shadow-color"),

        // https://tailwindcss.com/docs/opacity
        ["opacity", ..] => Ok("opacity"),

        // https://tailwindcss.com/docs/mix-blend-mode
        ["mix", "blend", mode @ ..] if valid_blend(mode)=> Ok("mix-blend-mode"),

        // https://tailwindcss.com/docs/blur
        ["blur", ..] => Ok("blur"),

        // https: //tailwindcss.com/docs/brightness
        ["brightness", ..] => Ok("brightness"),

        // https://tailwindcss.com/docs/contrast
        ["contrast", ..] => Ok("contrast"),

        // https://tailwindcss.com/docs/drop-shadow
        // v4.1: drop-shadow-<color> support
        ["drop", "shadow", "none"] => Ok("drop-shadow"),
        ["drop", "shadow", size] if is_t_shirt_size(size) => Ok("drop-shadow"),
        ["drop", "shadow"] => Ok("drop-shadow"),
        ["drop", "shadow", ..] => Ok("drop-shadow-color"),

        // https://tailwindcss.com/docs/grayscale
        ["grayscale", ..] => Ok("grayscale"),

        // https://tailwindcss.com/docs/hue-rotate
        ["hue", "rotate", ..] => Ok("hue-rotate"),

        // https://tailwindcss.com/docs/invert
        ["invert", ..] => Ok("invert"),

        // https://tailwindcss.com/docs/saturate
        ["saturate", ..] => Ok("saturate"),

        // https://tailwindcss.com/docs/sepia
        ["sepia", ..] => Ok("sepia"),

        // https://tailwindcss.com/docs/backdrop-blur
        ["backdrop", "blur", ..] => Ok("backdrop-blur"),

        // https://tailwindcss.com/docs/backdrop-brightness
        ["backdrop", "brightness", ..] => Ok("backdrop-brightness"),

        // https://tailwindcss.com/docs/backdrop-contrast
        ["backdrop", "contrast", ..] => Ok("backdrop-contrast"),

        // https://tailwindcss.com/docs/backdrop-grayscale
        ["backdrop", "grayscale", ..] => Ok("backdrop-grayscale"),

        // https://tailwindcss.com/docs/backdrop-hue-rotate
        ["backdrop", "hue", "rotate", ..] => Ok("backdrop-hue-rotate"),

        // https://tailwindcss.com/docs/backdrop-invert
        ["backdrop", "invert", ..] => Ok("backdrop-invert"),

        // https://tailwindcss.com/docs/backdrop-opacity
        ["backdrop", "opacity", ..] => Ok("backdrop-opacity"),

        // https://tailwindcss.com/docs/backdrop-saturate
        ["backdrop", "saturate", ..] => Ok("backdrop-saturate"),

        // https://tailwindcss.com/docs/backdrop-sepia
        ["backdrop", "sepia", ..] => Ok("backdrop-sepia"),

        // https://tailwindcss.com/docs/table-layout
        ["table", "auto" | "fixed"] => Ok("table-layout"),

        // https://tailwindcss.com/docs/caption-side
        ["caption", "top" | "bottom"] => Ok("caption-side"),

        // v4: https://tailwindcss.com/docs/transition-behavior
        ["transition", "normal" | "discrete"] => Ok("transition-behavior"),

        // https://tailwindcss.com/docs/transition-property
        ["transition", ..] => Ok("transition-property"),

        // https://tailwindcss.com/docs/transition-duration
        // Accepts: duration-150, duration-[240ms], duration-[.5s]
        ["duration", rest] if rest.parse::<usize>().is_ok() => Ok("transition-duration"),
        ["duration"] if is_arbitrary_time(arbitrary) => Ok("transition-duration"),

        // https://tailwindcss.com/docs/transition-timing-function
        ["ease", ..] => Ok("transition-timing-function"),

        // https://tailwindcss.com/docs/transition-delay
        // Accepts: delay-150, delay-[240ms], delay-[.5s]
        ["delay", rest] if rest.parse::<usize>().is_ok() => Ok("transition-delay"),
        ["delay"] if is_arbitrary_time(arbitrary) => Ok("transition-delay"),

        // ============================================================================
        // tailwindcss-animate / tw-animate-css plugin (Tailwind v4)
        // https://github.com/romboHQ/tailwindcss-animate
        // These classes provide entry/exit animations commonly used with shadcn/ui
        // ============================================================================

        // Animation state classes (required to trigger animations)
        ["animate", "in"] => Ok("animate-in-out"),
        ["animate", "out"] => Ok("animate-in-out"),

        // https://tailwindcss.com/docs/animate (generic animate-* utilities)
        ["animate", ..] => Ok("animate"),

        // Fade animations (opacity): fade-in, fade-in-0 to fade-in-100, fade-out, fade-out-0 to fade-out-100
        ["fade", "in", ..] => Ok("animate-opacity"),
        ["fade", "out", ..] => Ok("animate-opacity"),

        // Zoom animations (scale): zoom-in, zoom-in-0 to zoom-in-200, zoom-out, zoom-out-0 to zoom-out-200
        ["zoom", "in", ..] => Ok("animate-scale"),
        ["zoom", "out", ..] => Ok("animate-scale"),

        // Spin animations (rotation): spin-in, spin-in-0 to spin-in-360, spin-out, spin-out-0 to spin-out-360
        ["spin", "in", ..] => Ok("animate-rotate"),
        ["spin", "out", ..] => Ok("animate-rotate"),

        // Slide animations (translate)
        // slide-in-from-top-*, slide-in-from-bottom-*, slide-in-from-left-*, slide-in-from-right-*
        ["slide", "in", "from", "top", ..] => Ok("animate-translate-y"),
        ["slide", "in", "from", "bottom", ..] => Ok("animate-translate-y"),
        ["slide", "in", "from", "left", ..] => Ok("animate-translate-x"),
        ["slide", "in", "from", "right", ..] => Ok("animate-translate-x"),
        // slide-out-to-top-*, slide-out-to-bottom-*, slide-out-to-left-*, slide-out-to-right-*
        ["slide", "out", "to", "top", ..] => Ok("animate-translate-y"),
        ["slide", "out", "to", "bottom", ..] => Ok("animate-translate-y"),
        ["slide", "out", "to", "left", ..] => Ok("animate-translate-x"),
        ["slide", "out", "to", "right", ..] => Ok("animate-translate-x"),

        // https://tailwindcss.com/docs/scale
        // v4: scale-none resets individual transform
        ["scale", "none"] => Ok("scale"),
        ["scale", "x", "none"] => Ok("scale-x"),
        ["scale", "y", "none"] => Ok("scale-y"),
        ["scale", "z", "none"] => Ok("scale-z"),
        ["scale", "x", rest] if rest.parse::<usize>().is_ok() => Ok("scale-x"),
        ["scale", "x"] if arbitrary.parse::<usize>().is_ok() => Ok("scale-x"),
        ["scale", "y", rest] if rest.parse::<usize>().is_ok() => Ok("scale-y"),
        ["scale", "y"] if arbitrary.parse::<usize>().is_ok() => Ok("scale-y"),
        // v4: scale-z for 3D transforms
        ["scale", "z", rest] if rest.parse::<usize>().is_ok() => Ok("scale-z"),
        ["scale", "z"] if arbitrary.parse::<usize>().is_ok() => Ok("scale-z"),
        ["scale", rest] if rest.parse::<usize>().is_ok() => Ok("scale"),
        // [1.75] is valid
        ["scale"] if arbitrary.parse::<f32>().is_ok() => Ok("scale"),

        // https://tailwindcss.com/docs/rotate
        // v4: rotate-none resets individual transform
        ["rotate", "none"] => Ok("rotate"),
        ["rotate", rest] if rest.parse::<usize>().is_ok() => Ok("rotate"),
        ["rotate"] if arbitrary.parse::<usize>().is_ok() => Ok("rotate"),
        // v4: 3D rotations
        ["rotate", "x", "none"] => Ok("rotate-x"),
        ["rotate", "y", "none"] => Ok("rotate-y"),
        ["rotate", "z", "none"] => Ok("rotate-z"),
        ["rotate", "x", ..] => Ok("rotate-x"),
        ["rotate", "y", ..] => Ok("rotate-y"),
        ["rotate", "z", ..] => Ok("rotate-z"),

        // https://tailwindcss.com/docs/translate
        // v4: translate-none resets individual transform
        ["translate", "none"] => Ok("translate"),
        ["translate", "x", "none"] => Ok("translate-x"),
        ["translate", "y", "none"] => Ok("translate-y"),
        ["translate", "z", "none"] => Ok("translate-z"),
        ["translate", "x", ..]  => Ok("translate-x"),
        ["translate", "y", ..]  => Ok("translate-y"),
        // v4: translate-z for 3D transforms
        ["translate", "z", ..] => Ok("translate-z"),

        // https://tailwindcss.com/docs/skew
        // v4: skew-none resets individual transform
        ["skew", "none"] => Ok("skew"),
        ["skew", "x", "none"] => Ok("skew-x"),
        ["skew", "y", "none"] => Ok("skew-y"),
        ["skew", "x", ..]  => Ok("skew-x"),
        ["skew", "y", ..]  => Ok("skew-y"),

        // https://tailwindcss.com/docs/transform-origin
        ["origin", ..] => Ok("transform-origin"),

        // v4: https://tailwindcss.com/docs/perspective
        ["perspective", "origin", ..] => Ok("perspective-origin"),
        ["perspective", "none"] => Ok("perspective"),
        ["perspective", ..] => Ok("perspective"),

        // v4: https://tailwindcss.com/docs/backface-visibility
        ["backface", "visible" | "hidden"] => Ok("backface-visibility"),

        // v4: transform-3d utility
        ["transform", "3d"] => Ok("transform-3d"),

        // https://tailwindcss.com/docs/transform (Tailwind v4)
        // The `transform` class enables GPU acceleration for transforms.
        ["transform"] | ["transform", "gpu"] | ["transform", "none"] => Ok("transform"),

        // https://tailwindcss.com/docs/accent-color
        ["accent", ..] => Ok("accent-color"),

        // v4: https://tailwindcss.com/docs/color-scheme
        ["scheme", "light"] | ["scheme", "dark"] | ["scheme", "light", "dark"] => Ok("color-scheme"),

        // https://tailwindcss.com/docs/appearance
        ["appearance", "none" | "auto"] => Ok("appearance"),

        // https://tailwindcss.com/docs/cursor
        ["cursor", ..] => Ok("cursor"),

        // https://tailwindcss.com/docs/caret-color
        ["caret", ..] => Ok("caret-color"),

        // https://tailwindcss.com/docs/pointer-events
        ["pointer", "events",  "none" | "auto"] => Ok("pointer-events"),

        // https://tailwindcss.com/docs/resize
        ["resize"] | ["resize", "none" | "y" |"x"]   => Ok("resize"),

        // https://tailwindcss.com/docs/scroll-behavior
        ["scroll", "auto" | "smooth"] => Ok("scroll-behavior"),

        // https://tailwindcss.com/docs/scroll-margin
        ["scroll", rest, ..] if rest.starts_with('m') => Ok("scroll-margin"),

        // https://tailwindcss.com/docs/scroll-padding
        ["scroll", rest, ..] if rest.starts_with('p') => Ok("scroll-padding"),

        // https://tailwindcss.com/docs/scroll-snap-align
        // snap-start, snap-end, snap-center (2 elements)
        ["snap", "start" | "end" | "center"] => Ok("scroll-snap-align"),
        // snap-align-none (3 elements)
        ["snap", "align", "none"] => Ok("scroll-snap-align"),

        // https://tailwindcss.com/docs/scroll-snap-stop#forcing-snap-position-stops
        ["snap", "normal"] | ["snap", "always"] => Ok("scroll-snap-stop"),

        // https://tailwindcss.com/docs/scroll-snap-type
        ["snap", "none" | "x" | "y" | "both" | "mandatory" | "proximity"] => {
            Ok("scroll-snap-type")
        }

        // https://tailwindcss.com/docs/touch-action
        ["touch", "auto" | "none" | "manipulation"] => Ok("touch"),
        ["touch", "pan", "x" | "left" | "right"] => Ok("touch-x"),
        ["touch", "pan", "y"| "up"| "down"] => Ok("touch-y"),
        ["touch", "pinch", "zoom"] => Ok("touch-pz"),

        // https://tailwindcss.com/docs/user-select
        ["select" , "none" | "text" | "all" | "auto"] => Ok("user-select"),

        // https://tailwindcss.com/docs/will-change
        ["will", "change", ..] => Ok("will-change"),

        // https://tailwindcss.com/docs/fill
        ["fill", ..] => Ok("fill"),

        // https://tailwindcss.com/docs/stroke-width
        ["stroke", rest] if rest.parse::<usize>().is_ok() => Ok("stroke-width"),
        ["stroke"] if is_valid_length(arbitrary) => Ok("stroke-width"),

        // https://tailwindcss.com/docs/stroke
        ["stroke", ..]=> {
            Ok("stroke")
        },

        // https://tailwindcss.com/docs/screen-readers
        ["sr", "only"] | ["not", "sr", "only"] => Ok("screen-readers"),

        // Typography plugin: https://github.com/tailwindlabs/tailwindcss-typography
        ["prose"] => Ok("prose"),
        ["not", "prose"] => Ok("not-prose"),
        ["prose", ..] => Ok("prose"),

        // https://tailwindcss.com/docs/forced-color-adjust
        ["forced", "color", "adjust", "auto" | "none"] => Ok("forced-color-adjust"),

        // v4: https://tailwindcss.com/docs/field-sizing
        ["field", "sizing", "content" | "fixed"] => Ok("field-sizing"),

        // v4: https://tailwindcss.com/docs/mask-image
        ["mask", "none"] => Ok("mask-image"),
        ["mask", "type", ..] => Ok("mask-type"),
        ["mask", "position", ..] => Ok("mask-position"),
        ["mask", "size", ..] => Ok("mask-size"),
        ["mask", "repeat", ..] => Ok("mask-repeat"),
        ["mask", "origin", ..] => Ok("mask-origin"),
        ["mask", "clip", ..] => Ok("mask-clip"),
        ["mask", "composite", ..] => Ok("mask-composite"),
        // v4.1: directional gradient masks
        ["mask", "t", ..] => Ok("mask-linear-t"),
        ["mask", "r", ..] => Ok("mask-linear-r"),
        ["mask", "b", ..] => Ok("mask-linear-b"),
        ["mask", "l", ..] => Ok("mask-linear-l"),
        // v4.1: radial/conic gradient masks
        ["mask", "radial", ..] => Ok("mask-radial"),
        ["mask", "conic", ..] => Ok("mask-conic"),
        ["mask", ..] => Ok("mask-image"),

        // https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-parent-state
        ["group"] => Ok("group"),
        ["group", ..] => Ok("group"), // named group: group/item (parser splits on - so group/input-group becomes ["group/input", "group"])
        [first, ..] if first.starts_with("group/") => Ok("group"), // handles group/name when no - follows

        // https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-sibling-state
        ["peer"] => Ok("peer"),
        ["peer", ..] => Ok("peer"), // named peer: peer/item
        [first, ..] if first.starts_with("peer/") => Ok("peer"), // handles peer/name when no - follows

        // BEM-style classes (e.g., toast__container, block__element--modifier)
        // These are custom classes, not Tailwind utilities - pass them through
        [first, ..] if first.contains("__") => Ok("custom-bem"),

        _ => Err("Invalid Tailwind class"),
    }
}

fn valid_blend(mode: &[&str]) -> bool {
    matches!(
        mode,
        ["normal"]
            | ["multiply"]
            | ["screen"]
            | ["overlay"]
            | ["darken"]
            | ["lighten"]
            | ["color", "dodge"]
            | ["color", "burn"]
            | ["hard", "light"]
            | ["soft", "light"]
            | ["difference"]
            | ["exclusion"]
            | ["hue"]
            | ["saturation"]
            | ["color"]
            | ["luminosity"]
            | ["lighter"]
    )
}

fn valid_trbl(mode: &[&str], arbitrary: &str, success: &'static str, error: &'static str) -> Result<&'static str> {
    if mode.len() == 1 && valid_top_right_bottom_left(mode[0]) {
        return Ok(success);
    }
    if is_valid_length(arbitrary) {
        return Ok(success);
    }

    Err(error)
}

fn valid_top_right_bottom_left(mode: &str) -> bool {
    mode == "auto"
        || mode == "full"
        || mode == "px"
        || validators::parse_single_digit_decimal(mode)
        || parse_fraction(mode).is_some()
}

fn valid_break_after(mode: &str) -> bool {
    matches!(mode, "auto" | "avoid" | "all" | "avoid-page" | "page" | "left" | "right" | "column")
}

// Need starts_with for this https://tailwindcss.com/docs/font-size#setting-the-line-height
fn valid_text_size(mode: &str) -> bool {
    mode == "base"
        || mode.starts_with("xs")
        || mode.ends_with("xs")
        || mode.starts_with("sm")
        || mode.ends_with("sm")
        || mode.starts_with("md")
        || mode.ends_with("md")
        || mode.starts_with("lg")
        || mode.ends_with("lg")
        || mode.starts_with("xl")
        || mode.ends_with("xl")
}

fn parse_fraction_or_usize(input: &str) -> bool {
    parse_fraction(input).map(|_| ()).or_else(|| input.parse::<usize>().ok().map(|_| ())).is_some()
}

fn parse_fraction(input: &str) -> Option<(usize, usize)> {
    let (a, b) = input.split_once('/')?;
    let a = a.parse::<usize>().ok()?;
    let b = b.parse::<usize>().ok()?;
    Some((a, b))
}

/// Parses a decimal fraction like "1/0.8" or "16/9" (supports f64)
fn parse_decimal_fraction(input: &str) -> bool {
    let Some((a, b)) = input.split_once('/') else {
        return false;
    };
    a.parse::<f64>().is_ok() && b.parse::<f64>().is_ok()
}

fn is_t_shirt_size(input: &str) -> bool {
    input.ends_with("xs")
        || input.ends_with("sm")
        || input.ends_with("md")
        || input.ends_with("lg")
        || input.ends_with("xl")
}

fn is_valid_length(input: &str) -> bool {
    validators::length::parse(input).is_ok() && !is_valid_color(input)
}

fn is_valid_color(input: &str) -> bool {
    validators::color::parse(input).is_ok()
}

fn is_arbitrary_value(input: &str) -> bool {
    validators::arbitrary::parse(input).is_ok()
}

fn is_arbitrary_len(input: &str) -> bool {
    is_valid_arbitrary_value(input, |label| label == "length", is_valid_length)
}

fn is_arbitrary_bg_image(input: &str) -> bool {
    is_valid_arbitrary_value(
        input,
        |label| label == "image" || label == "url",
        |string| validators::image::parse(string).is_ok(),
    )
}

fn is_arbitrary_size(input: &str) -> bool {
    is_valid_arbitrary_value(input, |label| label == "length" || label == "size" || label == "percentage", |_| false)
}

/// Validates CSS time values for duration/delay arbitrary values
/// Accepts: "240ms", "0.5s", ".5s", "1s", pure numbers, or CSS variables
fn is_arbitrary_time(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    // Pure number (e.g., "150")
    if input.parse::<usize>().is_ok() {
        return true;
    }
    // CSS time with ms unit (e.g., "240ms", "0ms")
    if let Some(num) = input.strip_suffix("ms") {
        return num.parse::<f64>().is_ok();
    }
    // CSS time with s unit (e.g., "0.5s", ".5s", "1s")
    if let Some(num) = input.strip_suffix('s') {
        return num.parse::<f64>().is_ok() || num.starts_with('.') && num[1..].parse::<f64>().is_ok();
    }
    // CSS variable or calc()
    input.starts_with("var(") || input.starts_with("calc(")
}

fn is_valid_arbitrary_value(input: &str, label: impl Fn(&str) -> bool, func: impl Fn(&str) -> bool) -> bool {
    match validators::arbitrary::parse(input).ok() {
        Some((_, (Some(captured_label), _))) => label(captured_label),
        Some((_, (None, rest))) => func(rest),
        None => false,
    }
}

/* ========================================================== */
/*                       🧪 TESTS 🧪                          */
/* ========================================================== */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len() {
        assert!(is_valid_length("10px"));
        assert!(is_valid_length("10%"));
        assert!(is_valid_length("100rem"));

        assert!(!is_valid_length("hsl(350_80%_0%)"), "no color");
        assert!(!is_valid_length("--my-0"), "double negative");
    }

    #[test]
    fn parse_stroke() {
        let result = get_collision_id(&["stroke"], "10px");
        assert_eq!(result, Ok("stroke-width"));

        let result = get_collision_id(&["stroke"], "hsl(350_80%_0%)");
        assert_eq!(result, Ok("stroke"));
    }

    #[test]
    fn parse_margin() {
        let result = get_collision_id(&["my", "2"], "");
        assert_eq!(result, Ok("margin-y"));

        let result = get_collision_id(&["m", "2"], "");
        assert_eq!(result, Ok("margin"));

        let result = get_collision_id(&["m"], "2px");
        assert_eq!(result, Ok("margin"));

        let result = get_collision_id(&["my"], "10rem");
        assert_eq!(result, Ok("margin-y"));
    }

    #[test]
    fn parse_inset() {
        let result = get_collision_id(&["inset", "auto"], "");
        assert_eq!(result, Ok("inset"));

        let result = get_collision_id(&["inset", "0"], "");
        assert_eq!(result, Ok("inset"));

        let result = get_collision_id(&["inset"], "10px");
        assert_eq!(result, Ok("inset"));
    }

    #[test]
    fn parse_inset_logical_properties() {
        // inset-inline-start-* → "start"
        let result = get_collision_id(&["inset", "inline", "start", "full"], "");
        assert_eq!(result, Ok("start"));

        let result = get_collision_id(&["inset", "inline", "start", "auto"], "");
        assert_eq!(result, Ok("start"));

        let result = get_collision_id(&["inset", "inline", "start"], "10px");
        assert_eq!(result, Ok("start"));

        // inset-inline-end-* → "end"
        let result = get_collision_id(&["inset", "inline", "end", "full"], "");
        assert_eq!(result, Ok("end"));

        let result = get_collision_id(&["inset", "inline", "end"], "10px");
        assert_eq!(result, Ok("end"));

        // inset-inline-* → "inset-x"
        let result = get_collision_id(&["inset", "inline", "0"], "");
        assert_eq!(result, Ok("inset-x"));

        let result = get_collision_id(&["inset", "inline"], "10px");
        assert_eq!(result, Ok("inset-x"));

        // inset-block-start-* → "top"
        let result = get_collision_id(&["inset", "block", "start", "full"], "");
        assert_eq!(result, Ok("top"));

        let result = get_collision_id(&["inset", "block", "start"], "-4px");
        assert_eq!(result, Ok("top"));

        // inset-block-end-* → "bottom"
        let result = get_collision_id(&["inset", "block", "end", "full"], "");
        assert_eq!(result, Ok("bottom"));

        let result = get_collision_id(&["inset", "block", "end"], "10px");
        assert_eq!(result, Ok("bottom"));

        // inset-block-* → "inset-y"
        let result = get_collision_id(&["inset", "block", "0"], "");
        assert_eq!(result, Ok("inset-y"));

        let result = get_collision_id(&["inset", "block"], "10px");
        assert_eq!(result, Ok("inset-y"));
    }

    #[test]
    fn parse_len() {
        assert!(is_valid_length("calc(theme(fontSize.4xl)/1.125)"));
        let result = get_collision_id(&["text"], "length:theme(someScale.someValue)");
        assert_eq!(result, Ok("font-size"));

        assert!(is_valid_length("calc(theme(fontSize.4xl)/1.125)"));
        let result = get_collision_id(&["text"], "calc(theme(fontSize.4xl)/1.125)");
        assert_eq!(result, Ok("font-size"));
    }

    #[test]
    fn parse_text_color() {
        assert!(!is_arbitrary_len("color:0"), "shouldn't be a length");
        let result = get_collision_id(&["text"], "color:0");
        assert_eq!(result, Ok("text-color"));
    }

    #[test]
    fn parse_margin_arb() {
        let result = get_collision_id(&["m"], "length:var(--c)");
        assert_eq!(result, Ok("margin"));
    }

    #[test]
    fn parse_border_color_arb() {
        assert!(!is_arbitrary_len("color:rgb(var(--color-gray-500-rgb)/50%)"));
        let result = get_collision_id(&["border"], "color:rgb(var(--color-gray-500-rgb)/50%)");
        assert_eq!(result, Ok("border-color"));

        let result = get_collision_id(&["border", "some", "color"], "");
        assert_eq!(result, Ok("border-color"));

        let result = get_collision_id(&["border", "b"], "");
        assert_eq!(result, Ok("border-w-b"));
    }

    #[test]
    fn parse_group_and_peer() {
        // group
        let result = get_collision_id(&["group"], "");
        assert_eq!(result, Ok("group"));

        // named group: group/item
        let result = get_collision_id(&["group", "item"], "");
        assert_eq!(result, Ok("group"));

        // named group with dash: group/input-group (parser splits as ["group/input", "group"])
        let result = get_collision_id(&["group/input", "group"], "");
        assert_eq!(result, Ok("group"));

        // named group without dash: group/sidebar
        let result = get_collision_id(&["group/sidebar"], "");
        assert_eq!(result, Ok("group"));

        // peer
        let result = get_collision_id(&["peer"], "");
        assert_eq!(result, Ok("peer"));

        // named peer: peer/item
        let result = get_collision_id(&["peer", "sidebar"], "");
        assert_eq!(result, Ok("peer"));

        // named peer with dash: peer/form-input
        let result = get_collision_id(&["peer/form", "input"], "");
        assert_eq!(result, Ok("peer"));

        // named peer without dash
        let result = get_collision_id(&["peer/form"], "");
        assert_eq!(result, Ok("peer"));
    }

    #[test]
    fn parse_v4_3d_transforms() {
        // scale-z
        let result = get_collision_id(&["scale", "z", "50"], "");
        assert_eq!(result, Ok("scale-z"));

        // rotate-x, rotate-y, rotate-z
        let result = get_collision_id(&["rotate", "x", "45"], "");
        assert_eq!(result, Ok("rotate-x"));
        let result = get_collision_id(&["rotate", "y", "90"], "");
        assert_eq!(result, Ok("rotate-y"));
        let result = get_collision_id(&["rotate", "z", "180"], "");
        assert_eq!(result, Ok("rotate-z"));

        // translate-z
        let result = get_collision_id(&["translate", "z", "10"], "");
        assert_eq!(result, Ok("translate-z"));

        // perspective
        let result = get_collision_id(&["perspective", "none"], "");
        assert_eq!(result, Ok("perspective"));
        let result = get_collision_id(&["perspective", "500"], "");
        assert_eq!(result, Ok("perspective"));

        // perspective-origin
        let result = get_collision_id(&["perspective", "origin", "center"], "");
        assert_eq!(result, Ok("perspective-origin"));

        // backface-visibility
        let result = get_collision_id(&["backface", "visible"], "");
        assert_eq!(result, Ok("backface-visibility"));
        let result = get_collision_id(&["backface", "hidden"], "");
        assert_eq!(result, Ok("backface-visibility"));
    }

    #[test]
    fn parse_v4_outline_hidden() {
        let result = get_collision_id(&["outline", "hidden"], "");
        assert_eq!(result, Ok("outline-style"));
    }

    #[test]
    fn parse_v4_field_sizing() {
        let result = get_collision_id(&["field", "sizing", "content"], "");
        assert_eq!(result, Ok("field-sizing"));
        let result = get_collision_id(&["field", "sizing", "fixed"], "");
        assert_eq!(result, Ok("field-sizing"));
    }

    #[test]
    fn parse_v4_text_shadow() {
        let result = get_collision_id(&["text", "shadow"], "");
        assert_eq!(result, Ok("text-shadow"));
        let result = get_collision_id(&["text", "shadow", "none"], "");
        assert_eq!(result, Ok("text-shadow"));
        let result = get_collision_id(&["text", "shadow", "sm"], "");
        assert_eq!(result, Ok("text-shadow"));
        let result = get_collision_id(&["text", "shadow", "lg"], "");
        assert_eq!(result, Ok("text-shadow"));
        // text-shadow-color
        let result = get_collision_id(&["text", "shadow", "red", "500"], "");
        assert_eq!(result, Ok("text-shadow-color"));
    }

    #[test]
    fn parse_v4_inset_shadow() {
        let result = get_collision_id(&["inset", "shadow"], "");
        assert_eq!(result, Ok("inset-shadow"));
        let result = get_collision_id(&["inset", "shadow", "none"], "");
        assert_eq!(result, Ok("inset-shadow"));
        let result = get_collision_id(&["inset", "shadow", "sm"], "");
        assert_eq!(result, Ok("inset-shadow"));
        // inset-shadow-color
        let result = get_collision_id(&["inset", "shadow", "red", "500"], "");
        assert_eq!(result, Ok("inset-shadow-color"));
    }

    #[test]
    fn parse_v4_inset_ring() {
        let result = get_collision_id(&["inset", "ring"], "");
        assert_eq!(result, Ok("inset-ring"));
        let result = get_collision_id(&["inset", "ring", "2"], "");
        assert_eq!(result, Ok("inset-ring"));
        // inset-ring-color
        let result = get_collision_id(&["inset", "ring", "red", "500"], "");
        assert_eq!(result, Ok("inset-ring-color"));
    }

    #[test]
    fn parse_v4_mask() {
        let result = get_collision_id(&["mask", "none"], "");
        assert_eq!(result, Ok("mask-image"));
        let result = get_collision_id(&["mask", "gradient", "to", "r"], "");
        assert_eq!(result, Ok("mask-image"));
        let result = get_collision_id(&["mask", "type", "alpha"], "");
        assert_eq!(result, Ok("mask-type"));
        let result = get_collision_id(&["mask", "position", "center"], "");
        assert_eq!(result, Ok("mask-position"));
        let result = get_collision_id(&["mask", "size", "cover"], "");
        assert_eq!(result, Ok("mask-size"));
        let result = get_collision_id(&["mask", "repeat"], "");
        assert_eq!(result, Ok("mask-repeat"));
        let result = get_collision_id(&["mask", "origin", "content"], "");
        assert_eq!(result, Ok("mask-origin"));
        let result = get_collision_id(&["mask", "clip", "text"], "");
        assert_eq!(result, Ok("mask-clip"));
        let result = get_collision_id(&["mask", "composite", "add"], "");
        assert_eq!(result, Ok("mask-composite"));
    }

    #[test]
    fn parse_v4_inset_does_not_conflict() {
        // Make sure inset-shadow and inset-ring don't conflict with inset positioning
        let result = get_collision_id(&["inset", "0"], "");
        assert_eq!(result, Ok("inset"));
        let result = get_collision_id(&["inset", "x", "0"], "");
        assert_eq!(result, Ok("inset-x"));
        let result = get_collision_id(&["inset", "y", "auto"], "");
        assert_eq!(result, Ok("inset-y"));
    }

    #[test]
    fn parse_v4_gradients() {
        // bg-linear-*
        let result = get_collision_id(&["bg", "linear", "to", "r"], "");
        assert_eq!(result, Ok("background-image"));
        let result = get_collision_id(&["bg", "linear", "45"], "");
        assert_eq!(result, Ok("background-image"));

        // bg-conic-*
        let result = get_collision_id(&["bg", "conic"], "");
        assert_eq!(result, Ok("background-image"));

        // bg-radial-*
        let result = get_collision_id(&["bg", "radial"], "");
        assert_eq!(result, Ok("background-image"));
    }

    #[test]
    fn parse_v4_font_stretch() {
        let result = get_collision_id(&["font", "stretch", "50%"], "");
        assert_eq!(result, Ok("font-stretch"));
        let result = get_collision_id(&["font", "stretch", "condensed"], "");
        assert_eq!(result, Ok("font-stretch"));
    }

    #[test]
    fn parse_v4_color_scheme() {
        let result = get_collision_id(&["scheme", "light"], "");
        assert_eq!(result, Ok("color-scheme"));
        let result = get_collision_id(&["scheme", "dark"], "");
        assert_eq!(result, Ok("color-scheme"));
        let result = get_collision_id(&["scheme", "light", "dark"], "");
        assert_eq!(result, Ok("color-scheme"));
    }

    #[test]
    fn parse_v4_transform_3d() {
        let result = get_collision_id(&["transform", "3d"], "");
        assert_eq!(result, Ok("transform-3d"));
    }

    #[test]
    fn parse_v4_container_queries() {
        // @container
        let result = get_collision_id(&["@container"], "");
        assert_eq!(result, Ok("@container"));

        // @container/name (parser splits @container/card-header as ["@container/card", "header"])
        let result = get_collision_id(&["@container/card", "header"], "");
        assert_eq!(result, Ok("@container"));

        // @container/simple-name
        let result = get_collision_id(&["@container/sidebar"], "");
        assert_eq!(result, Ok("@container"));
    }

    #[test]
    fn parse_bem_classes() {
        // BEM-style classes should pass through without error
        let result = get_collision_id(&["toast__container"], "");
        assert_eq!(result, Ok("custom-bem"));

        let result = get_collision_id(&["block__element"], "");
        assert_eq!(result, Ok("custom-bem"));

        let result = get_collision_id(&["card__header", "title"], "");
        assert_eq!(result, Ok("custom-bem"));
    }

    #[test]
    fn parse_v4_transform_resets() {
        // scale-none
        let result = get_collision_id(&["scale", "none"], "");
        assert_eq!(result, Ok("scale"));
        let result = get_collision_id(&["scale", "x", "none"], "");
        assert_eq!(result, Ok("scale-x"));

        // rotate-none
        let result = get_collision_id(&["rotate", "none"], "");
        assert_eq!(result, Ok("rotate"));
        let result = get_collision_id(&["rotate", "x", "none"], "");
        assert_eq!(result, Ok("rotate-x"));

        // translate-none
        let result = get_collision_id(&["translate", "none"], "");
        assert_eq!(result, Ok("translate"));
        let result = get_collision_id(&["translate", "x", "none"], "");
        assert_eq!(result, Ok("translate-x"));

        // skew-none
        let result = get_collision_id(&["skew", "none"], "");
        assert_eq!(result, Ok("skew"));
        let result = get_collision_id(&["skew", "x", "none"], "");
        assert_eq!(result, Ok("skew-x"));
    }

    #[test]
    fn parse_v4_1_overflow_wrap() {
        let result = get_collision_id(&["wrap", "break", "word"], "");
        assert_eq!(result, Ok("overflow-wrap"));
        let result = get_collision_id(&["wrap", "anywhere"], "");
        assert_eq!(result, Ok("overflow-wrap"));
        let result = get_collision_id(&["wrap", "normal"], "");
        assert_eq!(result, Ok("overflow-wrap"));
    }

    #[test]
    fn parse_v4_1_safe_alignment() {
        // justify-center-safe
        let result = get_collision_id(&["justify", "center", "safe"], "");
        assert_eq!(result, Ok("justify-content"));

        // items-center-safe
        let result = get_collision_id(&["items", "center", "safe"], "");
        assert_eq!(result, Ok("align-items"));

        // items-baseline-last
        let result = get_collision_id(&["items", "baseline", "last"], "");
        assert_eq!(result, Ok("align-items"));

        // self-baseline-last
        let result = get_collision_id(&["self", "baseline", "last"], "");
        assert_eq!(result, Ok("align-self"));
    }

    #[test]
    fn parse_v4_1_drop_shadow_color() {
        let result = get_collision_id(&["drop", "shadow"], "");
        assert_eq!(result, Ok("drop-shadow"));
        let result = get_collision_id(&["drop", "shadow", "lg"], "");
        assert_eq!(result, Ok("drop-shadow"));
        let result = get_collision_id(&["drop", "shadow", "cyan", "500"], "");
        assert_eq!(result, Ok("drop-shadow-color"));
    }

    #[test]
    fn parse_v4_1_mask_gradients() {
        // directional masks
        let result = get_collision_id(&["mask", "t", "from", "50%"], "");
        assert_eq!(result, Ok("mask-linear-t"));
        let result = get_collision_id(&["mask", "r", "to", "100%"], "");
        assert_eq!(result, Ok("mask-linear-r"));
        let result = get_collision_id(&["mask", "b", "from", "0%"], "");
        assert_eq!(result, Ok("mask-linear-b"));
        let result = get_collision_id(&["mask", "l", "to", "50%"], "");
        assert_eq!(result, Ok("mask-linear-l"));

        // radial/conic masks
        let result = get_collision_id(&["mask", "radial"], "");
        assert_eq!(result, Ok("mask-radial"));
        let result = get_collision_id(&["mask", "radial", "at", "center"], "");
        assert_eq!(result, Ok("mask-radial"));
        let result = get_collision_id(&["mask", "conic"], "");
        assert_eq!(result, Ok("mask-conic"));
    }

    #[test]
    fn parse_v4_transition_behavior() {
        let result = get_collision_id(&["transition", "discrete"], "");
        assert_eq!(result, Ok("transition-behavior"));
        let result = get_collision_id(&["transition", "normal"], "");
        assert_eq!(result, Ok("transition-behavior"));
    }

    #[test]
    fn parse_typography_prose() {
        // prose base
        let result = get_collision_id(&["prose"], "");
        assert_eq!(result, Ok("prose"));

        // not-prose (escape from prose formatting)
        let result = get_collision_id(&["not", "prose"], "");
        assert_eq!(result, Ok("not-prose"));

        // prose sizes
        let result = get_collision_id(&["prose", "sm"], "");
        assert_eq!(result, Ok("prose"));
        let result = get_collision_id(&["prose", "lg"], "");
        assert_eq!(result, Ok("prose"));
        let result = get_collision_id(&["prose", "xl"], "");
        assert_eq!(result, Ok("prose"));
        let result = get_collision_id(&["prose", "2xl"], "");
        assert_eq!(result, Ok("prose"));

        // prose colors
        let result = get_collision_id(&["prose", "gray"], "");
        assert_eq!(result, Ok("prose"));
        let result = get_collision_id(&["prose", "slate"], "");
        assert_eq!(result, Ok("prose"));
        let result = get_collision_id(&["prose", "zinc"], "");
        assert_eq!(result, Ok("prose"));

        // prose-invert
        let result = get_collision_id(&["prose", "invert"], "");
        assert_eq!(result, Ok("prose"));
    }

    /// https://tailwindcss.com/docs/flex-shrink
    /// v4 syntax: shrink-0, shrink-[n]
    #[test]
    fn parse_shrink() {
        let result = get_collision_id(&["shrink", "0"], "");
        assert_eq!(result, Ok("flex-shrink"));
        let result = get_collision_id(&["shrink"], "");
        assert_eq!(result, Ok("flex-shrink"));
    }

    /// https://tailwindcss.com/docs/flex-grow
    /// v4 syntax: grow-0, grow-[n]
    #[test]
    fn parse_grow() {
        let result = get_collision_id(&["grow", "0"], "");
        assert_eq!(result, Ok("flex-grow"));
        let result = get_collision_id(&["grow"], "");
        assert_eq!(result, Ok("flex-grow"));
    }

    /// https://tailwindcss.com/docs/scroll-snap-align
    /// Classes: snap-start, snap-end, snap-center, snap-align-none
    #[test]
    fn parse_scroll_snap_align() {
        // snap-start
        let result = get_collision_id(&["snap", "start"], "");
        assert_eq!(result, Ok("scroll-snap-align"));

        // snap-end
        let result = get_collision_id(&["snap", "end"], "");
        assert_eq!(result, Ok("scroll-snap-align"));

        // snap-center
        let result = get_collision_id(&["snap", "center"], "");
        assert_eq!(result, Ok("scroll-snap-align"));

        // snap-align-none
        let result = get_collision_id(&["snap", "align", "none"], "");
        assert_eq!(result, Ok("scroll-snap-align"));
    }

    /// https://tailwindcss.com/docs/scroll-snap-type
    /// Classes: snap-none, snap-x, snap-y, snap-both, snap-mandatory, snap-proximity
    #[test]
    fn parse_scroll_snap_type() {
        let result = get_collision_id(&["snap", "none"], "");
        assert_eq!(result, Ok("scroll-snap-type"));

        let result = get_collision_id(&["snap", "x"], "");
        assert_eq!(result, Ok("scroll-snap-type"));

        let result = get_collision_id(&["snap", "y"], "");
        assert_eq!(result, Ok("scroll-snap-type"));

        let result = get_collision_id(&["snap", "both"], "");
        assert_eq!(result, Ok("scroll-snap-type"));

        let result = get_collision_id(&["snap", "mandatory"], "");
        assert_eq!(result, Ok("scroll-snap-type"));

        let result = get_collision_id(&["snap", "proximity"], "");
        assert_eq!(result, Ok("scroll-snap-type"));
    }

    /// https://tailwindcss.com/docs/transition-duration
    /// https://tailwindcss.com/docs/transition-delay
    /// Supports arbitrary time values: duration-[240ms], delay-[.5s]
    #[test]
    fn parse_duration_delay_arbitrary() {
        // duration with standard values
        let result = get_collision_id(&["duration", "150"], "");
        assert_eq!(result, Ok("transition-duration"));

        // duration with arbitrary ms
        let result = get_collision_id(&["duration"], "240ms");
        assert_eq!(result, Ok("transition-duration"));

        // duration with arbitrary ms (another value)
        let result = get_collision_id(&["duration"], "160ms");
        assert_eq!(result, Ok("transition-duration"));

        // duration with arbitrary seconds
        let result = get_collision_id(&["duration"], "0.5s");
        assert_eq!(result, Ok("transition-duration"));

        // duration with arbitrary pure number
        let result = get_collision_id(&["duration"], "300");
        assert_eq!(result, Ok("transition-duration"));

        // delay with standard values
        let result = get_collision_id(&["delay", "150"], "");
        assert_eq!(result, Ok("transition-delay"));

        // delay with arbitrary ms
        let result = get_collision_id(&["delay"], "240ms");
        assert_eq!(result, Ok("transition-delay"));

        // delay with arbitrary seconds
        let result = get_collision_id(&["delay"], ".5s");
        assert_eq!(result, Ok("transition-delay"));

        // delay with CSS variable
        let result = get_collision_id(&["delay"], "var(--delay)");
        assert_eq!(result, Ok("transition-delay"));
    }

    /// https://tailwindcss.com/docs/transform
    /// Legacy transform utility for GPU acceleration
    #[test]
    fn parse_transform_legacy() {
        let result = get_collision_id(&["transform"], "");
        assert_eq!(result, Ok("transform"));

        let result = get_collision_id(&["transform", "gpu"], "");
        assert_eq!(result, Ok("transform"));

        let result = get_collision_id(&["transform", "none"], "");
        assert_eq!(result, Ok("transform"));
    }

    /// https://github.com/romboHQ/tailwindcss-animate
    /// Animation state classes: animate-in, animate-out
    #[test]
    fn parse_animate_in_out() {
        let result = get_collision_id(&["animate", "in"], "");
        assert_eq!(result, Ok("animate-in-out"));

        let result = get_collision_id(&["animate", "out"], "");
        assert_eq!(result, Ok("animate-in-out"));
    }

    /// https://github.com/romboHQ/tailwindcss-animate
    /// Fade animations: fade-in, fade-in-0, fade-in-50, fade-out, fade-out-100
    #[test]
    fn parse_fade_animations() {
        // fade-in (base)
        let result = get_collision_id(&["fade", "in"], "");
        assert_eq!(result, Ok("animate-opacity"));

        // fade-in-0
        let result = get_collision_id(&["fade", "in", "0"], "");
        assert_eq!(result, Ok("animate-opacity"));

        // fade-in-50
        let result = get_collision_id(&["fade", "in", "50"], "");
        assert_eq!(result, Ok("animate-opacity"));

        // fade-out (base)
        let result = get_collision_id(&["fade", "out"], "");
        assert_eq!(result, Ok("animate-opacity"));

        // fade-out-0
        let result = get_collision_id(&["fade", "out", "0"], "");
        assert_eq!(result, Ok("animate-opacity"));

        // fade-out-100
        let result = get_collision_id(&["fade", "out", "100"], "");
        assert_eq!(result, Ok("animate-opacity"));
    }

    /// https://github.com/romboHQ/tailwindcss-animate
    /// Zoom animations: zoom-in, zoom-in-50, zoom-out, zoom-out-95
    #[test]
    fn parse_zoom_animations() {
        let result = get_collision_id(&["zoom", "in"], "");
        assert_eq!(result, Ok("animate-scale"));

        let result = get_collision_id(&["zoom", "in", "50"], "");
        assert_eq!(result, Ok("animate-scale"));

        let result = get_collision_id(&["zoom", "out"], "");
        assert_eq!(result, Ok("animate-scale"));

        let result = get_collision_id(&["zoom", "out", "95"], "");
        assert_eq!(result, Ok("animate-scale"));
    }

    /// https://github.com/romboHQ/tailwindcss-animate
    /// Spin animations: spin-in, spin-in-180, spin-out, spin-out-90
    #[test]
    fn parse_spin_animations() {
        let result = get_collision_id(&["spin", "in"], "");
        assert_eq!(result, Ok("animate-rotate"));

        let result = get_collision_id(&["spin", "in", "180"], "");
        assert_eq!(result, Ok("animate-rotate"));

        let result = get_collision_id(&["spin", "out"], "");
        assert_eq!(result, Ok("animate-rotate"));

        let result = get_collision_id(&["spin", "out", "90"], "");
        assert_eq!(result, Ok("animate-rotate"));
    }

    /// https://github.com/romboHQ/tailwindcss-animate
    /// Slide animations: slide-in-from-top-4, slide-out-to-bottom-2, etc.
    #[test]
    fn parse_slide_animations() {
        // slide-in-from-top-4
        let result = get_collision_id(&["slide", "in", "from", "top", "4"], "");
        assert_eq!(result, Ok("animate-translate-y"));

        // slide-in-from-bottom-2
        let result = get_collision_id(&["slide", "in", "from", "bottom", "2"], "");
        assert_eq!(result, Ok("animate-translate-y"));

        // slide-in-from-left-full
        let result = get_collision_id(&["slide", "in", "from", "left", "full"], "");
        assert_eq!(result, Ok("animate-translate-x"));

        // slide-in-from-right-1/2
        let result = get_collision_id(&["slide", "in", "from", "right", "1/2"], "");
        assert_eq!(result, Ok("animate-translate-x"));

        // slide-out-to-top-4
        let result = get_collision_id(&["slide", "out", "to", "top", "4"], "");
        assert_eq!(result, Ok("animate-translate-y"));

        // slide-out-to-bottom-2
        let result = get_collision_id(&["slide", "out", "to", "bottom", "2"], "");
        assert_eq!(result, Ok("animate-translate-y"));

        // slide-out-to-left-full
        let result = get_collision_id(&["slide", "out", "to", "left", "full"], "");
        assert_eq!(result, Ok("animate-translate-x"));

        // slide-out-to-right-1/2
        let result = get_collision_id(&["slide", "out", "to", "right", "1/2"], "");
        assert_eq!(result, Ok("animate-translate-x"));
    }

    /// https://tailwindcss.com/docs/ring-offset-width
    /// Classes: ring-offset-0, ring-offset-1, ring-offset-2, ring-offset-4, ring-offset-8
    #[test]
    fn parse_ring_offset_width() {
        let result = get_collision_id(&["ring", "offset", "0"], "");
        assert_eq!(result, Ok("ring-offset-width"));
        let result = get_collision_id(&["ring", "offset", "2"], "");
        assert_eq!(result, Ok("ring-offset-width"));
        let result = get_collision_id(&["ring", "offset", "4"], "");
        assert_eq!(result, Ok("ring-offset-width"));
    }

    /// https://tailwindcss.com/docs/ring-offset-color
    /// Classes: ring-offset-white, ring-offset-black, ring-offset-red-500, etc.
    #[test]
    fn parse_ring_offset_color() {
        let result = get_collision_id(&["ring", "offset", "white"], "");
        assert_eq!(result, Ok("ring-offset-color"));
        let result = get_collision_id(&["ring", "offset", "red", "500"], "");
        assert_eq!(result, Ok("ring-offset-color"));
        let result = get_collision_id(&["ring", "offset", "transparent"], "");
        assert_eq!(result, Ok("ring-offset-color"));
    }
}
