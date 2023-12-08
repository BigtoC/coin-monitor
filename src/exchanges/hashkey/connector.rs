use http::HeaderMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::exchanges::signer::{sign, SignError};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolPriceTicker {
    pub s: String, // Symbol
    pub p: String, // Price
}

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
struct HashKeyConnector {
    /// HASHKEY_API_KEY
    pub api_key: String,
    /// HASHKEY_SECRET_KEY
    pub secret_key: String
}

impl HashKeyConnector {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self { api_key, secret_key }
    }

    pub fn sign(&self, uri: &str) -> Result<crate::exchanges::mexc::connector::Signature, SignError> {
        let timestamp = OffsetDateTime::now_utc().millisecond().to_string();
        let raw_sign = uri.to_owned() + timestamp.clone().as_str();
        let encoded_sign = sign(raw_sign.clone(), self.secret_key.clone()).unwrap();
        let full_uri = raw_sign.clone() + "&signature=" + encoded_sign.as_str();

        Ok(crate::exchanges::mexc::connector::Signature { signature: encoded_sign, timestamp, full_uri })
    }

    pub fn build_headers(&self) -> Result<HeaderMap, ()> {
        let mut headers = HeaderMap::new();
        headers.insert("X-HK-APIKEY", self.api_key.parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        Ok(headers)
    }
}
