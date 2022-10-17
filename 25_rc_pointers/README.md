# Smart pointers: Reference Counted (rc)
Reference Counted or `rc` pointers are used to a keep track of values that have multiple owners. Usually values only have one owner like a variable. When this variable goes out of scope the value is automatically dropped. If multiple variables own a value we don't want the value to be dropped until all the variables go out of scope. By default, one variable going out of scope would cause the value to be dropped.

**Note** that `rc` is only to be used in single-threaded scenarios. In multi-threaded programmes there is another way to do reference counting which is discussed in a later lesson.

Take this example of a linked list where list `a` starts with value 5 and continues to 10 and Nil. List `b` starts with 3 and then continues into list `a`s values. List `c` does the same as list `b` except it begins with the value 4. So lists `b` and `c` both own list `a`.

```
b --> 3, -
          |
    a ---> --> 5, --> 10, --> Nil
          |
c --> 4, -
```

In this example we've kinda created a super basic graph database where two nodes have a relationship to a third node.

**Note** that `rc` is for reading only. You cannot mutate the original value from one of the multiple owners as that would violate Rusts borrowing rules.