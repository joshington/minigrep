/*
	==grep - globally search a regular expression and print------
	- grep searches a specified file for a specified string.
	- grep takes as its args afilename and astring. then it reads the file, finds lines in that file that contain
	- the string arg, and prints those lines
*/
//crate minigrep.

extern crate minigrep;

use std::env; //to enable minigrep to read the values of cmdline args we pass to it, we'll need a func 

use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect(); //we use the collect function to create many kinds of collections
	//we explicitly annotate the type of args to specify that we want a vector of strings.

	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);//will stop the program immediately and return the number that 
		//was passed as the exit status code.
	});

	
	if let Err(e) =  minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
