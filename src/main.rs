use std::env;
use std::process;
use minigrep::Config;
fn main() {
    // Retrieve command-line arguments and collect them into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Get the args from the new function and if the args are not present throw an error and exit the program
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // call the run function and if any error occurs throw an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}

