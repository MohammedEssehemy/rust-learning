# Patterns and Matching

## Usage

- `match` Arms

    ```rust
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
    ```

    One requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the match expression must be accounted for. One way to ensure you’ve covered every possibility is to have a catchall pattern for the last arm: for example, a variable name matching any value can never fail and thus covers every remaining case.

    A particular pattern _will match anything, but it never binds to a variable, so it’s often used in the last match arm. The_ pattern can be useful when you want to ignore any value not specified, for example.

- Conditional `if let` Expressions

- `while let` Conditional Loops

- `for` Loops

- `let` Statements

- Function Parameters

## Refutability

Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.

A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen. Match guards are useful for expressing more complex ideas than a pattern alone allows.
