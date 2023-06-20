use std::{collections::HashMap, error::Error, fs};

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(config.query.as_str(), contents.as_str())
    } else {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    };

    for (line, paragraph) in results {
        println!("{}. {}", line, paragraph);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, String)> {
    let mut results: HashMap<usize, String> = HashMap::new();

    contents.lines().enumerate().for_each(|(number, line)| {
        if line.contains(query) {
            results.insert(number + 1, line.to_string());
        }
    });

    let mut results = results.into_iter().collect::<Vec<_>>();
    results.sort_by(|a, b| a.0.cmp(&b.0));

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, String)> {
    let mut results: HashMap<usize, String> = HashMap::new();

    contents.lines().enumerate().for_each(|(number, line)| {
        if line.to_lowercase().contains(query) {
            results.insert(number + 1, line.to_string());
        }
    });

    let mut results = results.into_iter().collect::<Vec<_>>();
    results.sort_by(|a, b| a.0.cmp(&b.0));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_search_correctly_a_query_in_a_file() {
        let query = "duct";
        let contents = [
            "Rust:",
            "safe, fast, productive.",
            "Pick three.",
            "Duct tape.",
        ];

        let expected = [(2, contents[1].to_string())];

        assert_eq!(
            search(query, contents.join("\n").as_str()),
            expected,
            "Search failed"
        );
    }
}
