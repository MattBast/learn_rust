# Unrecoverable errors
SOmetimes there is an unhandled error that requires the programme crash and stop running. In Rust this is called `panic!` which despite its name is actually quite an ordered shut down that includes printing a failure message, cleaning up the data in each function and quitting. If you add something like the following to the `Cargo.toml` file you can `panic!` in abort mode which leaves the cleanup to the operating system:

```toml
[profile.release]
panic = 'abort'
```

`panic!` mode is the default response Rust has for unhandled errors. Or it can be explicitly called in the code if the developer wants the programme to quit.