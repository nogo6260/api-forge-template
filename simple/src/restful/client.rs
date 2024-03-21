use crate::errors::*;
use crate::options::Options;
use crate::restful::traits::FetchResponse;
use async_trait::async_trait;
use common::restful::Client as BaseClient;
use reqwest::header::HeaderMap;
use reqwest::{Response, StatusCode};
use serde::Serialize;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct Payload<T>
where
    T: Serialize + Send + Clone,
{
    pub endpoint: Endpoint,
    pub data: Option<T>,
    pub extra: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Endpoint {
    pub path: String,
    pub method: reqwest::Method,
    pub private: bool,
}

impl Endpoint {
    pub fn new<T>(method: reqwest::Method, path: T, private: bool) -> Self
    where
        T: AsRef<str>,
    {
        Endpoint {
            path: path.as_ref().to_string(),
            method,
            private,
        }
    }
}

pub struct Client {
    pub options: Options,
    client: BaseClient,
}

impl Client {
    pub fn new(options: Options) -> Self {
        // default headers
        let header = HeaderMap::new();
        let client = BaseClient::new(header, options.timeout.clone());
        Self { options, client }
    }
}

#[async_trait]
impl<'a> FetchResponse for Client {
    async fn fetch<P>(&self, payload: Payload<P>) -> Result<Response>
    where
        P: serde::ser::Serialize + Send + Clone,
    {
        let Payload {
            endpoint,
            data,
            extra,
        } = payload.into();

        let method = endpoint.method.clone();
        // complete the request url
        let url = String::default();
        // complete the request headers
        let headers: HeaderMap = HeaderMap::default();
        // select one of the query/form/json request data types
        let resp = self.client.send(method, url, Some(headers)).await?;
        //let resp = self.client.send_form(method,url,data,Some(headers)).await?;
        //let resp = self.client.send_json(method,url,data,Some(headers)).await?;
        Ok(resp)
    }

    async fn response_handler<T, Fut>(
        &self,
        response: Response,
        parser: impl FnOnce(Response) -> Fut + Send,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Fut: Future<Output = Result<T>> + Send,
    {
        match response.status() {
            StatusCode::OK => Ok(parser(response)
                .await
                .map_err(|e| Error::ParseResponse(e.to_string()))?),
            s => Err(Error::Msg(format!("Received response: {s:?}"))),
        }
    }
}
