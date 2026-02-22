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
        let app_database_url = std::env::var("APP_DATABASE_URL").ok();
        let database = if app_database_url.is_some() {
            DatabaseConfig {
                app_database_url,
                host: std::env::var("DATABASE_HOST").unwrap_or_default(),
                port: std::env::var("DATABASE_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(5432),
                username: std::env::var("DATABASE_USERNAME").unwrap_or_default(),
                password: std::env::var("DATABASE_PASSWORD").unwrap_or_default(),
                database: std::env::var("DATABASE_NAME").unwrap_or_default(),
            }
        } else {
            DatabaseConfig {
                app_database_url: None,
                host: std::env::var("DATABASE_HOST")?,
                port: std::env::var("DATABASE_PORT")?.parse()?,
                username: std::env::var("DATABASE_USERNAME")?,
                password: std::env::var("DATABASE_PASSWORD")?,
                database: std::env::var("DATABASE_NAME")?,
            }
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
    pub app_database_url: Option<String>,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct AuthConfig {
    pub project_id: String,
}
