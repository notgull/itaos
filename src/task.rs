// MIT/Apache2 

use crate::directive::Directive;
use orphan_crippler::{Sender, Receiver, two};
use std::any::Any;

pub type Task<T> = Receiver<T>;
pub(crate) type ServerTask = Sender<Directive>;

#[inline]
pub(crate) fn create_task<T: Any + Send>(dir: Directive) -> (Task<T>, ServerTask) {
    let (send, recv) = two::<Directive, T>(dir);
    (recv, send)
}
