use crate::errors::*;
use crate::restful::{Client, Payload};
use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;

pub trait GeneralRequest<'a>
where
    for<'b> &'b Self::Sender: TryInto<Payload<Self::Payload>, Error = Error>,
{
    type Payload: Serialize + Send + Sync + Clone;
    type Response: DeserializeOwned;
    type Sender: Sender<Payload = Self::Payload, Response = Self::Response>;
    fn new_request(client: &'a Client, data: Self::Payload) -> Self::Sender;
}

#[async_trait]
pub trait FetchResponse {
    async fn fetch<P>(&self, payload: Payload<P>) -> Result<Response>
    where
        P: Serialize + Send + Clone;

    async fn response_handler<T, Fut>(
        &self,
        response: Response,
        parser: impl FnOnce(Response) -> Fut + Send,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Fut: Future<Output = Result<T>> + Send;

    async fn fetch_with_parser<T, P>(
        &self,
        payload: Payload<P>,
        parser: impl FnOnce(String) -> Result<T> + Send,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize + Send + Sync + Clone,
    {
        let response = self.fetch(payload).await?;
        self.response_handler(response, |data| async { parser(data.text().await?) })
            .await
    }
}

#[async_trait]
pub trait Sender: Sized
where
    for<'a> &'a Self: TryInto<Payload<Self::Payload>, Error = Error>,
{
    type Payload: Serialize + Send + Sync + Clone;
    type Response: DeserializeOwned;
    fn client(&self) -> &Client;

    async fn send(self) -> Result<Self::Response>;

    async fn send_with_type<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let payload = (&self).try_into()?;
        let resp = self
            .client()
            .fetch_with_parser(payload, |text| Ok(serde_json::from_str(&text)?))
            .await?;
        Ok(resp)
    }
    async fn send_with_parser<T>(self, parser: impl FnOnce(String) -> Result<T> + Send) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let payload = (&self).try_into()?;
        let resp = self.client().fetch_with_parser(payload, parser).await?;
        Ok(resp)
    }
}
