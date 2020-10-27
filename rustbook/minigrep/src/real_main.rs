use std::env;
use std::fs;

// Responsiblities of main.rs - 
// Calling CLI parsing logic
// Setting up configurations
// Calling a `run` fcn in lib.rs
// handling an error for that run
fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = parse_config(&args);
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong!");

    println!("With text:\n{}", contents);
}


struct Config {
    query: String,
    filename: String,
}


impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}


//fn parse_config(args: &[String]) -> Config {
    //let query = args[1].clone();
    //let filename = args[2].clone();

    //Config { query, filename }
//}
