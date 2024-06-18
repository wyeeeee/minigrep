use std::env;
use std::process;
mod utils;
use utils::query::run;
use utils::config::Config;
fn main() {
    let config =  Config::new(env::args()).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(&config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}