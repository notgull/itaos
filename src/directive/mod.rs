// MIT/Apache2 License

mod process;

pub(crate) struct Directive {
    pub(crate) id: usize,
    pub(crate) data: DirectiveData,
}

pub(crate) enum DirectiveData {
    Quit,
    RegisterManager,
}
