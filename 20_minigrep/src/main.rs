use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    // unwrap extracts a Config struct from the return if the function processed
    // correctly. If it returns an error the else function will run.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // when an error is returned print a message
        println!("Problem parsing arguments: {err}");
        // and then stop the program without a panic and return the number 1
        // as the exit code
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
         .expect("Should have been able to read the file");
     println!("With text:\n{contents}");
}

// the query substring anf the filepath will always be used together
// so it makes sense that they exist within the same struct. This also
// makes it easier to modularise this setup functionality.
struct Config {
    query: String,
    file_path: String,
}


impl Config {
    // we can then use an initialising function to make sure the arguments
    // passed on the CLI are good. Returning a `Result` enum enables us to
    // propagate the error message up to the calling function.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // add error handling to allow for the app to fail more elegantly
        if args.len() < 3 {
            // return an Err to present an unhappy path to the calling function
            return Err("Less than three arguments were passed. Make sure to include a query and a filepath.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // returning an Ok value fits the happy path return into the Result enum
        return Ok(Config { query, file_path });
    }
}