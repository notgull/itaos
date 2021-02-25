// MIT/Apache2 License

use crate::window::Window;

mod process;

pub(crate) struct Directive {
    pub(crate) id: usize,
    pub(crate) data: DirectiveData,
}

pub(crate) enum DirectiveData {
    Quit,
    RegisterManager,
    CreateWindow,
    Hide(Window),
    Show(Window),
    Move {
        window: Window,
    },
}
