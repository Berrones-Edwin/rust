use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
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
    let content_file =
        fs::read_to_string(config.file_path).expect("Should have been to read the file");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content_file)
    } else {
        search(&config.query, &content_file)
    };

    for line in results {
        println!("{line}");
    }

    // println!("With text:\n{content_file}");
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // pub fn search(query: &str, content: &str) -> Vec<&str> {

    let mut my_vector = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            // println!("{}", line);
            my_vector.push(line);
        }
    }
    my_vector
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut my_vector = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            my_vector.push(line)
        }
    }
    my_vector
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn case_insentive() {
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
