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
    MouseEntered {
        window: Window,
        x: f64,
        y: f64,
    },
    MouseExited {
        window: Window,
        x: f64,
        y: f64,
    },
    KeyUp {
        window: Window,
        key: Key,
        modifiers: Modifiers,
    },
    KeyDown {
        window: Window,
        key: Key,
        modifiers: Modifiers,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct Modifiers {
        const SHIFT = 1 << 0;
        const CTRL = 1 << 1;
        const ALT = 1 << 2;
        const CAPS_LOCK = 1 << 3;
        const COMMAND = 1 << 4;
    }
}
