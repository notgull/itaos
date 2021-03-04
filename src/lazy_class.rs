// MIT/Apache2 License

use objc::runtime::Class;
use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

#[doc(hidden)]
pub(crate) struct LazyClass {
    // the internal class
    class: AtomicPtr<Class>,
}

impl Cache {
    #[inline]
    pub const fn new() -> Self {
        Self {
            class: AtomicPtr::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub fn get_or_init<F: FnOnce() -> &'static Class>(&'static self, f: F) -> &'static Class {
        match self.class.load(Ordering::Acquire) {
            ptr::null_mut() => {
                let p = f();
                self.class.store(p as *const Class, Ordering::Release);
                p
            }
            p => unsafe { &*p },
        }
    }
}
