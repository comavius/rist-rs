use super::*;

#[derive(Default)]
pub struct LoggingSettings<CBHandler: LogCallbackHandler> {
    pub log_level: LogLevel,
    pub log_handler: LogHandler<CBHandler>,
}

#[derive(Default)]
pub enum LogHandler<CBHandler: LogCallbackHandler> {
    CallbackHandler(CBHandler),
    SocketAndStream((Option<std::net::UdpSocket>, Option<std::os::fd::OwnedFd>)),
    #[default]
    NoHandler,
}

/// Rust wrapper allocates and manages the lifecycle of a [`librist_sys::rist_logging_settings`]
/// because [`librist_sys::rist_logging_set`] has complex side effect.
pub struct RistLoggingSettings<'a> {
    pinned: std::pin::Pin<Box<RistLoggingSettingsPinned>>,
    _callback_handler: Option<utility::TypeErasedBox<'a>>,
    _log_stream_cfile: Option<CFile>,
}

struct CFile {
    fp: std::ptr::NonNull<libc::FILE>,
}

impl CFile {
    fn new(fd: std::os::fd::OwnedFd) -> Self {
        use std::os::fd::AsRawFd;
        let fp = unsafe { libc::fdopen(fd.as_raw_fd(), b"w\0".as_ptr() as *const i8) };
        Self {
            fp: std::ptr::NonNull::new(fp).unwrap(),
        }
    }

    fn cast_io_file(&self) -> *mut librist_sys::FILE {
        self.fp.as_ptr().cast()
    }
}

impl Drop for CFile {
    fn drop(&mut self) {
        unsafe {
            libc::fclose(self.fp.as_ptr());
        }
    }
}

struct RistLoggingSettingsPinned {
    raw: librist_sys::rist_logging_settings,
    _pin: std::marker::PhantomPinned,
}

impl<'a, CBHandler: LogCallbackHandler + 'a> Into<RistLoggingSettings<'a>>
    for LoggingSettings<CBHandler>
{
    fn into(self) -> RistLoggingSettings<'a> {
        let log_level = <LogLevel as Into<RistLogLevel>>::into(self.log_level).0;
        let (log_cb, callback_handler, log_cb_arg, log_socket, log_stream, log_stream_cfile) =
            match self.log_handler {
                LogHandler::CallbackHandler(handler) => {
                    let log_cb = CBHandler::handle_raw
                        as unsafe extern "C" fn(
                            arg: *mut ::std::os::raw::c_void,
                            arg1: librist_sys::rist_log_level,
                            msg: *const ::std::os::raw::c_char,
                        ) -> ::std::os::raw::c_int;
                    let callback_handler = utility::TypeErasedBox::new(handler);
                    let log_cb_arg = callback_handler.as_ptr();
                    (
                        Some(log_cb),
                        Some(callback_handler),
                        Some(log_cb_arg),
                        None,
                        None,
                        None,
                    )
                }
                LogHandler::SocketAndStream((socket, stream)) => {
                    use std::os::fd::AsRawFd;
                    let log_socket = socket.map(|s| s.as_raw_fd());
                    let log_stream_cfile = stream.map(CFile::new);
                    let log_stream = log_stream_cfile.as_ref().map(|cfile| cfile.cast_io_file());
                    (None, None, None, log_socket, log_stream, log_stream_cfile)
                }
                LogHandler::NoHandler => (None, None, None, None, None, None),
            };
        RistLoggingSettings {
            pinned: Box::pin(RistLoggingSettingsPinned {
                raw: librist_sys::rist_logging_settings {
                    log_level,
                    log_cb,
                    log_cb_arg: log_cb_arg.unwrap_or(std::ptr::null_mut()),
                    log_socket: log_socket.unwrap_or(-1),
                    log_stream: log_stream.unwrap_or(std::ptr::null_mut()),
                },
                _pin: std::marker::PhantomPinned,
            }),
            _callback_handler: callback_handler,
            _log_stream_cfile: log_stream_cfile,
        }
    }
}

impl<'a> StatefulWrapper<librist_sys::rist_logging_settings> for RistLoggingSettings<'a> {
    fn as_ptr(&self) -> *const librist_sys::rist_logging_settings {
        &self.pinned.as_ref().get_ref().raw
    }

    fn as_mut_ptr(&mut self) -> *mut librist_sys::rist_logging_settings {
        unsafe { &mut self.pinned.as_mut().get_unchecked_mut().raw }
    }
}
