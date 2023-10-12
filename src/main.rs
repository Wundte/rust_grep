use std::{env, process};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments ({err})");
        process::exit(1);
    });
    
    run(config);
}

fn run(config: Config) {
    println!("running with: {}, {}", config.query, config.file_path);
    
    let content = fs::read_to_string(config.file_path).expect("Was unable to read from file");
    println!("with contents: {content}");
}

struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments were provided.")
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}