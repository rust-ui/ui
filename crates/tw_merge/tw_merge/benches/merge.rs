use divan::Bencher;
use tw_merge::merge::{merge_classes, tw_merge_slice};

fn main() {
    divan::main();
}

const LENS: &[usize] = &[10, 20, 40, 60, 80, 100];
const SAMPLE_COUNT: u32 = 1000;
const SAMPLE_SIZE: u32 = 100;

#[divan::bench(
    args = LENS,
    sample_count = SAMPLE_COUNT,
    sample_size = SAMPLE_SIZE
)]
fn tailwind_merge(bencher: Bencher, len: usize) {
    bencher.with_inputs(|| generate_random_classes(len).join(" ")).bench_values(|class| merge_classes(class.as_str()));
}

#[divan::bench(
    args = LENS,
    sample_count = SAMPLE_COUNT,
    sample_size = SAMPLE_SIZE
)]
fn tailwind_merge_slice(bencher: Bencher, len: usize) {
    bencher.with_inputs(|| generate_random_classes(len)).bench_values(|class| tw_merge_slice(&class));
}

// create a vec with the a length of len and fill it with random data
fn generate_random_classes(n: usize) -> Vec<&'static str> {
    let mut classes_vec = Vec::with_capacity(n);
    for _ in 0..n {
        let index = fastrand::usize(..TAILWIND_CLASSES.len());
        let element = TAILWIND_CLASSES[index];
        classes_vec.push(element);
    }
    classes_vec
}

const TAILWIND_CLASSES: [&str; 108] = [
    "bg-red-500",
    "text-xl",
    "p-4",
    "m-2",
    "rounded-lg",
    "shadow-md",
    "flex",
    "items-center",
    "justify-between",
    "gap-4",
    "hover:bg-blue-500",
    "active:text-white",
    "sm:text-base",
    "md:bg-green-500",
    "lg:p-8",
    "xl:m-10",
    "2xl:rounded-full",
    "focus:outline-none",
    "disabled:opacity-50",
    "group-hover:bg-purple-500",
    "transition-all",
    "duration-300",
    "ease-in-out",
    "rotate-45",
    "scale-105",
    "translate-x-2",
    "skew-y-6",
    "bg-[#123456]",
    "text-[14px]",
    "p-[30px]",
    "m-[5%]",
    "w-[calc(100%-2rem)]",
    "h-[50vh]",
    "gap-[2rem]",
    "grid-cols-[repeat(auto-fill,minmax(250px,1fr))]",
    "border-t-[3px]",
    "border-opacity-[0.85]",
    "shadow-[0_35px_60px_-15px_rgba(0,0,0,0.3)]",
    "backdrop-blur-[5px]",
    "bg-gradient-to-r",
    "from-[#6ee7b7]",
    "via-[#3b82f6]",
    "to-[#9333ea]",
    "text-[color:var(--custom-color)]",
    "hover:bg-[length:200px_100px]",
    "flex",
    "flex-col",
    "flex-row",
    "grid",
    "bg-blue-100",
    "bg-opacity-50",
    "bg-gradient-to-br",
    "from-teal-400",
    "to-blue-500",
    "p-4",
    "m-2",
    "space-x-4",
    "rounded-lg",
    "border-2",
    "border-dashed",
    "border-red-300",
    "shadow-xl",
    "opacity-75",
    "mix-blend-multiply",
    "w-1/2",
    "min-w-full",
    "max-w-xs",
    "h-12",
    "min-h-screen",
    "scale-90",
    "rotate-12",
    "translate-x-1/4",
    "skew-y-3",
    "xl:origin-top-right",
    "cursor-pointer",
    "select-none",
    "resize-y",
    "transition-colors",
    "duration-200",
    "ease-in-out",
    "delay-150",
    "animate-spin",
    "hover:bg-pink-700",
    "active:text-white",
    "focus:ring-4",
    "focus:outline-none",
    "disabled:opacity-25",
    "visited:text-purple-600",
    "dark:touch-pan-x",
    "hover:touch-pan-x",
    "touch-manipulation",
    "touch-pan-x",
    "touch-pan-y",
    "touch-pinch-zoom",
    "border-[color:rgb(var(--color-gray-500-rgb)/50%))]",
    "border-border",
    "grid-rows-[repeat(20,minmax(0,1fr))]",
    "bg-[percentage:30%]",
    "bg-[percentage:30%]",
    "bg-[length:200px_100px]",
    "mt-[calc(theme(fontSize.4xl)/1.125)]",
    "bg-[linear-gradient(.)]",
    "content-[attr(data-content)]",
    "hover:[@media_screen{@media(hover:hover)}]:underline",
    "[&>*]:[&_div]:underline",
    "[&[data-foo][data-bar]:not([data-baz])]:line-through",
    "!-inset-x-px",
    "hover:m-[length:var(--c)]",
];

#[test]
fn assert_unique() {
    let mut set = std::collections::HashSet::new();
    for class in TAILWIND_CLASSES.iter() {
        assert!(set.insert(class), "duplicate class: {}", class);
    }
}
