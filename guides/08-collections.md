# Collections

Rust’s standard library includes a number of very useful data structures called collections. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.


- A vector allows you to store a variable number of values of the same type.
- A string is a collection of characters.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## String VS str

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. string slices are references to some `UTF-8` encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, `UTF-8` encoded string type. 

Strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling `UTF-8` data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving `non-ASCII` characters later in your development life cycle.

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.