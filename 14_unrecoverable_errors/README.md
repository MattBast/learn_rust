# Unrecoverable errors
Sometimes there is an unhandled error that requires the programme crash and stop running. In Rust this is called `panic!` which despite its name is actually quite an ordered shut down that includes printing a failure message, cleaning up the data in each function and quitting. If you add something like the following to the `Cargo.toml` file you can `panic!` in abort mode which leaves the cleanup to the operating system:

```toml
[profile.release]
panic = 'abort'
```

`panic!` mode is the default response Rust has for unhandled errors. Or it can be explicitly called in the code if the developer wants the programme to quit.

## When to panic and when not to
By default we always want to give our code a chance to recover from an error. `panic!` should thus be considered a last resort in production ready code. Here are some guidelines on when it might be best to allow for a `panic!` instead of building in coce to recover from the error:

- **not production code:** when building prototypes, tests and example code
- **you know more than the compiler:** when we can ensure a function will pass but the compiler wouldn't understand way. For instance, knowing the IP address of the server is something the developer might know but the compiler wouldn't so the developer can be confident this function will never fail.
- **insecure or harmful behaviour:** if the programme (or user) starts to do something that is harmful or insecure sometimes the safest thing to do is to quit the programme entirely i.e. `panic!`.
- **bugs:** if there's a bug in the code it is almost always better to `panic!` rather than have the programme do something unexpected. The code needs to be fixed before it is allowed to do anything else.

It is of course annoying to have lots of error handling code padding every function in your code. Where appropriate Rust developers can lean a little on Rusts type system and compiler to catch errors during compilation instead of needing explicit error handling code. For instance, if a number in a function should never be negative the developer could use a `u32` type and rely on the compiler checking for negative values getting passed to the function. 

Going one step further we could define a custom type. Presume we need a number to be between 1 and 100 (like in lesson twos guessing game) and we don't want to have to write error handling everytime we work with a `u32` type assuming this logic. Creating a custom type like this means we only need to write the check once. We also get a potential performance boost from this approach as the compiler does the checking work at compile time rather than at runtime.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```


Situations like this are something developers should look to balance as they start writing code to handle exceptions. 