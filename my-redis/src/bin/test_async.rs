// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn learn_song() -> String {
    println!("learn_song");
    String::from("_____")
}

async fn sing_song(song: String) {
    println!("sing_song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    println!("learn_and_sing start");
    let song = learn_song().await;
    sing_song(song).await;
    println!("learn_and_sing end");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    println!("async_main start");
    futures::join!(f1, f2);
    println!("async_main end");
}

fn main() {
    block_on(async_main());
}
