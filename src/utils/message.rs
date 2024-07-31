use colored::Colorize;

pub fn prefix_message(prefix: &str, message: &str) -> String {
    format!("{prefix}{} {message}", ":".bold())
}

pub fn error_message(message: &str) -> String {
    prefix_message(&"error".red().bold().to_string(), message)
}
