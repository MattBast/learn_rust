use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // try to open a file that doesn't exist. The file open function returns
    // a `Result<T, E>` enum. `T` is filled by a `std::fs::File` type which is
    // Rusts standard file handle while `E` is filled by a `std::io::Error` type.
    let greeting_file_result = File::open("hello.txt");

    // match statements are a good way to handle errors. Simply check for an `OK` which
    // tells us the statement succeeded or an `Err` to handle any error. To handle specific 
    // exceptions differently with a match statement we would need to start nesting match 
    // statements. We can use the `error.kind()` method on an error type variable to get 
    // more information about the error and respond to it's different types.
    let greeting_file = match greeting_file_result {
        // the ideal situation is that the file exists and its handle is returned
        Ok(file) => file,
        // otherwise we take a look at the type of error we received
        Err(error) => match error.kind() {
            // if the error is that the file doesn't exist, try to create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                // if we succeed in creating the file, return a file handle as if it existed
                // before
                Ok(file_create) => file_create,
                // if we fail to create the file though, panic with the unrecoverable error
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            // finally add a catch all for errors we're not prepared for and panic
            other_error => {
                panic!("An unrecoverable error occurred: {:?}", other_error);
            },
        },
    };

    // there are more concise ways of writing nested error handling using closures
    // and the unwrap_or_else method. Unwrap requires us to write an if, else statement
    // to respond to the possibility of the error it extracted from the files return. The
    // logic is exactly the same as the above match statement but should be a bit easier to 
    // read.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // an even cleaner way of handling errors is to use the methods built into `Result<T, E>`
    // method. `unwrap` is one such method that will either return the variable inside `Ok<T>`
    // if the function succeeded or call `panic!` if an error is returned
    let greeting_file = File::open("hello.text").unwrap();

    // `expect` is another such method that lets us pick the `panic!` message. `expect` will help you
    // debug a future error by givng you more contextual information on what has gone wrong.
    let greeting_file = File::open("hello.text")
        .expect("the file `hello.txt` should be included in this project");

    // it is worth noting that `unwrap` and `expect` are most appropriate when writing examples,
    // prototypes and tests. They always result in panic! when an error comes up which should be
    // considered a last resort when building robust, production-ready code.

    println!("{:?}", greeting_file);
}

// when writing a function you can return a Result enum with the T and E replaced
// with the data types you expect returned depending on if the function succeeds or
// fails. This enables the function to propagate it's error so the calling function
// can handle the error. If the function succeeds it will return an `OK` value containing
// a string.If it fails though it will return an `Err` containing an `io::Error` type.
fn read_username_from_file() -> Result<String, std::io::Error> {
    let greeting_file_result = File::open("hello.text");

    // start by seeing if the file can be opened. Explicitly call return if we get an
    // error here so the function ends and the error is propagated up.
    let mut username_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    // now try to convert the files contents to a string. As this match is the last
    // expression in the function its return value will be the functions returned value
    // so there's no need to call return explicitly here.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// once again, the match statements of the above function look a little too verbose
// and can make the code tricky to read. Rust provides a nice shortcut for this kind
// of code with the ? opertor. Be warned though that this can only be used with functions
// that return a `Result` enum, `Option` enum or other type from `FromResidual`.
fn cleaner_read_username_from_file() -> Result<String, std::io::Error> {
    // the ? operator at the end of this statement will do the same as the first match
    // expression in the above function. That is it will either return a file handler
    // and keep going with the function or stop the function and return an Err type.
    let mut username_file = File::open("hello.txt")?;

    // the same can also be used for the second match statement as well
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    // but this time we need to explicitly state what is going to be returned
    return Ok(username);
}


fn even_cleaner_read_username_from_file() -> Result<String, std::io::Error> {
    // this function could be made even shorter by chaining some of the function calls together
    // as so
    let mut username = String::new();
    let mut username_file = File::open("hello.txt").read_to_string(&mut username)?;
    return Ok(username);
}

fn imported_read_username_from_file() -> Result<String, io::Error> {
    // the very shortest way to do this though is to use pre-written code
    // from the standard library which has implemented this logic for us already in
    // the read_to_string method that both reads a file and parses its contents into
    // a string.
    fs::read_to_string("hello.txt");
}