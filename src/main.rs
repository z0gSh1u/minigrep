use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone(); // the string to search
    let filename = args[2].clone(); // the file to search in

    Config { query, filename }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: minigrep <query> <filename>");
    }
    let config = parse_args(&args);

    // read the file
    let file_content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
}
