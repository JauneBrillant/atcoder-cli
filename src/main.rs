use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login {},
    New { contest_number: String },
    Test { problem_alphabet: char },
    Submit { problem_alphabet: char },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login {} => commands::login::execute(),
        Commands::New { contest_number } => commands::new::execute(contest_number),
        Commands::Test { problem_alphabet } => commands::test::execute(problem_alphabet),
        Commands::Submit { problem_alphabet } => commands::submit::execute(problem_alphabet),
    }
}
