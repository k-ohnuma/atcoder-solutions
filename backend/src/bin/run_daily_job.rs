use anyhow::Result;
use backend::{init_logger, run_daily_job};
use registry::Registry;
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = AppConfig::new()?;
    init_logger(&app_config)?;
    let registry = Registry::new(app_config);

    run_daily_job(&registry).await?;
    Ok(())
}
