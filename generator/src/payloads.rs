use forge_marco::Forge;
use crate::{forge_config, Code};

forge_config! {
    #[derive(Forge)]
    #[config(path = "/api/v1/assets", method = "GET")]
    #[config(response= "models::Assets")]
    pub struct Asset;
}