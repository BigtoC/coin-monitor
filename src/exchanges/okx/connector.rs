use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::exchanges::okx::dto::ApiResponse;
use crate::exchanges::signer::{sign, base64_encode};
use crate::utils::error::{HttpError, SignError};
use crate::utils::http_client::HttpClient;

/// The APIKey definition of OKX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OkxConnector {
    /// OK_API_KEY
    pub api_key: String,
    /// OK_SECRET
    pub secret_key: String,
    /// OK_PASSPHRASE
    pub passphrase: String,

    pub data_source: String
}

/// Signature
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    /// Signature.
    #[serde(rename = "sign")]
    pub signature: String,

    /// Timestamp.
    pub timestamp: String
}

impl OkxConnector {
    pub fn new(api_key: String, secret_key: String, passphrase: String) -> Self {
        let data_source =  "OKX".to_string();
        Self {
            api_key,
            secret_key,
            passphrase,
            data_source
        }
    }

    // Reference: https://www.okx.com/docs-v5/en/#overview-rest-authentication
    pub fn sign(&self, method: &str, uri: &str, timestamp: OffsetDateTime) -> Result<Signature, SignError> {
        let timestamp = timestamp
            .replace_millisecond(timestamp.millisecond())
            .map_err(|_| SignError::ConvertTimestamp)
            .unwrap()
            .format(&Rfc3339)
            .map_err(|_| SignError::FormatTimestamp).unwrap();

        let raw_sign = timestamp.clone() + method + uri;

        let raw_sign = sign(raw_sign, self.secret_key.clone()).unwrap();

        Ok(Signature { signature: base64_encode(raw_sign), timestamp })
    }

    pub fn build_headers(&self, signature: Signature) -> Result<HeaderMap, ()> {
        let mut headers = HeaderMap::new();
        headers.insert("OK-ACCESS-KEY", self.api_key.parse().unwrap());
        headers.insert("OK-ACCESS-SIGN", signature.signature.parse().unwrap());
        headers.insert("OK-ACCESS-TIMESTAMP", signature.timestamp.parse().unwrap());
        headers.insert("OK-ACCESS-PASSPHRASE", self.passphrase.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        Ok(headers)
    }

    pub async fn http_client<T>(&self, url: String, uri: String) -> Result<Vec<T>, HttpError>
        where
            T: serde::de::DeserializeOwned + Clone {
        let data_source = self.data_source.clone();
        let timestamp = OffsetDateTime::now_utc();
        let signature = self.sign("GET", &*uri.clone(), timestamp).unwrap();
        let headers = self.build_headers(signature).unwrap();

        let client = HttpClient::new(data_source.clone());
        let response = client.send_request(url, uri, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<ApiResponse<T>>()
                .await
                .expect(&*format!("[{}] Failed to deserialize response", data_source.clone()));

            if parsed_response.code == "0" {
                Ok(parsed_response.data)
            } else {
                eprintln!("[{}] {:?}", data_source.clone(), parsed_response.msg);
                Err(HttpError::ResponseDataError)
            }
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        }
    }
}
