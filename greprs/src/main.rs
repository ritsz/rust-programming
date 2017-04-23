extern crate greprs;

use std::env;
use std::process;
use greprs::Config;

fn main() {
    /*
    for x in env::args() {
        println!(".. {:?}", x);
    }
    */
    let args: Vec<String> = env::args().collect();

    /*
       Cannot move out of indexed content.
       Will have to use ref or &.
       let filename = args[1];
     */

    /* Unwrap of handle the error using closure */
    let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
            });
    
    println!("{:?}", config);
    
    /* Pattern matching on return data */
    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

