use anyhow::Result;
use backend::{init_logger, run};
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = AppConfig::new()?;
    init_logger(&app_config)?;
    run(app_config).await?;
    Ok(())
}
