use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::utils::error::SignError;

type HmacSha256 = Hmac<Sha256>;

/// Reference: https://github.com/Nouzan/exc/tree/527479c6e10dd7118c481c8f848ecac40f4262f7/exc-okx
pub fn sign(raw_sign: String, secret_key: String) -> Result<String, SignError> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|_| SignError::SecretKeyLength).unwrap();
    mac.update(raw_sign.as_bytes());

    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}
