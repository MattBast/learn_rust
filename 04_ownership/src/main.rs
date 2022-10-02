fn main() {
    let s1 = String::from("Hello");
    
    // this function takes ownership of the variable s1 so anything
    // we want to do with the variable must be done within the function.
    calculate_length(s1);
    // when this function ends, the variable will be dropped
    // and s1 cannot be used again unless re-declared
    
    let s2 = String::from("world!");

    // ownership of a variable can be transferred back by
    // returning it meaning the parent function can continue
    // using the variable. As a side note, you can return more 
    // than one variable using a tuple.
    let (s3, s3_len) = calculate_length_return(s2);
    // as the function returned a value, it did not get
    // dropped.
    println!("The length of \"{}\" is {}.", s3, s3_len);

    let s4 = String::from("Hello world!");

    // returning lots of values though can be tedious 
    // so finally we can use references to avoid variables
    // getting dropped.
    let s4_len = calculate_length_reference(&s4);
    // references just borrow a variables value instead
    // of taking ownership of it meaning there is nothing
    // to drop when the function ends.

    println!("The length of \"{}\" is {}.", s4, s4_len);
}

fn calculate_length(s: String) {
    println!("The length of \"{}\" is {}.", s, s.len());
}

fn calculate_length_return(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}

fn calculate_length_reference(s: &String) -> usize {
    return s.len();
}
