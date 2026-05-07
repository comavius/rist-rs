use super::*;

/// A builder for creating [`Logger`] instances.
///
/// TODO: `log_stream` and `log_socket` mode is not supported for now.
#[derive(Default)]
pub enum LoggerBuilder<CBLogger = StderrCallBackLogger>
where
    CBLogger: CallBackLogger,
{
    CallBack(std::sync::Arc<CBLogger>),
    #[default]
    NoLogging,
}

impl<CBLogger> Clone for LoggerBuilder<CBLogger>
where
    CBLogger: CallBackLogger,
{
    fn clone(&self) -> Self {
        match self {
            LoggerBuilder::CallBack(logger) => LoggerBuilder::CallBack(logger.clone()),
            LoggerBuilder::NoLogging => LoggerBuilder::NoLogging,
        }
    }
}

impl<CBLogger> LoggerBuilder<CBLogger>
where
    CBLogger: CallBackLogger,
{
    pub(crate) fn initialize(&self) -> Logger<CBLogger> {
        match self {
            LoggerBuilder::CallBack(logger) => Logger {
                log_cb_arg: Some(std::sync::Arc::into_raw(logger.clone()) as *const CBLogger),
                _phantom: std::marker::PhantomData,
            },
            LoggerBuilder::NoLogging => Logger {
                log_cb_arg: None,
                _phantom: std::marker::PhantomData,
            },
        }
    }

    /// Returns `log_cb` function.
    pub(crate) fn log_cb(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            *mut std::os::raw::c_void,
            rist_rs_sys::rist_log_level,
            *const ::std::os::raw::c_char,
        ) -> i32,
    > {
        match self {
            LoggerBuilder::CallBack(_) => Some(CBLogger::call_raw),
            LoggerBuilder::NoLogging => None,
        }
    }
}

/// [`Logger`] is a wrapper for managing logging functionality.
///
/// ### Safety(Internal)
/// Drop of this instance means that logger is no longer available.
pub struct Logger<CBLogger = StderrCallBackLogger>
where
    CBLogger: CallBackLogger,
{
    log_cb_arg: Option<*const CBLogger>,
    _phantom: std::marker::PhantomData<CBLogger>,
}

impl<CBLogger> Drop for Logger<CBLogger>
where
    CBLogger: CallBackLogger,
{
    fn drop(&mut self) {
        if let Some(ptr) = self.log_cb_arg {
            unsafe {
                std::sync::Arc::from_raw(ptr as *const CBLogger);
            }
        }
    }
}

impl<CBLogger> Logger<CBLogger>
where
    CBLogger: CallBackLogger,
{
    /// Returns a mutable pointer to the log callback argument.
    ///
    /// # Safety
    /// The returned pointer must not be used after the [`Logger`] instance is dropped.
    pub(crate) unsafe fn as_log_cb_arg(&self) -> *mut std::os::raw::c_void {
        self.log_cb_arg.unwrap_or(std::ptr::null_mut()) as *mut std::os::raw::c_void
    }

    pub(crate) fn log_socket(&self) -> i32 {
        -1
    }

    pub(crate) fn log_stream(&self) -> *mut rist_rs_sys::_IO_FILE {
        std::ptr::null_mut()
    }
}

/// When user-defined callback returns [`CallBackLoggerUserCBFailedError`],
/// callback of the internal C library will be returned non-zero integer.
#[derive(Debug, Clone)]
pub struct CallBackLoggerUserCBFailedError {}

/// [`CallBackLogger`] represents both `log_cb` and `log_cb_arg`.
pub trait CallBackLogger: Send + Sync {
    /// `call` will be called when the C library needs to log a message
    /// and a pair of log level and message is available properly.
    fn call(&self, log_level: &LogLebel, msg: &str) -> Result<(), CallBackLoggerUserCBFailedError>;

    /// Called when the wrapper library detects some malformed behaviour of the C library.
    fn call_malformed(
        &self,
        error: CallBackLoggerError,
    ) -> Result<(), CallBackLoggerUserCBFailedError> {
        self.call(
            &LogLebel::Error,
            &format!("rust-wrapper: CallBackLogger::call_malformed: {:?}", error),
        )?;
        Err(CallBackLoggerUserCBFailedError {})
    }

    /// Called when the wrapper library failed to acquire the pointer of [`CallBackLogger`] instance.
    fn call_malformed_global(
        error: CallBackLoggerError,
    ) -> Result<(), CallBackLoggerUserCBFailedError> {
        println!(
            "rust-wrapper: CallBackLogger::call_malformed_global: {:?}",
            error
        );
        Err(CallBackLoggerUserCBFailedError {})
    }
}

#[derive(Debug, Clone)]
pub enum CallBackLoggerError {
    UnknownEnumVariant(UnknownEnumVariantError),
    InvalidPointer(InvalidPointerError),
    Utf8Error(std::str::Utf8Error),
}

pub(crate) trait CallBackLoggerInternal: CallBackLogger {
    unsafe extern "C" fn call_raw(
        arg: *mut std::os::raw::c_void,
        log_level: rist_rs_sys::rist_log_level,
        msg: *const ::std::os::raw::c_char,
    ) -> i32
    where
        Self: Sized,
    {
        if arg.is_null() {
            return Self::panic_free_call_malformed_global(CallBackLoggerError::InvalidPointer(
                InvalidPointerError {
                    message: "Invalid pointer for `arg` in log_cb.",
                },
            ))
            .is_err() as i32;
        }
        let logger = unsafe { &*(arg as *const Self) };
        let log_level: LogLebel = match log_level.try_into() {
            Ok(level) => level,
            Err(error) => {
                return logger
                    .panic_free_call_malformed(CallBackLoggerError::UnknownEnumVariant(error))
                    .is_err() as i32;
            }
        };
        if msg.is_null() {
            return logger
                .panic_free_call_malformed(CallBackLoggerError::InvalidPointer(
                    InvalidPointerError {
                        message: "Invalid pointer for `msg` in log_cb.",
                    },
                ))
                .is_err() as i32;
        }
        let msg: &str = match unsafe { std::ffi::CStr::from_ptr(msg).to_str() } {
            Ok(s) => s,
            Err(error) => {
                return logger
                    .panic_free_call_malformed(CallBackLoggerError::Utf8Error(error))
                    .is_err() as i32;
            }
        };
        logger.panic_free_call(&log_level, msg).is_err() as i32
    }

    fn panic_free_call(
        &self,
        log_level: &LogLebel,
        msg: &str,
    ) -> Result<(), CallBackLoggerUserCBFailedError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.call(log_level, msg)))
            .map_err(|_| CallBackLoggerUserCBFailedError {})
            .flatten()
    }

    fn panic_free_call_malformed(
        &self,
        error: CallBackLoggerError,
    ) -> Result<(), CallBackLoggerUserCBFailedError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.call_malformed(error)))
            .map_err(|_| CallBackLoggerUserCBFailedError {})
            .flatten()
    }

    fn panic_free_call_malformed_global(
        error: CallBackLoggerError,
    ) -> Result<(), CallBackLoggerUserCBFailedError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::call_malformed_global(error)
        }))
        .map_err(|_| CallBackLoggerUserCBFailedError {})
        .flatten()
    }
}

impl<T: CallBackLogger> CallBackLoggerInternal for T {}

pub struct StderrCallBackLogger {
    log_level: LogLebel,
}

impl CallBackLogger for StderrCallBackLogger {
    fn call(&self, log_level: &LogLebel, msg: &str) -> Result<(), CallBackLoggerUserCBFailedError> {
        if self.log_level.is_important_than_or_equal_to(log_level) {
            eprintln!("[{}] {}", log_level, msg);
        }
        Ok(())
    }
}
