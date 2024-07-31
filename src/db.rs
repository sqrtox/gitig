use crate::error::{Error, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use platform_dirs::AppDirs;
use rusqlite::{params, Connection, Params, Statement};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub body: String,
}

pub fn resolve_profile_name(profile_name: Option<String>) -> Result<String> {
    if let Some(profile_name) = profile_name {
        return Ok(profile_name);
    }

    let db = Database::connect()?;

    let profile_names = db
        .select_profiles()?
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>();

    if profile_names.is_empty() {
        return Err(Error::NoProfilesToChoose);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile")
        .items(&profile_names)
        .default(0)
        .interact()?;

    Ok(profile_names[selection].clone())
}

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn connect() -> Result<Self> {
        let db_url = AppDirs::new(Some(APP_NAME), true)
            .ok_or(Error::AppDirsNotDetected)
            .map(|app_dirs| app_dirs.data_dir.join("profile.db"))?;
        let connection = Connection::open(db_url)?;

        connection.execute(
            "
            CREATE TABLE IF NOT EXISTS profile (
                id INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                body TEXT NOT NULL
            );
        ",
            (),
        )?;

        Ok(Self { connection })
    }

    fn query_map_profiles<P: Params>(params: P, stmt: &mut Statement) -> Result<Vec<Profile>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Profile {
                    name: row.get(0)?,
                    body: row.get(1)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn select_profile_by_name(&self, name: &str) -> Result<Option<Profile>> {
        Ok(Self::query_map_profiles(
            params![name],
            &mut (self
                .connection
                .prepare("SELECT name, body FROM profile WHERE name = (?1)")?),
        )?
        .first()
        .cloned())
    }

    pub fn select_profiles_by_filter(&self, filter: &str) -> Result<Vec<Profile>> {
        Self::query_map_profiles(
            // TODO: Is this really safe...?
            params![format!("%{filter}%")],
            &mut (self
                .connection
                .prepare("SELECT name, body FROM profile WHERE name LIKE (?1)")?),
        )
    }

    pub fn select_profiles(&self) -> Result<Vec<Profile>> {
        Self::query_map_profiles(
            (),
            &mut (self.connection.prepare("SELECT name, body FROM profile")?),
        )
    }

    pub fn delete_profile_by_name(&self, name: &str) -> Result<()> {
        self.connection
            .execute("DELETE FROM profile WHERE name = (?1)", params![name])?;

        Ok(())
    }

    pub fn update_profile_body(&self, profile: &Profile) -> Result<()> {
        self.connection.execute(
            "UPDATE profile SET body = (?2) WHERE name = (?1)",
            params![profile.name, profile.body],
        )?;

        Ok(())
    }

    pub fn insert_profile(&self, profile: &Profile) -> Result<()> {
        self.connection.execute(
            "INSERT INTO profile (name, body) VALUES (?1, ?2)",
            params![profile.name, profile.body],
        )?;

        Ok(())
    }
}
