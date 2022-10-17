use std::env;
use std::process;

// import the library crate
use iterators::Config;

fn main() {
    // this time pass the args iterator directly into the config build method
    // instead of needing to save it to a variable. This also saves us having
    // to collect the iterator contents ahead of building the config struct.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = iterators::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}