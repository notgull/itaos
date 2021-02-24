// MIT/Apache2 License

use crate::{event::Event, manager::GuiThread};
use std::cell::RefCell;

/// Thread-side data associated with the runtime manager.
pub(crate) struct ManagerData {
    pub(crate) runtime_id: usize,
    pub(crate) event_handler: RefCell<Box<dyn Fn(&GuiThread, Event)>>,
}
