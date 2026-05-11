pub struct TypeErasedBox {
    ptr: std::ptr::NonNull<std::os::raw::c_void>,
    drop_fn: unsafe fn(std::ptr::NonNull<std::os::raw::c_void>),
}

impl TypeErasedBox {
    pub fn new<T>(value: T) -> Self {
        unsafe fn drop_impl<T>(ptr: std::ptr::NonNull<std::os::raw::c_void>) {
            let _ = unsafe { Box::from_raw(ptr.as_ptr() as *mut T) };
        }
        let ptr = unsafe {
            std::ptr::NonNull::new_unchecked(
                Box::into_raw(Box::new(value)) as *mut std::os::raw::c_void
            )
        };
        Self {
            ptr,
            drop_fn: drop_impl::<T>,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut std::os::raw::c_void {
        self.ptr.as_ptr()
    }
}

impl Drop for TypeErasedBox {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.ptr);
        }
    }
}
