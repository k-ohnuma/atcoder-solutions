use anyhow::Result;

pub struct AppConfig {
    pub atcoder_problems: AtcoderProblemsConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub server: ServerConfig,
    pub log: LogConfig,
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
        let server = ServerConfig {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
        };
        let log = LogConfig {
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        };

        Ok(Self {
            atcoder_problems,
            database,
            auth,
            server,
            log,
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

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct LogConfig {
    pub rust_log: String,
}
