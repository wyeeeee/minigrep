use crate::utils::config::Config;
pub fn parse(args:&[String]) -> Result<Config, &'static str>{
    if args.len() < 3{
        return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    
    Ok(Config {query,file_path})
}
