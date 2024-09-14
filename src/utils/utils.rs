use core::time::Duration;

use crate::time::{self, time_manager};


pub struct Perf {
    stamp: Duration
}

impl Perf {
    pub fn start() -> Self {
        Perf { stamp: time::time_manager().uptime() }
    }
    pub fn stop(self) -> Duration {
        time_manager().uptime() - self.stamp
    }
}