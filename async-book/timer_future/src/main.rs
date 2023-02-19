use std::time::Duration;
use timer_future::{AsyncRunTime, TimerFuture};

fn main() {
    let async_runtime = AsyncRunTime::new();
    // Spawn a task to print before and after waiting on a timer.
    async_runtime.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // This will print "howdy!", pause, and then print "done!".
    async_runtime.run();
}
