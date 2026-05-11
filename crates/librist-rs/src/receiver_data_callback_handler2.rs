pub use super::*;

pub trait ReceiverDataCallbackHandler2: Send + Sync {
    fn handle(&self, data_block: &mut RistDataBlock) -> Result<(), CallbackFailedError>;
}

pub(crate) trait ReceiverDataCallbackRawHandler2: ReceiverDataCallbackHandler2 {
    unsafe extern "C" fn handle_raw(
        arg: *mut std::os::raw::c_void,
        data_block: *mut librist_sys::rist_data_block,
    ) -> std::os::raw::c_int {
        let handler = unsafe { (arg as *mut Box<Self>).as_mut() };
        let handler = if let Some(handler) = handler {
            handler
        } else {
            return -1;
        };
        let handler = handler.as_ref();
        let data_block = unsafe { &mut *data_block };
        let data_block = std::ptr::NonNull::new(data_block);
        let data_block = if let Some(data_block) = data_block {
            data_block
        } else {
            return -1;
        };
        let mut data_block = RistDataBlock::from_ptr(data_block);
        match handler.handle(&mut data_block) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

impl<T> ReceiverDataCallbackRawHandler2 for T where T: ReceiverDataCallbackHandler2 {}
