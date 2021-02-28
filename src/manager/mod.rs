// MIT/Apache2 License

use crate::{
    directive::Directive,
    task::{create_task, ServerTask},
};
use flume::{Receiver, Sender};

pub(crate) mod data;
pub(crate) mod eclass;
mod thread;

pub(crate) use thread::DirectiveThreadMessage;

/// Runs the thread that manages the Appkit thread.
pub struct GuiThread {
    sender: Sender<Option<ServerTask>>,
}

impl GuiThread {
    /// Create a new interface to the GUI thread.
    #[inline]
    pub fn new() -> Self {
        let sender = thread::get_gt_sender();

        Self { sender }
    }

    #[inline]
    pub(crate) fn from_raw(sender: Sender<Option<ServerTask>>) -> Self {
        Self { sender }
    }

    #[inline]
    pub(crate) fn into_inner(self) -> Sender<Option<ServerTask>> {
        self.sender
    }
}
