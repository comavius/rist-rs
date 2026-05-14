use super::*;

/// [`librist_sys::rist_logging_settings::log_socket`] and
/// [`librist_sys::rist_logging_settings::log_stream`] are not supported,
/// because its hard to integrate with rust centric interfaces.
///
/// If you want to use there features, please reimplement them as
/// [`LogCallbackHandler`] impls.
#[derive(Default)]
pub struct LoggingSettings<CBHandler: LogCallbackHandler> {
    pub log_level: LogLevel,
    pub log_handler: CBHandler,
}

/// Rust wrapper allocates and manages the lifecycle of a [`librist_sys::rist_logging_settings`]
/// because [`librist_sys::rist_logging_set`] has complex side effect.
pub struct RistLoggingSettings {
    pinned: std::pin::Pin<Box<RistLoggingSettingsPinned>>,
    _callback_handler: Option<utility::TypeErasedBox>,
}

unsafe impl Sync for RistLoggingSettings {}
unsafe impl Send for RistLoggingSettings {}

struct RistLoggingSettingsPinned {
    raw: librist_sys::rist_logging_settings,
    _pin: std::marker::PhantomPinned,
}

impl<CBHandler: LogCallbackHandler> Into<RistLoggingSettings> for LoggingSettings<CBHandler> {
    fn into(self) -> RistLoggingSettings {
        let log_level = <LogLevel as Into<RistLogLevel>>::into(self.log_level).0;

        let log_cb = CBHandler::handle_raw
            as unsafe extern "C" fn(
                arg: *mut ::std::os::raw::c_void,
                arg1: librist_sys::rist_log_level,
                msg: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int;
        let callback_handler = utility::TypeErasedBox::new(self.log_handler);
        let log_cb_arg = callback_handler.as_ptr();
        RistLoggingSettings {
            pinned: Box::pin(RistLoggingSettingsPinned {
                raw: librist_sys::rist_logging_settings {
                    log_level,
                    log_cb: Some(log_cb),
                    log_cb_arg,
                    log_socket: -1,
                    log_stream: std::ptr::null_mut(),
                },
                _pin: std::marker::PhantomPinned,
            }),
            _callback_handler: Some(callback_handler),
        }
    }
}

impl StatefulWrapper<librist_sys::rist_logging_settings> for RistLoggingSettings {
    fn as_ptr(&self) -> *const librist_sys::rist_logging_settings {
        &self.pinned.as_ref().get_ref().raw
    }

    fn as_mut_ptr(&mut self) -> *mut librist_sys::rist_logging_settings {
        unsafe { &mut self.pinned.as_mut().get_unchecked_mut().raw }
    }
}
