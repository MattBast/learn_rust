# Closures and iterators
Closures and iterators are separate concepts but tend to work in concert together. They come mostly from functional languages and despite being abstract versions of functions and for loops respectively, they have no negative impct on a programs performance.

The code in the `src` section of this lesson is a re-fctored version of `20_minigrep` showing how minigrep could be improved with closures and iterators.

## Closures
Closures are anonymous functions that respond to the environment they're called in. That makes them sound more complex than they really are. They just need to return something and can use variables passed to them. Other than that they're functions without names. 

They are defined with two pipes (`||`). If a variable is passed to the closure it is placed inside the pipes. Any logic or function to the right of the pipes is what is returned by the closure. The below super simple closure takes `x` as a variable and returns the same x value without doing anything to it:

```rust
let example_closure = |x| x;
```

Which means when the closure variable is used like this it will just return the "hello" string and the integer 5:

```rust
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

## Iterators
Iterators are much like loops in that they iterate over a list of items one item at a time. They are declared lazily meaning they don't have to be used straight away. A method can be applied later to start iterating over the list.

All iterators use the `next()` method to get the next item in the list like so:

```rust
let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter();

assert_eq!(v1_iter.next(), Some(&1));
assert_eq!(v1_iter.next(), Some(&2));
assert_eq!(v1_iter.next(), Some(&3));
assert_eq!(v1_iter.next(), None);
```

Methods like `sum()` and `map()` can also be used to consume an iterator like so:

```rust
let v1 = vec![1, 2, 3];
let mut v1_iter = v1.iter();
let total: i32 = v1_iter.sum();
assert_eq!(total, 6)

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

The v2 example is also a good example of how closures work really well with closures.