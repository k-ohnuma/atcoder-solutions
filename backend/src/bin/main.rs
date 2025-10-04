use anyhow::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    init_logger()?;

    Ok(())
}

enum Environment {
    Dev,
    Stg,
    Prd,
}

fn which_env() -> Result<Environment> {
    let env = std::env::var("ENV")?;
    match env.as_str() {
        "dev" => Ok(Environment::Dev),
        "stg" => Ok(Environment::Stg),
        "prd" => Ok(Environment::Prd),
        _ => Ok(Environment::Dev),
    }
}

fn init_logger() -> Result<()> {
    let env = which_env()?;
    let log_level = match env {
        Environment::Dev => "debug",
        _ => "info",
    };
    let env_filter = tracing_subscriber::EnvFilter::from(log_level);

    let subscriber = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
        
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(ErrorLayer::default())
        .try_init()?;

    Ok(())
}
