use std::{alloc, ops::Deref, ptr::NonNull};

pub struct Box<T> {
    inner: NonNull<T>,
}
impl<T> Box<T> {
    pub fn new(value: T) -> Self {
        let layout = alloc::Layout::new::<T>();
        let ptr = unsafe { alloc::alloc(layout) }.cast::<T>();

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        unsafe { ptr.write(value) };

        Self {
            inner: unsafe { NonNull::new_unchecked(ptr) },
        }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        let layout = alloc::Layout::new::<T>();
        unsafe {
            // Tをdrop
            self.inner.as_ptr().drop_in_place();
            // メモリを解放
            alloc::dealloc(self.inner.as_ptr().cast(), layout);
        };
    }
}

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.inner.as_ref() }
    }
}
