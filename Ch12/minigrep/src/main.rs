use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

#[allow(dead_code)]
#[allow(unused_variables)]


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
    	eprintln!("Problem processing arguements: {}", err);
    	process::exit(1);
    });
    
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

