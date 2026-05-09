pub fn rist_create_flow_id() -> u32 {
    unsafe { librist_sys::rist_flow_id_create() }
}
