use super::*;

pub enum RecoveryMode {
    Unconfigured,
    Disabled,
    Time,
}

pub struct RistRecoveryMode(pub(crate) librist_sys::rist_recovery_mode);

impl Default for RecoveryMode {
    fn default() -> Self {
        <RecoveryMode as TryFrom<RistRecoveryMode>>::try_from(RistRecoveryMode(
            librist_sys::RIST_DEFAULT_RECOVERY_MODE,
        ))
        .unwrap()
    }
}

impl Into<RistRecoveryMode> for RecoveryMode {
    fn into(self) -> RistRecoveryMode {
        match self {
            RecoveryMode::Unconfigured => {
                RistRecoveryMode(librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_UNCONFIGURED)
            }
            RecoveryMode::Disabled => {
                RistRecoveryMode(librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_DISABLED)
            }
            RecoveryMode::Time => {
                RistRecoveryMode(librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_TIME)
            }
        }
    }
}

impl TryFrom<RistRecoveryMode> for RecoveryMode {
    type Error = UnknownEnumError;

    fn try_from(value: RistRecoveryMode) -> Result<Self, Self::Error> {
        match value.0 {
            librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_UNCONFIGURED => {
                Ok(RecoveryMode::Unconfigured)
            }
            librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_DISABLED => {
                Ok(RecoveryMode::Disabled)
            }
            librist_sys::rist_recovery_mode_RIST_RECOVERY_MODE_TIME => Ok(RecoveryMode::Time),
            _ => Err(UnknownEnumError {
                enum_type: "rist_recovery_mode",
                value: value.0 as i32,
            }),
        }
    }
}
