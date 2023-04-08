use std::sync::atomic::{AtomicBool, Ordering};

static QUEUE_CANCELLED: AtomicBool = AtomicBool::new(false);

pub fn set_queue_cancelled(value: bool) {
    QUEUE_CANCELLED.store(value, Ordering::Relaxed);
}

pub fn get_queue_cancelled() -> bool {
    return QUEUE_CANCELLED.load(Ordering::Relaxed);
}