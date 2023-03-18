use futures::{future, select};
fn main () {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => {
                total += a;
                println!("a is done");
            },
            b = b_fut => {
                total += b;
                println!("b is done");
            },
            complete => {
                println!("all done");
                break;
            },
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    assert_eq!(total, 10);
}