mod cli;
mod common;
mod config;
mod login;
mod logout;
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    env_logger::Builder::new()
    .filter_level(log::LevelFilter::Info)
    .parse_default_env()
    .init();

    cli::run().await?;

    Ok(())
}
