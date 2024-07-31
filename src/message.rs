use colored::Colorize;

pub fn identifier(name: &str) -> String {
    format!("'{name}'").yellow().to_string()
}

fn prefix(prefix: &str) -> String {
    format!("{}{} ", prefix, ":".bold())
}

pub fn error_prefix() -> String {
    prefix(&"error".red().bold().to_string())
}

pub fn successfully_prefix() -> String {
    prefix(&"successfully".green().bold().to_string())
}
