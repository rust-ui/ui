use icons::{Anchor, ArrowRight, CircleDashed};
use leptos::prelude::*;
use strum::Display;

// * This file was generated automatically with build_registry.

#[derive(Clone, Debug, Display)]
pub enum CommandCategory {
    Pages,
    Components,
    Hooks,
}

#[derive(Clone, Debug)]
pub struct CommandItemData {
    pub label: &'static str,
    pub href: &'static str,
    pub category: CommandCategory,
    /// CLI install command slug, e.g. Some("alert_dialog") for components, None for pages/hooks.
    pub add_cmd: Option<&'static str>,
}

impl CommandItemData {
    pub fn to_icon(&self) -> AnyView {
        match self.category {
            CommandCategory::Pages => view! { <ArrowRight /> }.into_any(),
            CommandCategory::Components => view! { <CircleDashed /> }.into_any(),
            CommandCategory::Hooks => view! { <Anchor /> }.into_any(),
        }
    }
}

/// PAGES_ITEMS
pub const PAGES_ITEMS: &[CommandItemData] = &[
    CommandItemData { label: "Blocks", href: "/blocks", category: CommandCategory::Pages, add_cmd: None },
    CommandItemData { label: "CLI", href: "/docs/components/cli", category: CommandCategory::Pages, add_cmd: None },
    CommandItemData { label: "Charts", href: "/charts", category: CommandCategory::Pages, add_cmd: None },
    CommandItemData { label: "Create", href: "/create", category: CommandCategory::Pages, add_cmd: None },
    CommandItemData { label: "Icons", href: "/icons", category: CommandCategory::Pages, add_cmd: None },
    CommandItemData {
        label: "Installation",
        href: "/docs/components/installation",
        category: CommandCategory::Pages,
        add_cmd: None,
    },
];

/// COMPONENTS_ITEMS
pub const COMPONENTS_ITEMS: &[CommandItemData] = &[
    CommandItemData {
        label: "Accordion",
        href: "/docs/components/accordion",
        category: CommandCategory::Components,
        add_cmd: Some("accordion"),
    },
    CommandItemData {
        label: "Alert",
        href: "/docs/components/alert",
        category: CommandCategory::Components,
        add_cmd: Some("alert"),
    },
    CommandItemData {
        label: "Alert Dialog",
        href: "/docs/components/alert-dialog",
        category: CommandCategory::Components,
        add_cmd: Some("alert_dialog"),
    },
    CommandItemData {
        label: "Animate",
        href: "/docs/components/animate",
        category: CommandCategory::Components,
        add_cmd: Some("animate"),
    },
    CommandItemData {
        label: "Animate Group",
        href: "/docs/components/animate-group",
        category: CommandCategory::Components,
        add_cmd: Some("animate_group"),
    },
    CommandItemData {
        label: "Aspect Ratio",
        href: "/docs/components/aspect_ratio",
        category: CommandCategory::Components,
        add_cmd: Some("aspect_ratio"),
    },
    CommandItemData {
        label: "AutoForm",
        href: "/docs/components/auto-form",
        category: CommandCategory::Components,
        add_cmd: Some("auto_form"),
    },
    CommandItemData {
        label: "Avatar",
        href: "/docs/components/avatar",
        category: CommandCategory::Components,
        add_cmd: Some("avatar"),
    },
    CommandItemData {
        label: "Badge",
        href: "/docs/components/badge",
        category: CommandCategory::Components,
        add_cmd: Some("badge"),
    },
    CommandItemData {
        label: "Bottom Nav",
        href: "/docs/components/bottom-nav",
        category: CommandCategory::Components,
        add_cmd: Some("bottom_nav"),
    },
    CommandItemData {
        label: "Breadcrumb",
        href: "/docs/components/breadcrumb",
        category: CommandCategory::Components,
        add_cmd: Some("breadcrumb"),
    },
    CommandItemData {
        label: "Button",
        href: "/docs/components/button",
        category: CommandCategory::Components,
        add_cmd: Some("button"),
    },
    CommandItemData {
        label: "Button Action",
        href: "/docs/components/button-action",
        category: CommandCategory::Components,
        add_cmd: Some("button_action"),
    },
    CommandItemData {
        label: "Button Group",
        href: "/docs/components/button-group",
        category: CommandCategory::Components,
        add_cmd: Some("button_group"),
    },
    CommandItemData {
        label: "Callout",
        href: "/docs/components/callout",
        category: CommandCategory::Components,
        add_cmd: Some("callout"),
    },
    CommandItemData {
        label: "Card",
        href: "/docs/components/card",
        category: CommandCategory::Components,
        add_cmd: Some("card"),
    },
    CommandItemData {
        label: "Card Carousel",
        href: "/docs/components/card-carousel",
        category: CommandCategory::Components,
        add_cmd: Some("card_carousel"),
    },
    CommandItemData {
        label: "Carousel",
        href: "/docs/components/carousel",
        category: CommandCategory::Components,
        add_cmd: Some("carousel"),
    },
    CommandItemData {
        label: "Checkbox",
        href: "/docs/components/checkbox",
        category: CommandCategory::Components,
        add_cmd: Some("checkbox"),
    },
    CommandItemData {
        label: "Chips",
        href: "/docs/components/chips",
        category: CommandCategory::Components,
        add_cmd: Some("chips"),
    },
    CommandItemData {
        label: "Collapsible",
        href: "/docs/components/collapsible",
        category: CommandCategory::Components,
        add_cmd: Some("collapsible"),
    },
    CommandItemData {
        label: "Combobox",
        href: "/docs/components/combobox",
        category: CommandCategory::Components,
        add_cmd: Some("combobox"),
    },
    CommandItemData {
        label: "Command",
        href: "/docs/components/command",
        category: CommandCategory::Components,
        add_cmd: Some("command"),
    },
    CommandItemData {
        label: "Context Menu",
        href: "/docs/components/context-menu",
        category: CommandCategory::Components,
        add_cmd: Some("context_menu"),
    },
    CommandItemData {
        label: "Data Table",
        href: "/docs/components/data-table",
        category: CommandCategory::Components,
        add_cmd: Some("data_table"),
    },
    CommandItemData {
        label: "Date Picker",
        href: "/docs/components/date-picker",
        category: CommandCategory::Components,
        add_cmd: Some("date_picker"),
    },
    CommandItemData {
        label: "Dialog",
        href: "/docs/components/dialog",
        category: CommandCategory::Components,
        add_cmd: Some("dialog"),
    },
    CommandItemData {
        label: "Direction Provider",
        href: "/docs/components/direction-provider",
        category: CommandCategory::Components,
        add_cmd: Some("direction_provider"),
    },
    CommandItemData {
        label: "Drag and Drop",
        href: "/docs/components/drag-and-drop",
        category: CommandCategory::Components,
        add_cmd: Some("drag_and_drop"),
    },
    CommandItemData {
        label: "Drawer",
        href: "/docs/components/drawer",
        category: CommandCategory::Components,
        add_cmd: Some("drawer"),
    },
    CommandItemData {
        label: "Dropdown Menu",
        href: "/docs/components/dropdown-menu",
        category: CommandCategory::Components,
        add_cmd: Some("dropdown_menu"),
    },
    CommandItemData {
        label: "Dropzone",
        href: "/docs/components/dropzone",
        category: CommandCategory::Components,
        add_cmd: Some("dropzone"),
    },
    CommandItemData {
        label: "Empty",
        href: "/docs/components/empty",
        category: CommandCategory::Components,
        add_cmd: Some("empty"),
    },
    CommandItemData {
        label: "Field",
        href: "/docs/components/field",
        category: CommandCategory::Components,
        add_cmd: Some("field"),
    },
    CommandItemData {
        label: "Form",
        href: "/docs/components/form",
        category: CommandCategory::Components,
        add_cmd: Some("form"),
    },
    CommandItemData {
        label: "Hover Card",
        href: "/docs/components/hover-card",
        category: CommandCategory::Components,
        add_cmd: Some("hover_card"),
    },
    CommandItemData {
        label: "Input",
        href: "/docs/components/input",
        category: CommandCategory::Components,
        add_cmd: Some("input"),
    },
    CommandItemData {
        label: "Input Group",
        href: "/docs/components/input-group",
        category: CommandCategory::Components,
        add_cmd: Some("input_group"),
    },
    CommandItemData {
        label: "Input OTP",
        href: "/docs/components/input-otp",
        category: CommandCategory::Components,
        add_cmd: Some("input_otp"),
    },
    CommandItemData {
        label: "Input Phone",
        href: "/docs/components/input-phone",
        category: CommandCategory::Components,
        add_cmd: Some("input_phone"),
    },
    CommandItemData {
        label: "Item",
        href: "/docs/components/item",
        category: CommandCategory::Components,
        add_cmd: Some("item"),
    },
    CommandItemData {
        label: "Kbd",
        href: "/docs/components/kbd",
        category: CommandCategory::Components,
        add_cmd: Some("kbd"),
    },
    CommandItemData {
        label: "Label",
        href: "/docs/components/label",
        category: CommandCategory::Components,
        add_cmd: Some("label"),
    },
    CommandItemData {
        label: "Marquee",
        href: "/docs/components/marquee",
        category: CommandCategory::Components,
        add_cmd: Some("marquee"),
    },
    CommandItemData {
        label: "Menubar",
        href: "/docs/components/menubar",
        category: CommandCategory::Components,
        add_cmd: Some("menubar"),
    },
    CommandItemData {
        label: "MultiSelect",
        href: "/docs/components/multi-select",
        category: CommandCategory::Components,
        add_cmd: Some("multi_select"),
    },
    CommandItemData {
        label: "Navigation Menu",
        href: "/docs/components/navigation_menu",
        category: CommandCategory::Components,
        add_cmd: Some("navigation_menu"),
    },
    CommandItemData {
        label: "Pagination",
        href: "/docs/components/pagination",
        category: CommandCategory::Components,
        add_cmd: Some("pagination"),
    },
    CommandItemData {
        label: "Popover",
        href: "/docs/components/popover",
        category: CommandCategory::Components,
        add_cmd: Some("popover"),
    },
    CommandItemData {
        label: "Pressable",
        href: "/docs/components/pressable",
        category: CommandCategory::Components,
        add_cmd: Some("pressable"),
    },
    CommandItemData {
        label: "Progress",
        href: "/docs/components/progress",
        category: CommandCategory::Components,
        add_cmd: Some("progress"),
    },
    CommandItemData {
        label: "Radio Button",
        href: "/docs/components/radio-button",
        category: CommandCategory::Components,
        add_cmd: Some("radio_button"),
    },
    CommandItemData {
        label: "Radio Button Group",
        href: "/docs/components/radio-button-group",
        category: CommandCategory::Components,
        add_cmd: Some("radio_button_group"),
    },
    CommandItemData {
        label: "Scroll Area",
        href: "/docs/components/scroll-area",
        category: CommandCategory::Components,
        add_cmd: Some("scroll_area"),
    },
    CommandItemData {
        label: "Select",
        href: "/docs/components/select",
        category: CommandCategory::Components,
        add_cmd: Some("select"),
    },
    CommandItemData {
        label: "Separator",
        href: "/docs/components/separator",
        category: CommandCategory::Components,
        add_cmd: Some("separator"),
    },
    CommandItemData {
        label: "Sheet",
        href: "/docs/components/sheet",
        category: CommandCategory::Components,
        add_cmd: Some("sheet"),
    },
    CommandItemData {
        label: "Shimmer",
        href: "/docs/components/shimmer",
        category: CommandCategory::Components,
        add_cmd: Some("shimmer"),
    },
    CommandItemData {
        label: "Skeleton",
        href: "/docs/components/skeleton",
        category: CommandCategory::Components,
        add_cmd: Some("skeleton"),
    },
    CommandItemData {
        label: "Slider",
        href: "/docs/components/slider",
        category: CommandCategory::Components,
        add_cmd: Some("slider"),
    },
    CommandItemData {
        label: "Sonner",
        href: "/docs/components/sonner",
        category: CommandCategory::Components,
        add_cmd: Some("sonner"),
    },
    CommandItemData {
        label: "Spinner",
        href: "/docs/components/spinner",
        category: CommandCategory::Components,
        add_cmd: Some("spinner"),
    },
    CommandItemData {
        label: "Status",
        href: "/docs/components/status",
        category: CommandCategory::Components,
        add_cmd: Some("status"),
    },
    CommandItemData {
        label: "Switch",
        href: "/docs/components/switch",
        category: CommandCategory::Components,
        add_cmd: Some("switch"),
    },
    CommandItemData {
        label: "Table",
        href: "/docs/components/table",
        category: CommandCategory::Components,
        add_cmd: Some("table"),
    },
    CommandItemData {
        label: "Tabs",
        href: "/docs/components/tabs",
        category: CommandCategory::Components,
        add_cmd: Some("tabs"),
    },
    CommandItemData {
        label: "Textarea",
        href: "/docs/components/textarea",
        category: CommandCategory::Components,
        add_cmd: Some("textarea"),
    },
    CommandItemData {
        label: "Theme Toggle",
        href: "/docs/components/theme-toggle",
        category: CommandCategory::Components,
        add_cmd: Some("theme_toggle"),
    },
    CommandItemData {
        label: "Toast",
        href: "/docs/components/toast",
        category: CommandCategory::Components,
        add_cmd: Some("toast"),
    },
    CommandItemData {
        label: "Toggle Group",
        href: "/docs/components/toggle-group",
        category: CommandCategory::Components,
        add_cmd: Some("toggle_group"),
    },
    CommandItemData {
        label: "Tooltip",
        href: "/docs/components/tooltip",
        category: CommandCategory::Components,
        add_cmd: Some("tooltip"),
    },
];

/// HOOKS_ITEMS
pub const HOOKS_ITEMS: &[CommandItemData] = &[
    CommandItemData {
        label: "Use Copy Clipboard",
        href: "/docs/hooks/use-copy-clipboard",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use History",
        href: "/docs/hooks/use-history",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Horizontal Scroll",
        href: "/docs/hooks/use-horizontal-scroll",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Is Mobile",
        href: "/docs/hooks/use-is-mobile",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Lock Body Scroll",
        href: "/docs/hooks/use-lock-body-scroll",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Locks",
        href: "/docs/hooks/use-locks",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Media Query",
        href: "/docs/hooks/use-media-query",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Press Hold",
        href: "/docs/hooks/use-press-hold",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
    CommandItemData {
        label: "Use Random",
        href: "/docs/hooks/use-random",
        category: CommandCategory::Hooks,
        add_cmd: None,
    },
];
