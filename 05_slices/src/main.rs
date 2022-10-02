fn main() {
    let s = String::from("Hello world");
    let s_len = s.len();

    // slices can be used to extract the prefix or suffix of a word.
    println!("The first three letters of \"{}\" are \"{}\".", s, &s[..3]);
    println!("The last three letters of \"{}\" are \"{}\".", s, &s[s_len-3..]);

    // in this function a slice is used to extract a strings first word
    // by getting the index of the first whitespace characters and then
    // slicing on that index
    let first_word = first_word(&s);
    println!("The first word of \"{}\" is \"{}\".", s, first_word);
}

// &str a string slice type is signified by `&str`
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}