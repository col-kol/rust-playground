use std::fs;
use serde_json::{Result, Value};

pub fn run(filename: &str) -> Result<()> {
    // print file name
    println!("Filename: {:?}", filename);
    // read in the file
    let json_file = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // println!("json_file:\n{}", json_file);
    let json: Value = serde_json::from_str(&json_file)
        .expect("JSON is malformed");
    println!("Value:\n{}", json);

    Ok(())
}

pub fn parse_args(args: &[String]) -> &str {
    let filename = &args[1];
    
    filename
}
