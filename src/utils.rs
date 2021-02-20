use std::fs::File;
use std::{io, env};
use std::io::BufRead;
use std::path::Path;
use std::process;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn display_lines_when_match(lines: &mut io::Lines<io::BufReader<File>>, pattern: &str) {
    for line_r in lines {
        if let Ok(line) = line_r {
            if line.contains(pattern) {
                println!("{}", line)
            }
        }
    }
}

pub fn get_env_args() -> (&'static str, &'static str) {
    let show_usage_and_exit = || {
        eprintln!("Usage: rgerp [pattern] [FILE]");
        process::exit(1);
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        show_usage_and_exit();
    }

    let pattern = args[1].trim();
    let filename = args[2].trim();

    if filename.is_empty() || pattern.is_empty() {
        show_usage_and_exit();
    }

    return (pattern, filename);
}
