use crate::db::{resolve_profile_name, Database, Profile};
use crate::error::Result;
use crate::message;
use edit::edit as edit_contents;

pub fn edit(profile_name: Option<String>) -> Result<()> {
    let profile_name = resolve_profile_name(profile_name)?;
    let db = Database::connect()?;

    if let Some(profile) = db.select_profile_by_name(&profile_name)? {
        let body = edit_contents(profile.body)?;

        db.update_profile_body(&Profile {
            name: profile.name,
            body,
        })?;

        println!(
            "{}Edited profile {}",
            message::successfully_prefix(),
            message::identifier(&profile_name)
        );
    } else {
        let body = edit_contents("")?;

        db.insert_profile(&Profile {
            name: profile_name.clone(),
            body,
        })?;

        println!(
            "{}Created profile {}",
            message::successfully_prefix(),
            message::identifier(&profile_name)
        );
    };

    Ok(())
}
