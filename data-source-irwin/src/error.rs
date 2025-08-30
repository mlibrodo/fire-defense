use std::fmt;

/// Custom error types for the IRWIN client
#[derive(Debug)]
pub enum IrwinError {
    /// HTTP client errors
    HttpClientError(reqwest::Error),
    
    /// API request errors
    RequestError(String),
    
    /// API response errors
    ApiError {
        status: u16,
        message: String,
    },
    
    /// Serialization/deserialization errors
    SerializationError(serde_json::Error),
    
    /// URL parsing errors
    UrlError(url::ParseError),
    
    /// Header value errors
    HeaderError(reqwest::header::InvalidHeaderValue),
}

impl fmt::Display for IrwinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrwinError::HttpClientError(err) => write!(f, "HTTP client error: {}", err),
            IrwinError::RequestError(msg) => write!(f, "Request error: {}", msg),
            IrwinError::ApiError { status, message } => write!(f, "API error ({}): {}", status, message),
            IrwinError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            IrwinError::UrlError(err) => write!(f, "URL error: {}", err),
            IrwinError::HeaderError(err) => write!(f, "Header error: {}", err),
        }
    }
}

impl std::error::Error for IrwinError {}

impl From<reqwest::Error> for IrwinError {
    fn from(err: reqwest::Error) -> Self {
        IrwinError::HttpClientError(err)
    }
}

impl From<serde_json::Error> for IrwinError {
    fn from(err: serde_json::Error) -> Self {
        IrwinError::SerializationError(err)
    }
}

impl From<url::ParseError> for IrwinError {
    fn from(err: url::ParseError) -> Self {
        IrwinError::UrlError(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for IrwinError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        IrwinError::HeaderError(err)
    }
}
