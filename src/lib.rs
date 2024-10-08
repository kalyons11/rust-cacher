use std::error::Error;
use std::{env, fs};

mod cache;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments have been provided");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path.clone())?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results.iter() {
        println!("{line}")
    }

    let key = &format!("{0}/{1}", config.query, config.file_path);
    let value = &results.len().to_string();
    println!("Setting {key} to {value}");
    cache::set(key, value)?;
    let value = cache::get(key)?;
    println!("Got {value} for {key}.");
    println!("Incrementing {key}");
    cache::incr(key)?;
    let value_incr = cache::get(key)?;
    println!("Got {value_incr} for {key} after incrementing.");
    println!("Decrementing {key}");
    cache::decr(key)?;
    let value_decr = cache::get(key)?;
    println!("Got {value_decr} for {key} after decrementing.");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config =
            Config::build(&["cacher".to_string(), "one".to_string(), "two".to_string()]).unwrap();
        assert_eq!(config.query, "one");
        assert_eq!(config.file_path, "two");
    }

    #[test]
    fn test_config_error() {
        let _config = Config::build(&["cacher".to_string()])
            .expect_err("Testing handling of insufficient arguments");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
