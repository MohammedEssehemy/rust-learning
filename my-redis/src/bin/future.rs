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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            println!("Did a check");
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
