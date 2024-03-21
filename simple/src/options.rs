#[derive(Debug, Clone)]
pub struct Options {
    pub host: String,
    pub timeout: Option<u64>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            host: "https://api.simple.com".into(),
            timeout: None,
        }
    }
}
