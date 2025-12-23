use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct MyBox<T> {
    inner: NonNull<T>,
}
impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let layout = alloc::Layout::new::<T>();

        if layout.size() == 0 {
            let inner: NonNull<T> = NonNull::dangling();
            // dropさせない
            unsafe { inner.as_ptr().write(value) };
            return Self { inner };
        }

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

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let layout = alloc::Layout::new::<T>();
        unsafe {
            // Tをdrop
            self.inner.as_ptr().drop_in_place();

            if layout.size() != 0 {
                // メモリを解放
                alloc::dealloc(self.inner.as_ptr().cast(), layout);
            }
        };
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.inner.as_ref() }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn drop_is_called_once() {
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct D;
        impl Drop for D {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        drop(MyBox::new(D));
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn drop_is_called_once_for_zst() {
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct Z;
        impl Drop for Z {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        drop(MyBox::new(Z));
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn zst_drop_is_called_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct Z;
        impl Drop for Z {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        drop(MyBox::new(Z));
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn alignment_is_respected() {
        #[repr(align(64))]
        #[allow(unused)]
        struct A(u8);

        let b = MyBox::new(A(1));
        let addr = &*b as *const A as usize;
        assert_eq!(addr % 64, 0);
    }

    #[test]
    fn deref_mut_works() {
        let mut b = MyBox::new(10);
        *b = 20;
        assert_eq!(*b, 20);
    }

    #[test]
    fn can_take_refs() {
        let mut b = MyBox::new(String::from("hi"));
        let r: &str = &b;
        assert_eq!(r, "hi");

        let rm: &mut String = &mut b;
        rm.push('!');
        assert_eq!(&*b, "hi!");
    }
}
