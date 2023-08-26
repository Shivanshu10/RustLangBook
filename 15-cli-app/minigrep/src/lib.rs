use std::error::Error;
use std::fs;
use std::env;

// () nothing
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Contents: {}", contents);

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    return result;
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if (args.len() < 3) {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config{ query, filename, case_sensitive});
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "duct";

        assert_eq!(vec!["duct"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "Duct";

        assert_eq!(vec!["Duct"], search_case_insensitive(query, contents));
    }
}
