mod cli;
mod commands;
mod db;
mod error;
mod git;
mod message;
mod utils;

use clap::Parser;
use cli::{Cli, Command};
use std::process::ExitCode;

fn main() -> ExitCode {
    let Cli { command } = Cli::parse();
    let result = match command {
        Command::Delete { profile_name, yes } => commands::delete(profile_name, yes),
        Command::Edit { profile_name } => commands::edit(profile_name),
        Command::List { filter } => commands::list(filter),
        Command::Switch {
            profile_name,
            unset,
        } => commands::switch(profile_name, unset),
        Command::View {
            profile_name,
            color,
        } => commands::view(profile_name, color),
    };

    if let Err(err) = result {
        println!("{}{}", message::error_prefix(), &err.to_string());

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
