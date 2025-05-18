use clap::{Args, Parser, Subcommand}; // Bring clap derive macros into scope [oai_citation_attribution:0‡docs.rs](https://docs.rs/clap/latest/clap/#:~:text=use%20clap%3A%3AParser%3B)

/// Define the command-line interface using clap derive macros
#[derive(Parser)]
#[command(
    name = "devano",
    author = "Mackenzie Bowes",
    version = "1.0.0",
    about = "Fullstack Devano CLI",
    long_about = "Built by Mackenzie Bowes, https://mackenziebowes.com",
    propagate_version = true    // Propagate version to subcommands
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "ui", about = "UI-related tooling")]
    Ui,

    #[command(name = "add", about = "Internal Tooling for installing deps")]
    Add(AddArgs),

    #[command(name = "feat", about = "Add a feature")]
    Feature,

    #[command(name = "new", about = "Create a new Devano monorepo")]
    New,
}

#[derive(Args)]
pub struct AddArgs {
    pub value: String,
}
