use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub email: Option<String>,
    pub signing_key: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde_as(as = "Vec<(_, _)>")]
    pub profiles: HashMap<String, Profile>,
}

impl Config {
    pub fn get_profile_names(&self) -> Vec<String> {
        self.profiles
            .keys()
            .sorted()
            .map(|v| v.clone())
            .collect::<Vec<_>>()
    }

    pub fn select_profile_name(&self, prefer: Option<&String>) -> Option<String> {
        if let Some(prefer) = prefer {
            return Some(prefer.clone());
        }

        let profile_names = self.get_profile_names();

        if profile_names.len() < 1 {
            return None;
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select profile")
            .items(&profile_names)
            .default(0)
            .interact()
            .unwrap();

        Some(profile_names[selection].clone())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }
}
