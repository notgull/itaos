// MIT/Apache2 License

use crate::{directive_event_class, Directive};
use breadthread::{
    AddOrRemovePtr, BreadThread, Completer, Controller, LoopCycle, PinnedThreadHandle, ThreadHandle,
};
use objc::{
    prelude::*,
    runtime::{Object, YES},
};
use once_cell::unsync::{Lazy, OnceCell};
use orphan_crippler::{two, Receiver, Sender};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    mem,
    sync::atomic::{Ordering, Usize},
};

pub struct ItaosThread<'evh> {
    inner: BreadThread<'evh, ItaosController<'evh>>,
}

impl<'evh> ItaosThread<'evh> {
    #[inline]
    pub fn new() -> crate::Result<ItaosThread<'evh>> {
        struct UndoOnDrop;

        impl Drop for UndoOnDrop {
            #[cold]
            fn drop(&mut self) {
                ONE_AND_DONE.store(false, Ordering::SeqCst);
            }
        }

        if ONE_AND_DONE
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(crate::Error::Exclusive);
        }

        let _guard = UndoOndrop;

        let bt = BreadThread::try_new(ItaosController {
            handle: OnceCell::new(),
            distant_future: Lazy::new(|| {
                let NSDate = class!(NSDate);
                let distant_future: *mut Object = unsafe { msg_send![NSDate, distantFuture] };
                distant_future
            }),
            shared_application: Lazy::new(|| {
                let NSApplication = class!(NSApplication);
                let shared_application: *mut Object =
                    unsafe { msg_send![NSApplication, sharedApplication] };
                shared_application
            }),
            stop_loop: Cell::new(false),
        })?;
        let th = bt.handle().pin();

        bt.with(|ctrl| ctrl.handle.set(th).unwrap());

        mem::forget(_guard);

        Ok(bt)
    }
}

pub struct ItaosThreadHandle<'evh> {
    inner: ThreadHandle<'evh, ItaosController<'evh>>,
}

pub struct PinnedItaosThreadHandle<'evh> {
    inner: PinnedThreadHandle<'evh, ItaosController<'evh>>,
}

pub trait Sender {
    #[doc(hidden)]
    fn send<T: Any + Send>(&self, directive: Directive) -> crate::Result<Receiver<T>>;
}

impl<'evh> Sender for ItaosThread<'evh> {
    #[inline]
    fn send<T: Any + Send>(&self, directive: Directive) -> crate::Result<Receiver<T>> {
        let r = self.inner.send_directive(directive)?;
        Ok(r)
    }
}

impl<'evh> Sender for ItaosThreadHandle<'evh> {
    #[inline]
    fn send<T: Any + Send>(&self, directive: Directive) -> crate::Result<Receiver<T>> {
        let r = self.inner.send_directive(directive)?;
        Ok(r)
    }
}

pub(crate) struct ItaosController<'evh> {
    // handle to the bread thread
    handle: OnceCell<PinnedItaosThreadHandle<'evh>>,
    // cached value of "[NSDate distantFuture]"
    distant_future: Lazy<*mut Object>,
    // cached value of "[NSApplication sharedApplication]"
    shared_application: Lazy<*mut Object>,
    // should we break the application?
    stop_loop: Cell<bool>,
}

impl<'evh> Controller for ItaosController<'evh> {
    type Directive = Directive;
    type DirectiveAdaptor = DirectiveAdaptor;
    type Error = crate::Error;
    type Event = crate::Event;
    type Pointers = Option<AddOrRemovePtr>;

    #[inline]
    fn directive_adaptor(&self) -> DirectiveAdaptor {
        DirectiveAdaptor {
            app: *self.shared_application,
        }
    }

    #[inline]
    fn loop_cycle(&self) -> Result<LoopCycle<Event, Directive>, Error> {
        use cocoa::appkit::NSEventMask;

        let distant_future = *self.distant_future;
        let app = *self.shared_application;

        loop {
            if self.stop_loop.get() {
                return LoopCycle::Break;
            }

            // get an event
            let event: *mut Object = unsafe {
                msg_send![app, nextEventMatchingMask: NSEventMask::NSAnyEventMask.bits()
                               untilDate: distant_future
                               inMode: cocoa::foundation::NSDefaultRunLoopMode
                               dequeue: YES]
            };

            if !event.is_null() {
                // get the event's type
                let ty: cocoa::foundation::NSUInteger = unsafe { msg_send![event, type] };
                let subty: c_short = unsafe { msg_send![event, subtype] };

                // if this is a directive, transform it into one and then return it to be processed
                if ty == cocoa::foundation::NSEventType::NSApplicationDefined as u64
                    && subty == crate::NS_DIRECTIVE_TYPE
                {
                    let directive: *mut c_void = unsafe { msg_send![event, itaosDirective] };
                    let directive: Box<Sender<Directive>> = Box::from_raw(directive.cast());

                    return LoopCycle::Directive(*directive);
                }

                // send the event on, but clone a reference to it first
                let e: *mut Object = unsafe { msg_send![event, retain] };
                let _release = ReleaseOnDrop(e);
                let _: () = unsafe { msg_send![app, sendEvent: event] };

                if let Some(event) = crate::parse_event(e, ty, subty) {
                    return LoopCycle::Event(event);
                }
            }
        }
    }

    #[inline]
    fn process_directive<C: Completer>(
        &self,
        directive: Directive,
        completer: &mut C,
    ) -> Option<AddOrRemovePtr> {
        directive.process(self, completer)
    }
}

struct DirectiveAdaptor {
    app: *mut Object,
}

// SAFETY: Although NSApplication is not thread safe, the "postEvent" message is, and we only use that.
unsafe impl Send for DirectiveAdaptor {}

impl breadthread::DirectiveAdaptor<Directive> for DirectiveAdaptor {
    #[inline]
    fn send(&mut self, directive: Sender<Directive>) {
        // create an instance of ItaosDirectiveEvent
        let de: *mut Object = unsafe { msg_send![directive_event_class(), alloc] };

        let directive: *mut c_void = Box::into_raw(Box::new(directive)).cast();

        let de: *mut Object = unsafe { msg_send![de, initWithDirective: directive] };

        // put it into the event queue, preferably close to the front
        let _: () = unsafe { msg_send![self.app, postEvent: de atStart: YES] };
    }
}

/// There can only ever be one `ItaosController` in any given program.
static ONE_AND_DONE: AtomicBool = AtomicBool::new(false);

struct ReleaseOnDrop(*mut Object);

impl Drop for ReleaseOnDrop {
    #[inline]
    fn drop(&mut self) {
        let _: () = unsafe { msg_send![self.0, release] };
    }
}
