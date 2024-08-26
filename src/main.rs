use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read to the file");

    dbg!(contents);
    dbg!(config.query); // TODO use this later
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments have been provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config =
            Config::new(&["minigrep".to_string(), "one".to_string(), "two".to_string()]).unwrap();
        assert_eq!(config.query, "one");
        assert_eq!(config.file_path, "two");
    }

    #[test]
    fn test_config_error() {
        let _config = Config::new(&["minigrep".to_string()])
            .expect_err("Testing handling of insufficient arguments");
    }
}
