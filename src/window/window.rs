// MIT/Apache2 License

use super::get_vector_view_class;
use crate::{
    event::{process_event, Event},
    lazy_class::LazyClass,
    manager::data::ManagerData,
    util::Id,
    window::Window,
};
use cocoa::{
    appkit::{NSBackingStoreType, NSWindowStyleMask},
    foundation::NSRect,
};
use objc::{
    declare::ClassDecl,
    rc::StrongPtr,
    runtime::{Class, Object, Sel, BOOL, NO, YES},
};
use once_cell::sync::Lazy;
use std::{
    ffi::c_void,
    mem::{self, ManuallyDrop},
    ptr::NonNull,
    rc::Rc,
};

#[inline]
pub(crate) fn get_window_class() -> &'static Class {
    ITAOS_WINDOW_CLASS.get_or_init(initialize_window_class)
}

static ITAOS_WINDOW_CLASS: LazyClass = LazyClass::new();

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
fn initialize_window_class() -> &'static Class {
    // we inherit from the NSWindow class
    let superclass = class!(NSWindow);
    let mut itaos_window = ClassDecl::new("ItaosWindow", superclass).unwrap();

    // pointer to the manager data
    itaos_window.add_ivar::<*mut c_void>(MDATA);

    // helper function to get the mdata
    extern "C" fn mdata(this: &Object, _sel: Sel) -> *mut c_void {
        Rc::as_ptr(&get_mdata(this)) as *const c_void as *mut c_void
    }

    unsafe {
        itaos_window.add_method(sel!(mdata), mdata as extern "C" fn(&Object, Sel) -> *mut c_void);
    }

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
    extern "C" fn send_event(this: &Object, _sel: Sel, event: Id) {
        let _: () = unsafe {
            msg_send![
                super(this, this.class().superclass().unwrap()),
                sendEvent: event
            ]
        };
    }

    unsafe {
        itaos_window.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&Object, Sel, Id),
        );
    }

    // initialize our window
    extern "C" fn init_with_content_rect(
        this: &Object,
        _sel: Sel,
        content_rect: NSRect,
        style_mask: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
        screen: Id,
    ) -> Id {
        // first, initialize our superior's window
        let this: Id = unsafe {
            msg_send![super(this, this.class().superclass().unwrap()),
                                          initWithContentRect: content_rect
                                          styleMask: style_mask
                                          backing: backing
                                          defer: defer
                                          screen: screen]
        };
        // then, set some basic properties
        unsafe {
            let _: () = msg_send![this, setAcceptsMouseMovedEvents: YES];
            let _: () = msg_send![this, setDelegate: this];
            let _: () = msg_send![this, setReleasedWhenClosed: YES];
        }

        // initialize our root view and set it as so
        let root_view = unsafe {
            let root_view: Id = msg_send![get_vector_view_class(), alloc];
            let root_view: Id = msg_send![root_view, initWithFrame: content_rect];
            root_view
        };
        unsafe { msg_send![this, setContentView: root_view] };
        unsafe { msg_send![root_view, release] };

        // we are done
        this
    }

    unsafe {
        itaos_window.add_method(
            sel!(initWithContentRect: contentRect: styleMask: backing: defer: screen),
            init_with_content_rect
                as extern "C" fn(
                    &Object,
                    Sel,
                    NSRect,
                    NSWindowStyleMask,
                    NSBackingStoreType,
                    BOOL,
                    Id,
                ) -> Id,
        );
    }

    itaos_window.register()
}
