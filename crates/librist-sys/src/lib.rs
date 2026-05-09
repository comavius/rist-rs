#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/librist.rs"));
pub mod git_version {
    include!(concat!(env!("OUT_DIR"), "/git_version.rs"));
}

// bindgen failed to generate constants for some enum values.
// see `include/librist/peer.h`.
pub const RIST_DEFAULT_CONGESTION_CONTROL_MODE: rist_congestion_control_mode =
    rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_NORMAL;
pub const RIST_DEFAULT_RECOVERY_MODE: rist_recovery_mode =
    rist_recovery_mode_RIST_RECOVERY_MODE_TIME;
pub const RIST_DEFAULT_TIMING_MODE: rist_timing_mode = rist_timing_mode_RIST_TIMING_MODE_SOURCE;
pub const RIST_DEFAULT_PROFILE: rist_profile = rist_profile_RIST_PROFILE_MAIN;
