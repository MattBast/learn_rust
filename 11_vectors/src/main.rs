fn main() {
    // initialise an empty vector. Vectors must be set to mutable
    // if you'd like to add data to it later
    let mut v1: Vec<i32> = Vec::new();
    
    // initialise a vector with some initial values
    let mut v2: Vec<i32> = vec![1, 2, 3];

    // add some new values to the vector
    v1.push(10);
    v1.push(11);
    v1.push(12);

    // access vector elements with a reference and an index
    println!("The first value of v2 is {}.", &v2[0]);

    // Rust will panic and crash if you try to access an index
    // that doesn't exist. Match with the option (Some) enum 
    // is a safe way to check that the index exists
    let third_elem: Option<&i32> = v2.get(3);
    match third_elem {
        Some(third_elem) => println!("The first value of v2 is {}.", third_elem),
        _ => println!("There is no third element in v2."),
    }

    // we can also access a vectors elements through a for loop
    // we must use a reference of the vector for this
    println!("Starting a loop for v1");
    for v in &v1 {
        println!("{}", v);
    }

    // we can also edit the values in the loop by making the reference mutable
    println!("Starting a mutable loop for v2");
    for v in &mut v2 {
        *v *= 50;
        println!("{}", *v);
    }

    // enums are a good workaround for using multiple data types in a vector as
    // the enums own type will remain consistent in every vector element
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let spreadsheet_row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("Hello world")),
        SpreadsheetCell::Float(2.2),
    ];

    for cell in &spreadsheet_row {
        println!("{:?}", cell);
    }
}
