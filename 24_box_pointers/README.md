# Smart pointers: Box
Genrally speaking a pointer is a varible containing an address in memory i.e. it "refers to" or "points to" some data stored in memory. References (`&`) are the most common type of pointer which are just an address which is used to "borrow" a value. Smart pointers also contain an address but also include other metadata and capabilities. A `String` and a `Vec<t>` are both examples of smart pointers from the standard library.

## Box pointer
A `Box<T>` is a smart pointer used for allocating values on the Heap (rather than the Stack) and keeping an address to this part of the Heap on the Stack. This is handy for transferring the ownership of a large amount of data to a new owner without needing to make a copy of all this data. This is much more performant than copying all the data. It's also useful when you don't know the size of a variable at compile time. This makes it useful for needing a flexible amount of memory.

