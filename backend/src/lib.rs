use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Context, Result, anyhow};
use axum::{Router, http::StatusCode};
use interface::{
    handler::problem::import_problem,
    route::{
        contest::build_contests_routers, health::build_health_check_routers,
        problem::build_problem_routers, solution::build_solution_routers, user::build_user_routers,
        version::build_version_routers,
    },
};
use registry::Registry;
use shared::{
    config::AppConfig,
    env::{Environment, which_env},
};
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
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

    if matches!(env, Environment::Dev) {
        let subscriber = tracing_subscriber::fmt::layer()
            .pretty()
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(false);
        tracing_subscriber::registry()
            .with(subscriber)
            .with(env_filter)
            .with(ErrorLayer::default())
            .try_init()?;
    } else {
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
    }
    Ok(())
}

pub async fn run() -> Result<()> {
    let app_config = AppConfig::new()?;
    let registry = Registry::new(app_config);
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_version_routers())
        .merge(build_user_routers())
        .merge(build_problem_routers())
        .merge(build_solution_routers())
        .merge(build_contests_routers())
        .with_state(registry.to_owned())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        );

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    tokio::spawn({
        let reg = registry.to_owned();
        async move {
            loop {
                tracing::info!("execute to fetch problems");
                let _ = run_dayly_job(&reg).await;
                tracing::info!("see you tommorow!");
                tokio::time::sleep(std::time::Duration::from_secs(24 * 60 * 60)).await;
            }
        }
    });
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}

pub async fn run_dayly_job(reg: &Registry) -> Result<()> {
    match import_problem(reg).await {
        StatusCode::OK => Ok(()),
        _ => Err(anyhow!("daily fetch failed")),
    }?;

    Ok(())
}
