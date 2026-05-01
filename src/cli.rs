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
    /// Add a new task
    Add {
        /// Task Description
        #[arg(short, long)]
        title: String,

        /// Priority Level
        #[arg(short, long, default_value = "medium")]
        priority: Priority,

        /// Task due date
        #[arg(short, long)]
        due: Option<String>,
    },

    /// List all the task (with an optional filter)
    List {
        #[arg(short, long)]
        filter: Option<String>,
    },
}
