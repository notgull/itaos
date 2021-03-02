// MIT/Apache2 License

use crate::{graphics::Graphics, window::Window};

mod process;
mod translate;

pub(crate) use process::process_event;
pub(crate) use translate::translate_nsevent;

#[derive(Debug)]
pub enum Event {
    Quit,
    Close(Window),
    Paint(Window, Graphics),
}
