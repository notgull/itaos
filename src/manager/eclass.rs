// MIT/Apache2 License

use crate::{Directive, Id};
use cocoa::{appkit, base::nil, foundation};
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
    extern "C" fn get_directive(this: &Object, _sel: Sel) -> *mut c_void {
        unsafe { *this.get_ivar(DIRECTIVE) }
    }
    unsafe {
        decl.add_method(
            sel!(directive),
            get_directive as extern "C" fn(&Object, Sel) -> *mut c_void,
        )
    };

    // A method that creates a new instance of this class.
    extern "C" fn initialize_event(this: &mut Object, _sel: Sel, directive: *mut c_void) -> Id {
        unsafe { this.set_ivar(DIRECTIVE, directive) };

        // initialize the event above
        let ty: foundation::NSUInteger = appkit::NSEventType::NSApplicationDefined as _;
        unsafe {
            msg_send![super(this, this.class().superclass().unwrap()), 
                      otherEventWithType:ty
                      location: foundation::NSPoint::new(0.0, 0.0)
                      modifierFlags: 0
                      timestamp: 0
                      windowNumber: 0
                      context: nil
                      subtype: DIRECTIVE_EVENT_SUBTYPE
                      data1: 0
                      data2: 0]
        }
    }
    unsafe {
        decl.add_method(
            sel!(initWithDirective:),
            initialize_event as extern "C" fn(&mut Object, Sel, *mut c_void) -> Id,
        )
    };

    decl.register()
}
