// MIT/Apache2 License

use crate::window::Window;

#[derive(Debug)]
pub enum Event {
    Quit,
    Close(Window),
}
