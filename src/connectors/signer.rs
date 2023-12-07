use hmac::{Hmac, Mac};
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Reference: https://github.com/Nouzan/exc/tree/527479c6e10dd7118c481c8f848ecac40f4262f7/exc-okx

/// Error type for signing.
#[derive(Debug, Error)]
pub enum SignError {
    /// Format timestamp error.
    #[error("format timestamp error")]
    FormatTimestamp,

    /// Convert timestamp error.
    #[error("convert timestamp error")]
    ConvertTimestamp,

    /// SecretKey length error.
    #[error("secret_key length error")]
    SecretKeyLength,
}

pub fn sign(raw_sign: String, secret_key: String) -> Result<String, SignError> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|_| SignError::SecretKeyLength).unwrap();
    mac.update(raw_sign.as_bytes());

    Ok(base64::encode(mac.finalize().into_bytes()))
}
