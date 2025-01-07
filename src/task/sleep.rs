use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

struct Sleep {
    start: Instant,
    duration: Duration,
}

impl Sleep {
    fn new(ms: u64) -> Self {
        Sleep {
            start: Instant::now(),
            duration: Duration::from_millis(ms),
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start.elapsed() >= self.duration {
            Poll::Ready(())
        } else {
            // Register the current task for a wakeup when woken up
            let waker = cx.waker().clone();
            let remaining = self.duration - self.start.elapsed();



            Poll::Pending
        }
    }
}
