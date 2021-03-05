use std::{env, process};
use rgrep::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(emsg) = run(&config) {
        eprintln!("Application error: {}", emsg);
        process::exit(1);
    }
}