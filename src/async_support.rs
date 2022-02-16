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

// TODO: Audit to make sure that this is safe to be Send/Sync.
//  Need to research Swift class thread safety. If there are cases where this can be unsafe then
//  we can just have one tokio runtime per thread (lazily initialized) and then run async functions
//  on the same thread that spawned them.
//  Or some other approach that guarantees thread safety.
//  Make sure to think through the implications of non thread-safe types in the async function's
//  arguments or its return type.
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
