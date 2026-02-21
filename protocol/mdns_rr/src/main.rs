mod message;
mod settings;
mod metrics;
mod app;
mod utils;
pub mod http;
pub mod kafka;
pub mod ws;
mod swarm;

use crate::app::App;
use env_logger::{Builder, Env};
use futures_util::StreamExt;
use libp2p::swarm::NetworkBehaviour;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
    let app: App = App::new().await?;
    app.run().await;

    Ok(())
}
