use crate::errors::Error;
use crate::errors::*;
use crate::options::Options;
use crate::restful::ExtraType;
use crate::websocket::command::StreamCommand;
use crate::websocket::events::StreamEvent;
use crate::{restful, websocket};
use common::restful::traits::{GeneralRequest, Sender};
use common::restful::FeedData;
use graceful_futures::Lifetime;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc;

// {{project-name | upper_camel_case}}
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

    pub fn general<'a, PayloadType, Response, MySender>(&'a self, data: PayloadType) -> MySender
        where
        //  payload
        PayloadType: GeneralRequest<
            'a,
            PayloadType,
            Response,
            Error = Error,
            Extra = ExtraType,
            Client = restful::Client,
            Sender = MySender,
        >,
        PayloadType: Serialize + Clone + Sync + Send,
        //  response
        Response: DeserializeOwned,
        //  request
        MySender: Sender<
            PayloadType,
            Response,
            Error = Error,
            Extra = ExtraType,
            Client = restful::Client,
        >,
        for<'b> &'b MySender: TryInto<FeedData<PayloadType, ExtraType>, Error = Error>,
    {
        PayloadType::new_request(&self.client, data)
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
