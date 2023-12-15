use reqwest::header::HeaderMap;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::exchanges::signer::sign;
use crate::utils::error::SignError;

/// Http response of /api/v5/market/ticker
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketTickerResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<HashMap<String, String>>
}

/// The APIKey definition of OKX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OkxConnector {
    /// OK_API_KEY
    pub api_key: String,
    /// OK_SECRET
    pub secret_key: String,
    /// OK_PASSPHRASE
    pub passphrase: String,
}

/// Signature
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    /// Signature.
    #[serde(rename = "sign")]
    pub signature: String,

    /// Timestamp.
    pub timestamp: String,
}

impl OkxConnector {
    pub fn new(api_key: String, secret_key: String, passphrase: String) -> Self {
        Self {
            api_key,
            secret_key,
            passphrase
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

        let encoded_sign = sign(raw_sign, self.secret_key.clone()).unwrap();

        Ok(Signature { signature: encoded_sign, timestamp })
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
}
