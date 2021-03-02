// MIT/Apache2 License

use crate::{task::ServerTask, window::Window, Key};
use cocoa::appkit::{NSBackingStoreType, NSWindowStyleMask};

mod process;

pub(crate) enum Directive {
    OffloadFunction(Box<dyn FnOnce(ServerTask) + Send + Sync>),
    Quit,
    CreateWindow {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: bool,
        screen: Option<Key>,
    },
    Hide(Window),
    Show(Window),
    Close(Window),
    Move {
        window: Window,
    },
}
