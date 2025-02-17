use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title } => {
            println!("Adding task: {}", title);
        }
        Commands::List => {
            println!("Listing all tasks");
        }
        Commands::Do { id } => {
            println!("Marking task as done: {}", id);
        }
        Commands::Undo { id } => {
            println!("Marking task as undone: {}", id);
        }
        Commands::Delete { id } => {
            println!("Deleting task: {}", id);
        }
    }
}
