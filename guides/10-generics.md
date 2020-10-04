# Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, functions can take parameters of some generic type instead of a concrete type, like `i32` or `String`. 

We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. **When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.**

## Performance of Code Using Generics

You might be wondering whether there is a runtime cost when you’re using generic type parameters. The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.


## Traits: Defining Shared Behavior

A `trait` tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use `trait bounds` to specify that a generic can be any type that has certain behavior.

By using a `trait bound` with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified `traits`. 

We can also conditionally implement a `trait` for any type that implements another `trait`. Implementations of a trait on any type that satisfies the `trait bounds` are called `blanket implementations` and are extensively used in the Rust standard library. 

`Traits` and `trait bounds` let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the `trait bound` information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t implement the type which defines the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

Another kind of generic that we’ve already been using is called `lifetimes`. Rather than ensuring that a type has the behavior we want, `lifetimes` ensure that references are valid as long as we need them to be.

## Validating References with Lifetimes

Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.