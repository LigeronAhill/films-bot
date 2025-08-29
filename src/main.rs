use anyhow::Result;
mod app;

#[tokio::main]
async fn main() -> Result<()> {
    app::run().await?;
    Ok(())
}
