use super::*;

pub struct RistDataBlock {
    raw_ptr: std::ptr::NonNull<librist_sys::rist_data_block>,
}

impl RistDataBlock {
    pub(crate) fn from_ptr(raw_ptr: std::ptr::NonNull<librist_sys::rist_data_block>) -> Self {
        Self { raw_ptr: raw_ptr }
    }
}

impl Drop for RistDataBlock {
    fn drop(&mut self) {
        unsafe {
            librist_sys::rist_receiver_data_block_free2(&mut self.raw_ptr.as_ptr());
        }
    }
}

impl StatefulWrapper<librist_sys::rist_data_block> for RistDataBlock {
    fn as_mut_ptr(&mut self) -> *mut librist_sys::rist_data_block {
        self.raw_ptr.as_ptr()
    }

    fn as_ptr(&self) -> *const librist_sys::rist_data_block {
        self.raw_ptr.as_ptr()
    }
}
