use super::*;

pub struct PeerConfig {
    pub version: i32,
    pub address_family: i32,
    pub initiate_conn: i32,
    pub address: String,
    pub miface: String,
    pub physical_port: u16,
    pub virt_dst_port: u16,
    pub recovery_mode: RecoveryMode,
    pub recovery_maxbitrate: u32,
    pub recovery_maxbitrate_return: u32,
    pub recovery_length_min: u32,
    pub recovery_length_max: u32,
    pub recovery_reorder_buffer: u32,
    pub recovery_rtt_min: u32,
    pub recovery_rtt_max: u32,
    pub weight: u32,
    pub secret: String,
    pub key_size: i32,
    pub key_rotation: u32,
    pub compression: i32,
    pub cname: String,
    pub congestion_control_mode: CongestionControlMode,
    pub min_retries: u32,
    pub max_retries: u32,
    pub session_timeout: u32,
    pub keepalive_interval: u32,
    pub timing_mode: TimingMode,
    pub srp_username: String,
    pub srp_password: String,
}

impl Default for PeerConfig {
    fn default() -> Self {
        Self {
            version: librist_sys::RIST_PEER_CONFIG_VERSION as i32,
            address_family: i32::default(),
            initiate_conn: i32::default(),
            address: String::default(),
            miface: String::default(),
            physical_port: u16::default(),
            virt_dst_port: librist_sys::RIST_DEFAULT_VIRT_DST_PORT as u16,
            recovery_mode: RecoveryMode::default(),
            recovery_maxbitrate: librist_sys::RIST_DEFAULT_RECOVERY_MAXBITRATE,
            recovery_maxbitrate_return: librist_sys::RIST_DEFAULT_RECOVERY_MAXBITRATE_RETURN,
            recovery_length_min: librist_sys::RIST_DEFAULT_RECOVERY_LENGTH_MIN,
            recovery_length_max: librist_sys::RIST_DEFAULT_RECOVERY_LENGTH_MAX,
            recovery_reorder_buffer: librist_sys::RIST_DEFAULT_RECOVERY_REORDER_BUFFER,
            recovery_rtt_min: librist_sys::RIST_DEFAULT_RECOVERY_RTT_MIN,
            recovery_rtt_max: librist_sys::RIST_DEFAULT_RECOVERY_RTT_MAX,
            weight: u32::default(),
            secret: String::default(),
            key_size: i32::default(),
            key_rotation: u32::default(),
            compression: i32::default(),
            cname: String::default(),
            congestion_control_mode: CongestionControlMode::default(),
            min_retries: librist_sys::RIST_DEFAULT_MIN_RETRIES,
            max_retries: librist_sys::RIST_DEFAULT_MAX_RETRIES,
            session_timeout: u32::default(),
            keepalive_interval: u32::default(),
            timing_mode: TimingMode::default(),
            srp_username: String::default(),
            srp_password: String::default(),
        }
    }
}

impl TryInto<RistPeerConfig> for PeerConfig {
    type Error = std::ffi::NulError;

    fn try_into(self) -> Result<RistPeerConfig, Self::Error> {
        Ok(RistPeerConfig {
            pinned: Box::pin(RistPeerConfigPinned {
                raw: librist_sys::rist_peer_config {
                    version: self.version,
                    address_family: self.address_family,
                    initiate_conn: self.initiate_conn,
                    address: to_c_buf(&self.address)?,
                    miface: to_c_buf(&self.miface)?,
                    physical_port: self.physical_port,
                    virt_dst_port: self.virt_dst_port,
                    recovery_mode: Into::<RistRecoveryMode>::into(self.recovery_mode).0,
                    recovery_maxbitrate: self.recovery_maxbitrate,
                    recovery_maxbitrate_return: self.recovery_maxbitrate_return,
                    recovery_length_min: self.recovery_length_min,
                    recovery_length_max: self.recovery_length_max,
                    recovery_reorder_buffer: self.recovery_reorder_buffer,
                    recovery_rtt_min: self.recovery_rtt_min,
                    recovery_rtt_max: self.recovery_rtt_max,
                    weight: self.weight,
                    secret: to_c_buf(&self.secret)?,
                    key_size: self.key_size,
                    key_rotation: self.key_rotation,
                    compression: self.compression,
                    cname: to_c_buf(&self.cname)?,
                    congestion_control_mode: Into::<RistCongestionControlMode>::into(
                        self.congestion_control_mode,
                    )
                    .0,
                    min_retries: self.min_retries,
                    max_retries: self.max_retries,
                    session_timeout: self.session_timeout,
                    keepalive_interval: self.keepalive_interval,
                    timing_mode: Into::<RistTimingMode>::into(self.timing_mode).0,
                    srp_username: to_c_buf(&self.srp_username)?,
                    srp_password: to_c_buf(&self.srp_password)?,
                },
                _pin: std::marker::PhantomPinned,
            }),
        })
    }
}

fn to_c_buf<const N: usize>(s: &str) -> Result<[std::os::raw::c_char; N], std::ffi::NulError> {
    let mut buf = [0i8; N];
    let cstr = std::ffi::CString::new(s)?;
    let bytes = cstr.as_bytes_with_nul();
    let len = bytes.len().min(N);
    for i in 0..len {
        buf[i] = bytes[i] as i8;
    }
    Ok(buf)
}

pub struct RistPeerConfig {
    pinned: std::pin::Pin<Box<RistPeerConfigPinned>>,
}

struct RistPeerConfigPinned {
    raw: librist_sys::rist_peer_config,
    _pin: std::marker::PhantomPinned,
}

impl StatefulWrapper<librist_sys::rist_peer_config> for RistPeerConfig {
    fn as_ptr(&self) -> *const librist_sys::rist_peer_config {
        &self.pinned.as_ref().get_ref().raw
    }

    fn as_mut_ptr(&mut self) -> *mut librist_sys::rist_peer_config {
        unsafe { &mut self.pinned.as_mut().get_unchecked_mut().raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_impl_eq() {
        let rs_v: RistPeerConfig = PeerConfig::default().try_into().unwrap();
        let rs_v = rs_v.pinned.raw;
        let mut c_v = std::mem::MaybeUninit::<librist_sys::rist_peer_config>::uninit();
        unsafe { librist_sys::rist_peer_config_defaults_set(c_v.as_mut_ptr()) };
        let c_v = unsafe { *c_v.as_ptr() };
        assert_eq!(rs_v.version, c_v.version);
        assert_eq!(rs_v.address_family, c_v.address_family);
        assert_eq!(rs_v.initiate_conn, c_v.initiate_conn);
        assert_eq!(rs_v.address, c_v.address);
        assert_eq!(rs_v.miface, c_v.miface);
        assert_eq!(rs_v.physical_port, c_v.physical_port);
        assert_eq!(rs_v.virt_dst_port, c_v.virt_dst_port);
        assert_eq!(rs_v.recovery_mode, c_v.recovery_mode);
        assert_eq!(rs_v.recovery_maxbitrate, c_v.recovery_maxbitrate);
        assert_eq!(
            rs_v.recovery_maxbitrate_return,
            c_v.recovery_maxbitrate_return
        );
        assert_eq!(rs_v.recovery_length_min, c_v.recovery_length_min);
        assert_eq!(rs_v.recovery_length_max, c_v.recovery_length_max);
        assert_eq!(rs_v.recovery_reorder_buffer, c_v.recovery_reorder_buffer);
        assert_eq!(rs_v.recovery_rtt_min, c_v.recovery_rtt_min);
        assert_eq!(rs_v.recovery_rtt_max, c_v.recovery_rtt_max);
        assert_eq!(rs_v.weight, c_v.weight);
        assert_eq!(rs_v.secret, c_v.secret);
        assert_eq!(rs_v.key_size, c_v.key_size);
        assert_eq!(rs_v.key_rotation, c_v.key_rotation);
        assert_eq!(rs_v.compression, c_v.compression);
        assert_eq!(rs_v.cname, c_v.cname);
        assert_eq!(rs_v.congestion_control_mode, c_v.congestion_control_mode);
        assert_eq!(rs_v.min_retries, c_v.min_retries);
        assert_eq!(rs_v.max_retries, c_v.max_retries);
        assert_eq!(rs_v.session_timeout, c_v.session_timeout);
        assert_eq!(rs_v.keepalive_interval, c_v.keepalive_interval);
        assert_eq!(rs_v.timing_mode, c_v.timing_mode);
        assert_eq!(rs_v.srp_username, c_v.srp_username);
        assert_eq!(rs_v.srp_password, c_v.srp_password);
    }
}
