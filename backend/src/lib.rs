use std::net::SocketAddr;

use anyhow::{Context, Result, anyhow};
use axum::{
    Router,
    http::{Request, StatusCode},
};
use interface::{
    handler::problem::import_problem,
    route::{
        contest::build_contests_routers, health::build_health_check_routers,
        problem::build_problem_routers, solution::build_solution_routers, user::build_user_routers,
        version::build_version_routers,
    },
};
use registry::Registry;
use shared::config::AppConfig;
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger(app_config: &AppConfig) -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::new(app_config.log.rust_log.clone());

    let subscriber = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(ErrorLayer::default())
        .try_init()?;
    Ok(())
}

pub async fn run(app_config: AppConfig) -> Result<()> {
    let addr = format!("{}:{}", app_config.server.host, app_config.server.port)
        .parse::<SocketAddr>()
        .context("failed to parse bind address from HOST/PORT")?;
    let registry = Registry::new(app_config);
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_version_routers())
        .merge(build_user_routers())
        .merge(build_problem_routers())
        .merge(build_solution_routers())
        .merge(build_contests_routers())
        .with_state(registry.to_owned())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = request
                        .extensions()
                        .get::<RequestId>()
                        .and_then(|v| v.header_value().to_str().ok())
                        .or_else(|| {
                            request
                                .headers()
                                .get("x-request-id")
                                .and_then(|v| v.to_str().ok())
                        })
                        .unwrap_or("");
                    let content_length = request
                        .headers()
                        .get("content-length")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");
                    let user_agent = request
                        .headers()
                        .get("user-agent")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");
                    tracing::span!(
                        Level::INFO,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        request_id = %request_id,
                        content_length = %content_length,
                        user_agent = %user_agent
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));

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

pub async fn run_daily_job(reg: &Registry) -> Result<()> {
    match import_problem(reg).await {
        StatusCode::OK => Ok(()),
        _ => Err(anyhow!("daily fetch failed")),
    }?;

    Ok(())
}
