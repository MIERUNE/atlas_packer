use std::time::{Duration, Instant};

pub struct ExecutionTimer {
    start_time: Instant,
}

impl ExecutionTimer {
    pub fn start() -> Self {
        ExecutionTimer {
            start_time: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}
