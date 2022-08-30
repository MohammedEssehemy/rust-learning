# Tokio

`Tokio` is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications. It gives the flexibility to target a wide range of systems, from large servers with dozens of cores to small embedded devices.

At a high level, `Tokio` provides a few major components:

    A multi-threaded runtime for executing asynchronous code.
    An asynchronous version of the standard library.
    A large ecosystem of libraries.

`Tokio` is designed for IO-bound applications where each individual task spends most of its time waiting for IO. If the only thing your application does is run computations in parallel, you should be using `rayon`. That said, it is still possible to "mix & match" if you need to do both.
