use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    // the build method no longer takes a reference to a String vector so we'll
    // switch it to an Iterator type input
    pub fn build(
    	mut args: impl Iterator<Item=String>
    ) -> Result<Config, &'static str> {
        // use the next method to get the arguments instead of indexing a Vector.
        // The first item in the CLI args is the calling function (Rust) so we'll 
        // skip it.
        args.next();

        // the next item is the query string. Use next to get it and use match
        // to catch the error where the user hasn't included a query string
        let query = match args.next() {
        	Some(arg) => arg,
        	None => return Err("Didn't get a query string."),
        };


        // do the same for the file path
        let file_path = match args.next() {
        	Some(arg) => arg,
        	None => return Err("Didn't get a file path"),
        };

        // check if an environment variable is present to set the case
        // sensitivity.
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        // and finally check for the optional ignore case argument.
        // this time don't throw an error if it's not included.
        ignore_case = match args.next() {
        	Some(_arg) => true,
        	None => ignore_case,
        };
        
        return Ok(Config { query, file_path, ignore_case });
    }
}

// the run function reads a file and prints its contents. It will either return an
// empty tuple (the equivalent of a null return in Rust) or an error of something 
// goes wrong. Using the Box trait gives us flexibility on what type of error we 
// return
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
    	search_case_insensitive(&config.query, &contents)
    } else {
    	search(&config.query, &contents)
    };

    for line in results {
    	println!("{line}");
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	// swap the for loop for an iterator with a closure. The `lines()`
	// method generates an iterator where each item is a line of a
	// multi-line string. `filter()` is used to consume the iterator. 
	// `filter()` generates an iterator so collect is used to return
	// a Vector.
	return contents
		.lines()
		.filter(|line| line.contains(query))
		.collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();

	// do the same with the case insensitive search adding `to_lowercase()` to
	// the closure.
	return contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect();
}

 #[cfg(test)]
 mod tests {
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(
			vec!["safe, fast, productive."], 
		 	search(query, contents)
		);
	} 

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}