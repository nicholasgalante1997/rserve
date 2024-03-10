use colored::Colorize;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!(
            "{}: {}",
            "info".bold().truecolor(128, 172, 248),
            message.bold()
        );
    }

    pub fn warn(message: &str) {
        println!(
            "{}: {}",
            "warn".bold().truecolor(255, 231, 110),
            message.bold()
        );
    }

    pub fn error(message: &str) {
        eprintln!(
            "{}: {}",
            "error".bold().truecolor(255, 78, 82),
            message.bold()
        );
    }

    pub fn debug(message: &str) {
        println!(
            "{}: {}",
            "debug".bold().truecolor(120, 219, 203),
            message.bold()
        );
    }
}
