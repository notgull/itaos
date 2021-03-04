// MIT/Apache2 License

use crate::{
    directive::Directive,
    manager::get_gt_sender,
    task::{self, ServerTask},
};
use clever_graphics::{context::Context, spawner::Spawner};
use flume::Sender;
use std::any::Any;

/// A spawner used to spawn tasks on our Appkit thread.
#[derive(Debug)]
pub struct AppkitSpawner(Sender<Option<ServerTask>>);

impl AppkitSpawner {
    #[inline]
    pub(crate) fn new() -> Self {
        Self(get_gt_sender())
    }
}

impl Spawner for AppkitSpawner {
    #[inline]
    fn spawn_blocking<T: Send + Sync + 'static, F: FnOnce() -> T + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> orphan_crippler::Receiver<T> {
        let (t, s) = task::create_task::<T>(Directive::OffloadFunction(move |srvtask| {
            let res = f();
            srvtask.complete::<T>(res);
        }));

        if let Err(e) = self.0.try_send(Some(s)) {
            log::error!("Failed to offload task to server: {:?}", e);
        }

        t
    }
}

/// The type that is used to operate graphics.
pub type Graphics = Context<AppkitSpawner>;
