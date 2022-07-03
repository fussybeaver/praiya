#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HttpClientError {
        #[from]
        err: http::Error,
    },
    #[error(transparent)]
    HyperResponseError {
        #[from]
        err: hyper::Error,
    },
    #[error("Pagerduty API responded with status code {status_code}, application code {app_code}, message: {message}")]
    PraiyaResponseServerError {
        status_code: u16,
        app_code: i16,
        message: String,
    },
    #[error("Failed to deserialize JSON at column {column}: {message}")]
    JsonDataError { message: String, column: usize },
    #[error("Timeout error")]
    RequestTimeoutError,
    #[error("Oversized payload error")]
    OversizedPayloadError { len: u64 },
    #[error(transparent)]
    JsonDeserializeError {
        #[from]
        err: serde_json::Error,
    },
    #[error(transparent)]
    IOError {
        #[from]
        err: std::io::Error,
    },
    #[error(transparent)]
    SerdeQSError {
        #[from]
        err: serde_qs::Error,
    },
    #[error(transparent)]
    URLParseError {
        #[from]
        err: url::ParseError,
    },
    #[error(transparent)]
    ToStrError {
        #[from]
        err: http::header::ToStrError,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ConflictInner {
    pub message: Option<String>,
    pub code: Option<i16>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct Conflict {
    pub error: Option<ConflictInner>,
}
