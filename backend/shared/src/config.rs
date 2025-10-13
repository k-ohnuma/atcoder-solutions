use anyhow::Result;

pub struct AppConfig {
    pub atcoder_problems: AtcoderProblemsConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let atcoder_problems = AtcoderProblemsConfig {
            base_endpoint: std::env::var("ATCODER_PROBLEMS_BASE_ENDPOINT")?,
        };
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };
        let auth = AuthConfig {
            project_id: std::env::var("FIREBASE_PROJECT_ID")?,
        };

        Ok(Self {
            atcoder_problems,
            database,
            auth,
        })
    }
}

pub struct AtcoderProblemsConfig {
    pub base_endpoint: String,
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct AuthConfig {
    pub project_id: String,
}
