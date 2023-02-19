mod color;
mod command;
mod config;
mod git;

use clap::Parser;
use colored::{ColoredString, Colorize};
use command::Commands;
use config::Profile;
use confy::ConfyError;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
fn main() -> Result<(), ConfyError> {
    let app_name = env!("CARGO_PKG_NAME");
    let mut cfg = confy::load::<config::Config>(app_name, None)?;
    let arg = command::Arg::parse();

    (|| match &arg.command {
        Commands::Config { edit } => {
            match confy::get_configuration_file_path(app_name, None) {
                Err(_) => {
                    println!(
                        "{} Could not obtain the path to the configuration file",
                        color::error()
                    );
                }
                Ok(config_file_path) => {
                    if !edit {
                        println!("{}", config_file_path.display());

                        return;
                    }

                    let result = edit::edit_file(&config_file_path);

                    if result.is_err() {
                        println!(
                            "{} Could not open editor\n\n {} The configuration file can be found here '{}'",
                            color::error(),
                            color::note(),
                            config_file_path.display()
                        );
                    }
                }
            };
        }
        Commands::Create {
            profile_name,
            name,
            email,
            signing_key,
            yes,
        } => {
            if !yes && cfg.profiles.contains_key(profile_name) {
                let answer = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("A profile with the specified name already exists, do you want to overwrite it?")
                    .default(false)
                    .interact()
                    .unwrap();

                if !answer {
                    return;
                }
            }

            cfg.profiles.insert(
                profile_name.to_owned(),
                Profile {
                    name: name.to_owned(),
                    email: email.to_owned(),
                    signing_key: signing_key.to_owned(),
                },
            );

            println!(
                "{} Created profile {}",
                color::successfully(),
                color::identifier(profile_name)
            );
        }
        Commands::List => {
            let profiles = cfg.get_profile_names();

            println!(
                "{}\n{}",
                "Profiles:".bold().underline(),
                if profiles.len() > 0 {
                    profiles
                        .iter()
                        .map(|s| format!("  {s}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                        .bold()
                } else {
                    ColoredString::from("  No profile yet")
                }
            );
        }
        Commands::Delete { profile_name, yes } => {
            let profile_name = cfg.select_profile_name(profile_name.as_ref());

            if profile_name.is_none() {
                println!("Not a single profile");

                return;
            }

            let profile_name = &profile_name.unwrap();

            if !yes {
                let answer = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Once erased, it cannot be reverted. Would you like to continue?")
                    .default(false)
                    .interact()
                    .unwrap();

                if !answer {
                    return;
                }
            }

            if !cfg.profiles.contains_key(profile_name) {
                println!("{} Non-existent profile name", color::error());

                return;
            }

            cfg.profiles.remove(profile_name);

            println!(
                "{} Deleted profile {}",
                color::successfully(),
                color::identifier(profile_name)
            );
        }
        Commands::Switch {
            scope,
            profile_name,
        } => {
            let profile_name = cfg.select_profile_name(profile_name.as_ref());

            if profile_name.is_none() {
                println!("Not a single profile");

                return;
            }

            let profile_name = &profile_name.unwrap();
            let profile = cfg.profiles.get(profile_name);

            match profile {
                Some(profile) => {
                    let mut git_config = git::GitConfig::new(scope);

                    git_config.set("user.name", Some(&profile.name));

                    if let Some(email) = &profile.email {
                        git_config.set("user.email", Some(email));
                    } else {
                        git_config.set("user.email", None);
                    }

                    if let Some(signing_key) = &profile.signing_key {
                        git_config.set("user.signingkey", Some(signing_key));
                        git_config.set("commit.gpgsign", Some("true"));
                    } else {
                        git_config.set("user.signingkey", None);
                        git_config.set("commit.gpgsign", Some("false"));
                    }

                    if !git_config.error {
                        println!(
                            "{} Switched to profile {}",
                            color::successfully(),
                            color::identifier(profile_name)
                        );
                    }
                }
                None => {
                    println!("{} Non-existent profile name", color::error());
                }
            }
        }
    })();

    Ok(())
}
