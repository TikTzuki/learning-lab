use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use futures_util::FutureExt;
use log::info;
use prometheus_client::{encoding::text::encode, registry::Registry};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

const METRICS_CONTENT_TYPE: &str = "application/openmetrics-text;charset=utf-8;version=1.0.0";

pub(crate) async fn metrics_server(registry: Registry, addr: SocketAddr) -> Result<(), std::io::Error> {
    let service = MetricService::new(registry);
    let server = Router::new()
        .route("/metrics", get(respond_with_metrics))
        .with_state(service);
    let tcp_listener = TcpListener::bind(addr).await
        .expect("Failed to bind metrics server address ");
    let local_addr = tcp_listener.local_addr()?;
    // tracing::info!(metrics_server=%format!("http://{}/metrics", local_addr));
    info!("Metrics server started at {}", local_addr);
    axum::serve(tcp_listener, server.into_make_service()).await
        .expect("Failed to start metrics server");
    Ok(())
}

#[derive(Clone)]
pub(crate) struct MetricService {
    reg: Arc<Mutex<Registry>>,
}

pub async fn respond_with_metrics(State(state): State<MetricService>) -> impl IntoResponse {
    let mut sink = String::new();
    let reg = state.get_reg();
    encode(&mut sink, &reg.lock().unwrap()).unwrap();
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, METRICS_CONTENT_TYPE)],
        sink,
    )
}

// pub async fn respond_with_metrics2(State(state): State<AppState>) -> impl IntoResponse {
//     let mut sink = String::new();
//     let it = &state.registry;
//     let it2 = &Arc::clone(it);
//     let reg = &it2.lock().unwrap();
//     encode(&mut sink, reg).unwrap();
//
//     (
//         StatusCode::OK,
//         [(axum::http::header::CONTENT_TYPE, METRICS_CONTENT_TYPE)],
//         sink,
//     )
// }

type SharedRegistry = Arc<Mutex<Registry>>;

impl MetricService {
    fn new(registry: Registry) -> Self {
        Self {
            reg: Arc::new(Mutex::new(registry)),
        }
    }

    fn get_reg(&self) -> SharedRegistry {
        Arc::clone(&self.reg)
    }
}