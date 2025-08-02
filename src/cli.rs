use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "why",
    version = "0.1.0 (01-08-2025)",
    about = "An educational Programming language",
    override_usage = "why <file>",
)]

pub struct Cli {
    pub file: Option<String>,
}
