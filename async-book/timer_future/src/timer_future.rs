use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

pub struct TimerFuture {
    start: Instant,
    duration: Duration,
    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this to tell
    /// `TimerFuture`'s task to wake up and move forward.
    waker: Arc<Mutex<Option<Waker>>>,
}


impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        if self.is_completed() {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled again
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            let mut self_waker = self.waker.lock().unwrap();
            *self_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}


impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided timeout.
    pub fn new(duration: Duration) -> Self {
        let timer_future = TimerFuture { 
            start: Instant::now(),
            duration,
            waker: Arc::new(Mutex::new(None))
         };


        // Spawn the new thread
        let arc_waker = timer_future.waker.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut mutex_waker = arc_waker.lock().unwrap();
            // Wake up the last task on which the future was polled, if exists.
            if let Some(waker) = mutex_waker.take() {
                waker.wake();
            }
        });
        timer_future
    }

    fn is_completed(&self) -> bool {
        Instant::now() >= (self.start + self.duration)
    }
}