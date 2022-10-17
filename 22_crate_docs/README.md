# Crates.io documentation
When a crate is shared via "Crates.io" it will need good documentation if anyone is to use it effectively. This can be achieved with "documentation comments" in the code. Much like you can comment code with two salshes `\\`, you can add documentation with three slashes `\\\`. These three slashes accept markdown formatting. 

When the following command is run the docuumentation comments will be transformed into a HTML document which is placed in the `target` directory and which is then opened in the browser:

```bash
cargo doc --open
```

To build but not open the docs, omit the `--open` flag.