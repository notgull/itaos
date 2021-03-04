// MIT/Apache2 License

use crate::{lazy_class::LazyClass, util::Id};
use cocoa::{
    foundation::{NSNotFound, NSRange, NSRect},
};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, YES},
};
use std::ptr;

mod gl;
mod vector;

pub(crate) mod vector_subview;

//pub(crate) use gl::get_gl_view_class;
pub(crate) use vector::get_vector_view_class;

#[inline]
pub(crate) fn get_view_class() -> &'static Class {
    ITAOS_VIEW_CLASS.get_or_init(create_itaos_view_class)
}

static ITAOS_VIEW_CLASS: LazyClass = LazyClass::new();

const TRACKING_AREA: &'static str = "tracking_area";
const MARKED_RANGE: &'static str = "marked_range";
const SELECTED_RANGE: &'static str = "selected_range";

#[inline]
fn create_itaos_view_class() -> &'static Class {
    // create a new ClassDecl
    let mut itaos_view = ClassDecl::new("ItaosView", class!(NSView)).unwrap();

    // add an ivar to keep track of the tracking area
    itaos_view.add_ivar::<Id>(TRACKING_AREA);

    // set up the initialization method
    extern "C" fn init_with_frame(this: &mut Object, _sel: Sel, frame: NSRect) -> Id {
        let this_but_id: Id = unsafe {
            msg_send![
                super(this, this.class().superclass().unwrap()),
                initWithFrame: frame
            ]
        };
        /*        if !this_but_id.is_null() {
            unsafe {
                let _: () = msg_send![this_but_id, addTrackingArea: tracking_area];
                this.set_ivar::<Id>(TRACKING_AREA, tracking_area);
            }
        }*/
        this_but_id
    }

    unsafe {
        itaos_view.add_method(
            sel!(initWithFrame:),
            init_with_frame as extern "C" fn(&mut Object, Sel, NSRect) -> Id,
        );
    }

    itaos_view.register()
}
