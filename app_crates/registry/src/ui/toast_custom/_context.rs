use std::fmt::Display;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;

use crate::ui::toast_custom::_builder::ToastBuilder;
use crate::ui::toast_custom::_data::{ToastData, ToastId, ToastLevel};

#[derive(Clone, Debug)]
pub struct ToasterContext {
    stats: Arc<Mutex<ToasterStats>>,
    pub queue_signal: RwSignal<Vec<ToastData>>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
    visible: u32,
    total: u64,
}

impl ToasterContext {
    pub fn toast(&self, builder: ToastBuilder) {
        let Ok(mut stats) = self.stats.lock() else { return };
        let stats = &mut *stats;
        let toast = builder.build(stats.total + 1);

        let mut queue = self.queue_signal.get_untracked();
        queue.push(toast);
        self.queue_signal.set(queue);
        stats.visible += 1;
        stats.total += 1;
    }

    pub fn info<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Info));
    }

    pub fn success<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Success));
    }

    pub fn warn<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Warn));
    }

    pub fn error<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Error));
    }

    pub fn loading<T>(&self, message: T) -> ToastId
    where
        T: Display,
    {
        let Ok(mut stats) = self.stats.lock() else { return 0 };
        let stats = &mut *stats;
        let toast = ToastBuilder::new(message).with_level(ToastLevel::Loading).with_expiry(None).build(stats.total + 1);
        let id = toast.id;

        let mut queue = self.queue_signal.get_untracked();
        queue.push(toast);
        self.queue_signal.set(queue);
        stats.visible += 1;
        stats.total += 1;

        id
    }

    /// Update a toast's message, description, and level (e.g. loading → success).
    pub fn update(&self, id: ToastId, level: ToastLevel, message: impl Into<String>, description: Option<String>) {
        self.queue_signal.update(|queue| {
            if let Some(toast) = queue.iter_mut().find(|t| t.id == id) {
                toast.level = level;
                toast.message = message.into();
                toast.description = description;
            }
        });
    }

    pub fn clear(&self) {
        for toast in &self.queue_signal.get_untracked() {
            toast.clear_signal.set(true);
        }
    }

    /// Removes the toast corresponding with the supplied `ToastId`.
    pub fn remove(&self, toast_id: ToastId) {
        let index = self
            .queue_signal
            .get_untracked()
            .iter()
            .enumerate()
            .find(|(_, toast)| toast.id == toast_id)
            .map(|(index, _)| index);

        if let Some(index) = index {
            let mut queue = self.queue_signal.get_untracked();
            queue.remove(index);
            self.queue_signal.set(queue);

            if let Ok(mut stats) = self.stats.lock() {
                stats.visible = stats.visible.saturating_sub(1);
            }
        }
    }
}

impl Default for ToasterContext {
    fn default() -> Self {
        ToasterContext { stats: Arc::new(Mutex::new(ToasterStats::default())), queue_signal: RwSignal::new(Vec::new()) }
    }
}
