pub use super::*;

pub trait LogCallbackHandler: Send + Sync {
    fn handle(&self, log_level: LogLevel, message: &str) -> Result<(), CallbackFailedError>;
}

pub(crate) trait LogCallbackRawHandler: LogCallbackHandler {
    unsafe extern "C" fn handle_raw(
        arg: *mut std::os::raw::c_void,
        log_level: librist_sys::rist_log_level,
        message: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int {
        let handler = unsafe { (arg as *mut Box<Self>).as_mut() };
        let handler = if let Some(handler) = handler {
            handler
        } else {
            return -1;
        };
        let handler = handler.as_ref();
        let message = unsafe {
            std::ffi::CStr::from_ptr(message)
                .to_str()
                .unwrap_or("Invalid UTF-8 string")
        };
        let log_level = match <RistLogLevel as TryInto<LogLevel>>::try_into(RistLogLevel(log_level))
        {
            Ok(level) => level,
            Err(_) => return -1,
        };
        match handler.handle(log_level, message) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}
impl<T> LogCallbackRawHandler for T where T: LogCallbackHandler {}
