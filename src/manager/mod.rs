// MIT/Apache2 License

use crate::{
    directive::{Directive, DirectiveData},
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
    manager_id: usize,
}

impl GuiThread {
    /// Create a new interface to the GUI thread.
    #[inline]
    pub fn new() -> Self {
        let sender = thread::get_gt_sender();

        // send a task so we can get our manager id
        let (manid_task, srvtask) = create_task::<usize>(Directive {
            id: 0,
            data: DirectiveData::RegisterManager,
        });
        sender
            .try_send(Some(srvtask))
            .expect("Thread shouldn't die while we're initializing it");
        let manager_id = manid_task.wait();
        Self { sender, manager_id }
    }

    /// Create a new interface to the GUI thread, async redox.
    #[inline]
    pub async fn new_async() -> Self {
        let sender = thread::get_gt_sender();

        // send a task so we can get our manager id
        let (manid_task, srvtask) = create_task::<usize>(Directive {
            id: 0,
            data: DirectiveData::RegisterManager,
        });
        sender
            .try_send(Some(srvtask))
            .expect("Thread shouldn't die while we're initializing it");
        let manager_id = manid_task.await;
        Self { sender, manager_id }
    }

    #[inline]
    pub(crate) fn from_raw(sender: Sender<Option<ServerTask>>, id: usize) -> Self {
        Self {
            sender,
            manager_id: id,
        }
    }
}
