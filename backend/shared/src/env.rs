pub enum Environment {
    Dev,
    Stg,
    Prd,
}

pub fn parse_env(env: &str) -> Environment {
    match env {
        "dev" => Environment::Dev,
        "stg" => Environment::Stg,
        "prd" => Environment::Prd,
        _ => Environment::Dev,
    }
}
