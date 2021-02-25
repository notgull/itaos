// MIT/Apache2 License

use crate::window::Window;

mod process;
mod translate;

pub(crate) use process::process_event;
pub(crate) use translate::translate_nsevent;

#[derive(Debug)]
pub enum Event {
    Quit,
    Close(Window),
}
