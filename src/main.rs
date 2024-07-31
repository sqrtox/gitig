mod cli;
mod commands;
mod error;
mod utils;

use clap::Parser;
use cli::{Cli, Command};
use std::process::ExitCode;
use utils::message::error_message;

fn main() -> ExitCode {
    let Cli { command } = Cli::parse();
    let result = match command {
        Command::Edit { profile_name } => commands::edit(profile_name),
        Command::List { filter } => commands::list(filter),
        Command::Switch {
            profile_name,
            unset,
        } => commands::switch(profile_name, unset),
    };

    if let Err(err) = result {
        println!("{}", error_message(&err.to_string()));

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
