use clap::{Parser, Subcommand};
mod commands;
use commands::{login, new, submit, test};

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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login {} => login::execute().await?,
        Commands::New { contest_number } => new::execute(contest_number).await?,
        Commands::Test { problem_alphabet } => test::execute(problem_alphabet).await?,
        Commands::Submit { problem_alphabet } => submit::execute(problem_alphabet).await?,
    }

    Ok(())
}
