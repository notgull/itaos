// MIT/Apache2 License

use crate::{Directive, Id};
use cocoa::appkit;
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    sel,
};
use std::ffi::c_void;

pub const DIRECTIVE_EVENT_SUBTYPE: i16 = 0x1337;

#[inline]
pub fn create_itaosevent_class() -> &'static Class {
    const DIRECTIVE: &'static str = "directive";

    let superclass = class!(NSEvent);
    let mut decl = ClassDecl::new("ItaosEvent", superclass).unwrap();

    decl.add_ivar::<*mut c_void>(DIRECTIVE);

    // A method that gets the directive's pointer.
    extern "C" fn get_directive(this: &Object, _sel: &Sel) -> *mut c_void {
        unsafe { *this.get_ivar(DIRECTIVE) }
    }
    unsafe { decl.add_method(sel!(directive), get_directive) };

    // A method that creates a new instance of this class.
    extern "C" fn initialize_event(this: &Object, _sel: &Sel, directive: *mut c_void) -> Id {
        unsafe { *this.set_ivar(DIRECTIVE) };

        // initialize the event above
        let ty: appkit::NSUInteger = appkit::NSEventType::NSApplicationDefined as _;
        unsafe {
            msg_send![super(this, this.class().superclass().unwrap()), 
                      otherEventWithType:ty
                      location: appkit::NSZeroPoint
                      modifierFlags: 0
                      timestamp: 0
                      windowNumber: 0
                      context: objc::runtime::NIL
                      subtype: DIRECTIVE_EVENT_SUBTYPE
                      data1: 0
                      data2: 0]
        }
    }
    unsafe { decl.add_method(sel!(initWithDirective:), initialize_event) };

    decl.register()
}
