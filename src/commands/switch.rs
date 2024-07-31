use crate::db::{resolve_profile_name, Database};
use crate::error::{Error, Result};
use crate::git::{set_git_config, unset_git_config};
use crate::message;
use gix_config::{file::Metadata, File};

pub fn switch(profile_name: Option<String>, unset: bool) -> Result<()> {
    let profile_name = resolve_profile_name(profile_name)?;
    let db = Database::connect()?;
    let profile = db
        .select_profile_by_name(&profile_name)?
        .ok_or(Error::ProfileNotFound(profile_name.clone()))?;
    let profile_body = profile.body.bytes().collect::<Vec<_>>();
    let config =
        File::from_bytes_no_includes(&profile_body, Metadata::default(), Default::default())?;
    let mut effects = 0;

    for section in config.sections() {
        let header = section.header();
        let subsection_name = header.subsection_name();
        let name = if let Some(subsection_name) = subsection_name {
            header.name().to_string() + "." + &subsection_name.to_string()
        } else {
            header.name().to_string()
        };

        for value_name in section.value_names() {
            let config_name = name.clone() + "." + value_name.as_ref();

            effects += 1;

            if unset {
                unset_git_config(&config_name)?;
            } else {
                for config_value in section.values(value_name.as_ref()) {
                    set_git_config(&config_name, &config_value.to_string())?;
                }
            }
        }
    }

    println!(
        "{}Switched to profile {} (effected {})",
        message::successfully_prefix(),
        message::identifier(&profile_name),
        effects
    );

    Ok(())
}
