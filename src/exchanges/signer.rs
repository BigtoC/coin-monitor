use base64::{Engine as _, engine::general_purpose};
use ethers::utils::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::utils::error::SignError;

/// Reference: https://github.com/Nouzan/exc/tree/527479c6e10dd7118c481c8f848ecac40f4262f7/exc-okx
pub fn sign(raw_sign: String, secret_key: String) -> Result<Hmac::<Sha256>, SignError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .map_err(|_| SignError::SecretKeyLength).unwrap();
    mac.update(raw_sign.as_bytes());

    Ok(mac)
}

pub fn base64_encode(raw: Hmac::<Sha256>) -> String {
    return general_purpose::STANDARD.encode(raw.finalize().into_bytes())
}

pub fn hex_encode(raw: Hmac::<Sha256>) -> String {
    return hex::encode(raw.finalize().into_bytes())
}
