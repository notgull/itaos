// MIT/Apache2 License

use super::Directive;
use crate::{manager::data::ManagerData, util::Id, task::ServerTask};
use objc::{runtime::{YES, Object}, rc::StrongPtr};
use std::{ptr, rc::Rc};

const nil: *mut Object = ptr::null_mut();

impl Directive {
    #[inline]
    pub(crate) fn process(self, task: ServerTask, data: &Rc<ManagerData>) {
        match self {
            Directive::Show(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, makeKeyAndOrderFront:nil] };
                let _: () = unsafe { msg_send![data.shared_application, activateIgnoringOtherApps:YES] };
                task.send::<()>(());
            }
            Directive::Hide(win) => {
                let win = unsafe { win.as_ptr() }.as_ptr();
                let _: () = unsafe { msg_send![win, orderOut:nil] };
                task.send::<()>(());
            }
            _ => panic!("Illegal directive"),
        }
    }
}
