use super::*;

pub struct LoggingSettings {
    pub log_level: LogLevel,
    pub log_socket: Option<std::net::UdpSocket>,
    pub log_stream: Option<std::os::fd::OwnedFd>,
}

/// Rust wrapper allocates and manages the lifecycle of a [`librist_sys::rist_logging_settings`]
/// because [`librist_sys::rist_logging_set`] has complex side effect.
pub struct RistLoggingSettings {
    pinned: std::pin::Pin<Box<RistLoggingSettingsPinned>>,
}

struct RistLoggingSettingsPinned {
    raw: librist_sys::rist_logging_settings,
    _pin: std::marker::PhantomPinned,
}

impl RistLoggingSettings {
    pub(crate) fn new(raw: librist_sys::rist_logging_settings) -> Self {
        Self {
            pinned: Box::pin(RistLoggingSettingsPinned {
                raw,
                _pin: std::marker::PhantomPinned,
            }),
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
