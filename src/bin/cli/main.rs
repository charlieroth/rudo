use crate::cli::Cli;

fn main() {
    let cli = Cli::new();
    cli.run().unwrap();
}
