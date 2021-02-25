// MIT/Apache2 License

use crate::{
    event::{process_event, Event},
    manager::data::ManagerData,
    util::Id,
    window::Window,
};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, BOOL, NO, YES},
};
use std::{
    ffi::c_void,
    mem::{self, ManuallyDrop},
    ptr::NonNull,
    rc::Rc,
};

const MDATA: &str = "mdata";

// helper function to get the mdata from our window class object
#[inline]
fn get_mdata(this: &Object) -> ManuallyDrop<Rc<ManagerData>> {
    let p = unsafe { this.get_ivar::<*const c_void>(MDATA) };
    let ptr = ManuallyDrop::new(unsafe { Rc::from_raw(p.cast()) });
    ptr
}

// get the window ID used to identify this window
#[inline]
fn get_wid(this: &Object) -> Window {
    let p: NonNull<Object> = this.into();
    Window::from_ptr_nn(p.cast())
}

// initialize the class for the window
#[inline]
pub(crate) fn initialize_window_class() -> &'static Class {
    // we inherit from the NSWindow class
    let superclass = class!(NSWindow);
    let mut itaos_window = ClassDecl::new("ItaosWindow", superclass).unwrap();

    // pointer to the manager data
    itaos_window.add_ivar::<*mut c_void>(MDATA);

    // runs when the window should close
    extern "C" fn window_should_close(this: &mut Object, _sel: Sel, _sender: Id) -> BOOL {
        // run the event handler
        let mdata = get_mdata(this);
        let wid = get_wid(this);

        process_event(&mdata, Event::Close(wid));

        // subtract one from the window count
        let window_count = mdata.window_count.get().saturating_sub(1);
        mdata.window_count.set(window_count);

        // if the new window count is zero, send a quit event
        if window_count == 0 {
            process_event(&mdata, Event::Quit);
        }

        YES
    }

    unsafe {
        itaos_window.add_method(
            sel!(windowShouldClose:),
            window_should_close as extern "C" fn(&mut Object, Sel, Id) -> BOOL,
        );
    }

    // intercepts events and turns them into our type of events
    extern "C"

    itaos_window.register()
}
