use anyhow::Result;
use backend::{init_logger, run};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    run().await?;
    Ok(())
}
