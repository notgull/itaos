// MIT/Apache2 License

#[macro_use]
extern crate objc;

pub(crate) mod directive;
pub(crate) mod error;
pub(crate) mod lazy_class;
pub(crate) mod util;

pub mod event;
pub mod graphics;
pub mod key;
pub mod manager;
pub mod task;
pub mod window;

pub(crate) use directive::Directive;
pub(crate) use util::Id;

pub use error::{Error, Result};
pub use event::Event;
pub use key::Key;
pub use task::Task;

#[doc(hidden)]
#[macro_export]
macro_rules! objc_try {
    ($expr: expr) => {{
        (clever_graphics::objc_try!($expr)).map_err(|e| $crate::Error::ObjcException(e))
    }};
}
