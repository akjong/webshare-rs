const DEFAULT_HOST: &str = "https://proxy.webshare.io";

#[derive(Default, Debug, Clone)]
pub struct WebShareClient {
    pub client: reqwest::Client,
    pub host: String,
    pub token: String,
}

impl WebShareClient {
    pub fn new(token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            host: DEFAULT_HOST.to_string(),
            token: token.to_string(),
        }
    }

    pub fn request_builder(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.host, path))
            .header("Authorization", &self.token)
    }
}
