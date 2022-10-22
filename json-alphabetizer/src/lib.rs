use std::fs;
<<<<<<< HEAD
use serde_json::{Result, Value};
=======
use std::error::Error;
use serde_json;
>>>>>>> 163482db80fecd0768b54dd67e8e1f25f03765a3


pub fn run(filename: &str) -> Result<()> {
    // print file name
    println!("Filename: {:?}", filename);
    // read in the file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
<<<<<<< HEAD
    
    println!("contents:\n{}", contents);
    let json: Value = serde_json::from_str(&contents)
        .expect("JSON is malformed");
    println!("Value:\n{}", json);
=======
    //println!("\n{}", contents);
    alphabetize(contents);

>>>>>>> 163482db80fecd0768b54dd67e8e1f25f03765a3
    Ok(())
}

pub fn parse_args(args: &[String]) -> &str {
    let filename = &args[1];
    
    filename
}

pub fn alphabetize(data) {
    let people: Vec<String> = serde_json::from_str(data)?;
    println!("\n{}", people);
}
