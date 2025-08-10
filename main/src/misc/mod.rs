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

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        use chrono::{DateTime, Utc};
        use colored::Colorize;
        let dt: DateTime<Utc> = Utc::now();

        let formated_time_date: String = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{} [{}]: {}",
            formated_time_date,
            "INFO".blue().bold(),
            format!($($arg)*)
        );

    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        use chrono::{DateTime, Utc};
        use colored::Colorize;
        let dt: DateTime<Utc> = Utc::now();

        let formated_time_date: String = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{} [{}]: {}",
            formated_time_date,
            "ERROR".red().bold(),
            format!($($arg)*)
        );

    });
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ({
        use chrono::{DateTime, Utc};
        use colored::Colorize;
        let dt: DateTime<Utc> = Utc::now();

        let formated_time_date: String = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{} [{}]: {}",
            formated_time_date,
            "WARN".yellow().bold(),
            format!($($arg)*)
        );

    });
}
