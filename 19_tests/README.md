# Automated testing
Rust has an automated test library built-in and ready to use. It seems to work primarily on library crates which can be created as such:

```bash
cargo new tests --lib
```

Then just run:
```bash
cargo test
```

This will run all the code marked with the test decorator `#[test]`. It will run these tests on multiple threads meaning they run concurrently and thus faster. Be wary of this as this concurrency means that tests cannot share state, data or common resources.