mod tests;
mod utils;

fn main() {
    let show_usage_and_exit = |e| {
        eprintln!("{:?}\nUsage: rgerp [pattern] [FILE]", e);
        std::process::exit(1);
    };

    match utils::get_env_args() {
        Ok(hm) => {
            let filename = hm.get("filename").unwrap();
            let pattern = hm.get("pattern").unwrap();

            let mut lines = utils::read_lines(filename).unwrap();
            utils::display_lines_when_match(&mut lines, pattern.as_str());

            std::process::exit(0);
        },
        Err(e) => show_usage_and_exit(e),
    };
}
