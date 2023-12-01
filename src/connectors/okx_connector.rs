use std::collections::HashMap;
use base64;
use hmac::{Hmac, Mac};
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use thiserror::Error;
use time::{error::Format, format_description::well_known::Rfc3339, OffsetDateTime};

type HmacSha256 = Hmac<Sha256>;

/// Reference: https://github.com/Nouzan/exc/tree/527479c6e10dd7118c481c8f848ecac40f4262f7/exc-okx

/// Http response of /api/v5/market/ticker
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketTickerResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<HashMap<String, String>>
}

/// Error type for signing.
#[derive(Debug, Error)]
pub enum SignError {
    /// Format timestamp error.
    #[error("format timestamp error: {0}")]
    FormatTimestamp(#[from] Format),

    /// Convert timestamp error.
    #[error("convert timestamp error: {0}")]
    ConvertTimestamp(#[from] time::error::ComponentRange),

    /// SecretKey length error.
    #[error("secret_key length error")]
    SecretKeyLength,
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

    pub fn sign(&self, method: &str, uri: &str, timestamp: OffsetDateTime) -> Result<Signature, SignError> {
        let timestamp = timestamp
            .replace_millisecond(timestamp.millisecond())
            .unwrap()
            .format(&Rfc3339)
            .expect("Failed to format time");

        let raw_sign = timestamp.clone() + method + uri;
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|_| SignError::SecretKeyLength).unwrap();
        mac.update(raw_sign.as_bytes());

        let encoded_sign = base64::encode(mac.finalize().into_bytes());

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
