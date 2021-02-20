use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        show_usage_and_exit();
    }

    let filename = args[1].trim().to_string();
    let pattern = args[2].trim().to_string();

    if filename.is_empty() || pattern.is_empty() {
        show_usage_and_exit();
    }

    let mut lines = read_lines(filename).unwrap();
    display_lines_when_match(&mut lines, pattern.as_str());

    exit(0);
}

fn show_usage_and_exit() {
    eprintln!("Usage: rgerp [FILE] [pattern]");
    exit(1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn display_lines_when_match(lines: &mut io::Lines<io::BufReader<File>>, pattern: &str) {
    for line_r in lines {
        if let Ok(line) = line_r {
            if line.contains(pattern) {
                println!("{}", line)
            }
        }
    }
}
