use std::future::Future;

use async_trait::async_trait;
use chrono::Utc;
use common::restful::{Client as RestClient, Endpoint, RestResponse};
use common::restful::{FeedData, RequestError};
use common::restful::traits::*;
use hyper::body::Bytes;
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper::StatusCode;
use serde::Serialize;

use crate::errors::Error;
use crate::options::Options;
use crate::restful::ExtraType;

type Result<T> = core::result::Result<T, Error>;


pub struct Client {
    pub options: Options,
    client: RestClient,
}

impl Client {
    pub fn new(options: Options) -> Self {
        let client = RestClient::new();
        Self { options, client }
    }

    fn get_timestamp() -> u64 {
        Utc::now().timestamp_millis() as u64
    }

    fn create_signature_data(query: String, window: Option<ExtraType>) -> String {
        todo!()
    }

    fn create_signature(api_secret: String, data: String, instruction: &'static str) -> String {
        todo!()
    }
}

#[async_trait]
impl<'a> FetchResponse for Client {
    type Error = Error;
    type Extra = ExtraType;

    async fn fetch<PayloadType>(
        &self,
        feed_data: FeedData<PayloadType, Self::Extra>,
    ) -> Result<RestResponse>
    where
        PayloadType: Serialize + Send + Clone,
    {
        let FeedData {
            endpoint:
                Endpoint {
                    method,
                    path,
                    private,
                },
            data,
            extra,
            action_name,
        } = feed_data;

        let path_and_query = String::default();
        let body: Option<Bytes> = None;

        let uri = RestClient::create_uri(&self.options.host, path_and_query);
        let res = self
            .client
            .request(action_name, method, uri, body, |builder| {
                builder
                    .header(USER_AGENT, "tradix-rust")
                    .header(CONTENT_TYPE, "application/json; charset=utf-8")
            })
            .await?;

        Ok(res)
    }

    async fn response_handler<T, Fut>(
        &self,
        response: RestResponse,
        parser: impl FnOnce(String) -> Fut + Send,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Fut: Future<Output = std::result::Result<T, RequestError>> + Send,
    {
        let status = response.status;

        match status {
            StatusCode::OK => Ok(parser(response.content)
                .await
                .map_err(|e| Error::ParseResponse(e.to_string()))?),
            // add additional status codes
            s => Err(Error::Msg(format!("Received response: {s:?}"))),
        }
    }
}
