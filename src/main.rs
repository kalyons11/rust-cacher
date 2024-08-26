use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read to the file");

    dbg!(contents);
    dbg!(config.query); // TODO use this later
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments have been provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new(&["minigrep".to_string(), "one".to_string(), "two".to_string()]);
        assert_eq!(config.query, "one");
        assert_eq!(config.file_path, "two");
    }

    #[test]
    #[should_panic]
    fn test_config_panic() {
        let _config = Config::new(&["minigrep".to_string()]);
    }
}
