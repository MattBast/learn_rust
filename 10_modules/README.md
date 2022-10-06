# Modules
As projects get bigger they will need to be split into multiple files of code for the sake of readability. All these files exist in the `src` directory and this directory must still contain a `main.rs` file that is the entrypoint of the programme. Other modules called "crates" can then be imported. These crates can in turn have sub-modules whose code files sit a directory with the same name. For example:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

In this example `main.rs` is the entrypoint and `garden.rs` is a module. `vegetables.rs` is a sub-module of garden so sits in the `garden` directory.

Note: All modules are set to private mode by default. To make use of them, set them to public mode in the code.