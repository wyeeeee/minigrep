use crate::utils::config::Config;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&config.file_path).expect("Something went wrong reading the file");
    println!("With text:{}", contents);
    println!("With text:{}", config.query);
    Ok(())
}