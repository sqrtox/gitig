use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::db::{resolve_profile_name, Database};
use crate::error::{Error, Result};
use crate::message;

pub fn delete(profile_name: Option<String>, yes: bool) -> Result<()> {
    let db = Database::connect()?;
    let profile_name = resolve_profile_name(profile_name)?;

    if db.select_profile_by_name(&profile_name)?.is_none() {
        return Err(Error::ProfileNotFound(profile_name));
    }

    if !yes {
        let answer = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Once erased, it cannot be reverted. Would you like to continue?")
            .default(false)
            .interact()?;

        if !answer {
            println!("Aborted by user");

            return Ok(());
        }
    }

    db.delete_profile_by_name(&profile_name)?;

    println!(
        "{}Deleted profile {}",
        message::successfully_prefix(),
        message::identifier(&profile_name)
    );

    Ok(())
}
