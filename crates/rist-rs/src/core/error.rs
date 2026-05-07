use crate::*;

#[derive(Debug, Clone)]
pub struct UnknownEnumVariantError {
    pub message: &'static str,
}

#[derive(Debug, Clone)]
pub struct InvalidPointerError {
    pub message: &'static str,
}

#[derive(Debug, Clone)]
pub struct InternalError {
    pub message: &'static str,
}

#[derive(Debug, Clone)]
pub struct InvalidFlowId {
    pub message: &'static str,
}

#[derive(Debug, Clone)]
pub struct LibRistUnlikelyError {
    pub function: &'static str,
}

#[derive(Debug, Clone)]
pub enum LibRistError {
    MallocError,
    NullPeerError,
    InvalidStringLengthError,
    InvalidProfileError,
    MissingCallbackFunctionError,
    NullCredentialsError,
    UnknownError,
}

impl From<i32> for LibRistError {
    fn from(value: i32) -> Self {
        match value {
            rist_rs_sys::RIST_ERR_MALLOC => LibRistError::MallocError,
            rist_rs_sys::RIST_ERR_NULL_PEER => LibRistError::NullPeerError,
            rist_rs_sys::RIST_ERR_INVALID_STRING_LENGTH => LibRistError::InvalidStringLengthError,
            rist_rs_sys::RIST_ERR_INVALID_PROFILE => LibRistError::InvalidProfileError,
            rist_rs_sys::RIST_ERR_MISSING_CALLBACK_FUNCTION => {
                LibRistError::MissingCallbackFunctionError
            }
            rist_rs_sys::RIST_ERR_NULL_CREDENTIALS => LibRistError::NullCredentialsError,
            _ => LibRistError::UnknownError,
        }
    }
}
