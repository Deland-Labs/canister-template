use candid::{CandidType, Deserialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum CommonError {
    #[error("there is a unknown error raised")]
    Unknown,
    #[error("error from remote, {0:?}")]
    RemoteError(ErrorInfo),
    #[error("Unauthorized, please login first")]
    Unauthorized,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Length of {field:?} must be in range [{min:?}, {max:?})")]
    ValueShouldBeInRangeError {
        field: String,
        min: usize,
        max: usize,
    },
}

impl CommonError {
    pub(crate) fn code(&self) -> u32 {
        match self {
            CommonError::Unknown => 1,
            CommonError::RemoteError(_) => 2,
            CommonError::Unauthorized => 3,
            CommonError::PermissionDenied => 4,
            CommonError::ValueShouldBeInRangeError { .. } => 5,
        }
    }
}

/// Error information
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize)]
pub struct ErrorInfo {
    /// Error code
    pub code: u32,
    /// Error message
    pub message: String,
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

pub fn get_error_code(error: CommonError) -> ErrorInfo {
    ErrorInfo {
        code: error.code(),
        message: error.to_string(),
    }
}

pub type ServiceResult<T> = anyhow::Result<T, CommonError>;

/// A helper function to convert anyhow::Result<T, ICNSError> to ICNSResult<T>
pub type ActorResult<T> = Result<T, ErrorInfo>;

impl From<CommonError> for ErrorInfo {
    fn from(error: CommonError) -> Self {
        get_error_code(error)
    }
}

impl From<ErrorInfo> for CommonError {
    fn from(error: ErrorInfo) -> Self {
        CommonError::RemoteError(error)
    }
}

/// When export_service, actor responses will merged by enum type, so if there is two response with same Ok type, the second response will be ignored.
/// So there is no need to create more than one response type for two boolean ok.
#[derive(CandidType)]
pub enum BooleanActorResponse {
    Ok(bool),
    Err(ErrorInfo),
}

impl BooleanActorResponse {
    pub fn new(result: ServiceResult<bool>) -> BooleanActorResponse {
        match result {
            Ok(available) => BooleanActorResponse::Ok(available),
            Err(err) => BooleanActorResponse::Err(err.into()),
        }
    }
}