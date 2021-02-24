// MIT/Apache2 License

use crate::{
    event::Event,
    manager::{DirectiveThreadMessage, GuiThread},
    task::ServerTask,
};
use flume::{Receiver, Sender};
use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

/// Thread-side data associated with the runtime manager.
pub(crate) struct ManagerData {
    pub(crate) runtime_id: usize,
    pub(crate) event_handler: RefCell<Arc<dyn Fn(&GuiThread, Event) + Send + Sync>>,
    pub(crate) window_count: Cell<usize>,
    pub(crate) waiting: Cell<bool>,
    pub(crate) directive_sender: Sender<Option<ServerTask>>,
    pub(crate) directive_receiver: Receiver<Option<ServerTask>>,
    pub(crate) message_sender: Sender<DirectiveThreadMessage>,
}
