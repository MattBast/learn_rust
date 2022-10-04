# Match control flows
Match control flows are as their name suggests, a type of control flow. They are particularly well suited to working with enums as they can be used to detect the enums value and to trigger an action specific to the type. Match flows are like extended if, else statements but in a more concise format. Rust travels down each option in the match statement and stops travelling down until it reaches a condition that passes (returns true). For example:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```