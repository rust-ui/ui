use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Utils;

impl Utils {
    pub fn use_random_id() -> String {
        let mut hasher = DefaultHasher::new();

        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        counter.hash(&mut hasher);

        format!("_gen_id_{}", hasher.finish())
    }

    pub fn use_random_transition_name() -> String {
        let random_id = Utils::use_random_id();
        format!("view-transition-name: {random_id}")
    }
}
