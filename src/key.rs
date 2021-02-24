// MIT/Apache2 License

use std::{fmt, num::NonZeroUsize, ptr::NonNull};

/// A key that maps to a certain Win32 object.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Key {
    // we pretend this opaque pointer is a key, lol
    key: NonZeroUsize,
}

impl fmt::Debug for Key {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct WriteHex(NonZeroUsize);

        impl fmt::Debug for WriteHex {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:#X}", self.0)
            }
        }

        f.debug_struct("Key")
            .field("key", &WriteHex(self.key))
            .finish()
    }
}

impl Key {
    #[inline]
    pub(crate) const unsafe fn from_raw(key: NonZeroUsize) -> Self {
        Self { key }
    }

    #[inline]
    pub(crate) unsafe fn as_ptr(self) -> NonNull<()> {
        // SAFETY: key is guaranteed to be non-zero
        unsafe { NonNull::new_unchecked(self.key.get() as *mut ()) }
    }

    #[inline]
    pub(crate) fn from_ptr_nn(ptr: NonNull<()>) -> Self {
        // SAFETY: same as above
        Self {
            key: unsafe { NonZeroUsize::new_unchecked(ptr.as_ptr() as usize) },
        }
    }

    #[inline]
    pub(crate) fn from_ptr(ptr: *mut ()) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                key: unsafe { NonZeroUsize::new_unchecked(ptr as usize) },
            })
        }
    }
}
