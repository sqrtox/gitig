use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Arg {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Import settings from another configuration file")]
    Import {
        #[clap(help = "Path of the configuration file you want to import")]
        path: String,
    },
    #[clap(about = "Displays the location of the configuration file")]
    Config {
        #[clap(short, long, help = "Open in editor")]
        edit: bool,
    },
    #[clap(about = "Create or overwrite a new profile")]
    Create {
        #[clap(help = "Profile to be applied")]
        profile_name: String,
        #[clap(help = "git username")]
        name: String,
        #[clap(short, long, help = "git email address")]
        email: Option<String>,
        #[clap(short, long, help = "signing key")]
        signing_key: Option<String>,
        #[clap(short, long, help = "Approve all confirmations")]
        yes: bool,
    },
    #[clap(about = "View a list of profiles you have created")]
    List,
    #[clap(about = "Delete a profile")]
    Delete {
        #[clap(help = "Profile to be applied")]
        profile_name: Option<String>,
        #[clap(short, long, help = "Approve all confirmations")]
        yes: bool,
    },
    #[clap(about = "Apply the profile to the current repository")]
    Switch {
        #[clap(
            short,
            long,
            default_value = "local",
            help = "Range to which settings are applied"
        )]
        scope: crate::git::GitConfigScopes,
        #[clap(help = "Profile to be applied")]
        profile_name: Option<String>,
    },
}
