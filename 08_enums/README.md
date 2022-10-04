# Enums
Where structs allow developers to groups together common fields, enums allow developers to group together a set of related values. It's like a set of IP addresses where some are V4 and others are V6 and yet they still share many of the same attributes:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

