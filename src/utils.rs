use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

pub fn get_env_args() -> io::Result<HashMap<String, String>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid number of arguments."))
    } else {
        let pattern = &args[1][..];
        let filename = &args[2][..];

        if filename.is_empty() {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid filename."))
        } else if !Path::new(filename).exists() {
            Err(io::Error::new(io::ErrorKind::NotFound, "File don't exists."))
        } else if pattern.is_empty() {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid pattern."))
        } else {
            let mut result: HashMap<String, String> = HashMap::new();

            result.insert("filename".to_string(), filename.to_string());
            result.insert("pattern".to_string(), pattern.to_string());

            Ok(result)
        }
    }

}
