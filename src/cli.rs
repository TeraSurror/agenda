use clap::{Parser, Subcommand};

use crate::model::Priority;

#[derive(Parser)]
#[command(version, about = "A CLI task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        #[arg(short, long)]
        title: String,

        #[arg(short, long, default_value = "medium")]
        priority: Priority,

        #[arg(short, long)]
        due: Option<String>,
    },
}
