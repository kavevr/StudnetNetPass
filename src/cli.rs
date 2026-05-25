use clap::{Parser, Subcommand};

use crate::login;
use crate::logout;

#[derive(Parser, Debug)]
#[command(author, version, about = "")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// login
    Login,
    /// logout
    Logout,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => login::login().await?,
        Commands::Logout => logout::logout().await?,
    }
    Ok(())
}
