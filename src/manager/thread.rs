// MIT/Apache2 License

use super::{data::ManagerData, GuiThread};
use crate::{
    directive::Directive,
    event::Event,
    task::ServerTask,
    util::{memslot, Id, ThreadSafe},
};
use cocoa::{appkit, foundation};
use flume::{Receiver, Sender, TryRecvError};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    rc::StrongPtr,
    runtime::{Class, YES},
    sel,
};
use once_cell::sync::Lazy;
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    ptr,
    rc::Rc,
    sync::Arc,
    thread,
};
use tinyvec::{ArrayVec, TinyVec};

#[inline]
pub(crate) fn get_gt_sender() -> Sender<Option<ServerTask>> {
    GUI_THREAD.clone()
}

static GUI_THREAD: Lazy<Sender<Option<ServerTask>>> = Lazy::new(|| {
    let (mut send, recv) = flume::unbounded();
    let manager_copy = send.clone();

    thread::Builder::new()
        .name("itaos-runtime-thread".to_string())
        .spawn(move || {
            // put a massive ObjC exception catcher over this thread, just in case we miss something
            let f = move || {
                // set up cocoa's refcount mechanism
                let NSAutoreleasePool = class!(NSAutoreleasePool);
                let pool = unsafe {
                    let pool: Id = msg_send![NSAutoreleasePool, alloc];
                    let pool: Id = msg_send![pool, init];
                    StrongPtr::new(pool)
                };

                // create the channel that we tell the directive thread to stop or start on
                let (dt_send, dt_recv) = flume::unbounded::<DirectiveThreadMessage>();

                let dt_scopy = send.clone();
                let dt_rcopy = recv.clone();

                // initialize the shared application
                let NSApplication = class!(NSApplication);
                let shared_app: Id = unsafe { msg_send![NSApplication, sharedApplication] };

                // safety: we use shared_app in a thread-safe way
                let shared_app = unsafe { ThreadSafe::new(shared_app) };

                // window data we pass around between each window
                let manager_data = Rc::new(ManagerData {
                    event_handler: RefCell::new(Arc::new(|_, ev| log::warn!("Event ignored: {:?}", ev))),
                    window_count: Cell::new(0),
                    waiting: Cell::new(false),
                    directive_sender: send.clone(),
                    directive_receiver: recv.clone(),
                    message_sender: dt_send.clone(),
                    shared_application: shared_app.into_inner(),
                });

                // create the class we use to store tasks
                let itaos_event = super::eclass::create_itaosevent_class();

                // start another thread dedicated to receiving directives from the receiver
                thread::Builder::new()
                    .name("itaos-directive-thread".to_string())
                    .spawn(move || 'dtloop: loop {
                        // first, see if there are messages to process
                        match dt_recv.try_recv() {
                            Err(TryRecvError::Empty) => (),
                            Err(TryRecvError::Disconnected) => break 'dtloop,
                            Ok(DirectiveThreadMessage::Start) => (),
                            Ok(DirectiveThreadMessage::RunEvent(..)) => unreachable!(),
                            Ok(DirectiveThreadMessage::Stop) => loop {
                                match dt_recv.recv() {
                                    Err(_) => break 'dtloop,
                                    Ok(DirectiveThreadMessage::Start) => break,
                                    Ok(DirectiveThreadMessage::Stop) => (),
                                    Ok(DirectiveThreadMessage::RunEvent(ref gt, ev, func)) => {
                                        (func)(gt, ev);
                                        dt_scopy.send(None).unwrap();
                                    }
                                }
                            },
                        }

                        match dt_rcopy.recv() {
                            Err(_) => break,
                            Ok(None) => {
                                // this is forcing us to process a message
                            }
                            Ok(Some(srvtask)) => {
                                // push it onto the heap, then put the pointer in an event
                                let srvtask = Box::into_raw(Box::new(srvtask)) as *mut _;
                                let event = unsafe {
                                    let event: Id = msg_send![itaos_event, alloc];
                                    let event: Id = msg_send![event, initWithDirective:srvtask];
                                    event
                                };
                                let _: () = unsafe {
                                    msg_send![shared_app.into_inner(), postEvent: event
                                                                       atStart: YES]
                                };
                            }
                        }
                    }).expect("Unable to spawn directive thread");

                // main event loop
                let date_class = class!(NSDate);
                let date: Id = unsafe { msg_send![date_class, distantFuture] };
                loop {
                    // get an event from the event queue
                    let event: Id = unsafe {
                        msg_send![*shared_app, nextEventMatchingMask: appkit::NSEventMask::NSAnyEventMask
                                               untilDate: date
                                               inMode: foundation::NSDefaultRunLoopMode
                                               dequeue: YES]
                    };

                    // interpret a null event as a break
                    if event.is_null() {
                        break;
                    }

                    // if the event is one of ours, process the directive
                    let ty: foundation::NSUInteger = unsafe { msg_send![event, type] };
                    let subty: i16 = unsafe { msg_send![event, subtype] };
                    if ty == appkit::NSEventType::NSApplicationDefined as _
                        && subty == super::eclass::DIRECTIVE_EVENT_SUBTYPE
                    {
                        let srvtask: *mut c_void = unsafe { msg_send![event, directive] };
                        let mut srvtask: Box<ServerTask> = unsafe { Box::from_raw(srvtask.cast()) };
                        let directive = srvtask.input().unwrap();
                        directive.process(*srvtask, &manager_data);
                    } else {
                        // this event may or may not be relevant to our user
                        if let Some(ev) = crate::event::translate_nsevent(event) {
                            crate::event::process_event(&manager_data, ev);
                        }
                        // send the event on
                        let _: () = unsafe { msg_send![*shared_app, sendEvent: event] };
                    }
                }

                // dropping pool should automatically drain the pool
            };

            //if unsafe { obj_exception::r#try(f) }.is_err() {
            //    panic!("Uncaught exception");
            //}
            f();
        }).expect("Unable to spawn runtime thread");

    manager_copy
});

pub(crate) enum DirectiveThreadMessage {
    Start,
    Stop,
    RunEvent(
        GuiThread,
        Event,
        Arc<dyn Fn(&GuiThread, Event) + Send + Sync>,
    ),
}
