# Generics
There are times when you find yourself duplicating code because you need to perform the same function on variables of different types. Take this example where we want to find the largest integer or char in a vector (list). To obey Rusts type system we would need two functions to effectively do the same thing:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

This is where generics come in. By writing a function that takes a generic type we are able to combine these functions into one function which makes our code cleaner to read. Now you know this it's easier to understand what is going on with some of Rusts standard Enums like `Option` and `Result`:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Be warned though that using many different generic types can make code harder to read. If you find yourself using lots of generic types consider re-structuring the code to make it cleaner.

## Performance
You might think that using generics would slow down your code at runtime but it doesn't. This is achieved through monomorphization. This means that during compilation that Rust will go through the functions in reverse converting generic types into the concrete types that functions are called with or structs initialised at. This means the binary contains a copy of every generic type function but with each concrete type in the generic types place. So the code runs as if the developer had written it using concrete types.