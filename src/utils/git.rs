use std::process::{Command, Output};

use crate::error::Result;

fn get_git_config_command() -> Command {
    let mut command = Command::new("git");

    command.arg("config");
    command.arg("--local");

    command
}

pub fn set_git_config(name: &str, value: &str) -> Result<Output> {
    let mut command = get_git_config_command();

    command.arg(name);
    command.arg(value);

    Ok(command.output()?)
}

pub fn unset_git_config(name: &str) -> Result<Output> {
    let mut command = get_git_config_command();

    command.arg("--unset");
    command.arg(name);

    Ok(command.output()?)
}
