use leptos::prelude::*;

pub type ToastId = u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToastLevel {
    Info,
    Success,
    Warn,
    Error,
    Loading,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/* ========================================================== */
/*                       🧬 STRUCT 🧬                         */
/* ========================================================== */

#[derive(Clone, Debug)]
pub struct ToastData {
    pub id: ToastId,
    pub expiry: Option<u32>,
    pub message: String,
    pub description: Option<String>,
    pub progress: bool,
    pub dismissable: bool,
    pub clear_signal: RwSignal<bool>,

    pub level: ToastLevel,
    pub position: ToastPosition,
}
