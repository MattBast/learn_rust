fn main() {
    // adding a i32 value to a box is a silly example due to its size.
    // so imagine the 5 value being a much larger variable like a Vec<T>
    // with a million items in it.
    let b = Box::new(5);
    println!("b: {}", b);
}
