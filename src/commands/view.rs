use crate::db::{resolve_profile_name, Database};
use crate::error::{Error, Result};
use colored::Colorize;
use gix_config::parse::Event;

pub fn view(profile_name: Option<String>, color: bool) -> Result<()> {
    let db = Database::connect()?;
    let profile_name = resolve_profile_name(profile_name)?;
    let Some(profile) = db.select_profile_by_name(&profile_name)? else {
        return Err(Error::ProfileNotFound(profile_name));
    };

    if color {
        println!("{} >", profile_name.on_bright_black());

        gix_config::parse::from_bytes(profile.body.as_bytes(), &mut |event| match event {
            Event::Comment(contents) => print!("{}", contents.to_string().bright_black()),
            Event::SectionHeader(header) => {
                let key = header.to_string();

                print!(
                    "{}{}{}",
                    "[".yellow(),
                    key[1..key.len() - 1].to_string().bright_white(),
                    "]".yellow()
                );
            }
            Event::SectionValueName(name) => print!("{}", name.to_string().blue()),
            Event::KeyValueSeparator => print!("{}", "=".bright_white()),
            Event::Value(value) => print!("{}", value.to_string().bright_white()),
            Event::Whitespace(whitespace) => print!("{whitespace}"),
            Event::Newline(newline) => print!("{newline}"),
            Event::ValueNotDone(value) => print!("{}", value.to_string().bright_white()),
            Event::ValueDone(value) => print!("{}", value.to_string().bright_white()),
        })?;
    } else {
        println!("{} >\n{}", profile_name, profile.body)
    }

    Ok(())
}
