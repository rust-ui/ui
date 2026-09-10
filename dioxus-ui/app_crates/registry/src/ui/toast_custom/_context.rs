use std::fmt::Display;
use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

use crate::ui::toast_custom::_builder::ToastBuilder;
use crate::ui::toast_custom::_data::{ToastData, ToastId, ToastLevel};

#[derive(Clone, Debug)]
pub struct ToasterContext {
    stats: Arc<Mutex<ToasterStats>>,
    pub queue_signal: Signal<Vec<ToastData>>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
    visible: u32,
    total: u64,
}

impl ToasterContext {
    pub fn new(queue_signal: Signal<Vec<ToastData>>) -> Self {
        Self { stats: Arc::new(Mutex::new(ToasterStats::default())), queue_signal }
    }

    pub fn toast(&self, builder: ToastBuilder) {
        let Ok(mut stats) = self.stats.lock() else { return };
        let stats = &mut *stats;
        let toast = builder.build(stats.total + 1);

        let mut queue = self.queue_signal.peek().clone();
        queue.push(toast);
        self.queue_signal.clone().set(queue);
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

    pub fn clear(&self) {
        for toast in self.queue_signal.peek().iter() {
            toast.clear_signal.clone().set(true);
        }
    }

    /// Removes the toast corresponding with the supplied `ToastId`.
    pub fn remove(&self, toast_id: ToastId) {
        let index =
            self.queue_signal.peek().iter().enumerate().find(|(_, toast)| toast.id == toast_id).map(|(index, _)| index);

        if let Some(index) = index {
            let mut queue = self.queue_signal.peek().clone();
            queue.remove(index);
            self.queue_signal.clone().set(queue);

            if let Ok(mut stats) = self.stats.lock() {
                stats.visible -= 1;
            }
        }
    }
}
