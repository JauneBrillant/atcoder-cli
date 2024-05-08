use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::Write;
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

        if let Err(err) = create_src_files(project_dir) {
            eprintln!("Failed to create source files: {}", err);
        }
    }
}

fn create_cargo_project(project_dir: &Path) -> std::io::Result<()> {
    Command::new("cargo").arg("new").arg(project_dir).status()?;
    Ok(())
}

fn create_src_files(project_dir: &Path) -> std::io::Result<()> {
    let src_dir = project_dir.join("src");

    fs::remove_file(src_dir.join("main.rs"))?;

    for c in b'a'..b'h' {
        let filename = format!("{}.rs", c as char);
        let filepath = src_dir.join(filename);

        let mut file = File::create(filepath)?;
        file.write_all(b"fn main() {\n    println!(\"Hello, world!\");\n}\n")?;
    }

    Ok(())
}
