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

## Types of tests

There are two types of tests that Rust supports: **unit** and **integration**.

### Unit tests
These test individual functions, methods and structs inside a library. These tests are written inside the `src/lib.rs` file and support module files. At compile time any code marked with the `#[test]` decorator will not be included in the built binary.

### Integration tests
These tests test only the functions and types publically exposed by the library for other modules to use. The tests then are testing if these functions work end to end including any sub-functions they call. These tests belong in directory called `tests` and will import any of the functions they need to test. At compile time any files in the `tests` directory will be ignored.


When you run this command both unit and integration tests are run:
```bash
cargo test
```

To run just the integration tests run this:
```bash
cargo test --test <name_of_test_file>
```