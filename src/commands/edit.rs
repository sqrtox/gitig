use crate::db::{resolve_profile_name, Database, Profile};
use crate::error::Result;
use crate::message;
use edit::{edit_with_builder, Builder};

pub fn edit(profile_name: Option<String>) -> Result<()> {
    let profile_name = resolve_profile_name(profile_name)?;
    let db = Database::connect()?;

    if let Some(profile) = db.select_profile_by_name(&profile_name)? {
        let body = edit_with_builder(profile.body, Builder::new().suffix(".gitconfig"))?;

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
        let body = edit_with_builder("", Builder::new().suffix(".gitconfig"))?;

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
