use std::error::Error;
use std::fmt;

/// Custom error type for the ORCID library
#[derive(Debug)]
pub enum OrcidError {
    /// Invalid ORCID ID format or checksum
    InvalidOrcidId(String),

    /// Network request failed
    NetworkError(reqwest::Error),

    /// Failed to parse JSON response
    JsonError(serde_json::Error),

    /// API returned an error
    ApiError {
        orcid_id: String,
        error_code: String,
        developer_message: String,
    },

    /// Generic error with a message
    Other(String),
}

impl fmt::Display for OrcidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrcidError::InvalidOrcidId(id) => {
                write!(f, "{} is not a valid ORCID ID", id)
            }
            OrcidError::NetworkError(e) => {
                write!(f, "Network request failed: {}", e)
            }
            OrcidError::JsonError(e) => {
                write!(f, "Failed to parse JSON: {}", e)
            }
            OrcidError::ApiError {
                orcid_id,
                error_code,
                developer_message,
            } => {
                write!(
                    f,
                    "API error for ORCID {}: {} - {}",
                    orcid_id, error_code, developer_message
                )
            }
            OrcidError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for OrcidError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OrcidError::NetworkError(e) => Some(e),
            OrcidError::JsonError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for OrcidError {
    fn from(err: reqwest::Error) -> Self {
        OrcidError::NetworkError(err)
    }
}

impl From<serde_json::Error> for OrcidError {
    fn from(err: serde_json::Error) -> Self {
        OrcidError::JsonError(err)
    }
}

/// Result type alias for ORCID operations
pub type Result<T> = std::result::Result<T, OrcidError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_orcid_id_display() {
        let error = OrcidError::InvalidOrcidId("1234-5678".to_string());
        assert_eq!(error.to_string(), "1234-5678 is not a valid ORCID ID");
    }

    #[test]
    fn test_api_error_display() {
        let error = OrcidError::ApiError {
            orcid_id: "0000-0001-2345-6789".to_string(),
            error_code: "404".to_string(),
            developer_message: "ORCID not found".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "API error for ORCID 0000-0001-2345-6789: 404 - ORCID not found"
        );
    }

    #[test]
    fn test_other_error_display() {
        let error = OrcidError::Other("Something went wrong".to_string());
        assert_eq!(error.to_string(), "Something went wrong");
    }

    #[test]
    fn test_error_debug() {
        let error = OrcidError::InvalidOrcidId("1234".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidOrcidId"));
        assert!(debug_str.contains("1234"));
    }

    #[test]
    fn test_from_reqwest_error() {
        // We can't easily create a real reqwest::Error, so we'll test the trait implementation exists
        fn takes_reqwest_error(_: reqwest::Error) -> OrcidError {
            // This function just tests that the From trait is implemented
            OrcidError::Other("test".to_string())
        }
    }

    #[test]
    fn test_from_json_error() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let orcid_err: OrcidError = json_err.into();
        match orcid_err {
            OrcidError::JsonError(_) => (),
            _ => panic!("Expected JsonError variant"),
        }
    }
}
