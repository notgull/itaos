// MIT/Apache2 License

use super::DirectiveData;
use crate::{manager::data::ManagerData, util::Id, task::ServerTask};
use objc::{runtime::{YES, Object}, rc::StrongPtr};
use std::{ptr, rc::Rc};

impl DirectiveData {
    #[inline]
    pub(crate) fn process(self, task: ServerTask, data: &Rc<ManagerData>) {
        std::thread_local! {
            static SENDER_OBJECT: StrongPtr = {
                let ns_object = class!(NSObject);
                let sender_object: Id = unsafe { msg_send![ns_object, alloc] };
                let sender_object: Id = unsafe { msg_send![ns_object, init] };
                unsafe { StrongPtr::new(sender_object) }
            };
        }

        match self {
            DirectiveData::Show(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, makeKeyAndOrderFront:SENDER_OBJECT] };
                let _: () = unsafe { msg_send![data.shared_application, activateIgnoringOtherApps:YES] };
                task.send::<()>(());
            }
            DirectiveData::Hide(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, orderOut:SENDER_OBJECT] };
                task.send::<()>(());
            }
            _ => panic!("Illegal directive"),
        }
    }
}
