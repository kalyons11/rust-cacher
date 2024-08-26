use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments have been provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    dbg!(contents);
    dbg!(config.query); // TODO use this later
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config =
            Config::build(&["minigrep".to_string(), "one".to_string(), "two".to_string()]).unwrap();
        assert_eq!(config.query, "one");
        assert_eq!(config.file_path, "two");
    }

    #[test]
    fn test_config_error() {
        let _config = Config::build(&["minigrep".to_string()])
            .expect_err("Testing handling of insufficient arguments");
    }
}
