use shared::config::DatabaseConfig;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::str::FromStr;

fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    if let Some(url) = &cfg.app_database_url {
        return PgConnectOptions::from_str(url)
            .unwrap_or_else(|e| panic!("invalid APP_DATABASE_URL: {e}"));
    }

    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .password(&cfg.password)
        .username(&cfg.username)
        .database(&cfg.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        ConnectionPool(pool)
    }
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
