// MIT/Apache2 License

use std::ops;

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
