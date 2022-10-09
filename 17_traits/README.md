# Traits
It is sometimes useful to state that one type shares behviour with another. For instance integers and floats share much of the same behaviour in that they are numbers that can have mathematical operations performed on them. As such it is not uncommon for these types to be used interchangebly in a function. We can define similar shared behaviours as traits in our generic types and structs.

Note that these traits tend to be defined in the `lib.rs` file. Rust programmes are limited to just one of these files.