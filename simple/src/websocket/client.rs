use crate::errors::*;
use crate::websocket::command::StreamCommand;
use crate::websocket::events::StreamEvent;
use crate::websocket::{command, send_event};
use async_trait::async_trait;
use graceful_futures::Lifetime;
use reqwest::Url;
use restify::stream;
use restify::stream::ClientSupport;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct Client {
    lifetime: Arc<Lifetime>,
    pub subscribe_cmd: StreamCommand,
    internal: Arc<stream::Client>,
    id: Uuid,
}

impl Client {
    pub fn new(
        lifetime: Arc<Lifetime>,
        events_channel: mpsc::UnboundedSender<StreamEvent>,
    ) -> Client {
        let id = Uuid::new_v4();
        let support = Box::new(StreamSupport {
            events_channel,
            lifetime: lifetime.clone(),
            id: id.clone(),
        });
        let internal = stream::Client::new(id.clone(), support, lifetime.clone());
        Self {
            lifetime,
            subscribe_cmd: StreamCommand::default(),
            internal,
            id,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub async fn when_cancelled(&self) {
        self.lifetime
            .futures_cancellation_token
            .when_cancelled()
            .await
    }

    pub async fn open(self) -> crate::errors::Result<Client> {
        self.internal.connect_ws().await?;
        Ok(self)
    }

    pub async fn close(self) -> crate::errors::Result<Client> {
        self.internal.disconnect_ws().await;
        Ok(self)
    }

    pub async fn resend(&self) -> crate::errors::Result<()> {
        if self.subscribe_cmd.params.len() > 0 {
            let msg = serde_json::to_string(&self.subscribe_cmd)?;
            self.internal.send_message(msg)?;
            Ok(())
        } else {
            self.internal.disconnect_ws().await;
            Err(crate::errors::Error::Msg(
                "no subscriptions to resend".to_string(),
            ))
        }
    }

    pub fn send_cmd(mut self, cmd: StreamCommand) -> crate::errors::Result<Client> {
        match cmd.method {
            command::Method::Subscribe => {
                let mut set: HashSet<_> = self
                    .subscribe_cmd
                    .params
                    .into_iter()
                    .chain(cmd.params.into_iter())
                    .collect();

                self.subscribe_cmd.params = set.drain().collect();
            }
            command::Method::Unsubscribe => {
                let mut params = self.subscribe_cmd.params.clone();
                for item in cmd.params {
                    if let Some(index) = params.iter().position(|v| v == &item) {
                        params.remove(index);
                    }
                }
                self.subscribe_cmd.params = params;
            }
        }

        let msg = serde_json::to_string(&self.subscribe_cmd)?;
        self.internal.send_message(msg)?;

        Ok(self)
    }
}

pub struct StreamSupport {
    pub(super) events_channel: mpsc::UnboundedSender<StreamEvent>,
    lifetime: Arc<Lifetime>,
    id: Uuid,
}

impl StreamSupport {
    fn send(&self, event: StreamEvent) -> Result<()> {
        send_event(&self.events_channel, self.lifetime.clone(), self.id, event)
    }
}

#[async_trait]
impl ClientSupport for StreamSupport {
    fn on_connecting(&self) -> anyhow::Result<()> {
        self.send(StreamEvent::OnConnecting(self.id.to_string()))
            .map_err(Into::into)
    }

    fn on_connected(&self) -> anyhow::Result<()> {
        self.send(StreamEvent::OnConnected(self.id.to_string()))
            .map_err(Into::into)
    }

    fn on_disconnected(&self) -> anyhow::Result<()> {
        self.send(StreamEvent::OnDisconnected(self.id.to_string()))
            .map_err(Into::into)
    }

    fn on_message(&self, msg: &str) -> anyhow::Result<()> {
        serde_json::from_str(msg).map_err(Into::into)
    }

    async fn create_url(&self) -> anyhow::Result<Url> {
        Ok(Url::parse("wss://ws.simple.com")?)
    }
}
