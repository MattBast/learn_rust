use std::fs;
use std::error::Error;

// the query substring and the filepath will always be used together
// so it makes sense that they exist within the same struct. This also
// makes it easier to modularise this setup functionality.
pub struct Config {
    pub query: String,
    pub file_path: String,
}


impl Config {
    // we can then use an initialising function to make sure the arguments
    // passed on the CLI are good. Returning a `Result` enum enables us to
    // propagate the error message up to the calling function.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

// the run function reads a file and prints its contents. It will either return an
// empty tuple (the equivalent of a null return in Rust) or an error of something 
// goes wrong. Using the Box trait gives us flexibility on what type of error we 
// return
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // adding the ? suffix adds a shortcut to propagate any errors up to the 
    // calling function
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    return Ok(());
}