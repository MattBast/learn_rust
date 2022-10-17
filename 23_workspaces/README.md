# Workspaces
Much of the time a programme will be too large to comfortably fit inside a single crate. It is much better to split a programme into small interacting components and to organise them inside a workspace. When using a workspace there is one binary crate (a crate with a `main.rs` file), many library crates and a master `Cargo.toml` file that links all the crates together. It looks a little like this:

```
 ├── Cargo.lock
 ├── Cargo.toml
 ├── add_one
 │   ├── Cargo.toml
 │   └── src
 │       └── lib.rs
 ├── adder
 │   ├── Cargo.toml
 │   └── src
 │       └── main.rs
 └── target
```

All library crates need to be added to the binary creates `Cargo.toml` file as dependencies. These dependencies include the path to libraries `src/lib.rs` file.

External library dependencies need to be added to the library that uses them as well as the top level cargo file. If the external library is used in multiple crates the version listed in the top level cargo file will be used as a consistent version number for all of them.

Finally if you run tests from the top level directory in the workspace then the tests across all library crates will be run. Adding the `-p` flag and the name of a library crate will run only the tests from that crate. Like this:

```bash
cargo test -p add_one
```