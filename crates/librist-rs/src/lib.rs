pub mod receiver_data_callback_handler2;
pub mod receiver_session_timeout_callback_handler;
pub mod rist_congestion_control_mode;
pub mod rist_ctx;
pub mod rist_data_block;
pub mod rist_flow_id_create;
pub mod rist_logging_settings;
pub mod rist_nack_type;
pub mod rist_peer;
pub mod rist_peer_config;
pub mod rist_profile;
pub mod rist_recovery_mode;
pub mod rist_timing_mode;
mod utility;

pub use receiver_data_callback_handler2::*;
pub use receiver_session_timeout_callback_handler::*;
pub use rist_congestion_control_mode::*;
pub use rist_ctx::*;
pub use rist_data_block::*;
pub use rist_flow_id_create::*;
pub use rist_logging_settings::*;
pub use rist_nack_type::*;
pub use rist_peer::*;
pub use rist_peer_config::*;
pub use rist_profile::*;
pub use rist_recovery_mode::*;
pub use rist_timing_mode::*;

pub enum CommonError {
    CallFailed {
        function: &'static str,
        code: i32,
    },
    NullPointer {
        function: &'static str,
        value_type: &'static str,
    },
}
pub enum PeerOperationError {
    CallFailed { function: &'static str, code: i32 },
    InvalidPeer(RistPeer),
}

pub enum PeerGetSocketError {
    CallFailed { function: &'static str, code: i32 },
    InvalidPeer(RistPeer),
    InvalidSocket { socket: i32 },
}

#[derive(Debug)]
pub struct UnknownEnumError {
    enum_type: &'static str,
    value: i32,
}

pub use librist_sys::git_version::LIBRIST_GIT_COMMIT;

pub(crate) trait StatefulWrapper<T> {
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}
