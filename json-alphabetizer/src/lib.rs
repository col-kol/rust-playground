use std::fs;
use serde_json::{Result, Value};


pub fn run(filename: &str) -> Result<()> {
    // print file name
    println!("Filename: {:?}", filename);
    // read in the file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    println!("contents:\n{}", contents);
    let json: Value = serde_json::from_str(&contents)
        .expect("JSON is malformed");
    println!("Value:\n{}", json);
    Ok(())
}

pub fn parse_args(args: &[String]) -> &str {
    let filename = &args[1];
    
    filename
}


