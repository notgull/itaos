// MIT/Apache2 License

use crate::{graphics::Graphics, window::Window};
use gluten_keyboard::Key;

mod process;
mod translate;

pub(crate) use process::process_event;
pub(crate) use translate::translate_nsevent;

#[derive(Debug)]
pub enum Event {
    Quit,
    Close(Window),
    Paint(Window, Graphics),
    ButtonDown {
        window: Window,
        button: MouseButton,
        x: f64,
        y: f64,
    },
    ButtonUp {
        window: Window,
        button: MouseButton,
        x: f64,
        y: f64,
    },
    MouseDrag {
        window: Window,
        button: MouseButton,
        x: f64,
        y: f64,
    },
    MouseMove {
        window: Window,
        x: f64,
        y: f64,
    },
    MouseEntered { window: Window, x: f64, y: f64 },
    MouseExited { window: Window, x: f64, y: f64 },
    KeyUp { window: Window, key: Key },
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}
