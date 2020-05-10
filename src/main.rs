use std::env;
use std::process;
use minigrep::Config;
// use minigrep::Config;


fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
    // let contents = fs::read_to_string(config.filename).expect("Unable to read file");
    // println!("With text:\n{}", contents);
}

