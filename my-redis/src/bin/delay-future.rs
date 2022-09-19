// use tokio::net::TcpStream;

// async fn my_async_fn() {
//     println!("hello from async");
//     let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap_err();
//     println!("async TCP operation complete");
// }

// async fn my_future() {
//     println!("in closure");
// }

// #[tokio::main]
// async fn main() {
//     let what_is_this = my_async_fn();
//     println!("before execution");
//     let future = my_future();
//     // Nothing has been printed yet.
//     tokio::join!(what_is_this, future);
//     println!("after execution");
//     // Text has been printed and socket has been
//     // established and closed.
// }

// ----------------------------------------------

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    println!("before first");
    let half_sec = 5_u32 * 10_u32.pow(8);
    let start= Instant::now();
    let when = start + Duration::new(3, half_sec);
    let future = Delay { when };
    let out = future.await;
    let diff = Instant::now().duration_since(start);
    println!("first function: {}, {:?}", out, diff);

    // using notifier
    println!("before second");
    let start= Instant::now();
    let out2 = delay(Duration::new(3, half_sec)).await;
    let diff = Instant::now().duration_since(start);
    println!("second function: {}, {:?}", out2, diff);
}


use tokio::sync::Notify;
use std::sync::Arc;
use std::thread;


// using notifier
async fn delay(dur: Duration) -> String {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });


    notify.notified().await;
    String::from("done")
}
