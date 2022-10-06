fn main() {
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    // concatenate strings using format
    let sentence = format!("{} {} {}", tic, tac, toe);
    println!("The combined strings are: {}", sentence);

    // use slicing rather than indexing to access characters and substrings
    println!("The first word of sentence is: {}", &sentence[0..3]);

    // use `.chars()` to iterate over a strings characters
    println!("Starting the print of the sentences characters.");
    for c in sentence.chars() {
        println!("{}", c);
    }

    // use `.bytes()` to iterate over a strings bytes
    println!("Starting the print of the sentences bytes.");
    for b in sentence.bytes() {
        println!("{}", b);
    }
}
