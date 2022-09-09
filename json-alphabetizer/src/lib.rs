use std::fs;
use std::error::Error;
use serde_json;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    // print file name
    println!("Filename: {:?}", filename);
    // read in the file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    //println!("\n{}", contents);
    alphabetize(contents);

    Ok(())
}

pub fn parse_args(args: &[String]) -> &str {
    // TODO: convert this from 1 to N json files
    let filename = &args[1];
    
    filename
}

pub fn alphabetize(data) {
    let people: Vec<String> = serde_json::from_str(data)?;
    println!("\n{}", people);
}
