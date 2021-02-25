// MIT/Apache2 License

use std::{mem, ops, process::abort, ptr};

pub type Id = *mut objc::runtime::Object;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct ThreadSafe<T>(T);

unsafe impl<T> Send for ThreadSafe<T> {}
unsafe impl<T> Sync for ThreadSafe<T> {}

impl<T> ThreadSafe<T> {
    #[inline]
    pub(crate) unsafe fn new(val: T) -> Self {
        Self(val)
    }

    #[inline]
    pub(crate) fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for ThreadSafe<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for ThreadSafe<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Helper function to take a memory slot, take from it, and use the value, as long as the value is guaranteed to
/// return.
#[inline]
pub(crate) unsafe fn memslot<T, F: FnOnce(T) -> T>(slot: &mut T, mut f: F) {
    // SAFETY: &mut guarantees we're the only ones with access to it, so we can "take" it without repurcussion,
    //         as long as we don't access the slot after the take and before the restore
    // However, if the function panics, this no longer applies. Therefore, we abort on panic.
    struct Bomb;

    impl Drop for Bomb {
        #[inline]
        fn drop(&mut self) {
            abort();
        }
    }

    let _bomb = Bomb;

    // read the object from the slot
    let object = unsafe { ptr::read(slot as *mut T as *const T) };

    // run the slot function
    let replacement = f(object);

    // replace the object
    unsafe { ptr::write(slot, replacement) };

    // defuse the bomb
    mem::forget(_bomb);
}
