// MIT/Apache2 License

use super::{Event, MouseButton};
use crate::{util::Id, Key};
use cocoa::{appkit::NSEventType, foundation};
use std::mem;

/// Translate any given NSEvent into the appropriate iTaos event, and also return the runtime ID.
#[inline]
pub(crate) fn translate_nsevent(event: Id) -> Option<Event> {
    let ty: foundation::NSUInteger = unsafe { msg_send![event, type] };

    // SAFETY: NSEvents are guaranteed to be one of the above if it isn't zero
    let ty: NSEventType = if ty >= 1 && ty <= 34 && ![21, 32, 33].contains(&ty) {
        return None;
    } else {
        unsafe { mem::transmute::<foundation::NSUInteger, NSEventType>(ty) }
    };

    match ty {
        NSEventType::NSLeftMouseDown
        | NSEventType::NSRightMouseDown
        | NSEventType::NSOtherMouseDown
        | NSEventType::NSLeftMouseUp
        | NSEventType::NSRightMouseUp
        | NSEventType::NSOtherMouseUp => return translate_button_event(event, ty),
        NSEventType::NSLeftMouseDragged
        | NSEventType::NSRightMouseDragged
        | NSEventType::NSOtherMouseDragged
        | NSEventType::NSMouseMoved => return translate_mouse_event(event, ty),
        NSEventType::NSMouseEntered | NSEventType::NSMouseExited => {
            return translate_enter_event(event, ty)
        }
        _ => (),
    }

    None
}

#[inline]
fn translate_button_event(event: Id, ty: NSEventType) -> Option<Event> {
    let button = mouse_button_for_event(event)?;
    let (window, x, y) = window_and_point_for_event(event);
    let window = Key::from_ptr(window.cast())?;

    match ty {
        NSEventType::NSLeftMouseDown
        | NSEventType::NSRightMouseDown
        | NSEventType::NSOtherMouseDown => Some(Event::ButtonDown {
            window,
            x,
            y,
            button,
        }),
        NSEventType::NSLeftMouseUp | NSEventType::NSRightMouseUp | NSEventType::NSOtherMouseUp => {
            Some(Event::ButtonUp {
                window,
                x,
                y,
                button,
            })
        }
        _ => unreachable!(),
    }
}

#[inline]
fn mouse_button_for_event(event: Id) -> Option<MouseButton> {
    let button: foundation::NSInteger = unsafe { msg_send![event, buttonNumber] };
    Some(match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => {
            log::warn!("Unrecognized mouse button: {}", button);
            return None;
        }
    })
}

#[inline]
fn translate_mouse_event(event: Id, ty: NSEventType) -> Option<Event> {
    let button = mouse_button_for_event(event)?;
    let (window, x, y) = window_and_point_for_event(event);
    let window = Key::from_ptr(window.cast())?;

    match ty {
        NSEventType::NSLeftMouseDragged
        | NSEventType::NSRightMouseDragged
        | NSEventType::NSOtherMouseDragged => Some(Event::MouseDrag {
            window,
            x,
            y,
            button,
        }),
        NSEventType::NSMouseMoved => Some(Event::MouseMove { window, x, y }),
        _ => unreachable!(),
    }
}

#[inline]
fn translate_enter_event(event: Id, ty: NSEventType) -> Option<Event> {
    let (window, x, y) = window_and_point_for_event(event);
    let window = Key::from_ptr(window.cast())?;

    match ty {
        NSEventType::NSMouseEntered => Some(Event::MouseEntered { window, x, y }),
        NSEventType::NSMouseExited => Some(Event::MouseExited { window, x, y }),
        _ => unreachable!(),
    }
}

#[inline]
fn window_and_point_for_event(event: Id) -> (Id, f64, f64) {
    let window: Id = unsafe { msg_send![event, window] };
    let point: foundation::NSPoint = unsafe { msg_send![event, locationInWindow] };
    (window, point.x, point.y)
}
