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

pub fn setup_logger() {
    // define the colors for the console logging
    let colors = ColoredLevelConfig::new()
        .trace(fern::colors::Color::White)
        .debug(fern::colors::Color::BrightBlack)
        .info(fern::colors::Color::Cyan)
        .warn(fern::colors::Color::BrightYellow)
        .error(fern::colors::Color::BrightRed);

    // configure logging
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}", // format: '[YYYY-MM-DD][HH:MM:SS][target][level] msg'
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(FILTERED_LOG_LEVEL) // Set at the top of this file, filters all levels below it
        .chain(std::io::stdout()) // Add stdout as an output
        .apply().expect("Failed to create logger"); // create the logger
}