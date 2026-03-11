use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Tag {
    Accordion,
    Animation,
    Button,
    Card,
    Carousel,
    Css,
    Dialog,
    Dropdown,
    Gradient,
    Grid,
    Headings,
    Image,
    InlineJs,
    Input,
    Navigation,
    Parallax,
    Popover,
    Select,
    Table,
    Utils,
    ViewTransition,
}
