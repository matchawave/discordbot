mod environment;
pub use environment::*;
#[derive(Debug)]
pub struct ElapsedTime {
    start: std::time::Instant,
}

impl ElapsedTime {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
    pub fn elapsed_s(&self) -> u64 {
        self.start.elapsed().as_secs()
    }

    pub fn reset(&mut self) {
        self.start = std::time::Instant::now();
    }
    pub fn reset_and_get(&mut self) -> std::time::Duration {
        let elapsed = self.start.elapsed();
        self.reset();
        elapsed
    }
}
