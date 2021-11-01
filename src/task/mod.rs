pub mod executor;

use alloc::boxed::Box;
use core::future::Future;
use core::sync::atomic::{AtomicU64, Ordering};
use core::task::{Context, Poll};
use futures::task::FutureObj;
use futures::FutureExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);
impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskId, // new
    future: FutureObj<'static, ()>,
}

impl Task {
    pub fn new<Fut: Future<Output = ()> + Send + 'static>(future: Fut) -> Task {
        Task {
            id: TaskId::new(),
            future: FutureObj::new(Box::pin(future)),
        }
    }
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.future.poll_unpin(cx)
    }
}
