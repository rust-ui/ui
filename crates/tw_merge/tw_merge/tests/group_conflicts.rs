use tw_merge::merge::merge_classes;
use tw_merge::tw_merge;

#[test]
fn test_tw_merge_border_width_color() {
    let all_sides = tw_merge!("border-2", "border-blue-500");
    assert_eq!(all_sides, "border-2 border-blue-500");

    let arbitrary = tw_merge!("border-[100px]", "border-blue-500");
    assert_eq!(arbitrary, "border-[100px] border-blue-500");
}

#[test]
fn test_tw_merge_mixed_blend() {
    let classes = tw_merge!("mix-blend-normal", "mix-blend-multiply");
    assert_eq!(classes, "mix-blend-multiply");
}

#[test]
fn test_tw_merge_height() {
    let classes = tw_merge!("h-10", "h-min");
    assert_eq!(classes, "h-min");
}

#[test]
fn test_tw_merge_stroke() {
    let classes = tw_merge!("stroke-black", "stroke-1");
    assert_eq!(classes, "stroke-black stroke-1");

    let classes = tw_merge!("stroke-2", "stroke-[3px]");
    assert_eq!(classes, "stroke-[3px]");

    let classes = tw_merge!("stroke-black", "stroke-red-500", "stroke-blue-100");
    assert_eq!(classes, "stroke-blue-100");
}

#[test]
fn test_tw_merge_outline() {
    let classes = tw_merge!("outline-black", "outline-1");
    assert_eq!(classes, "outline-black outline-1");
}

#[test]
fn test_tw_merge_grayscale() {
    let classes = tw_merge!("grayscale-0", "grayscale-[50%]");
    assert_eq!(classes, "grayscale-[50%]");
}

#[test]
fn test_padding_narrowing() {
    let classes = tw_merge!("p-10", "px-5");
    assert_eq!(classes, "p-10 px-5");
    let classes = tw_merge!("px-5", "py-5", "p-10",);
    assert_eq!(classes, "p-10");
}

#[test]
fn test_gap_narrowing() {
    let classes = tw_merge!("gap-10", "gap-x-5");
    assert_eq!(classes, "gap-10 gap-x-5");

    let classes = tw_merge!("gap-x-5", "gap-y-5", "gap-10");
    assert_eq!(classes, "gap-10");
}

#[test]
fn merges_classes_from_same_group_correctly() {
    let class = "overflow-x-auto overflow-x-hidden";
    let result = merge_classes(class);
    assert_eq!(result, "overflow-x-hidden");

    let class = "basis-full basis-auto";
    let result = merge_classes(class);
    assert_eq!(result, "basis-auto");

    let class = "w-full w-fit";
    let result = merge_classes(class);
    assert_eq!(result, "w-fit");

    let class = "overflow-x-auto overflow-x-hidden overflow-x-scroll";
    let result = merge_classes(class);
    assert_eq!(result, "overflow-x-scroll");

    let class = "overflow-x-auto overflow-x-scroll hover:overflow-x-hidden";
    let result = merge_classes(class);
    assert_eq!(result, "overflow-x-scroll hover:overflow-x-hidden");

    let class = "overflow-x-auto overflow-x-scroll hover:overflow-x-hidden hover:overflow-x-auto";
    let result = merge_classes(class);
    assert_eq!(result, "overflow-x-scroll hover:overflow-x-auto");

    let class = "col-span-1 col-span-full";
    let result = merge_classes(class);
    assert_eq!(result, "col-span-full");
}

#[test]
fn merges_classes_from_font_variant_numeric_section_correctly() {
    let class = "lining-nums tabular-nums diagonal-fractions";
    let result = merge_classes(class);
    assert_eq!(result, "lining-nums tabular-nums diagonal-fractions");

    let class = "normal-nums tabular-nums diagonal-fractions";
    let result = merge_classes(class);
    assert_eq!(result, "tabular-nums diagonal-fractions");

    let class = "tabular-nums diagonal-fractions normal-nums";
    let result = merge_classes(class);
    assert_eq!(result, "normal-nums");

    let class = "tabular-nums proportional-nums";
    let result = merge_classes(class);
    assert_eq!(result, "proportional-nums");
}

#[test]
fn handles_color_conflicts_properly() {
    let class = "bg-grey-5 bg-hotpink";
    let result = merge_classes(class);
    assert_eq!(result, "bg-hotpink");

    let class = "hover:bg-grey-5 hover:bg-hotpink";
    let result = merge_classes(class);
    assert_eq!(result, "hover:bg-hotpink");

    let class = "stroke-[hsl(350_80%_0%)] stroke-[10px]";
    let result = merge_classes(class);
    assert_eq!(result, "stroke-[hsl(350_80%_0%)] stroke-[10px]");
}

#[test]
fn handles_conflicts_across_class_groups_correctly() {
    assert_eq!(merge_classes("inset-1 inset-x-1"), "inset-1 inset-x-1");
    assert_eq!(merge_classes("inset-x-1 inset-1"), "inset-1");
    assert_eq!(merge_classes("inset-x-1 left-1 inset-1"), "inset-1");
    assert_eq!(merge_classes("inset-x-1 inset-1 left-1"), "inset-1 left-1");
    assert_eq!(merge_classes("inset-x-1 right-1 inset-1"), "inset-1");
    assert_eq!(merge_classes("inset-x-1 right-1 inset-x-1"), "inset-x-1");
    assert_eq!(merge_classes("inset-x-1 right-1 inset-y-1"), "inset-x-1 right-1 inset-y-1");
    assert_eq!(merge_classes("right-1 inset-x-1 inset-y-1"), "inset-x-1 inset-y-1");
    assert_eq!(merge_classes("inset-x-1 hover:left-1 inset-1"), "hover:left-1 inset-1");
}

// https://tailwindcss.com/docs/top-right-bottom-left#using-logical-properties
#[test]
fn handles_inset_logical_property_conflicts_correctly() {
    // inset-inline-start/end conflict with start/end (same collision ID)
    assert_eq!(merge_classes("start-full inset-inline-start-auto"), "inset-inline-start-auto");
    assert_eq!(merge_classes("inset-inline-start-full start-auto"), "start-auto");
    assert_eq!(merge_classes("end-full inset-inline-end-auto"), "inset-inline-end-auto");
    assert_eq!(merge_classes("inset-inline-end-full end-auto"), "end-auto");

    // inset-block-start/end conflict with top/bottom (same collision ID)
    assert_eq!(merge_classes("top-full inset-block-start-auto"), "inset-block-start-auto");
    assert_eq!(merge_classes("inset-block-start-full top-auto"), "top-auto");
    assert_eq!(merge_classes("bottom-full inset-block-end-auto"), "inset-block-end-auto");
    assert_eq!(merge_classes("inset-block-end-full bottom-auto"), "bottom-auto");

    // inset-inline/block conflict with inset-x/y (same collision ID)
    assert_eq!(merge_classes("inset-x-full inset-inline-auto"), "inset-inline-auto");
    assert_eq!(merge_classes("inset-inline-full inset-x-auto"), "inset-x-auto");
    assert_eq!(merge_classes("inset-y-full inset-block-auto"), "inset-block-auto");
    assert_eq!(merge_classes("inset-block-full inset-y-auto"), "inset-y-auto");

    // inset overrides logical properties
    assert_eq!(merge_classes("inset-inline-start-full inset-1"), "inset-1");
    assert_eq!(merge_classes("inset-block-end-full inset-1"), "inset-1");

    // logical properties narrow from inset (same as physical)
    assert_eq!(merge_classes("inset-1 inset-inline-start-full"), "inset-1 inset-inline-start-full");
    assert_eq!(merge_classes("inset-1 inset-block-start-full"), "inset-1 inset-block-start-full");

    // arbitrary values
    assert_eq!(merge_classes("start-[10px] inset-inline-start-[20px]"), "inset-inline-start-[20px]");
    assert_eq!(merge_classes("top-[10px] inset-block-start-[20px]"), "inset-block-start-[20px]");
}

#[test]
fn ring_and_shadow_classes_do_not_create_conflict() {
    assert_eq!(merge_classes("ring shadow"), "ring shadow");
    assert_eq!(merge_classes("ring-2 shadow-md"), "ring-2 shadow-md");
    assert_eq!(merge_classes("shadow ring"), "shadow ring");
    assert_eq!(merge_classes("shadow-md ring-2"), "shadow-md ring-2");
}

#[test]
fn touch_classes_do_create_conflicts_correctly() {
    assert_eq!(merge_classes("touch-pan-x touch-pan-right"), "touch-pan-right");
    assert_eq!(merge_classes("touch-none touch-pan-x"), "touch-pan-x");
    assert_eq!(merge_classes("touch-pan-x touch-none"), "touch-none");
    assert_eq!(merge_classes("touch-pan-x touch-pan-y touch-pinch-zoom"), "touch-pan-x touch-pan-y touch-pinch-zoom");
    assert_eq!(
        merge_classes("touch-manipulation touch-pan-x touch-pan-y touch-pinch-zoom"),
        "touch-pan-x touch-pan-y touch-pinch-zoom"
    );
    assert_eq!(merge_classes("touch-pan-x touch-pan-y touch-pinch-zoom touch-auto"), "touch-auto");
}

#[test]
fn line_clamp_classes_do_create_conflicts_correctly() {
    assert_eq!(merge_classes("overflow-auto inline line-clamp-1"), "line-clamp-1");
    assert_eq!(merge_classes("line-clamp-1 overflow-auto inline"), "line-clamp-1 overflow-auto inline");
}

#[test]
fn test_line_height_font_size() {
    assert_eq!(merge_classes("leading-9 text-lg"), "text-lg");
}

#[test]
fn text_color_font_size() {
    assert_eq!(merge_classes("text-red-500 text-lg"), "text-red-500 text-lg");

    // https://tailwindcss.com/docs/font-size#setting-the-line-height
    assert_eq!(merge_classes("text-red-500/10 text-lg/9"), "text-red-500/10 text-lg/9");
}

#[test]
fn stroke_width() {
    assert_eq!(merge_classes("stroke-2 stroke-[3]"), "stroke-[3]");
}

#[test]
fn handles_negative_value_conflicts_correctly() {
    assert_eq!(merge_classes("-top-12 -top-2000"), "-top-2000");
}

#[test]
fn tailwind_3_4() {
    assert_eq!(merge_classes("text-red text-lg/8"), "text-red text-lg/8");
}
