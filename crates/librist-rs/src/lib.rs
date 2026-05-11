//! ## Supported C APIs
//!
//! ### `headers.h`
//! | C API | Rust API |
//! | - | - |
//! | `RIST_MAX_SEND_RETRIES` | |
//! | `RIST_MAX_STRING_SHORT` | |
//! | `RIST_MAX_STRING_LONG` | |
//! | `RIST_PEER_UDPSOCKET_VERSION` | |
//! | `RIST_UDP_CONFIG_VERSION` | |
//! | `RIST_ERR_MALLOC` | |
//! | `RIST_ERR_NULL_PEER` | |
//! | `RIST_ERR_INVALID_STRING_LENGTH` | |
//! | `RIST_ERR_INVALID_PROFILE` | |
//! | `RIST_ERR_MISSING_CALLBACK_FUNCTION` | |
//! | `RIST_ERR_NULL_CREDENTIALS` | |
//! | `rist_profile` | [`RistProfile`] |
//! | `rist_data_block_sender_flags` | |
//! | `rist_data_block_receiver_flags` | |
//! | `librist_multiplex_mode` | |
//! | `rist_ctx` | [`RistCtx`] |
//! | `rist_data_block` | [`RistDataBlock`] |
//! | `rist_udp_config` | |
//!
//! ### `librist_srp.h`
//! | C API | Rust API |
//! | - | - |
//! | `user_verifier_lookup_t` | |
//! | `librist_verifier_lookup_data_t` | |
//! | `user_verifier_lookup_2_t` | |
//! | `rist_enable_eap_srp` | (RIST_DEPRECATED) |
//! | `rist_enable_eap_srp_2` | |
//!
//! ### `librist.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_jitter_max_set` | |
//! | `rist_recovery_rtt_multiplier_set` | |
//! | `rist_start` | |
//! | `rist_destroy` | [`RistCtx::destroy`] |
//! | `rist_parse_udp_address` | (RIST_DEPRECATED) |
//! | `rist_parse_udp_address2` | |
//! | `rist_udp_config_free` | (RIST_DEPRECATED) |
//! | `rist_udp_config_free2` | |
//! | `librist_version` | |
//! | `librist_api_version` | |
//!
//! ### `logging.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_log_level` | |
//! | `LOGGING_SETTINGS_INITIALIZER` | |
//! | `rist_logging_settings` | [`RistLoggingSettings`] |
//! | `rist_log` | |
//! | `rist_logging_set` | (Intentionally not supported) |
//! | `rist_logging_set_global` | |
//! | `rist_logging_unset_global` | |
//! | `rist_logging_settings_free` | (RIST_DEPRECATED) |
//! | `rist_logging_settings_free2` | |
//!
//! ### `oob.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_oob_block` | |
//! | `rist_oob_write` | |
//! | `rist_oob_read` | |
//! | `rist_oob_callback_set` | |
//!
//! ### `opt.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_thread_callback_func_t` | |
//! | `rist_thread_callback_t` | |
//! | `rist_opt` | |
//! | `rist_set_opt` | |
//!
//! ### `peer.h`
//! | C API | Rust API |
//! | - | - |
//! | Default values of `rist_peer_config` | [`PeerConfig::default`] |
//! | `rist_timing_mode` | [`TimingMode`], [`RistTimingMode`] |
//! | `rist_recovery_mode` | [`RecoveryMode`], [`RistRecoveryMode`] |
//! | `rist_congestion_control_mode` | [`CongestionControlMode`], [`RistCongestionControlMode`] |
//! | `RIST_PEER_CONFIG_VERSION` | |
//! | `rist_peer_config` | [`PeerConfig`], [`RistPeerConfig`] |
//! | `rist_peer_config_defaults_set` | [`PeerConfig::default`] |
//! | `rist_parse_address` | (RIST_DEPRECATED) |
//! | `rist_parse_address2` | [`RistPeerConfig::parse_address2`] |
//! | `rist_peer_config_free` | (RIST_DEPRECATED) |
//! | `rist_peer_config_free2` | Not supported (Rust takes care of memory management) |
//! | `rist_peer_create` | [`RistCtx::peer_create`] |
//! | `rist_peer_destroy` | [`RistCtx::peer_destroy`] |
//! | `rist_peer_weight_set` | [`RistCtx::peer_set_weight`] |
//! | `rist_peer_get_socket` | [`RistCtx::peer_get_socket`] |
//! | `rist_connection_status` | |
//! | `connection_status_callback_t` | |
//! | `rist_connection_status_callback_set` | |
//! | `rist_auth_handler_set` | |
//! | `rist_peer_get_id` | |
//! | `rist_peer_get_cname` | |
//! | `rist_peer_update_secret` | |
//!
//! ### `receiver.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_receiver_create` | [`RistCtx::receiver_create`] |
//! | `rist_receiver_nack_type_set` | [`RistCtx::receiver_set_nack_type`] |
//! | `rist_receiver_set_output_fifo_size` | [`RistCtx::receiver_set_output_fifo_size`] |
//! | `rist_receiver_data_read` | (RIST_DEPRECATED) |
//! | `rist_receiver_data_read2` | [`RistCtx::receiver_read_data2`] |
//! | `receiver_data_callback_t` | (RIST_DEPRECATED) |
//! | `receiver_data_callback_t2` | [`ReceiverDataCallbackHandler2`] |
//! | `rist_receiver_data_callback_set` | (RIST_DEPRECATED) |
//! | `rist_receiver_data_callback_set2` | [`RistCtx::receiver_set_data_callback2`] |
//! | `receiver_session_timeout_callback_t` | [`ReceiverSessionTimeoutCallbackHandler`] |
//! | `rist_receiver_session_timeout_callback_set` | [`RistCtx::receiver_set_session_timeout_callback`] |
//! | `rist_receiver_data_block_free` | (RIST_DEPRECATED) |
//! | `rist_receiver_data_block_free2` | [`RistDataBlock::drop`] |
//! | `rist_receiver_data_notify_fd_set` | [`RistCtx::receiver_set_data_notify_fd`] |
//!
//! ### `sender.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_flow_id_create` | [`rist_create_flow_id`] |
//! | `rist_sender_create` | [`RistCtx::sender_create`] |
//! | `rist_sender_npd_get` | [`RistCtx::sender_get_npd`] |
//! | `rist_sender_npd_enable` | [`RistCtx::sender_enable_npd`] |
//! | `rist_sender_npd_disable` | [`RistCtx::sender_disable_npd`] |
//! | `rist_sender_flow_id_get` | [`RistCtx::sender_get_flow_id`] |
//! | `rist_sender_flow_id_set` | [`RistCtx::sender_set_flow_id`] |
//! | `rist_sender_data_write` | [`RistCtx::sender_write_data`] |
//!
//! ### `stats.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_stats_sender_peer` | |
//! | `rist_stats_receiver_peer` | |
//! | `rist_stats_receiver_flow` | |
//! | `rist_stats_type` | |
//! | `RIST_STATS_VERSION` | |
//! | `RIST_SENDER_STATS_VERSION` | |
//! | `rist_stats` | |
//! | `rist_stats_callback_set` | |
//! | `rist_sender_stats_callback_set` | |
//! | `rist_stats_free` | |
//!
//! ### `tun.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_tun_open` | |
//! | `rist_tun_close` | |
//! | `rist_tun_read` | |
//! | `rist_tun_write` | |
//! | `rist_tun_set_ip` | |
//! | `rist_tun_set_mtu` | |
//! | `rist_tun_bring_up` | |
//!
//! ### `tunnel.h`
//! | C API | Rust API |
//! | - | - |
//! | `rist_data_fd_stats` | |
//! | `RIST_DATA_FD_FLAG_TUN` | |
//! | `rist_sender_data_fd_set` | |
//! | `rist_receiver_data_fd_set` | |
//! | `rist_data_fd_stats_get` | |
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

pub use librist_sys;
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
pub enum ParseAddressError {
    CallFailed {
        function: &'static str,
        code: i32,
    },
    NullPointer {
        function: &'static str,
        value_type: &'static str,
    },
    NulString(std::ffi::NulError),
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
