use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Context, Result};
use axum::Router;
use interface::route::{health::build_health_check_routers, version::build_version_routers};
use shared::{
    config::AppConfig,
    env::{Environment, which_env},
};
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() -> Result<()> {
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

pub async fn run() -> Result<()> {
    let app_config = AppConfig::new()?;
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_version_routers())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}
