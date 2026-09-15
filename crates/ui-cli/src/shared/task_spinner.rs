use std::time::Duration;

use indicatif::ProgressBar;

const SPINNER_UPDATE_DURATION: u64 = 100;

pub struct TaskSpinner {
    spinner: ProgressBar,
}

impl TaskSpinner {
    pub fn new(message: &str) -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(Duration::from_millis(SPINNER_UPDATE_DURATION));
        Self { spinner }
    }

    pub fn set_message(&self, message: &str) {
        self.spinner.set_message(message.to_string());
    }

    pub fn finish_success(self, message: &str) {
        self.spinner.finish_with_message(format!("✔️ {message}"));
    }

    pub fn finish_with_message(self, message: &str) {
        self.spinner.finish_with_message(message.to_string());
    }
}
