// MIT/Apache2 License

use super::DirectiveData;
use crate::{manager::data::ManagerData, task::ServerTask};
use std::rc::Rc;

impl DirectiveData {
    #[inline]
    pub(crate) fn process(self, task: ServerTask, data: &Rc<ManagerData>) {}
}
