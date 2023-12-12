use http::HeaderMap;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::exchanges::signer::sign;
use crate::utils::error::SignError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
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

/// The APIKey definition of MEXC.
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MexcConnector {
    /// MEXC_API_KEY
    pub api_key: String,
    /// MEXC_SECRET_KEY
    pub secret_key: String
}

impl MexcConnector {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key
        }
    }

    pub fn sign(&self, uri: &str) -> Result<Signature, SignError> {
        let timestamp = OffsetDateTime::now_utc().millisecond().to_string();
        let raw_sign = uri.to_owned() + timestamp.clone().as_str();
        let encoded_sign = sign(raw_sign.clone(), self.secret_key.clone()).unwrap();
        let full_uri = raw_sign.clone() + "&signature=" + encoded_sign.as_str();

        Ok(Signature { signature: encoded_sign, timestamp, full_uri })
    }

    pub fn build_headers(&self) -> Result<HeaderMap, ()> {
        let mut headers = HeaderMap::new();
        headers.insert("X-MEXC-APIKEY", self.api_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        Ok(headers)
    }
}
