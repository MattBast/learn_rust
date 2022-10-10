fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// without lifetime parameters this function will fail as the compiler
// can't work out if it will be x or y that is returned. Instinctively
// both references are dropped due to the ownership rules of the compiler.
// To fix this we define the lifetime parameter 'a. This binds the lifetimes
// of the funtions return value and parameters together effectively forcing
// x and y to live as long as the entire function does.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    else {
        return y;
    }
}