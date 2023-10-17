use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments were provided.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {query, file_path, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    
    for line in result {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    
    result
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
    
    #[test]
    fn search_case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, contents));
    }
}