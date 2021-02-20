use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use std::{env, io};

mod tests;
mod utils;

fn main() {
    let (pattern, filename) = utils::get_env_args();
    let mut lines = utils::read_lines(filename).unwrap();
    utils::display_lines_when_match(&mut lines, pattern);

    exit(0);
}
