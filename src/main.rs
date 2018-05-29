extern crate extFinder;

use std::env;
use std::process;
use extFinder::Config;


fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
            println!("Problem occured: {}", err);
            process::exit(1);
            });
    if let Err(e) = extFinder::run(config)
    {
        println!("Problem occured: {}", e);
    }
}

