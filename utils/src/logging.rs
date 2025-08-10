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
