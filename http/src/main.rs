#![feature(hash_drain_filter)]
#![feature(result_option_inspect)]

use std::{env, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    body::{Body, BoxBody},
    http::{Request, Response},
    routing::{delete, get, patch, post},
    Router, ServiceExt,
};
use tower::Layer;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::Span;

use crate::{settings::HSettings, state::HInnerAppState};

mod bridge;
mod cmd;
mod handlers;
mod info;
mod logging;
mod settings;
mod state;
mod util;

#[tokio::main]
async fn main() {
    // Settings
    let config_path = env::args().nth(1);
    let settings = HSettings::new(config_path).unwrap();
    // Logging
    let _log_guard = logging::setup(settings.log.folder, &settings.log.level, settings.log.rotate);
    // Shared state
    let state = Arc::new(HInnerAppState::new(settings.cache.folder));

    // Cleanup task
    let state_cleanup = state.clone();
    tokio::spawn(async move {
        state_cleanup
            .ss_mgr
            .periodic_cleanup(settings.server.solsys_cleanup_interval, settings.server.solsys_lifetime)
            .await
    });

    // HTTP routing
    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/", get(handlers::root))
            .route("/source/:alias", post(handlers::create_source))
            .route("/source/:alias", delete(handlers::delete_source))
            .route("/solar_system", post(handlers::create_ss))
            .route("/solar_system/:ss_id", get(handlers::get_ss))
            .route("/solar_system/:ss_id", patch(handlers::change_ss))
            .route("/solar_system/:ss_id", delete(handlers::delete_ss))
            .route("/solar_system/:ss_id/fit", post(handlers::create_fit))
            .route("/solar_system/:ss_id/fit/:fit_id", get(handlers::get_fit))
            .route("/solar_system/:ss_id/fit/:fit_id", patch(handlers::change_fit))
            .route("/solar_system/:ss_id/fit/:fit_id", delete(handlers::delete_fit))
            .route("/solar_system/:ss_id/item", post(handlers::create_item))
            .route("/solar_system/:ss_id/item/:item_id", get(handlers::get_item))
            .route("/solar_system/:ss_id/item/:item_id", patch(handlers::change_item))
            .route("/solar_system/:ss_id/item/:item_id", delete(handlers::delete_item))
            .route("/solar_system/:ss_id/fleet", post(handlers::create_fleet))
            .route("/solar_system/:ss_id/fleet/:fleet_id", get(handlers::get_fleet))
            .route("/solar_system/:ss_id/fleet/:fleet_id", patch(handlers::change_fleet))
            .route("/solar_system/:ss_id/fleet/:fleet_id", delete(handlers::delete_fleet))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_: &Request<Body>| tracing::trace_span!("http"))
                    .on_request(|request: &Request<Body>, _span: &Span| {
                        tracing::debug!("started {} {}", request.method(), request.uri().path())
                    })
                    .on_response(|response: &Response<BoxBody>, latency: Duration, _span: &Span| {
                        tracing::debug!("response {} generated in {:?}", response.status(), latency)
                    }),
            )
            .with_state(state),
    );

    // Run app
    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
