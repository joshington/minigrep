use std::env;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;



pub struct Config {
	pub query:String,
	pub filename: String,
	pub case_sensitive: bool,
}



impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			panic!("not enough arguments");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		//==the env::var function returns a Result that will be the successful Ok variant that contains
		//the value of the environment variable if the envt variable is set.It will return the Err
		//variant if the envt variable is not set.
		//we are using the is_err mthd on the Result to check whether its an error and therefore unset
		//which means it should do acase-sensitive search.if the CASE_SENSITIVE envt variable is set
		//to anything, is_err will return false and the program will perform a case-insensitive search.

		Ok(Config  { query, filename, case_sensitive })
		//==we pass the value in the case_sensitive variable to the Config instance so the run 
		//function can read  the value and decide whether to call search or search_case_insensitive
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let mut f = File::open(config.filename)?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	//we need to call search from our run function, we need to pass the config.query value and the contents
	//that run reads from the file to the search func. then print each line returned from search

	//==we have added the case_sensitive field that holds a boolean.
	//we need the run function to check  the case_sensitive field's value and use that to 
	//decide whether to call the search function or the search_case_insensitive function.
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
	for line in results {
		println!("{}", line);
	}
	//for line in search(&config.query, &contents) {
	//	println!("{}", line);
	//}
	Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";
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

//==rust has a helpful mthd to handle line-by-line iteration of strings
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	//==storing matching lines - we need a way to store the lines that contain our query string.

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);

		}
	}
	results
	//storing the lines that match so we can return them.
}

/*
	1 - iterate through each line of the contents
	2 - check whether the line contains our query string.
	3 - if it does, add it to the list of values we're returning.
	4 - if it doesnt, do nothing.
	5 - return the list of results that match
*/


//====implementing the search case insensitive 
/*
	difference here is that we'll lowercase the query and each line so whatever the case of the 
	input args, they'll be the same case when we check whether the line contains the query.
*/

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase(); //query is now a String rather than a string slice
//because calling to_lowercase creates new data rather than referencing existing data.
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	results
}