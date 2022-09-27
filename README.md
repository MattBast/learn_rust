# Learn Rust
A directory where I'm following this introduction to Rust guide: https://doc.rust-lang.org/book/ch00-00-introduction.html. Each directory is a lesson and should be a runnable piece of code. Some useful commands to remember are:

1. Create a new project with Cargo.
```bash
cargo new <directory_name>
```

2. Build a test/debug version of the Rust code in the `src` directory.
```bash
cargo build
```

3. Run the executable produced by build.
```bash
./target/debug/<directory_name>
```

4. Build an optimised production version.
```bash
cargo build --release
```

5. Run the production executable produced by build.
```bash
./target/release/<directory_name>
```

6. Build (if the code has changed) and run the Rust code in the `src` directory.
```bash
cargo run
````

7. Check that the code in the `src` directory can be compiled (runs faster than `cargo build`).
```bash
cargo check
````


