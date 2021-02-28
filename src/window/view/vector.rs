// MIT/Apache2 License

use super::{get_view_class, vector_subview::get_vector_subview_class};
use crate::{lazy_class::LazyClass, manager::data::ManagerData, util::Id};
use cocoa::foundation::NSRect;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
};
use std::ffi::c_void;

#[inline]
pub(crate) fn get_vector_view_class() -> &'static Class {
    ITAOS_VECTOR_VIEW_CLASS.get_or_init(create_itaos_vector_view_class)
}

static ITAOS_VECTOR_VIEW_CLASS: LazyClass = LazyClass::new();

const SUBVIEW: &str = "subview";

#[inline]
fn create_itaos_vector_view_class() -> &'static Class {
    let mut vector_view = ClassDecl::new("ItaosVectorView", get_view_class()).unwrap();

    // the subview that we transparently use as our base
    vector_view.add_ivar::<Id>(SUBVIEW);

    // initialize the view
    extern "C" fn init_with_frame(this: &mut Object, _sel: Sel, frame: NSRect) -> Id {
        let this_but_id: Id = unsafe {
            msg_send![
                super(this, this.class().superclass().unwrap()),
                initWithFrame: frame
            ]
        };

        if !this_but_id.is_null() {
            let subview: Id = unsafe {
                let subview: Id = msg_send![this_but_id, alloc];
                msg_send![subview, initWithFrame: frame]
            };

            unsafe {
                let _: () = msg_send![this_but_id, addSubview: subview];
                this.set_ivar::<Id>(SUBVIEW, subview);
            }
        }

        this_but_id
    }

    unsafe {
        vector_view.add_method(
            sel!(initWithFrame:),
            init_with_frame as extern "C" fn(&mut Object, Sel, NSRect) -> Id,
        );
    }

    vector_view.register()
}
