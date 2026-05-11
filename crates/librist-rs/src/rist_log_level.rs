use super::*;

pub enum LogLevel {
    Disable,
    Error,
    Warn,
    Notice,
    Info,
    Debug,
    Simulate,
}

/// Default value of rist_log_level is Disable.
/// See: `LOGGING_SETTINGS_INITIALIZER` in logging.h
impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Disable
    }
}

pub struct RistLogLevel(pub(crate) librist_sys::rist_log_level);

impl Into<RistLogLevel> for LogLevel {
    fn into(self) -> RistLogLevel {
        match self {
            LogLevel::Disable => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_DISABLE),
            LogLevel::Error => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_ERROR),
            LogLevel::Warn => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_WARN),
            LogLevel::Notice => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_NOTICE),
            LogLevel::Info => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_INFO),
            LogLevel::Debug => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_DEBUG),
            LogLevel::Simulate => RistLogLevel(librist_sys::rist_log_level_RIST_LOG_SIMULATE),
        }
    }
}

impl TryFrom<RistLogLevel> for LogLevel {
    type Error = UnknownEnumError;

    fn try_from(value: RistLogLevel) -> Result<Self, <LogLevel as TryFrom<RistLogLevel>>::Error> {
        match value.0 {
            librist_sys::rist_log_level_RIST_LOG_DISABLE => Ok(LogLevel::Disable),
            librist_sys::rist_log_level_RIST_LOG_ERROR => Ok(LogLevel::Error),
            librist_sys::rist_log_level_RIST_LOG_WARN => Ok(LogLevel::Warn),
            librist_sys::rist_log_level_RIST_LOG_NOTICE => Ok(LogLevel::Notice),
            librist_sys::rist_log_level_RIST_LOG_INFO => Ok(LogLevel::Info),
            librist_sys::rist_log_level_RIST_LOG_DEBUG => Ok(LogLevel::Debug),
            librist_sys::rist_log_level_RIST_LOG_SIMULATE => Ok(LogLevel::Simulate),
            _ => Err(UnknownEnumError {
                enum_type: "rist_log_level",
                value: value.0 as i32,
            }),
        }
    }
}
