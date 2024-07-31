use crate::error::{Error, Result};
use std::process::{Command, Output};

fn get_git_config_command() -> Command {
    let mut command = Command::new("git");

    command.arg("config");
    command.arg("--local");

    command
}

fn handle_error(output: Output) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        Err(Error::GitConfig(String::from_utf8(output.stderr)?))
    }
}

pub fn set_git_config(name: &str, value: &str) -> Result<()> {
    let mut command = get_git_config_command();

    command.arg(name);
    command.arg(value);

    handle_error(command.output()?)
}

pub fn unset_git_config(name: &str) -> Result<()> {
    let mut command = get_git_config_command();

    command.arg("--unset");
    command.arg(name);

    handle_error(command.output()?)
}
