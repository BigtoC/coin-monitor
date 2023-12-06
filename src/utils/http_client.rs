use reqwest::{
    Client,
    header::HeaderMap
};

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
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let response = match self.client
            .get(url.clone() + &*uri.clone())
            .headers(headers)
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                panic!("[{}] Failed to get response from {url}/{uri}, error: {:?}", self.data_source, error);
            }
        };

        Ok(response)
    }
}
