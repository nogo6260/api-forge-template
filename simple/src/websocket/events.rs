use super::models::*;

#[derive(Deserialize)]
pub struct Message {
    //some message attributes
    pub data: DataEvent,
}

/// [DataEvent] is the event type for the message data.
#[derive(Debug, Clone, Deserialize)]
pub enum DataEvent {
    // some event
}

/// [StreamEvent] is the event type for the stream.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    OnConnecting(String),
    OnConnected(String),
    OnDisconnected(String),
    OnMessage(DataEvent),
}
