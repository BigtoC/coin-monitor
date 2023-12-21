use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::exchanges::signer::{sign, hex_encode};
use crate::utils::error::{HttpError, SignError};
use crate::utils::http_client::HttpClient;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    /// Signature.
    pub signature: String,

    /// Timestamp.
    pub timestamp: String,

    /// URI with timestamp and signature
    pub full_uri: String,
}

/// The APIKey definition of HashKey.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HashKeyConnector {
    /// HASHKEY_API_KEY
    pub api_key: String,
    /// HASHKEY_SECRET_KEY
    pub secret_key: String,

    pub data_source: String
}

impl HashKeyConnector {
    pub fn new(api_key: String, secret_key: String) -> Self {
        let data_source =  "HashKey".to_string();
        Self { api_key, secret_key, data_source }
    }

    pub fn sign(&self, uri: String, parameters: String) -> Result<Signature, SignError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis().to_string();
        let total_parameters: String = if parameters.is_empty() {
            "timestamp=".to_string() + timestamp.clone().as_str()
        } else {
            parameters.clone() + "&timestamp=" + timestamp.clone().as_str()
        };

        let raw_sign = sign(total_parameters.clone(), self.secret_key.clone())
            .expect(&*format!("Failed to create {} signature", self.data_source.clone()));

        let encoded_sign = hex_encode(raw_sign);

        let full_uri = uri.clone() + "?" + total_parameters.clone().as_str() + "&signature=" + encoded_sign.as_str();

        Ok(Signature { signature: encoded_sign, timestamp, full_uri })
    }

    pub fn build_headers(&self) -> Result<HeaderMap, ()> {
        let mut headers = HeaderMap::new();
        headers.insert("X-HK-APIKEY", self.api_key.parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        Ok(headers)
    }

    pub async fn http_client<T>(&self, url: String, uri: String, parameters: String) -> Result<T, HttpError>
        where
            T: serde::de::DeserializeOwned + Clone {
        let data_source = self.data_source.clone();
        let signature = self.sign(uri.clone(), parameters.clone()).unwrap();
        let headers = self.build_headers().unwrap();

        let client = HttpClient::new(data_source.clone());
        let uri_with_sign = signature.full_uri;
        let response = client.send_request(url, uri_with_sign, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<T>()
                .await
                .expect(&*format!("[{}] Failed to deserialize response", data_source.clone()));
            Ok(parsed_response.clone())
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        }
    }
}
