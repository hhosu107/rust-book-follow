use std::env;

use minigrep::Config;

fn main() {
    // dbg!(args);
    // For now, ignore error handling.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
