use crate::store::Store;
use clap::{Parser, Subcommand};

pub mod handlers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add", about = "Add a new task")]
    Add { title: String },
    #[command(name = "list", about = "List all tasks")]
    List,
    #[command(name = "delete", about = "Delete a task")]
    Delete { id: String },
    #[command(name = "do", about = "Mark a task as done")]
    Do { id: String },
    #[command(name = "undo", about = "Mark a task as undone")]
    Undo { id: String },
}

pub struct Cli {
    store: Store,
    args: Args,
}

impl Cli {
    pub fn new() -> Self {
        let db_path = "db.json";
        let mut store = Store::new(db_path).unwrap();
        if std::path::Path::new(db_path).exists() {
            store.load().unwrap();
        }

        let args = Args::parse();
        Self { args, store }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        match &self.args.command {
            Commands::Add { title } => handlers::add(&mut self.store, title),
            Commands::List => handlers::list(&mut self.store),
            Commands::Do { id } => {
                let id = id.parse::<u64>()?;
                handlers::do_task(&mut self.store, id)
            }
            Commands::Undo { id } => {
                let id = id.parse::<u64>()?;
                handlers::undo_task(&mut self.store, id)
            }
            Commands::Delete { id } => {
                let id = id.parse::<u64>()?;
                handlers::delete_task(&mut self.store, id)
            }
        }
    }
}
