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
    #[clap(about = "View profile contents (alias: `v`)", alias = "v")]
    View {
        #[clap(
            help = "Name of the profile whose contents you wish to view (if omitted, select from list)"
        )]
        profile_name: Option<String>,
    },
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
            short,
            long,
            help = "Removes settings included in the profile from the current repository settings"
        )]
        unset: bool,
    },
    #[clap(about = "Displays a list of profiles (alias: `l`)", alias = "l")]
    List {
        #[clap(short, long, help = "Filter results by partial match")]
        filter: Option<String>,
    },
    #[clap(about = "Delete specified profile (aliases: `del`, `remove`, `rm`)", aliases = ["del", "remove", "rm"])]
    Delete {
        #[clap(help = "Name of the profile to be deleted (if omitted, select from list)")]
        profile_name: Option<String>,
        #[clap(short, long, help = "Approve all confirmations")]
        yes: bool,
    },
}
