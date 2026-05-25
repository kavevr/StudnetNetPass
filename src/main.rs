mod cli;
mod common;
mod config;
mod login;
mod logout;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await?;

    Ok(())
}
