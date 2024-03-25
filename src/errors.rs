use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("File was not found: {0}")]
    IOError(std::io::Error, String),
    #[error("Error while Serailiazing and Deserializing data: {0}")]
    JSONError(serde_json::Error, String),
    #[error("Error while Serailiazing and Deserializing object data: {0}")]
    BincodeError(bincode::Error, String),
    #[error("Error while reading file and json: {0}")]
    JsonErrIOErr(serde_json::Error, std::io::Error, String),
    #[error("Invalid  Image ID")]
    InvalidImageID(String),
}

impl From<std::io::Error> for VerificationError {
    fn from(err: std::io::Error) -> Self {
        VerificationError::IOError(err, String::new())
    }
}

impl From<serde_json::Error> for VerificationError {
    fn from(err: serde_json::Error) -> Self {
        VerificationError::JSONError(err, String::new())
    }
}

impl From<bincode::Error> for VerificationError {
    fn from(err: bincode::Error) -> Self {
        VerificationError::BincodeError(err, String::new())
    }
}

impl From<(serde_json::Error, std::io::Error)> for VerificationError {
    fn from((json_err, io_err): (serde_json::Error, std::io::Error)) -> Self {
        VerificationError::JsonErrIOErr(json_err, io_err, String::new())
    }
}
