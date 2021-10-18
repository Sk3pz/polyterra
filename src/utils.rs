use std::time::{SystemTime, UNIX_EPOCH};
use better_term::style::Style;
use fern::colors::ColoredLevelConfig;

const FILTERED_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

pub fn systime() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Fatal error occurred: System time moved backwards! Are you a time traveler?")
        .as_millis() as usize
}

/// A method for better_term to clear the screen
pub fn color_flush() {
    println!("{}", Style::default());
}