use std::fmt::Display;

use leptos::prelude::RwSignal;

use crate::ui::toast_custom::_data::{ToastData, ToastId, ToastLevel, ToastPosition};

pub struct ToastBuilder {
    message: String,
    description: Option<String>,
    level: ToastLevel,
    dismissable: bool,
    expiry: Option<u32>,
    progress: bool,
    position: ToastPosition,
    is_sonner: bool,
}

impl ToastBuilder {
    #[must_use]
    pub fn new<T>(message: T) -> Self
    where
        T: Display,
    {
        ToastBuilder {
            progress: true,
            dismissable: true,
            expiry: Some(2_500),
            level: ToastLevel::Info,
            message: message.to_string(),
            description: None,
            position: ToastPosition::BottomRight,
            is_sonner: false,
        }
    }

    #[must_use]
    pub fn with_level(mut self, level: ToastLevel) -> Self {
        self.level = level;
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_dismissable(mut self, dismissable: bool) -> Self {
        self.dismissable = dismissable;
        self
    }

    #[must_use]
    pub fn with_progress(mut self, progress: bool) -> Self {
        self.progress = progress;
        self
    }

    #[must_use]
    pub fn with_expiry(mut self, expiry: Option<u32>) -> Self {
        self.expiry = expiry;
        self
    }

    #[must_use]
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn with_sonner(mut self, is_sonner: bool) -> Self {
        self.is_sonner = is_sonner;
        self
    }

    /// Builds the toast into a `ToastData` with the supplied ID.
    #[must_use]
    pub fn build(self, id: ToastId) -> ToastData {
        ToastData {
            id,
            level: self.level,
            expiry: self.expiry,
            message: self.message,
            description: self.description,
            position: self.position,
            progress: self.progress,
            dismissable: self.dismissable,
            clear_signal: RwSignal::new(false),
            is_sonner: self.is_sonner,
        }
    }
}
