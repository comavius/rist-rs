use super::*;

/// `RistLoggingSettingsBuilder` is a builder for creating `RistLoggingSettings` instances,
/// which wrap `rist_logging_settings`.
#[derive(Clone, Default)]
pub struct LoggingSettingsBuilder<CBLogger = StderrCallBackLogger>
where
    CBLogger: CallBackLogger,
{
    pub log_level: LogLebel,
    pub logger: LoggerBuilder<CBLogger>,
}

/// `RistLoggingSettings` is a wrapper for `rist_logging_settings`.
///
/// ### Safety(Internal)
/// Every wrapper types depending on the returned value of `RistLoggingSettings::as_mut_ptr`
/// must own the `RistLoggingSettings` instance. Otherwise, `rist_logging_settings` and/or
/// its logger may be invalidated.
pub struct LoggingSettings<CBLogger = StderrCallBackLogger>
where
    CBLogger: CallBackLogger,
{
    pinned: std::pin::Pin<Box<LoggingSettingsPinned>>,
    rist_logger: Logger<CBLogger>,
}

impl<CBLogger> LoggingSettings<CBLogger>
where
    CBLogger: CallBackLogger,
{
    /// Returns a mutable pointer to the `rist_logging_settings` instance.
    ///
    /// # Safety
    /// The returned pointer is only valid for the lifetime of the `RistLoggingSettings` instance.
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut rist_rs_sys::rist_logging_settings {
        unsafe {
            &mut std::pin::Pin::as_mut(&mut self.pinned)
                .get_unchecked_mut()
                .raw
        }
    }
}

struct LoggingSettingsPinned {
    raw: rist_rs_sys::rist_logging_settings,
    _pin: std::marker::PhantomPinned,
}

impl<CBLogger> LoggingSettingsBuilder<CBLogger>
where
    CBLogger: CallBackLogger,
{
    pub fn initialize(&self) -> LoggingSettings<CBLogger> {
        let log_level = (&self.log_level).into();
        let rist_logger = self.logger.initialize();
        let log_cb = self.logger.log_cb();
        let log_cb_arg = unsafe { rist_logger.as_log_cb_arg() };
        let log_socket = rist_logger.log_socket();
        let log_stream = rist_logger.log_stream();
        let inner = rist_rs_sys::rist_logging_settings {
            log_level,
            log_cb,
            log_cb_arg,
            log_socket,
            log_stream,
        };
        LoggingSettings {
            pinned: Box::pin(LoggingSettingsPinned {
                raw: inner,
                _pin: std::marker::PhantomPinned,
            }),
            rist_logger,
        }
    }
}
