use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { contest_number: String },
    Test { problem_number: String },
    Submit { problem_number: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { contest_number } => create_project(contest_number),
        Commands::Test { problem_number } => println!("test {}", problem_number),
        Commands::Submit { problem_number } => println!("submit {}", problem_number),
    }
}

fn create_project(contest_number: &str) {
    let project_dir = Path::new(contest_number);

    if project_dir.exists() {
        println!("Directory already exists: {}", project_dir.display());
    } else {
        if let Err(err) = create_cargo_project(project_dir) {
            eprintln!("Failed to create Cargo project: {}", err);
        }
    }
}

fn create_cargo_project(project_dir: &Path) -> std::io::Result<()> {
    Command::new("cargo").arg("new").arg(project_dir).status()?;
    Ok(())
}
