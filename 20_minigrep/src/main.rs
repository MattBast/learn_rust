use std::env;
use std::process;

// import the library crate
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // unwrap extracts a Config struct from the return if the function processed
    // correctly. If it returns an error the else function will run.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // when an error is returned print it to the command line with a preface.
        // using eprintln instead of println rediects this print to stderr instead
        // of stdout
        eprintln!("Problem parsing arguments: {err}");
        // and then stop the program without a panic and return the number 1
        // as the exit code
        process::exit(1);
    });

    // run returns either an error or nothing at all. In these cases we can use
    // the if let pattern to unwrap the error if we get one. Otherwise it will
    // just happily ignore the return value.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}