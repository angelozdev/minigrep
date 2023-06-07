use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(value) => value == "1",
            Err(_) => false,
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
