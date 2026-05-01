mod cli;
mod error;
mod model;
mod store;

use crate::{
    cli::{Cli, Command},
    store::add_task,
};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Add {
            title,
            priority,
            due,
        } => add_task(title, priority, due),
    }
}
