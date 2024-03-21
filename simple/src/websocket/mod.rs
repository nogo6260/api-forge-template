use crate::errors::*;
use crate::websocket::events::StreamEvent;
use graceful_futures::Lifetime;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub mod command;
pub mod events;
pub mod models;

mod client;

pub use client::*;

pub fn send_event(
    events_channel: &mpsc::UnboundedSender<StreamEvent>,
    lifetime: Arc<Lifetime>,
    id: Uuid,
    event: StreamEvent,
) -> Result<()> {
    match events_channel.send(event) {
        Ok(_) => Ok(()),
        Err(error) => {
            let msg = format!("Unable to send exchange event in {}: {}", id, error);
            tracing::error!("{}", msg);
            lifetime.spawn_graceful_shutdown(&msg);
            Err(Error::SendError("websocket".to_string(), msg))
        }
    }
}
