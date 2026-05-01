mod cli;
mod model;
mod store;

use crate::{
    cli::{Cli, Command},
    store::TaskStore,
};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mut task_store = TaskStore::new();

    match cli.command {
        Command::Add {
            title,
            priority,
            due,
        } => {
            task_store.add_task(title, priority, due);
            println!("{task_store:#?}");
        }
    }
}
