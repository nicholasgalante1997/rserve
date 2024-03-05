use colored::Colorize;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!("{}: {}", "info".bold().bright_blue(), message.bold());
    }

    pub fn warn(message: &str) {
        println!("{}: {}", "warn".bold().yellow(), message.bold());
    }

    pub fn error(message: &str) {
        eprintln!("{}: {}", "error".bold().red(), message.bold());
    }

    pub fn debug(message: &str) {
        println!("{}: {}", "debug".bold().green(), message.bold());
    }
}
