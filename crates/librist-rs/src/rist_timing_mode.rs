use super::*;

pub enum TimingMode {
    Source,
    Arrival,
    Rtc,
}

pub struct RistTimingMode(pub(crate) librist_sys::rist_timing_mode);

impl Default for TimingMode {
    fn default() -> Self {
        <TimingMode as TryFrom<RistTimingMode>>::try_from(RistTimingMode(
            librist_sys::RIST_DEFAULT_TIMING_MODE,
        ))
        .unwrap()
    }
}

impl Into<RistTimingMode> for TimingMode {
    fn into(self) -> RistTimingMode {
        match self {
            TimingMode::Source => {
                RistTimingMode(librist_sys::rist_timing_mode_RIST_TIMING_MODE_SOURCE)
            }
            TimingMode::Arrival => {
                RistTimingMode(librist_sys::rist_timing_mode_RIST_TIMING_MODE_ARRIVAL)
            }
            TimingMode::Rtc => RistTimingMode(librist_sys::rist_timing_mode_RIST_TIMING_MODE_RTC),
        }
    }
}

impl TryFrom<RistTimingMode> for TimingMode {
    type Error = UnknownEnumError;

    fn try_from(value: RistTimingMode) -> Result<Self, Self::Error> {
        match value.0 {
            librist_sys::rist_timing_mode_RIST_TIMING_MODE_SOURCE => Ok(TimingMode::Source),
            librist_sys::rist_timing_mode_RIST_TIMING_MODE_ARRIVAL => Ok(TimingMode::Arrival),
            librist_sys::rist_timing_mode_RIST_TIMING_MODE_RTC => Ok(TimingMode::Rtc),
            _ => Err(UnknownEnumError {
                enum_type: "rist_timing_mode",
                value: value.0 as i32,
            }),
        }
    }
}
