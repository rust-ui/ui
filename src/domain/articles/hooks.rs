use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static HORIZONTAL_RAIL_COUNTER: AtomicUsize = AtomicUsize::new(0);

const HORIZONTAL_RAIL_SCRIPT: &str = r"
    (() => {
        const setup = () => {
            const rail = document.getElementById('__RAIL_ID__');
            const previous = document.getElementById('__PREVIOUS_ID__');
            const next = document.getElementById('__NEXT_ID__');
            if (!rail || !previous || !next || rail.dataset.initialized) return;
            rail.dataset.initialized = 'true';
            const updateButtons = () => {
                previous.disabled = rail.scrollLeft <= 1;
                next.disabled = Math.ceil(rail.scrollLeft + rail.clientWidth) >= rail.scrollWidth - 1;
            };
            previous.addEventListener('click', () => rail.scrollBy({ left: -384, behavior: 'smooth' }));
            next.addEventListener('click', () => rail.scrollBy({ left: 384, behavior: 'smooth' }));
            rail.addEventListener('scroll', updateButtons, { passive: true });
            window.addEventListener('resize', updateButtons);
            updateButtons();
        };
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', setup, { once: true });
        } else {
            requestAnimationFrame(setup);
        }
    })();
";

pub struct HorizontalRail {
    rail: String,
    previous: String,
    next: String,
}

impl HorizontalRail {
    #[must_use]
    pub fn rail_id(&self) -> &str {
        &self.rail
    }

    #[must_use]
    pub fn previous_id(&self) -> &str {
        &self.previous
    }

    #[must_use]
    pub fn next_id(&self) -> &str {
        &self.next
    }

    #[must_use]
    pub fn script(&self) -> String {
        HORIZONTAL_RAIL_SCRIPT
            .replace("__RAIL_ID__", &self.rail)
            .replace("__PREVIOUS_ID__", &self.previous)
            .replace("__NEXT_ID__", &self.next)
    }
}

pub fn use_horizontal_rail(name: &'static str) -> HorizontalRail {
    let id = use_hook(|| HORIZONTAL_RAIL_COUNTER.fetch_add(1, Ordering::Relaxed));
    let prefix = format!("horizontal-rail-{name}-{id}");

    HorizontalRail {
        rail: format!("{prefix}-rail"),
        previous: format!("{prefix}-previous"),
        next: format!("{prefix}-next"),
    }
}
