use thiserror::Error;

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

/// Error type for http request.
#[derive(Debug, Error, Clone)]
pub enum HttpError {
    #[error("Error when processing a request")]
    RequestError,

    #[error("Response status is not success")]
    ResponseError,
    
    #[error("Response data doesn't match usable patten")]
    ResponseDataError
}
