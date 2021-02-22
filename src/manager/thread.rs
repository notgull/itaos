// MIT/Apache2 License

use crate::{
    task::ServerTask,
    util::{Id, ThreadSafe},
};
use core_foundation::runloop;
use flume::{Receiver, Sender};
use objc::{class, msg_send, rc::StrongPtr, sel, Class, ClassDecl};
use once_cell::sync::Lazy;
use std::{ptr, thread};

static GUI_THREAD: Lazy<Sender<ServerTask>> = Lazy::new(|| {
    let (send, recv) = flume::unbounded();

    thread::Builder::new()
        .name("itaos-runtime-thread".to_string())
        .spawn(move || {
            // put a massive ObjC exception catcher over this thread, just in case we miss something
            let f = move || {
                // we increment this number for every "runtime" that's created, and we only quit once
                // it's reached zero
                let mut runtime_count = 0usize;

                // set up cocoa's refcount mechanism
                let NSAutoreleasePool = class!(NSAutoreleasePool);
                let pool = unsafe {
                    let pool: Id = msg_send![NSAutoReleasePool, alloc];
                    let pool: Id = msg_send![pool, init];
                    StrongPtr::new(pool)
                };

                // initialize the shared application
                let NSApplication = class!(NSApplication);
                let shared_app: Id = unsafe { msg_send![NSApplication, sharedApplication] };

                // safety: we use shared_app in a thread-safe way
                let shared_app = unsafe { ThreadSafe::new(shared_app) };

                // create the class we use to store tasks
                let itaos_event = super::eclass::create_itaosevent_class();

                // start another thread dedicated to receiving directives from the receiver
                thread::Builder::new()
                    .name("itaos-directive-thread")
                    .spawn(move || loop {
                        match recv.recv() {
                            Err(_) => break,
                            Ok(srvtask) => {
                                // push it onto the heap, then put the pointer in an event
                                let srvtask = Box::into_raw(Box::new(srvtask)) as *mut _;
                                let event = unsafe {
                                    let event: Id = msg_send![itaos_event, alloc];
                                    let event: Id = msg_send![event, initWithDirective: srvtask];
                                    event
                                };
                                let _: () = unsafe {
                                    msg_send![shared_app.into_inner(), postEvent: event
                                                                                        atStart: 1]
                                };
                            }
                        }
                    });

                // main event loop
                let date_class = class!(NSDate);
                let date: Id = msg_send![date_class, distantFuture];
                loop {
                    // get an event from the event queue
                    let event: Id = unsafe {
                        msg_send![shared_app, nextEventMatchingMask: NSEventMaskAny
                                              untilDate: date
                                              inMode: NSDefaultRunLoopMode
                                              dequeue: YES]
                    };

                    // interpret a null event as a break
                    if event.is_null() {
                        break;
                    }

                    // if the event is one of ours, process the directive
                    let ty: appkit::NSUInteger = unsafe { msg_send![event, type] };
                    let subty: i16 = unsafe { msg_send![event, subtype] };
                    if ty == appkit::NSEventType::NSApplicationDefined as _
                        && subty == super::eclass::DIRECTIVE_EVENT_SUBTYPE
                    {
                        let srvtask: *mut c_void = unsafe { msg_send![event, directive] };
                        let srvtask: Box<ServerTask> = unsafe { Box::from_raw(directive.cast()) };
                        let directive = srvtask.directive();
                        directive.process(srvtask);
                    } else {
                        // send the event on
                        let _: () = unsafe { msg_send![shared_app, sendEvent: event] };
                    }
                }

                // dropping pool should automatically drain the pool
            };

            if unsafe { obj_exception::r#try(f) }.is_err() {
                panic!("Uncaught exception");
            }
        });

    send
});
