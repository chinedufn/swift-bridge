use once_cell::sync::Lazy;
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, SyncSender};

#[doc(hidden)]
pub static ASYNC_RUNTIME: Lazy<TokioRuntime> = Lazy::new(|| {
    let (sender, receiver) = std::sync::mpsc::sync_channel(10_000);

    let runtime = TokioRuntime { sender };

    runtime.start_runtime(receiver);

    runtime
});
type AsyncFnToSpawn = Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

#[doc(hidden)]
pub struct TokioRuntime {
    sender: SyncSender<AsyncFnToSpawn>,
}

// TODO: Audit to make sure that this is safe. Need to research Swift class thread safety
#[doc(hidden)]
pub struct SwiftCallbackWrapper(pub *mut std::ffi::c_void);
unsafe impl Send for SwiftCallbackWrapper {}
unsafe impl Sync for SwiftCallbackWrapper {}

#[doc(hidden)]
impl TokioRuntime {
    pub fn spawn_task(&self, task: AsyncFnToSpawn) {
        self.sender.send(task).unwrap();
    }

    fn start_runtime(&self, receiver: Receiver<AsyncFnToSpawn>) {
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    while let Ok(task) = receiver.recv() {
                        tokio::spawn(task);
                    }
                })
        });
    }
}
