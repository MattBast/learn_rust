# Strings
Strings are a little trickier to use in Rust as much of the lower level complexity that is involved in strings is exposed similar to how it is in C. Higher level languages like Python and Javascript handle much of this detail for the developer. The things to watch out for in Rust are:

- a strings characters cannot be accessed via an index, you must use a slice
- use the `format!` function to concatenate strings
- use `.chars()` to iterate over the characters in a string