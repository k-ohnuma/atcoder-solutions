use anyhow::Result;

pub enum Environment {
    Dev,
    Stg,
    Prd,
}

pub fn which_env() -> Result<Environment> {
    let env = std::env::var("ENV")?;
    match env.as_str() {
        "dev" => Ok(Environment::Dev),
        "stg" => Ok(Environment::Stg),
        "prd" => Ok(Environment::Prd),
        _ => Ok(Environment::Dev),
    }
}
