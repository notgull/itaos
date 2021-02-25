// MIT/Apache2 License

use crate::{
    event::Event,
    manager::{data::ManagerData, DirectiveThreadMessage, GuiThread},
};
use std::rc::Rc;

/// Given an event, run the event handler.
#[inline]
pub(crate) fn process_event(mdata: &Rc<ManagerData>, event: Event) {
    // tell the directive processing thread to stop
    mdata
        .message_sender
        .send(DirectiveThreadMessage::Stop)
        .unwrap();
    mdata.directive_sender.send(None).unwrap();

    // tell the directive processing thread to run the event handler
    let gt = GuiThread::from_raw(mdata.directive_sender.clone(), mdata.runtime_id);
    mdata
        .message_sender
        .send(DirectiveThreadMessage::RunEvent(
            gt,
            event,
            mdata.event_handler.borrow().clone(),
        ))
        .unwrap();

    // while it's running the event handler, process events
    loop {
        match mdata.directive_receiver.recv() {
            Ok(None) | Err(_) => break, // Ok(None) tells us we're done with it
            Ok(Some(mut srvtask)) => {
                let directive = srvtask.input().unwrap();
                directive.data.process(srvtask, mdata);
            }
        }
    }

    // tell the directive thread that it's okay to start again
    mdata
        .message_sender
        .send(DirectiveThreadMessage::Stop)
        .unwrap();
}
