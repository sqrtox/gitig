use anyhow::Error;
use std::process::Command;

#[derive(Debug, Clone)]
pub enum GitConfigScopes {
    Local,
    Global,
    System,
}

pub struct GitConfig {
    pub error: bool,
    pub scope: GitConfigScopes,
}

impl GitConfig {
    pub fn new(scope: &GitConfigScopes) -> Self {
        Self {
            error: false,
            scope: scope.clone(),
        }
    }

    pub fn set(&mut self, name: &str, value: Option<&str>) -> bool {
        if self.error {
            return false;
        }

        let mut command = Command::new("git");
        let scope = self.scope.to_string();

        command.arg("config");
        command.arg(format!("--{scope}"));

        match value {
            Some(value) => {
                command.arg(name);
                command.arg(value);
            }
            None => {
                command.arg("--unset");
                command.arg(name);
            }
        }

        let output = command.output().unwrap();

        if !output.stderr.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stderr));

            self.error = true;

            return false;
        }

        true
    }
}

impl ToString for GitConfigScopes {
    fn to_string(&self) -> String {
        match self {
            Self::Local => format!("local"),
            Self::Global => format!("global"),
            Self::System => format!("system"),
        }
    }
}

impl std::str::FromStr for GitConfigScopes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scope = if s == "system" {
            Self::System
        } else if s == "global" {
            Self::Global
        } else {
            Self::Local
        };

        Ok(scope)
    }
}
