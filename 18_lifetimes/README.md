# Lifetimes
All references (`&`) in Rust have lifetimes. That is they are dropped when their lifetime ends. By default a references lifetime is the scope of the expression it belongs in. This covers most use cases of references so a lot of the time we don't need to specify special lifetime rules. However there are times when we'll want to change the lifetime of a reference.

Take this generic example. We declare `r` which has a lifetime denoted by `'a`. It lives until the end of the main function. However it depends on `x` which is inside an inner scope. It has a lifetime of `'b` which ends when its scope ends. This makes `r` invalid and causes a `panic!` during compilation.
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

To fix this we would usually declare the `x` variable first so that it has a longer lifetime than `r`:
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Lifetimes are mostly used within functions whose parameters are references. They connect the lifetimes of the functions parameters and return values so that the return statement operates in a memory safe way.