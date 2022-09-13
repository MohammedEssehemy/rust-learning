# Fearless Concurrency

Concurrent programming, where different parts of a program execute independently, and parallel programming, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

- `Race conditions`, where threads are accessing data or resources in an inconsistent order
- `Deadlocks`, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- `Bugs` that happen only in certain situations and are hard to reproduce and fix reliably

Do not communicate by sharing memory; instead, share memory by communicating.

You might then wonder why all primitive types aren’t atomic and why standard library types aren’t implemented to use `Arc<T>` by default. The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to. If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.

`Mutex<T>` provides interior mutability, as the Cell family does. In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`

Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came with the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Any type composed entirely of Send types is automatically marked as Send as well. Almost all primitive types are Send, aside from raw pointers.

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread. Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.

Async runtimes are libraries used for executing async applications. Runtimes usually bundle together a reactor with one or more executors. Reactors provide subscription mechanisms for external events, like async I/O, interprocess communication, and timers. In an async runtime, subscribers are typically futures representing low-level I/O operations. Executors handle the scheduling and execution of tasks. They keep track of running and suspended tasks, poll futures to completion, and wake tasks when they can make progress. The word "executor" is frequently used interchangeably with "runtime".

For more info, check [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html)
