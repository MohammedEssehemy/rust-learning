# Fearless Concurrency

Concurrent programming, where different parts of a program execute independently, and parallel programming, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

- `Race conditions`, where threads are accessing data or resources in an inconsistent order
- `Deadlocks`, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- `Bugs` that happen only in certain situations and are hard to reproduce and fix reliably

Do not communicate by sharing memory; instead, share memory by communicating.
