pub use super::*;

pub trait ReceiverSessionTimeoutCallbackHandler: Send + Sync {
    fn handle(&self, flow_id: u32) -> Result<(), ReceiverDataCallBackHandleError2>;
}

pub struct ReceiverDataCallBackHandleError2 {}

pub(crate) trait ReceiverSessionTimeoutCallbackRawHandler:
    ReceiverSessionTimeoutCallbackHandler
{
    unsafe extern "C" fn handle_raw(
        arg: *mut std::os::raw::c_void,
        flow_id: u32,
    ) -> std::os::raw::c_int {
        let handler = unsafe { (arg as *mut Box<Self>).as_mut() };
        let handler = if let Some(handler) = handler {
            handler
        } else {
            return -1;
        };
        let handler = handler.as_ref();
        match handler.handle(flow_id) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

impl<T> ReceiverSessionTimeoutCallbackRawHandler for T where T: ReceiverSessionTimeoutCallbackHandler
{}
