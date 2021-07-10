// MIT/Apache2 License

/// A macro similar to the `msg_send` macro in the `objc` crate, but returns an error instead of panicking.
#[doc(hidden)]
#[macro_export]
macro_rules! msg_send {
    (super($obj:expr, $superclass:expr), $name:ident) => ({
        let sel = objc::sel!($name);
        let result;
        match objc::__send_super_message(&*$obj, $superclass, sel, ()) {
            Err(s) => return Err(s.into()),
            Ok(r) => result = r,
        }
        result
    });
    (super($obj:expr, $superclass:expr), $($name:ident : $arg:expr)+) => ({
        let sel = objc::sel!($($name:)+);
        let result;
        match objc::__send_super_message(&*$obj, $superclass, sel, ($($arg,)*)) {
            Err(s) => return Err(s.into()),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $name:ident) => ({
        let sel = objc::sel!($name);
        let result;
        match objc::__send_message(&*$obj, sel, ()) {
            Err(s) => return Err(s.into()),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $($name:ident : $arg:expr)+) => ({
        let sel = objc::sel!($($name:)+);
        let result;
        match objc::__send_message(&*$obj, sel, ($($arg,)*)) {
            Err(s) => return Err(s.into()),
            Ok(r) => result = r,
        }
        result
    });
}
