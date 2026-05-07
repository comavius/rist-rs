use super::*;

#[derive(Default, Copy, Clone)]
pub enum LogLebel {
    #[default]
    Disable = 0,
    Error = 1,
    Warn = 2,
    Notice = 3,
    Info = 4,
    Debug = 5,
    Simulate = 6,
}

impl LogLebel {
    pub fn is_important_than_or_equal_to(&self, other: &LogLebel) -> bool {
        *self as u32 >= *other as u32
    }
}

impl std::fmt::Display for LogLebel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLebel::Disable => write!(f, "Disable"),
            LogLebel::Error => write!(f, "Error"),
            LogLebel::Warn => write!(f, "Warn"),
            LogLebel::Notice => write!(f, "Notice"),
            LogLebel::Info => write!(f, "Info"),
            LogLebel::Debug => write!(f, "Debug"),
            LogLebel::Simulate => write!(f, "Simulate"),
        }
    }
}

impl Into<rist_rs_sys::rist_log_level> for &LogLebel {
    fn into(self) -> rist_rs_sys::rist_log_level {
        match self {
            LogLebel::Disable => rist_rs_sys::rist_log_level_RIST_LOG_DISABLE,
            LogLebel::Error => rist_rs_sys::rist_log_level_RIST_LOG_ERROR,
            LogLebel::Warn => rist_rs_sys::rist_log_level_RIST_LOG_WARN,
            LogLebel::Notice => rist_rs_sys::rist_log_level_RIST_LOG_NOTICE,
            LogLebel::Info => rist_rs_sys::rist_log_level_RIST_LOG_INFO,
            LogLebel::Debug => rist_rs_sys::rist_log_level_RIST_LOG_DEBUG,
            LogLebel::Simulate => rist_rs_sys::rist_log_level_RIST_LOG_SIMULATE,
        }
    }
}

impl TryFrom<rist_rs_sys::rist_log_level> for LogLebel {
    type Error = UnknownEnumVariantError;

    fn try_from(raw: rist_rs_sys::rist_log_level) -> Result<Self, UnknownEnumVariantError> {
        match raw {
            rist_rs_sys::rist_log_level_RIST_LOG_DISABLE => Ok(LogLebel::Disable),
            rist_rs_sys::rist_log_level_RIST_LOG_ERROR => Ok(LogLebel::Error),
            rist_rs_sys::rist_log_level_RIST_LOG_WARN => Ok(LogLebel::Warn),
            rist_rs_sys::rist_log_level_RIST_LOG_NOTICE => Ok(LogLebel::Notice),
            rist_rs_sys::rist_log_level_RIST_LOG_INFO => Ok(LogLebel::Info),
            rist_rs_sys::rist_log_level_RIST_LOG_DEBUG => Ok(LogLebel::Debug),
            rist_rs_sys::rist_log_level_RIST_LOG_SIMULATE => Ok(LogLebel::Simulate),
            _ => Err(UnknownEnumVariantError {
                message: "Unknown rist_log_level variant",
            }),
        }
    }
}
