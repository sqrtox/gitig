use crate::db::{resolve_profile_name, Database};
use crate::error::{Error, Result};

pub fn view(profile_name: Option<String>) -> Result<()> {
    let db = Database::connect()?;
    let profile_name = resolve_profile_name(profile_name)?;
    let Some(profile) = db.select_profile_by_name(&profile_name)? else {
        return Err(Error::ProfileNotFound(profile_name));
    };

    println!("{}", profile.body);

    Ok(())
}
