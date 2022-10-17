// this is a enum representing an item in a linked list
enum List {
    // linked lists can contain either a value with aan address
    // to the next linked list item
    Cons(i32, Rc<List>),
    // or a Nil generic type telling us this is the last item in
    // the linked list
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // we create a new linked list with two items and an ending Nil node.
    // declaring this with a new reference counter enables it to be owned
    // by multiple owners.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    
    // using Rc::clone creates a reference clone of list a meaning
    // only a copy of the address to a is added here. If we'd used
    // the standard clone command a deep copy would have been made
    // which would have been costly.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}
