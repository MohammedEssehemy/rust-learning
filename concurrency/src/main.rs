// use std::thread;
// use std::time::Duration;

// fn main() {
//     let spawned_thread_handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     spawned_thread_handle.join().unwrap();
// }

// ------------------------------
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let other_thread = thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         tx.send("test send".to_owned()).unwrap();
//         println!("sent")
//     });

//     other_thread.join().unwrap();
//     println!("before receive");
//     thread::sleep(Duration::from_secs(3));
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }
// // ------------------------------------------

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             for _ in 1..1000000 {};
//             println!("child_thread: start sleep");
//             thread::sleep(Duration::from_secs(1));
//             println!("child_thread: end sleep");
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// // --------------------------------------

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use chrono::Utc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("thread_1: hi"),
            String::from("thread_1: from"),
            String::from("thread_1: the"),
            String::from("thread_1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("thread_2: more"),
            String::from("thread_2: messages"),
            String::from("thread_2: for"),
            String::from("thread_2: you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        let now = Utc::now();
        println!("{}: Got: {}", now, received);
    }

    // --snip--
}
// -------------------------------