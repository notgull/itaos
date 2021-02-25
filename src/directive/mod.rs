// MIT/Apache2 License

use crate::window::Window;

mod process;

pub(crate) enum Directive {
    Quit,
    CreateWindow,
    Hide(Window),
    Show(Window),
    Move {
        window: Window,
    },
}
