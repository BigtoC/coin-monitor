use reqwest::{
    Client,
    header::HeaderMap
};
use crate::utils::error::HttpError;

pub struct HttpClient {
    client: Client,
    data_source: String
}

impl HttpClient {
    pub fn new(data_source: String) -> Self {
        let client = Client::new();
        Self {
            client,
            data_source
        }
    }

    pub async fn send_request(
        &self, url: String, uri: String, headers: HeaderMap
    ) -> Result<reqwest::Response, HttpError> {
        self.client
            .get(url.clone() + &*uri.clone())
            .headers(headers)
            .send()
            .await
            .map_err(|error| {
                eprintln!("[{} error] {}", self.data_source, error);
                HttpError::RequestError
            })
    }
}
