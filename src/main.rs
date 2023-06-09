use std::{env, process};

use minigrep::{config::Config, initialize::run};

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
