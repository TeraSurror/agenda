mod cli;
mod error;
mod model;
mod store;

use crate::{
    cli::{Cli, Command},
    store::{add_task, complete_task, delete_task, list_tasks},
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
        Command::List { filter } => list_tasks(filter),
        Command::Complete { id } => complete_task(id),
        Command::Delete { id } => delete_task(id),
    }
}
