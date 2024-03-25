use std::sync::Arc;

use anyhow::bail;
use graceful_futures::Lifetime;
use tokio::sync::mpsc;
use uuid::Uuid;

pub use client::*;

use crate::websocket::events::StreamEvent;

pub mod command;
pub mod events;
pub mod models;

mod client;

pub fn send_event(
    events_channel: &mpsc::UnboundedSender<StreamEvent>,
    lifetime: Arc<Lifetime>,
    id: Uuid,
    event: StreamEvent,
) -> anyhow::Result<()> {
    match events_channel.send(event) {
        Ok(_) => Ok(()),
        Err(error) => {
            let msg = format!("Unable to send exchange event in {}: {}", id, error);
            tracing::error!("{}", msg);
            lifetime.spawn_graceful_shutdown(&msg);
            bail!(msg)
        }
    }
}
