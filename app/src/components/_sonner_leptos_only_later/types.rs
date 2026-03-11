use leptos::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use wasm_bindgen::JsValue;

use super::toast_id::ToastId;

#[derive(Clone, Copy, Debug)]
pub struct ToastOptions {
    pub dismissible: bool,
    /// Duration until the toast should be dismissed
    pub duration: Option<Duration>,
    /// The position of the toast
    pub position: Option<ToasterPosition>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        ToastOptions {
            dismissible: true,
            duration: None,
            position: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Toast {
    pub id: ToastId,
    pub options: ToastOptions,
}

#[derive(Clone, Copy)]
pub struct Toasts {
    pub(crate) toasts: StoredValue<Vec<Toast>, LocalStorage>,
    pub(crate) views: StoredValue<HashMap<ToastId, AnyView>, LocalStorage>,
    pub(crate) trigger: RwSignal<usize>,
}

impl Toasts {
    /// Create a new toast
    pub fn toast(&self, toast: impl IntoView + 'static, id: Option<ToastId>, options: Option<ToastOptions>) {
        let id = id.unwrap_or_else(ToastId::new);
        let view_any = toast.into_any();

        // Store the toast metadata
        let toast = Toast {
            id,
            options: options.unwrap_or_default(),
        };
        self.toasts.update_value(|toasts| {
            toasts.insert(0, toast);
        });

        // Store the view separately
        self.views.update_value(|views| {
            views.insert(id, view_any);
        });

        self.trigger.update(|t| *t += 1);
    }

    pub fn dismiss(&self, toast_id: &ToastId) {
        self.toasts.update_value(|toasts| {
            if let Some(index) = toasts.iter().position(|t| &t.id == toast_id) {
                toasts.remove(index);
            };
        });

        // Remove the view as well
        self.views.update_value(|views| {
            views.remove(toast_id);
        });

        self.trigger.update(|t| *t += 1);
    }
}

/// Possible positions for the toasts
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ToasterPosition {
    BottomRight,
}

impl ToasterPosition {
    pub fn x(&self) -> String {
        "right".to_string()
    }

    pub fn y(&self) -> String {
        "bottom".to_string()
    }
}

/// Call this to dismiss the toast with the given id
pub fn dismiss_toast(toast_id: &ToastId) {
    let message = format!("LEPTOS_TOASTER:{}", toast_id.to_decodable_string());
    let _ = window().post_message(&JsValue::from_str(&message), "*");
}

pub fn decode_message(message: String) -> Option<ToastId> {
    if let Some(toast_id) = message.strip_prefix("LEPTOS_TOASTER:") {
        return Some(ToastId::decode_string(toast_id));
    }

    None
}

pub struct HeightT {
    pub toast_id: ToastId,
    pub height: f64,
}
