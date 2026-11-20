use crate::duckdb_state;
use std::{error, fmt};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
    InternalMalfunction,
    PermissionDenied,
    OperationAborted,
    DatabaseBusy,
    DatabaseLocked,
    OutOfMemory,
    ReadOnly,
    OperationInterrupted,
    SystemIoFailure,
    DatabaseCorrupt,
    NotFound,
    DiskFull,
    CannotOpen,
    FileLockingProtocolFailed,
    SchemaChanged,
    TooBig,
    ConstraintViolation,
    TypeMismatch,
    ApiMisuse,
    NoLargeFileSupport,
    AuthorizationForStatementDenied,
    ParameterOutOfRange,
    NotADatabase,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: ErrorCode,
    pub extended_code: duckdb_state,
}
impl Error {
    pub fn new(result_code: duckdb_state) -> Self {
        Self {
            code: ErrorCode::Unknown,
            extended_code: result_code,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error code {}: {}",
            self.extended_code,
            code_to_str(self.extended_code)
        )
    }
}
impl error::Error for Error {
    fn description(&self) -> &str {
        code_to_str(self.extended_code)
    }
}
pub fn code_to_str(_: duckdb_state) -> &'static str {
    "Unknown error code"
}
