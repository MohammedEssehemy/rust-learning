# Structs

A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.

A struct’s name should describe the significance of the pieces of data being grouped together.

The struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

## Tuble Structs

You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

## Unit-Like Structs

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

