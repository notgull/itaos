// MIT/Apache2 License

use crate::{
    event::{process_event, Event},
    graphics::{AppkitSpawner, Graphics},
    key::Key,
    lazy_class::LazyClass,
    manager::data::ManagerData,
    util::Id,
};
use cocoa::foundation::NSRect;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
};
use std::{ffi::c_void, mem::ManuallyDrop, ptr::NonNull, rc::Rc};

#[inline]
pub(crate) fn get_vector_subview_class() -> &'static Class {
    ITAOS_VECTOR_SUBVIEW_CLASS.get_or_init(create_itaos_vector_subview_class)
}

static ITAOS_VECTOR_SUBVIEW_CLASS: LazyClass = LazyClass::new();

#[inline]
fn create_itaos_vector_subview_class() -> &'static Class {
    let mut subview_class = ClassDecl::new("ItaosVectorSubview", class!(NSView)).unwrap();

    extern "C" fn draw_rect(this: &Object, _sel: Sel, rect: NSRect) {
        let context: *const c_void = unsafe {
            let context: Id = msg_send![class!(NSGraphicsContext), currentContext];
            msg_send![context, CGContext]
        };

        // create the context pointer
        let spawner = AppkitSpawner::new();
        let graphics = unsafe { Graphics::from_raw(context.cast(), spawner) };

        graphics.save().ok();

        // get the mdata so we can run the processor
        let win: Id = unsafe { msg_send![this, window] };
        let win: NonNull<Object> = match NonNull::new(win) {
            Some(win) => win,
            None => return,
        };

        let mdata: *const c_void = unsafe { msg_send![win.as_ptr(), mdata] };
        let mdata: ManuallyDrop<Rc<ManagerData>> =
            ManuallyDrop::new(unsafe { Rc::from_raw(mdata.cast()) });

        process_event(&mdata, Event::Paint(Key::from_ptr_nn(win.cast()), graphics));

        graphics.restore().ok();
    }

    subview_class.register()
}
