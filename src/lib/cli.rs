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
    args: Args,
}

impl Cli {
    pub fn new() -> Self {
        let args = Args::parse();
        Self { args }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        match &self.args.command {
            Commands::Add { title } => handlers::add(title),
            Commands::List => handlers::list(),
            Commands::Do { id } => {
                let id = id.parse::<u64>()?;
                handlers::do_task(id)
            }
            Commands::Undo { id } => {
                let id = id.parse::<u64>()?;
                handlers::undo_task(id)
            }
            Commands::Delete { id } => {
                let id = id.parse::<u64>()?;
                handlers::delete_task(id)
            }
        }
    }
}
