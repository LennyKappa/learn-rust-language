use std::fs;
use std::error::Error;
use std::env;


pub struct Config{
	pub query: String,
	pub filename: Option<String>,
	pub casesense: bool,
}

impl Config {
	pub fn new(args: std::env::Args) -> Result<Config,&'static str> {
		let mut args = args.peekable();
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("No query string.")
		};

		//let filename = args.peek().map(|_| {args});

		let filename = args.next().map(|file| {file.clone()});

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

	   	Ok(Config{query: query, filename: filename, casesense: case_sensitive})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.unwrap())?;

    let result = if config.casesense {
    	search(&config.query, &contents)
    } else {
    	search_ins(&config.query, &contents)
    };

    for line in result {
    	println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &'a str,text: &'a str) -> Vec<&'a str>{

	text.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_ins<'a>(query: &str,text: &'a str) -> Vec<&'a str>{
	let query = &query.to_lowercase();
	text.lines()
		.filter(|line| line.to_lowercase().contains(query))
		.collect()
	
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn try_it(){
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
	fn try_it_ins(){
		let query = "RuSt";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
	assert_eq!(
		vec!["Rust:","Trust me."],
        search_ins(query, contents)
		);
	}
}