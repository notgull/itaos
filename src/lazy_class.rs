// MIT/Apache2 License

use objc::runtime::Class;
use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

pub(crate) struct LazyClass {
    // the internal class
    class: AtomicPtr<Class>,
}

impl LazyClass {
    #[inline]
    pub const fn new() -> Self {
        Self {
            class: AtomicPtr::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub fn get_or_init<F: FnOnce() -> &'static Class>(&'static self, f: F) -> &'static Class {
        const NULL: *mut Class = ptr::null_mut();

        match self.class.load(Ordering::Acquire) {
            NULL => {
                let p = f();
                self.class.store(p as *const Class as *mut Class, Ordering::Release);
                p
            }
            p => unsafe { &*p },
        }
    }
}
