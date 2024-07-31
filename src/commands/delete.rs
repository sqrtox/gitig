use crate::db::{resolve_profile_name, Database};
use crate::error::{Error, Result};
use crate::message;

pub fn delete(profile_name: Option<String>) -> Result<()> {
    let db = Database::connect()?;
    let profile_name = resolve_profile_name(profile_name)?;

    if db.select_profile_by_name(&profile_name)?.is_none() {
        return Err(Error::ProfileNotFound(profile_name));
    }

    db.delete_profile_by_name(&profile_name)?;

    println!(
        "{}Deleted profile {}",
        message::successfully_prefix(),
        message::identifier(&profile_name)
    );

    Ok(())
}
