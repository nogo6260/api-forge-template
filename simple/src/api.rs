use crate::errors::Error;
use crate::errors::*;
use crate::options::Options;
use crate::restful::traits::*;
use crate::restful::Payload;
use crate::websocket::command::StreamCommand;
use crate::websocket::events::StreamEvent;
use crate::{restful, websocket};
use graceful_futures::Lifetime;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct {{project-name | upper_camel_case}} {
    client: restful::Client,
    pub lifetime: Arc<Lifetime>,
}

impl {{project-name | upper_camel_case}} {
    pub fn new(lifetime: Arc<Lifetime>, options: Options) -> Self {
        let options = Options { ..options };
        Self {
            client: restful::Client::new(options),
            lifetime,
        }
    }

    pub fn general<'a, P, R, S>(&'a self, data: P) -> S
    where
        //  payload
        P: GeneralRequest<'a, Payload = P, Response = R, Sender = S>
            + Serialize
            + Clone
            + Sync
            + Send,
        //  response
        R: DeserializeOwned,
        //  request
        S: Sender<Payload = P, Response = R>,
        for<'b> &'b S: TryInto<Payload<P>, Error = Error>,
    {
        P::new_request(&self.client, data)
    }

    pub async fn new_stream(
        &self,
        cmd: StreamCommand,
        event_channel: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<websocket::Client> {
        websocket::Client::new(self.lifetime.clone(), event_channel)
            .open()
            .await?
            .send_cmd(cmd)
    }
}
