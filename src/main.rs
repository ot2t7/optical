use anyhow::Result;

mod listener;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    listener::start().await?;
    return Ok(());
}
