// MIT/Apache2 License

use super::Directive;
use crate::{
    manager::data::ManagerData, objc_try, task::ServerTask, util::Id, window::get_window_class, Key,
};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::{
    rc::StrongPtr,
    runtime::{Object, YES},
};
use std::{
    ptr::{self, NonNull},
    rc::Rc,
};

const nil: *mut Object = ptr::null_mut();

impl Directive {
    #[inline]
    pub(crate) fn process(self, task: ServerTask, data: &Rc<ManagerData>) {
        match self {
            Directive::Offload(func) => {
                // we're just offloading some work onto our thread pool, run it
                (func)();
            }
            Directive::Show(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, makeKeyAndOrderFront: nil] };
                let _: () =
                    unsafe { msg_send![data.shared_application, activateIgnoringOtherApps: YES] };
                task.send::<()>(());
            }
            Directive::Hide(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, orderOut: nil] };
                task.send::<()>(());
            }
            Directive::Close(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, close] };
                task.send::<()>(());
            }
            Directive::CreateWindow {
                x,
                y,
                width,
                height,
                style,
                backing,
                defer,
                screen,
            } => {
                // create a new window
                let content_rect = NSRect {
                    origin: NSPoint { x, y },
                    size: NSSize { width, height },
                };
                let screen = match screen {
                    Some(screen) => unsafe { screen.as_ptr() }.as_ptr(),
                    None => nil,
                };
                let win: crate::Result<Id> = objc_try!(unsafe {
                    let win: Id = msg_send![get_window_class(), alloc];
                    msg_send![win, initWithContentRect:content_rect
                                   styleMask:style
                                   backing:backing
                                   defer:{ if defer { 1 } else { 0 } }
                                   screen:screen]
                });

                task.send::<crate::Result<Key>>(match win {
                    Err(e) => Err(e),
                    Ok(nil) => {
                        panic!("Unless an exception was thrown, win should not be null")
                    }
                    Ok(win) => {
                        // SAFETY: we know win is non-null
                        Key::from_ptr_nn(unsafe { NonNull::new_unchecked(win) })
                    }
                });
            }
            Directive::Quit => {}
            _ => panic!("Illegal directive"),
        }
    }
}
