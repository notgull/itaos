// MIT/Apache2 License

use crate::{util::Id, lazy_class::LazyClass, manager::data::ManagerData};
use objc::{declare::ClassDecl, runtime::{Sel, Class, Object}};

static ITAOS_VECTOR_SUBVIEW_CLASS: LazyClass = LazyClass::new();

#[inline]
fn create_itaos_vector_subview_class() -> &'static Class {
    let mut subview_class = ClassDecl::new("ItaosVectorSubview", class!(NSView));

    subview_class.register()
}
