use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[clap(about = "Create or edit a profile")]
    Edit {
        #[clap(help = "Name of the profile to create or edit (if omitted, select from list)")]
        profile_name: Option<String>,
    },
    #[clap(
        alias = "s",
        about = "Apply the profile to the current repository (alias: `s`)"
    )]
    Switch {
        #[clap(help = "Name of the profile to be applied (if omitted, select from list)")]
        profile_name: Option<String>,
        #[clap(
            help = "Removes settings included in the profile from the current repository settings",
            short,
            long
        )]
        unset: bool,
    },
    #[clap(about = "Displays a list of profiles")]
    List {
        #[clap(help = "Filter results by partial match", short, long)]
        filter: Option<String>,
    },
    #[clap(about = "Delete specified profile")]
    Delete {
        #[clap(help = "Name of the profile to be deleted (if omitted, select from list)")]
        profile_name: Option<String>,
    },
}
