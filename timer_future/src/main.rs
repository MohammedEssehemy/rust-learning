use std::time::Duration;
use timer_future::{new_runtime, TimerFuture};

fn main() {
    let (executor, spawner) = new_runtime();
    let fut = TimerFuture::new(Duration::new(5, 0));
    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(fut);
    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
