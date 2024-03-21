#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    #[default]
    Subscribe,
    Unsubscribe,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommand {
    pub method: Method,
    pub params: Vec<String>,
}

impl StreamCommand {
    pub fn new(method: Method) -> Self {
        Self {
            method,
            params: vec![],
        }
    }
    pub fn some_channel<T: Into<String>>(mut self, param: T) -> Self {
        self.params.push(param.into());
        self
    }
}
