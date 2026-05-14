use super::*;
static RIST_LOGGING_GLOBAL: std::sync::Mutex<Option<RistLoggingSettings>> =
    std::sync::Mutex::<Option<RistLoggingSettings>>::new(None);

/// [`librist_sys::rist_logging_set_global`]
/// [`librist_sys::rist_logging_unset_global`]
///
/// Set global logger and return [`None`] if uninitialized.
///
/// Return [`Some`] with argument if already initialized, without swapping logger.
pub fn rist_logging_set_or_replace_global(
    mut logging_settings: RistLoggingSettings,
) -> Result<Option<RistLoggingSettings>, CommonError> {
    let ptr = logging_settings.as_mut_ptr();
    if let Ok(mut global) = RIST_LOGGING_GLOBAL.lock() {
        let ret = if let Some(previous) = global.as_mut() {
            unsafe { librist_sys::rist_logging_unset_global() };
            Some(std::mem::replace(previous, logging_settings))
        } else {
            *global = Some(logging_settings);
            None
        };
        let code = unsafe { librist_sys::rist_logging_set_global(ptr) };
        if code != 0 {
            return Err(CommonError::CallFailed {
                function: "rist_logging_set_global",
                code,
            });
        };
        Ok(ret)
    } else {
        Ok(Some(logging_settings))
    }
}

/// [`librist_sys::rist_logging_set_global`]
///
/// Set global logger and return [`Ok`] if uninitialized,
/// return [`Err`] otherwise.
pub fn rist_logging_set_global(
    mut logging_settings: RistLoggingSettings,
) -> Result<(), SetGlobalLogHandlerError> {
    let ptr = logging_settings.as_mut_ptr();
    if let Ok(mut global) = RIST_LOGGING_GLOBAL.lock() {
        if let Some(_) = global.as_mut() {
            return Err(SetGlobalLogHandlerError::AlreadySet);
        }
        *global = Some(logging_settings);
        let code = unsafe { librist_sys::rist_logging_set_global(ptr) };
        if code != 0 {
            return Err(SetGlobalLogHandlerError::CallFailed {
                function: "rist_logging_set_global",
                code,
            });
        };
        Ok(())
    } else {
        // Lock failure means other thread is initializing global.
        Err(SetGlobalLogHandlerError::LockFailed)
    }
}

/// [`librist_sys::rist_logging_unset_global`]
///
/// Unset the global logger. Return [`Some`] with the previous logger if it was set, [`None`] otherwise.
pub fn rist_logging_unset_global() -> Result<RistLoggingSettings, UnsetGlobalLogHandlerError> {
    if let Ok(mut global) = RIST_LOGGING_GLOBAL.lock() {
        if let Some(logging_settings) = global.take() {
            unsafe { librist_sys::rist_logging_unset_global() };
            Ok(logging_settings)
        } else {
            Err(UnsetGlobalLogHandlerError::NotSetYet)
        }
    } else {
        Err(UnsetGlobalLogHandlerError::LockFailed)
    }
}
