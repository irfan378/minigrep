use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents from the file and if error occurs return an error
    let contents = fs::read_to_string(config.filename)?;

    //check wheather to use case_sensitive or not
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    // loop through the contents line by line and search for the string
    for line in results{
        // print the line
        println!("{}", line);
    }

    Ok(())
}

// create a struct for query and file
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Check the length of args
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Clone the second command-line argument and assign it to the `query` variable
        let query = args[1].clone();

        // Clone the third command-line argument and assign it to the `filename` variable
        let filename = args[2].clone();

        // use the env variable for setting case_sensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // return the query and filename
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // create a vector named results
    let mut results = Vec::new();
    // loop through  file contents line by line
    for line in contents.lines() {
        // check wheather the line contains the query
        if line.contains(query) {
            // if the line contains the query add the line to the results vector
            results.push(line)
        }
    }
    // return the results
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // pass the query variable and shadow the previous query variable and change the letters to lowercase
    let query = query.to_lowercase();
    // create a vector named results
    let mut results = Vec::new();
    // loop through  file contents line by line
    for line in contents.lines() {
        // first change the line to lowercase and then check wheather the line contains the query
        if line.to_lowercase().contains(&query) {
            // if the line contains the query add the line to the results vector
            results.push(line)
        }
    }
    // return the results
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        \nsafe,fast,productive
        pick three
        \nDuct tape
        ";

        assert_eq!(vec!["safe,fast,productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        \nsafe,fast,productive
        pick three
        \nTrust me.
        ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
