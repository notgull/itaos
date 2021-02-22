// MIT/Apache2 License

use crate::directive::Directive;
use flume::{Receiver, Sender};

pub(crate) mod eclass;
mod thread;

/// Runs the thread that manages the Appkit thread.
pub struct GuiThread {
    sender: Sender<Directive>,
}
