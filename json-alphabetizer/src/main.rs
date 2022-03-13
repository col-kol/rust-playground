use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    let json_file_name = json_alphabetizer::parse_args(&args);

    if let Err(result) = json_alphabetizer::run(json_file_name) {
        println!("Application error: {}", result);
        process::exit(1);
    }
}

