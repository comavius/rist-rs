pub struct TypeErasedBox<'a> {
    ptr: std::ptr::NonNull<std::os::raw::c_void>,
    drop_fn: unsafe fn(std::ptr::NonNull<std::os::raw::c_void>),
    _phantom: std::marker::PhantomData<&'a ()>,
}

unsafe impl<'a> Send for TypeErasedBox<'a> {}
unsafe impl<'a> Sync for TypeErasedBox<'a> {}

impl<'a> TypeErasedBox<'a> {
    pub fn new<T: Send + Sync + 'a>(value: T) -> Self {
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
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut std::os::raw::c_void {
        self.ptr.as_ptr()
    }
}

impl<'a> Drop for TypeErasedBox<'a> {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.ptr);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MultiThreadCounter {
        counter: std::sync::Arc<std::sync::Mutex<i32>>,
    }

    impl Drop for MultiThreadCounter {
        fn drop(&mut self) {
            *self.counter.lock().unwrap() += 1;
        }
    }

    #[test]
    fn test_type_erased_box_drop_works() {
        let inner = std::sync::Arc::new(std::sync::Mutex::new(0));
        let _box = TypeErasedBox::new(MultiThreadCounter {
            counter: inner.clone(),
        });
        assert_eq!(*inner.lock().unwrap(), 0);
        std::mem::drop(_box);
        assert_eq!(*inner.lock().unwrap(), 1);
    }
}
