use colored::{ColoredString, Colorize};

pub fn identifier(name: &str) -> ColoredString {
    format!("'{name}'").yellow()
}

pub fn note() -> ColoredString {
    "note:".bold()
}

pub fn error() -> ColoredString {
    "error:".bold().red()
}

pub fn successfully() -> ColoredString {
    "successfully:".bold().green()
}
