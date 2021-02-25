// MIT/Apache2 License

use super::Event;
use crate::util::Id;
use cocoa::foundation;

/// Translate any given NSEvent into the appropriate iTaos event, and also return the runtime ID.
#[inline]
pub(crate) fn translate_nsevent(event: Id) -> (Option<Event>, usize) {
    let ty: foundation::NSUInteger = unsafe { msg_send![event, type] };

    (None, 0)
}
