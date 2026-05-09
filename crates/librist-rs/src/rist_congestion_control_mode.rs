use super::*;

pub enum CongestionControlMode {
    Off,
    Normal,
    Aggressive,
}

pub struct RistCongestionControlMode(pub(crate) librist_sys::rist_congestion_control_mode);

impl Default for CongestionControlMode {
    fn default() -> Self {
        <CongestionControlMode as TryFrom<RistCongestionControlMode>>::try_from(
            RistCongestionControlMode(librist_sys::RIST_DEFAULT_CONGESTION_CONTROL_MODE),
        )
        .unwrap()
    }
}

impl Into<RistCongestionControlMode> for CongestionControlMode {
    fn into(self) -> RistCongestionControlMode {
        match self {
            CongestionControlMode::Off => RistCongestionControlMode(
                librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_OFF,
            ),
            CongestionControlMode::Normal => RistCongestionControlMode(
                librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_NORMAL,
            ),
            CongestionControlMode::Aggressive => RistCongestionControlMode(
                librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_AGGRESSIVE,
            ),
        }
    }
}

impl TryFrom<RistCongestionControlMode> for CongestionControlMode {
    type Error = UnknownEnumError;

    fn try_from(value: RistCongestionControlMode) -> Result<Self, Self::Error> {
        match value.0 {
            librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_OFF => {
                Ok(CongestionControlMode::Off)
            }
            librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_NORMAL => {
                Ok(CongestionControlMode::Normal)
            }
            librist_sys::rist_congestion_control_mode_RIST_CONGESTION_CONTROL_MODE_AGGRESSIVE => {
                Ok(CongestionControlMode::Aggressive)
            }
            _ => Err(UnknownEnumError {
                enum_type: "rist_congestion_control_mode",
                value: value.0 as i32,
            }),
        }
    }
}
