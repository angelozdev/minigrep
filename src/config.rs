use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip the first argument (program name)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

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
