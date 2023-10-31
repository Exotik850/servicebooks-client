use quick_oxibooks::{error::APIError, types::QBError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceBooksError {
    #[error(transparent)]
    TauriError(#[from] tauri::Error),
    #[error(transparent)]
    TokioIOError(#[from] tokio::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    SMTPError(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    QBError(#[from] QBError),
    #[error(transparent)]
    QBAPIError(#[from] APIError),
    #[error(transparent)]
    SPError(#[from] service_poxi::error::ServicePoxiError),
    #[error("Error(s) on {0} claim from Service Power: {1}")]
    ServicePowerClaimError(&'static str, String),
    #[error("Missing item when updating customer memo {0}")]
    MemoUpdateMissingItem(&'static str),
    #[error("ServicePower Claim missing claim identifier")]
    MissingClaimIdentifier,
    #[error("Couldn't parse {0} date: {1}")]
    DateParseError(&'static str, String),
    #[error("Could not parse phone number: {0}")]
    PhoneNumberParseError(String),
    #[error("Could not parse doc number: {0}")]
    DocNumberParseError(String),
    #[error("No claims in ServicePower Claim response")]
    EmptyClaimResponse,
    #[error("Could not grab lock on Quickbooks Manager")]
    QBLockError,
    #[error("{0} Manager not initialized")]
    UninitError(&'static str),
    #[error("Missing window for label: {0}")]
    MissingWindow(&'static str),
}

impl serde::Serialize for ServiceBooksError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
