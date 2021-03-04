// MIT/Apache2 License

pub(crate) mod view;
pub(crate) mod window;

use crate::{directive::Directive, manager::GuiThread, task::Task, Key};
use cocoa::appkit::{NSBackingStoreType, NSWindowStyleMask};

pub(crate) use view::get_vector_view_class;
pub(crate) use window::get_window_class;

pub type Window = Key;

impl GuiThread {
    #[inline]
    pub fn create_window(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: bool,
        screen: Option<Key>,
    ) -> crate::Result<Task<crate::Result<Window>>> {
        self.send_directive(Directive::CreateWindow {
            x,
            y,
            width,
            height,
            style,
            backing,
            defer,
            screen,
        })
    }
}
