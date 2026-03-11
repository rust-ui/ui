use leptos::prelude::*;

pub type ToastId = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastLevel {
    Info,
    Success,
    Warn,
    Error,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[derive(Clone, Debug)]
pub struct ToastData {
    pub id: ToastId,
    pub expiry: Option<u32>,
    pub message: String,
    pub progress: bool,
    pub dismissable: bool,
    pub clear_signal: RwSignal<bool>,

    pub level: ToastLevel,
    pub position: ToastPosition,
}
