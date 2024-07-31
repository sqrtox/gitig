use crate::db::Database;
use crate::error::Result;
use colored::{ColoredString, Colorize};

pub fn list(filter: Option<String>) -> Result<()> {
    let db = Database::connect()?;

    let profiles = if let Some(filter) = filter {
        db.select_profiles_by_filter(&filter)?
    } else {
        db.select_profiles()?
    };

    println!(
        "{}\n{}",
        "Profiles:".bold().underline(),
        if profiles.is_empty() {
            ColoredString::from("  No profile yet")
        } else {
            profiles
                .iter()
                .map(|p| format!("  {}", p.name))
                .collect::<Vec<_>>()
                .join("\n")
                .bold()
        }
    );

    Ok(())
}
