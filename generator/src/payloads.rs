use crate::{restify_config, Code};
use restify_marco::Restify;

restify_config! {
    #[derive(Restify)]
    #[config(path = "/api/v1/assets", method = "GET")]
    #[config(response= "models::Assets")]
    pub struct Asset;
}